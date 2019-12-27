use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Debug)]
enum SourceParam {
    Immediate(i64),
    Position(i64),
    Relative(i64),
}

#[derive(Debug)]
enum DestParam {
    Position(i64),
    Relative(i64),
}

#[derive(Debug)]
struct OpAdd {
    a: SourceParam,
    b: SourceParam,
    dest: DestParam,
}

#[derive(Debug)]
struct OpMul {
    a: SourceParam,
    b: SourceParam,
    dest: DestParam,
}

#[derive(Debug)]
struct OpIn {
    dest: DestParam,
}

#[derive(Debug)]
struct OpOut {
    val: SourceParam,
}

// jump if argument is not zero
#[derive(Debug)]
struct OpJnz {
    src: SourceParam,
    addr: SourceParam,
}

// jump if argument is zero
#[derive(Debug)]
struct OpJz {
    src: SourceParam,
    addr: SourceParam,
}

#[derive(Debug)]
struct OpGreater {
    a: SourceParam,
    b: SourceParam,
    dest: DestParam,
}

#[derive(Debug)]
struct OpEqual {
    a: SourceParam,
    b: SourceParam,
    dest: DestParam,
}

#[derive(Debug)]
struct OpBase {
    val: SourceParam,
}

#[derive(Debug)]
enum Instruction {
    Add(OpAdd),
    Mul(OpMul),
    In(OpIn),
    Out(OpOut),
    Jnz(OpJnz),
    Jz(OpJz),
    Gt(OpGreater),
    Eq(OpEqual),
    Base(OpBase),
    Stop,
}
use Instruction::*;

