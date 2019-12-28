use std::io::{self, BufRead};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
enum SourceParam {
    Immediate(i64),
    Position(i64),
    Relative(i64),
}

impl fmt::Display for SourceParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Immediate(val) => write!(f, "{:>6}", val),
            //Self::Position(addr) => write!(f, "[{:>4}]", addr),
            Self::Position(addr) => addr_nickname(addr).map(|name| write!(f, "{:>6}", name)).unwrap_or_else(|| write!(f, "[{:>4}]", addr)),
            Self::Relative(addr) => write!(f, "[{:>4} + base]", addr),
        }
    }
}

#[derive(Debug)]
enum DestParam {
    Position(i64),
    Relative(i64),
}


impl fmt::Display for DestParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            //Self::Position(addr) => write!(f, "[{:>4}]", addr),
            Self::Position(addr) => addr_nickname(addr).map(|name| write!(f, "{:>6}", name)).unwrap_or_else(|| write!(f, "[{:>4}]", addr)),
            Self::Relative(addr) => write!(f, "[{:>4} + base]", addr),
        }
    }
}

enum Param {
    Source(SourceParam),
    Dest(DestParam),
}

#[derive(Debug)]
struct OpAdd {
    a: SourceParam,
    b: SourceParam,
    dest: DestParam,
}

impl fmt::Display for OpAdd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "add {:>6} {:>6} => {:>6}", self.a, self.b, self.dest)
    }
}

#[derive(Debug)]
struct OpMul {
    a: SourceParam,
    b: SourceParam,
    dest: DestParam,
}

impl fmt::Display for OpMul {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mul {:>6} {:>6} => {:>6}", self.a, self.b, self.dest)
    }
}

#[derive(Debug)]
struct OpIn {
    dest: DestParam,
}

impl fmt::Display for OpIn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "in  {:>6}", self.dest)
    }
}

#[derive(Debug)]
struct OpOut {
    val: SourceParam,
}

impl fmt::Display for OpOut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "out {:>6}", self.val)
    }
}

// jump if argument is not zero
#[derive(Debug)]
struct OpJnz {
    src: SourceParam,
    addr: SourceParam,
}

impl fmt::Display for OpJnz {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "jnz {:>6} {:>6}", self.src, self.addr)
    }
}

// jump if argument is zero
#[derive(Debug)]
struct OpJz {
    src: SourceParam,
    addr: SourceParam,
}

impl fmt::Display for OpJz {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "jz  {:>6} {:>6}", self.src, self.addr)
    }
}

#[derive(Debug)]
struct OpLess {
    a: SourceParam,
    b: SourceParam,
    dest: DestParam,
}

impl fmt::Display for OpLess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "lt  {:>6} {:>6} => {:>6}", self.a, self.b, self.dest)
    }
}

#[derive(Debug)]
struct OpEqual {
    a: SourceParam,
    b: SourceParam,
    dest: DestParam,
}

impl fmt::Display for OpEqual {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "eq  {:>6} {:>6} => {:>6}", self.a, self.b, self.dest)
    }
}

#[derive(Debug)]
struct OpBase {
    val: SourceParam,
}

impl fmt::Display for OpBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "base {:>6}", self.val)
    }
}

#[derive(Debug)]
enum Instruction {
    Add(OpAdd),
    Mul(OpMul),
    In(OpIn),
    Out(OpOut),
    Jnz(OpJnz),
    Jz(OpJz),
    Lt(OpLess),
    Eq(OpEqual),
    Base(OpBase),
    Stop,
}
use Instruction::*;

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Add(op)  => write!(f, "{}", op),
            Mul(op)  => write!(f, "{}", op),
            In(op)   => write!(f, "{}", op),
            Out(op)  => write!(f, "{}", op),
            Jnz(op)  => write!(f, "{}", op),
            Jz(op)   => write!(f, "{}", op),
            Lt(op)   => write!(f, "{}", op),
            Eq(op)   => write!(f, "{}", op),
            Base(op) => write!(f, "{}", op),
            Stop     => write!(f, "stop"),
        }
    }
}

fn step(program: &[i64], ip: usize) -> Option<(usize, Instruction, Option<usize>)> {
    let opcode = program[ip] % 100;
    if opcode == 99 {
        return Some((ip + 1, Instruction::Stop, None));
    }

    let mode0 = program[ip] / 100 % 10;
    let mode1 = program[ip] / 1000 % 10;
    let mode2 = program[ip] / 10000 % 10;
    assert!(mode0 <= 2);
    assert!(mode1 <= 2);
    assert!(mode2 <= 2);

    let immflags = [mode0 == 1, mode1 == 1, mode2 == 1];
    let relflags = [mode0 == 2, mode1 == 2, mode2 == 2];

    let simm = |x| SourceParam::Immediate(program[ip + 1 + x]);
    let spos = |x| SourceParam::Position(program[ip + 1 + x]);
    let srel = |x| SourceParam::Relative(program[ip + 1 + x]);

    let dpos = |x| DestParam::Position(program[ip + 1 + x]);
    let drel = |x| DestParam::Relative(program[ip + 1 + x]);

    let input = |x: usize| {
        assert!(!(immflags[x] && relflags[x]));
        if immflags[x] { simm(x) }
        else if relflags[x] { srel(x) }
        else { spos(x) }
    };

    let output = |x: usize| {
        assert!(!immflags[x]);
        if relflags[x] { drel(x) }
        else { dpos(x) }
    };

    let val1_num = ||
        if immflags[1] {
            Some(program[ip + 2] as usize)
        } else if relflags[1] {
            println!("warning! indir rel jump"); None
        } else {
            println!("warning! indir jump"); None
        };

    match opcode {
        1 => { Some((
                    ip + 4,
                    Instruction::Add(OpAdd {
                        a: input(0),
                        b: input(1),
                        dest: output(2),
                    }),
                    None))
        },

        2 => { Some((
                    ip + 4,
                    Instruction::Mul(OpMul {
                        a: input(0),
                        b: input(1),
                        dest: output(2),
                    }),
                    None))
        },

        3 => { Some((
                    ip + 2,
                    Instruction::In(OpIn {
                        dest: output(0),
                    }),
                    None))
        },

        4 => { Some((
                    ip + 2,
                    Instruction::Out(OpOut {
                        val: input(0),
                    }),
                    None))
        },

        5 => { Some((
                    ip + 3,
                    Instruction::Jnz(OpJnz {
                        src: input(0),
                        addr: input(1),
                    }),
                    val1_num()))
        },

        6 => { Some((
                    ip + 3,
                    Instruction::Jz(OpJz {
                        src: input(0),
                        addr: input(1),
                    }),
                    val1_num()))
        },

        7 => { Some((
                    ip + 4,
                    Instruction::Lt(OpLess {
                        a: input(0),
                        b: input(1),
                        dest: output(2),
                    }),
                    None))
        },

        8 => { Some((
                    ip + 4,
                    Instruction::Eq(OpEqual {
                        a: input(0),
                        b: input(1),
                        dest: output(2),
                    }),
                    None))
        },

        9 => { Some((
                    ip + 2,
                    Instruction::Base(OpBase {
                        val: input(0),
                    }),
                    None))
        },

        _ => None // probably a data section after the program
    }
}

// newtype to not mix up with array indices
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd)]
struct ProgAddr(usize);

impl ProgAddr {
    fn value(&self) -> usize {
        self.0
    }
}

#[derive(Debug)]
struct AsmRow {
    ip: ProgAddr,
    next_ip: ProgAddr,
    instruction: Instruction,
    jump: Option<ProgAddr>,
}

fn execute(program: &[i64]) -> (Vec<AsmRow>, HashMap<ProgAddr, Vec<ProgAddr>>) {
    let mut ip = 0;
    let mut asm = Vec::new();
    let mut refs = HashMap::new();

    while let Some((next_ip, instruction, jump)) = step(program, ip) {
        asm.push(AsmRow {
            ip: ProgAddr(ip),
            next_ip: ProgAddr(next_ip),
            instruction,
            jump: jump.map(ProgAddr),
        });
        if let Some(dest) = jump {
            refs.entry(ProgAddr(dest)).or_insert(Vec::new()).push(ProgAddr(ip));
        }
        ip = next_ip;
    }

    (asm, refs)
}