fn step(program: &[i64], ip: usize) -> Option<(usize, String, Instruction, Option<usize>)> {
    let opcode = program[ip] % 100;
    if opcode == 99 {
        return Some((ip + 1, "stop".to_string(), Instruction::Stop, None));
    }
    let mode0 = program[ip] / 100 % 10;
    let mode1 = program[ip] / 1000 % 10;
    let mode2 = program[ip] / 10000 % 10;
    assert!(mode0 <= 2);
    assert!(mode1 <= 2);
    assert!(mode2 <= 2);
    let immflags = (mode0 == 1, mode1 == 1, mode2 == 1);
    let relflags = (mode0 == 2, mode1 == 2, mode2 == 2);

    let rel0 = || if relflags.0 { " + base" } else { "" };
    let rel1 = || if relflags.1 { " + base" } else { "" };
    let rel2 = || if relflags.2 { " + base" } else { "" };
    let imm0 = || format!("{}", program[ip + 1]);
    let imm1 = || format!("{}", program[ip + 2]);
    let imm2 = || format!("{}", program[ip + 3]);
    let val0 = || if immflags.0 { imm0() } else { format!("[{:>4}{}]", imm0(), rel0()) };
    let val1 = || if immflags.1 { imm1() } else { format!("[{:>4}{}]", imm1(), rel1()) };
    let val2 = || if immflags.2 { imm2() } else { format!("[{:>4}{}]", imm2(), rel2()) };
    let mut0 = || { assert!(!immflags.0); val0() };
    let mut2 = || { assert!(!immflags.2); val2() };

    let imm_inst = |x| SourceParam::Immediate(program[ip + 1 + x]);
    let pos_inst = |x| SourceParam::Position(program[ip + 1 + x]);
    let rel_inst = |x| SourceParam::Relative(program[ip + 1 + x]);

    let dpos_inst = |x| DestParam::Position(program[ip + 1 + x]);
    let drel_inst = |x| DestParam::Relative(program[ip + 1 + x]);

    let val0_inst = || if immflags.0 { imm_inst(0) } else if relflags.0 { rel_inst(0) } else { pos_inst(0) };
    let val1_inst = || if immflags.1 { imm_inst(1) } else if relflags.1 { rel_inst(1) } else { pos_inst(1) };

    let mut0_inst = || { assert!(!immflags.0); if relflags.0 { drel_inst(0) } else { dpos_inst(0) } };
    let mut2_inst = || { assert!(!immflags.2); if relflags.2 { drel_inst(2) } else { dpos_inst(2) } };

    let imm1_num = || program[ip + 2];
    let val1_num = ||
        if immflags.1 {
            Some(imm1_num() as usize)
        } else if relflags.1 {
            println!("warning! indir rel jump"); None
        } else {
            println!("warning! indir jump"); None
        };

    match opcode {
        1 => { Some((
                    ip + 4,
                    format!("add {:>6} {:>6} => {:>6}", val0(), val1(), mut2()),
                    Instruction::Add(OpAdd {
                        a: val0_inst(),
                        b: val1_inst(),
                        dest: mut2_inst(),
                    }),
                    None))
        },
        2 => { Some((
                    ip + 4,
                    format!("mul {:>6} {:>6} => {:>6}", val0(), val1(), mut2()),
                    Instruction::Mul(OpMul {
                        a: val0_inst(),
                        b: val1_inst(),
                        dest: mut2_inst(),
                    }),
                    None))
        },
        3 => { Some((
                    ip + 2,
                    format!("in  {:>6}", mut0()),
                    Instruction::In(OpIn {
                        dest: mut0_inst(),
                    }),
                    None))
        }
        4 => { Some((
                    ip + 2,
                    format!("out {:>6}", val0()),
                    Instruction::Out(OpOut {
                        val: val0_inst(),
                    }),
                    None))
        },
        5 => { Some((
                    ip + 3,
                    format!("jnz {:>6} {:>6}", val0(), val1()),
                    Instruction::Jnz(OpJnz {
                        src: val0_inst(),
                        addr: val1_inst(),
                    }),
                    val1_num()))
        },
        6 => { Some((
                    ip + 3,
                    format!("jz  {:>6} {:>6}", val0(), val1()),
                    Instruction::Jz(OpJz {
                        src: val0_inst(),
                        addr: val1_inst(),
                    }),
                    val1_num()))
        },
        7 => { Some((
                    ip + 4,
                    format!("gt  {:>6} {:>6} => {:>6}", val0(), val1(), mut2()),
                    Instruction::Gt(OpGreater {
                        a: val0_inst(),
                        b: val1_inst(),
                        dest: mut2_inst(),
                    }),
                    None))
        },
        8 => { Some((
                    ip + 4,
                    format!("eq  {:>6} {:>6} => {:>6}", val0(), val1(), mut2()),
                    Instruction::Eq(OpEqual {
                        a: val0_inst(),
                        b: val1_inst(),
                        dest: mut2_inst(),
                    }),
                    None))
        },
        9 => { Some((
                    ip + 2,
                    format!("base {:>6}", val0()),
                    Instruction::Base(OpBase {
                        val: val0_inst(),
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
    description: String,
    instruction: Instruction,
    jump: Option<ProgAddr>,
}

fn execute(program: &[i64]) -> (Vec<AsmRow>, HashMap<ProgAddr, Vec<ProgAddr>>) {
    let mut ip = 0;
    let mut asm = Vec::new();
    let mut refs = HashMap::new();

    while let Some((next_ip, description, instruction, jump)) = step(program, ip) {
        asm.push(AsmRow {
            ip: ProgAddr(ip),
            next_ip: ProgAddr(next_ip),
            description,
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
type BlockEdge = (ProgAddr, Option<ProgAddr>);
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
            bb_edges.insert(current, (row.next_ip, Some(jumpdest)));
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
                let old_top_edge = bb_edges.insert(orig_bb.top, (entrypoint, None));
                assert_eq!(old_top_edge,
                           Some((row_by_addr[&bottom_bb.bottom].next_ip, row_by_addr[&bottom_bb.bottom].jump)));
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

fn build_bbs(asm: &[AsmRow], refs: &HashMap<ProgAddr, Vec<ProgAddr>>) -> (Blocks, BlockEdges) {
    let (mut bbs, mut bb_edges) = read_blocks(asm);

    // some jumps might go to the middle of a bb; do another pass, split such bbs in two by
    // looking at each address some other jump refers to
    split_bbs(asm, refs, &mut bbs, &mut bb_edges);

    (bbs, bb_edges)
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
                .map(|row| format!("{:05}: {}", row.ip.0, row.description)).collect::<Vec<_>>();
            strings.join("\\l")
        } else {
            // the sentinel node has its own bb too
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
        let &(cont, jumpopt) = &bb_edges[&from];
        let frombb = &bbs[&from];
        let contbb = &bbs[&cont];
        println!("L{}_{} -> L{}_{} [label=\"fall\"]", frombb.top.value(), frombb.bottom.value(),
            (contbb.top).value(), (contbb.bottom).value());
        if let Some(jump) = jumpopt {
            let jumpbb = &bbs[&jump];
            println!("L{}_{} -> L{}_{} [label=\"jmp\"]", frombb.top.value(), frombb.bottom.value(),
                jumpbb.top.value(), jumpbb.bottom.value());
        }
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

        println!("{:05}: {:<25} {:<30}{}", row.ip.value(), raw_numbs, row.description, jumpfrom);
    }

    // "data segment"
    let mut ip = asm.last().unwrap().next_ip.0;
    while ip < program.len() {
        let right = if ip + 10 < program.len() { ip + 10 } else { program.len() };
        println!("{:05}: {:>4?}", ip, &program[ip..right]);
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