#[derive(Debug, PartialEq)]
struct BasicBlock {
    // rows only aren't enough because jumps use actual addresses
    top: ProgAddr,
    bottom: ProgAddr,
    source_rows: Option<(usize, usize)>
}

impl BasicBlock {
    fn new(top: ProgAddr, bottom: ProgAddr, source_rows: (usize, usize)) -> Self {
        BasicBlock {
            top,
            bottom,
            source_rows: Some(source_rows),
        }
    }

    fn new_sentinel(addr: ProgAddr) -> Self {
        BasicBlock {
            top: addr,
            bottom: addr,
            source_rows: None,
        }
    }

    fn is_sentinel(&self) -> bool {
        self.source_rows.is_none()
    }
}

type Blocks = HashMap<ProgAddr, BasicBlock>;
type BlockEdge = (Option<ProgAddr>, Option<ProgAddr>);
type BlockEdges = HashMap<ProgAddr, BlockEdge>;

fn read_blocks(asm: &[AsmRow]) -> (Blocks, BlockEdges) {

    // begin -> (begin, end, (beginrow idx in asm, endrow idx in asm) or None if last sentinel)
    let mut bbs: Blocks = Blocks::new();
    // two edges: ip -> (direct fallthrough ip, branching ip if any) using the starting addr as bb identifier
    let mut bb_edges: BlockEdges = BlockEdges::new();

    // bb start data
    let mut current = ProgAddr(0);
    let mut current_i = 0;

    for (i, row) in asm.iter().enumerate() {
        // a basic block ends at a jump instruction
        if let Some(jumpdest) = row.jump {
            // - bbs:
            // o bb at newip will get created next in this loop
            // o bb at jumpdest will get created below if jumpdest isn't a normal beginning of a bb
            // - edges:
            // o newip becomes valid next in this loop (last instruction has a sentinel)
            // o jumpdest becomes valid on a future iteration or during the split
            // o the direct edge resulting from the split will get created then
            // o the split top half has only one edge
            // x (TODO does this work if the bb jumps into itself?)
            bbs.insert(current, BasicBlock::new(current, row.ip, (current_i, i)));
            bb_edges.insert(current, (Some(row.next_ip), Some(jumpdest)));
            current = row.next_ip;
            current_i = i + 1;
        }
    }
    // last bb must end with a jump
    assert!(current == asm.last().unwrap().next_ip);
    // insert sentinel end node for the last continuation
    bbs.insert(current, BasicBlock::new_sentinel(current));

    (bbs, bb_edges)
}

fn split_bbs(asm: &[AsmRow], refs: &HashMap<ProgAddr, Vec<ProgAddr>>,
             bbs: &mut Blocks, bb_edges: &mut BlockEdges) {
    let row_by_addr: HashMap<ProgAddr, &AsmRow> = asm.iter().map(|row| (row.ip, row)).collect();

    for &entrypoint in refs.keys() {
        if bbs.contains_key(&entrypoint) {
            // boring
            continue;
        }

        // we're in the middle of a bb; look up for its top
        let mut found = false;
        for addr in (0..entrypoint.0).rev() {
            let addr = ProgAddr(addr);

            if let Some(orig_bb) = bbs.get_mut(&addr) {
                // split orig_bb such that another starts at entrypoint
                // [origbegin, entrypoint - entrypoint_prev_instruction.size] and [entrypoint, orig_end]
                let (rowi, topbb_last_row) = asm.iter().enumerate()
                    .find(|(_i, row)| row.next_ip == entrypoint).unwrap(); // or row_by_addr - 1?
                let before_ep = topbb_last_row.ip;
                // (beginaddr, endaddr, (beginrow idx in asm, endrow idx in asm))
                // no jump from the top, just the fallthrough
                let top_bb = BasicBlock::new(orig_bb.top, before_ep, (orig_bb.source_rows.unwrap().0, rowi));
                // jumps from the bottom stay same
                let bottom_bb = BasicBlock::new(entrypoint, orig_bb.bottom, (rowi + 1, orig_bb.source_rows.unwrap().1));
                // top at the same address replaces orig, bottom is newly inserted

                // fallthrough edge only
                let old_top_edge = bb_edges.insert(orig_bb.top, (Some(entrypoint), None));
                assert_eq!(old_top_edge,
                           Some((Some(row_by_addr[&bottom_bb.bottom].next_ip), row_by_addr[&bottom_bb.bottom].jump)));
                *orig_bb = top_bb;

                // destination
                let prev_at_ep = bbs.insert(entrypoint, bottom_bb);
                assert_eq!(prev_at_ep, None); // if there was one, this split wouldn't have happened
                bb_edges.insert(entrypoint, old_top_edge.unwrap());
                found = true;
                break;
            }
        }
        assert!(found);
    }
}

// note that if the program is self-modifying and alters these operands, then the missing edges
// will be very confusing in the diagram. Let's hope that it a) doesn't happen or b) will be
// detectable.
fn cut_edges(asm: &[AsmRow], _refs: &HashMap<ProgAddr, Vec<ProgAddr>>,
             bbs: &mut Blocks, bb_edges: &mut BlockEdges) {
    for bb in bbs.values() {
        if let Some(rows) = bb.source_rows {
            let row = &asm[rows.1];
            if let Jnz(op) = &row.instruction {
                if let SourceParam::Immediate(val) = op.src {
                    let mut edge = bb_edges.get_mut(&bb.top).unwrap();
                    if val != 0 {
                        // jump always taken
                        edge.0 = None;
                    } else {
                        // jump never taken
                        edge.1 = None;
                    }
                }
            } else if let Jz(op) = &row.instruction {
                if let SourceParam::Immediate(val) = op.src {
                    let mut edge = bb_edges.get_mut(&bb.top).unwrap();
                    if val == 0 {
                        // jump always taken
                        edge.0 = None;
                    } else {
                        // jump never taken
                        edge.1 = None;
                    }
                }
            } else if let Stop = &row.instruction {
                let mut edge = bb_edges.get_mut(&bb.top).unwrap();
                let sentinel = bbs.values().find(|bb| bb.is_sentinel()).unwrap();
                edge.0 = Some(sentinel.top);
            }
        }
    }
}

fn build_bbs(asm: &[AsmRow], refs: &HashMap<ProgAddr, Vec<ProgAddr>>) -> (Blocks, BlockEdges) {
    let (mut bbs, mut bb_edges) = read_blocks(asm);

    // some jumps might go to the middle of a bb; do another pass, split such bbs in two by
    // looking at each address some other jump refers to
    split_bbs(asm, refs, &mut bbs, &mut bb_edges);

    cut_edges(asm, refs, &mut bbs, &mut bb_edges);

    (bbs, bb_edges)
}

fn inst_edge_hint(inst: &Instruction, is_jump: bool) -> String {
    match inst {
        Jnz(OpJnz { src: SourceParam::Immediate(_), .. })  => "".to_string(),
        Jnz(OpJnz { src, .. }) if  is_jump => format!("({} != 0)", src),
        Jnz(OpJnz { src, .. }) if !is_jump => format!("({} == 0)", src),

        Jz(OpJz { src: SourceParam::Immediate(_), .. })  => "".to_string(),
        Jz(OpJz { src, .. }) if  is_jump => format!("({} == 0)", src),
        Jz(OpJz { src, .. }) if !is_jump => format!("({} != 0)", src),
        _ => "".to_string(),
    }
}

fn print_edgedesc(asm: &[AsmRow], bbs: &Blocks, frombb: &BasicBlock, nextopt: Option<ProgAddr>, title: &str, is_jump: bool) {
    if let Some(next) = nextopt {
        let nextbb = &bbs[&next];
        let rows = frombb.source_rows.unwrap();
        let row = &asm[rows.1];
        let hint = inst_edge_hint(&row.instruction, is_jump);
        println!("L{}_{} -> L{}_{} [label=\"{}\\n{}\"]",
                 frombb.top.value(), frombb.bottom.value(),
                 (nextbb.top).value(), (nextbb.bottom).value(),
                 title, hint);
    }

}

fn addr_nickname(addr: i64) -> Option<&'static str> {
    match addr {
        1033 => Some("r_in"),
        1032 => Some("r0"),
        1034 => Some("r1"),
        1035 => Some("r2"),
        1036 => Some("r3"),
        1037 => Some("r4"),
        1038 => Some("r5"),
        1039 => Some("r6"),
        1040 => Some("r7"),
        1041 => Some("r8"),
        1042 => Some("r9"),
        1043 => Some("ra"),
        1044 => Some("rb"),
        _ => None,
    }
}

fn nicknames(param: Param) -> Option<&'static str> {
    match param {
        Param::Source(SourceParam::Relative(addr)) => addr_nickname(addr),
        Param::Dest(DestParam::Relative(addr)) => addr_nickname(addr),
        _ => None,
    }
}

fn graphviz(_program: &[i64], asm: &[AsmRow], refs: &HashMap<ProgAddr, Vec<ProgAddr>>) {
    println!("digraph G {{");
    println!("node [shape=box, fontname=monospace]");
    println!();

    let (bbs, bb_edges) = build_bbs(asm, refs);

    // sort the starting addresses that the names start with for consistent bbs; graphviz cares
    // about the input order
    let mut bb_addrs = bbs.keys().copied().collect::<Vec<ProgAddr>>();
    bb_addrs.sort();

    // dump bb names first for easier debugging of output

    for &bb_addr in &bb_addrs {
        let bb = &bbs[&bb_addr];
        let label = if let Some(coords) = bb.source_rows {
            let first_row = coords.0;
            let last_row = coords.1;
            let strings = asm[first_row..=last_row].iter()
                .map(|row| format!("{:05}: {}", row.ip.0, row.instruction)).collect::<Vec<_>>();
            strings.join("\\l")
        } else {
            // the sentinel node has its own bb too, the last instruction may advance to it
            "END".to_string()
        };
        println!("L{}_{} [label=\"{}\\l\"]", bb.top.value(), bb.bottom.value(), label);
    }

    println!();

    // make the start bb easier to spot
    let first_bb = &bbs[&ProgAddr(0)];
    println!("BOOT -> L{}_{}", first_bb.top.value(), first_bb.bottom.value());

    for &from in &bb_addrs {
        let bb = &bbs[&from];
        if bb.is_sentinel() {
            // XXX: sentinel edge for end?
            continue;
        }

        let &(contopt, jumpopt) = &bb_edges[&from];
        let frombb = &bbs[&from];
        print_edgedesc(asm, &bbs, frombb, contopt, "fall", false);
        print_edgedesc(asm, &bbs, frombb, jumpopt, "jump", true);
    }

    println!("}}");
}

fn rawasm(program: &[i64], asm: &[AsmRow], refs: &HashMap<ProgAddr, Vec<ProgAddr>>) {
    for row in asm {
        let raw_numbs = format!("{:>4?}", &program[row.ip.value()..row.next_ip.value()]);
        let jumpfrom = &match refs.get(&row.ip) {
            // hmm, could &Vec<ProcAddr> be cast into &Vec<usize> for printing? unsafe only?
            Some(sources) => format!("{:>4?}", sources.iter().map(|x| x.value()).collect::<Vec<_>>()),
            None => "|".to_string(),
        };

        // HACK: stringize instruction first, then pad that
        println!("#{:05}: {:<25} {:<30}{}", row.ip.value(), raw_numbs, format!("{}", row.instruction), jumpfrom);
    }

    // "data segment"
    let mut ip = asm.last().unwrap().next_ip.0;
    while ip < program.len() {
        let right = if ip + 10 < program.len() { ip + 10 } else { program.len() };
        println!("#{:05}: {:>4?}", ip, &program[ip..right]);
        ip += 10;
    }
}

fn analyze(program: &[i64]) {
    let (asm, refs) = execute(program);

    if true {
        rawasm(program, &asm, &refs);
    }

    if true {
        graphviz(program, &asm, &refs);
    }
}

fn main() {
    let program: Vec<i64> = io::stdin().lock().lines().next().unwrap().unwrap()
        .split(',').map(|n| n.parse().unwrap()).collect();

    analyze(&program);
}
