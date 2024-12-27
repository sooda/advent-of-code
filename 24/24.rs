#![feature(adt_const_params, let_chains, impl_trait_in_assoc_type)]

use std::io::{self, Read};
use std::collections::HashMap;
use std::marker::{PhantomData, ConstParamTy};
use std::iter;
use std::ops::{BitOr, Add};

#[derive(Debug, PartialEq, Eq, Clone, Copy, ConstParamTy)]
enum GateOp {
    And,
    Or,
    Xor
}

#[derive(Debug, Clone)]
struct Gate {
    op: GateOp,
    in_a: String,
    in_b: String,
    out: String,
}

type States = HashMap<String, bool>;

struct Device {
    states: States,
    gates: Vec<Gate>,
}

fn z_number(states: &States) -> u64 {
    let mut z = 0;
    for (k, &v) in states {
        if v && let Some((_, n)) = k.split_once('z') {
            let n = n.parse::<u64>().unwrap();
            z |= 1 << n;
        }
    }
    z
}

// all your bit are belong to us
fn get_signal(signal: &str, gates: &[Gate], states: &mut States) -> bool {
    if let Some(&val) = states.get(signal) {
        val
    } else {
        let gate = gates.iter().find(|g| g.out == signal).unwrap();
        let a = get_signal(&gate.in_a, gates, states);
        let b = get_signal(&gate.in_b, gates, states);
        let val = match gate.op {
            GateOp::And => a && b,
            GateOp::Or => a || b,
            GateOp::Xor => (a || b) && a != b,
        };
        states.insert(signal.to_string(), val);
        val
    }
}

fn simulate_z(device: &Device) -> u64 {
    let mut states = device.states.clone();
    for g in &device.gates {
        get_signal(&g.out, &device.gates, &mut states);
    }
    z_number(&states)
}

fn xy_into_z(x: u64, y: u64) -> States {
    let mut states = States::new();
    for i in 0..64 {
        states.insert(format!("x{i:0>2}"), (x & (1 << i)) != 0);
        states.insert(format!("y{i:0>2}"), (y & (1 << i)) != 0);
    }
    states
}

// a verification node in the circuit: test if a wire matches expectations, then optionally chain
// into more nodes if any.
trait Node {
    fn test(init: &str, wire: &str, device: &Device) -> Option<String>;
}

// electric circuit whose integrity we're about to validate
struct Circuit<N: Node>(PhantomData<N>);

impl<N: Node> Circuit<N> {
    fn new(_n: N) -> Self { Self(PhantomData) }

    fn verify_t<'a>(device: &'a Device) -> impl Iterator<Item=String> + 'a where N: 'a {
        device.gates.iter()
            .flat_map(|g| {
                iter::once(&g.in_a)
                    .chain(iter::once(&g.in_b))
            })
            .filter_map(|wire| N::test(&wire, &wire, device))
    }

    fn verify<'a>(&self, device: &'a Device) -> impl Iterator<Item=String> + 'a where N: 'a  {
        Self::verify_t(device)
    }
}

// not a node really, just a way to terminate recursive types
#[derive(Copy, Clone)]
struct End;

// for each wire with name starting with "x"
#[derive(Copy, Clone)]
struct X<const A: i32, const B: i32, N: Node>(PhantomData<N>);

impl<const A: i32, const B: i32, N: Node> X<A, B, N> {
    fn new() -> Self { Self(PhantomData) }
}

// for at least one gate in the chain, this must hold
#[derive(Copy, Clone)]
struct Op<const O: GateOp, N: Node>(PhantomData<N>);

impl<const O: GateOp, N: Node> Op<O, N> {
    fn new() -> Self { Self(PhantomData) }
}

type Xor<N> = Op<{GateOp::Xor}, N>;
type Or<N> = Op<{GateOp::Or}, N>;
type And<N> = Op<{GateOp::And}, N>;

// for at least one wire, must be D diff from the initial wire name index
#[derive(Copy, Clone)]
struct Z<const D: i32>;

impl<const D: i32> Z<D> {
    fn new() -> Self { Self }
}

impl<const A: i32, const B: i32, N: Node> Node for X<A, B, N> {
    fn test(init: &str, wire: &str, device: &Device) -> Option<String> {
        // this monadish stuff is some serious gourmet shit
        // try each wire like "x00" or "x42"
        wire.split_once("x")
            .and_then(|(_, n)| n.parse::<i32>().ok())
            .filter(|&n| n >= A && n <= B)
            .and_then(|_| N::test(init, wire, device))
    }
}

impl<const O: GateOp, N: Node> Node for Op<O, N> {
    fn test(init: &str, wire: &str, device: &Device) -> Option<String> {
        // a map from names to gates would be great but this is enough for now
        let mut gates_match = device.gates.iter()
            .filter(|g| g.op == O && (g.in_a == wire || g.in_b == wire))
            .map(|g| &g.out)
            .peekable();
        if gates_match.peek().is_some() {
            // worked, find first error if any
            gates_match.filter_map(move |w| N::test(init, w, device))
                .next()
        } else {
            // no downstream wires, this must be bad
            Some(wire.to_string())
        }
    }
}

impl<const D: i32> Node for Z<D> {
    fn test(init: &str, wire: &str, _device: &Device) -> Option<String> {
        // is this too abused magic? iff z matches, we return None, else the wire is bad
        wire.split_once("z")
            .and_then(|(_, n)| n.parse::<i32>().ok())
            .filter(|&n| n - init.split_once("x").unwrap().1.parse::<i32>().unwrap() == D)
            .map_or_else(|| Some(wire.to_string()), |_| None)
    }
}

impl Node for End {
    fn test(_init: &str, wire: &str, _device: &Device) -> Option<String> {
        // a loose End should not be used on its own but replaced later with a real consumer
        Some(wire.to_string())
    }
}

// The above is good enough to write:
// Circuit::<X::<0, 0,  And::<                  Xor::<Z::<1>>>>>::verify_t(device);
// Circuit::<X::<1, 43, And::<             Or::<Xor::<Z::<1>>>>>>::verify_t(device);
//
// Let's make it more ergonomic though.
//
// join X<A,B,And<Xor<End>>> and Or<End>
// join And<Xor<End>> and Or<End>
// join Xor<End> and Or<End>
// join End and Or<End>
//   -> X<A,B,And<Xor<Or<End>>>>

// A node implementing Link<Rhs> can do self.join(rhs) to produce Self::Output where Rhs is
// appended at the last linked list level. Output::new() is typically good enough because the
// "values" (like r) of these types do not carry anything; this is just compile time business.
trait Link<Rhs: Node>: Node {
    type Output: Node;
    fn join(self, r: Rhs) -> Self::Output;
}

// produce a new X where Rhs is propagated to the child node
impl<R: Node, const A: i32, const B: i32, N: Link<R>> Link<R> for X<A, B, N> {
    type Output = X<A, B, <N as Link<R>>::Output>;
    fn join(self, _r: R) -> Self::Output {
        Self::Output::new() // no need to actually join the values; see above comment
    }
}

// produce a new Op where Rhs is propagated to the child node
impl<R: Node, const O: GateOp, N: Link<R>> Link<R> for Op<O, N> {
    type Output = Op<O, <N as Link<R>>::Output>;
    fn join(self, _r: R) -> Self::Output {
        Self::Output::new() // as above
    }
}

// join End with anything R, get R
impl<R: Node> Link<R> for End {
    type Output = R;
    fn join(self, r: R) -> R {
        r // Node doesn't need to implement new() though, so we do this
    }
}

// Save some typing. Boo, can't use function params as generic const args (yet?)
fn x<const A: i32, const B: i32>() -> X<A, B, End> {
    X::new()
}

fn z<const D: i32>() -> Z<D> {
    Z::new()
}

// Conveniently deduce N for Circuit at call site
fn verify_circuit<'a, N: Node + 'a>(n: N, device: &'a Device) -> impl Iterator<Item=String> + 'a {
    Circuit::new(n).verify(device)
}

// The above is good enough to write:
//
// let and = And::<End>::new();
// let xor = Xor::<End>::new();
// let or = Or::<End>::new();
// verify_circuit(x::<0, 0>().join(and).join(xor).join(z::<1>()), device);
// verify_circuit(x::<1, 43>().join(and).join(or).join(xor).join(z::<1>()), device);
//
// Let's make it more ergonomic though.

// This would avoid some copypasta (orphan rule) but require stuff like:
//   let or = BitOrWrapper(Or::<End>::new());
//   let xx = BitOrWrapper(x::<1, 44>());
//   xx | or; // produces another BitOrWrapper
// There's so few nodes that let's impl BitOr manually for them.
/*
struct BitOrWrapper<T: Node>(T);
impl<R: Node, L: Link<R>> BitOr<BitOrWrapper<R>> for BitOrWrapper<L> {
    type Output = BitOrWrapper<<L as Link<R>>::Output>;
    fn bitor(self, rhs: BitOrWrapper<R>) -> Self::Output {
        BitOrWrapper(self.0.join(rhs.0))
    }
}
*/

// a node implementing BitOr<Rhs> can do self | rhs to produce Self::Output
impl<R: Node, const A: i32, const B: i32, N: Link<R>> BitOr<R> for X<A, B, N> {
    type Output = <Self as Link<R>>::Output;
    fn bitor(self, r: R) -> Self::Output {
        self.join(r)
    }
}

impl<R: Node, const O: GateOp, N: Link<R>> BitOr<R> for Op<O, N> {
    type Output = <Self as Link<R>>::Output;
    fn bitor(self, r: R) -> Self::Output {
        self.join(r)
    }
}

// The above is good enough to write:
//
// verify_circuit(x::<0, 0>() | and | xor | z::<1>(), device);
// verify_circuit(x::<1, 43>() | and | or | xor | z::<1>(), device);
//
// Let's make it more ergonomic though.

// NOTE: with the easier pipe syntax, could make the object type tree work like:
// N<X<1,42>, N<And, N<Or, N<Xor, N<End>>>>>
// to decouple linked list node traversal and contents with a consistent node type

// proxy obj for triggering Circuit::run, may want to simplify Device eventually, and it is more
// explicit and readable at the call site to do: | verify(device) than: | device.
struct Verify<'a>(&'a Device);

// Verify(d) at the call site would also work but this abstracts implementation
fn verify(d: &Device) -> Verify {
    Verify(d)
}

// a bit lazy to have just X implement this, but it's the only sensible head for a final chain
// (besides Y of course, but that's redundant for now)
impl<'a, const A: i32, const B: i32, N: Node + 'a> BitOr<Verify<'a>> for X<A, B, N> {
    type Output = impl Iterator<Item=String> + 'a;
    fn bitor(self, r: Verify<'a>) -> Self::Output {
        verify_circuit(self, r.0)
    }
}

// The above is good enough to write:
//
// x::<0, 0>()  | and | xor | z::<1>() | verify(device);
// x::<1, 43>() | and | or | xor | z::<1>() | verify(device);
//
// Let's make it more ergonomic though.

// adapter for a Node chain or a (recursive) Group of two or more Node chains
trait Verifiable {
    fn verivisit<'a>(device: &'a Device) -> impl Iterator<Item=String> + 'a where Self: 'a;
}

#[derive(Copy, Clone)]
struct Group<V: Verifiable, W: Verifiable>(PhantomData<V>, PhantomData<W>);

impl<V: Verifiable, W: Verifiable> Group<V, W> {
    fn new() -> Self { Self(PhantomData, PhantomData) }
}

fn group<V: Verifiable, W: Verifiable>(_: V, _: W) -> Group<V, W> {
    Group::new()
}

// trivial terminal element, like verify_circuit() but with a type rather than value
impl<N: Node> Verifiable for N {
    fn verivisit<'a>(device: &'a Device) -> impl Iterator<Item=String> + 'a where Self: 'a {
        Circuit::<N>::verify_t(device)
    }
}

// a Group verifies a Node or a Group and then a Node or a Group; jazz music stops at a Node eventually
// all reports from each chain have to be returned, not just the first chain
impl<V: Verifiable, W: Verifiable> Verifiable for Group<V, W> {
    fn verivisit<'a>(device: &'a Device) -> impl Iterator<Item=String> + 'a where Self: 'a {
        V::verivisit(device).chain(W::verivisit(device))
    }
}

// like verify_circuit() but for groups
fn verify_group<'a, V: Verifiable, W: Verifiable>(_: Group<V, W>, device: &'a Device)
        -> impl Iterator<Item=String> + 'a where Group<V, W>: 'a {
    Group::<V, W>::verivisit(device)
}

// The above is good enough to write:
//
// let a = x::<0, 0>()  | and | xor | z::<1>();
// let b = x::<1, 43>() | and | or | xor | z::<1>();
// let _ = verify_group(group(a, b), device);
//
// Let's make it more ergonomic though.

// again this has to be implemented for concrete types because orphan rule
impl<R: Verifiable, const A: i32, const B: i32, N: Node> Add<R> for X<A, B, N> {
    type Output = Group<Self, R>;
    fn add(self, _r: R) -> Self::Output {
        Self::Output::new()
    }
}

// greatest desires: like Link::join to keep order consistent
//   Group<A, Group<B, C>> + D -- Group<A, Group<B, Group<C, D>>>
// however, iter type like behavior for now: ABC stays and Rhs joins from left
//   Group<C, Group<B, A>> + D -- Group<D, Group<C, Group<B, A>>>
// doesn't matter in practice though
impl<R: Verifiable, V: Verifiable, W: Verifiable> Add<R> for Group<V, W> {
    type Output = Group<R, Group<V, W>>;
    fn add(self, _r: R) -> Self::Output {
        Self::Output::new()
    }
}

// The above is good enough to write:
//
// let a = x::<0, 0>()  | and | xor | z::<1>();
// let b = x::<1, 43>() | and | or | xor | z::<1>();
// let _ = verify_group(a + b, device);
//
// Let's make it more ergonomic though.

impl<'a, V: Verifiable + 'a, W: Verifiable + 'a> BitOr<Verify<'a>> for Group<V, W> {
    type Output = impl Iterator<Item=String> + 'a;
    fn bitor(self, r: Verify<'a>) -> Self::Output {
        verify_group(self, r.0)
    }
}

// The above is good enough to write:
//
// let a = x::<0, 0>()  | and | xor | z::<1>();
// let b = x::<1, 43>() | and | or | xor | z::<1>();
// let _ = a + b | verify(device);
//
// Let's make it more flexible though. TODO: coming soon:
//
// - verify(device, algo_validate);
// - verify(device, algo_visit);
//
// to see that all wires were visited by the validation chains.

fn swapped_pairs(device: &Device) -> String {
    if false {
        // just a smoke test, not conclusive
        for ai in 0..45 {
            for bi in 0..45 {
                let a = 1 << ai;
                let b = 1 << bi;
                let z = a + b;
                let states = xy_into_z(a, b);
                let zz = simulate_z(&Device { states, gates: device.gates.clone() });
                if zz != z {
                    println!("bad {ai:>2} {bi:>2} -- 0x{a:>13x} + 0x{b:>13x} -- 0x{zz:x}");
                }
            }
        }
    }

    // v1:

    if true {
        // this also works but too much turbofish
        let _first_xz     = Circuit::<X::<0, 0,  Xor::<                        Z::<0>>>>::verify_t(device);
        let _xz           = Circuit::<X::<1, 44, Xor::<                  Xor::<Z::<0>>>>>::verify_t(device);
        let _first_carry  = Circuit::<X::<0, 0,  And::<                  Xor::<Z::<1>>>>>::verify_t(device);
        let _carry        = Circuit::<X::<1, 43, And::<             Or::<Xor::<Z::<1>>>>>>::verify_t(device);
        let _first2_carry = Circuit::<X::<0, 0,  And::<       And::<Or::<Xor::<Z::<2>>>>>>>::verify_t(device);
        let _carry2       = Circuit::<X::<1, 42, And::<  Or::<And::<Or::<Xor::<Z::<2>>>>>>>>::verify_t(device);
    }

    // v2:

    // "and" is shorter than "and()", hence no functions
    let and = And::<End>::new();
    let xor = Xor::<End>::new();
    let or = Or::<End>::new();

    let _ = verify_circuit(x::<0, 0>().join(and).join(xor).join(z::<1>()), device);
    let _ = verify_circuit(x::<1, 43>().join(and).join(or).join(xor).join(z::<1>()), device);

    // v3:

    let _ = verify_circuit(x::<0, 0>()  | and | xor | z::<1>(), device);
    let _ = verify_circuit(x::<1, 43>() | and | or | xor | z::<1>(), device);

    // v4:

    let _ = x::<0, 0>()  | and | xor | z::<1>() | verify(device);
    let _ = x::<1, 43>() | and | or | xor | z::<1>() | verify(device);

    // v5:

    let a = x::<0, 0>()  | and | xor | z::<1>();
    let b = x::<1, 43>() | and | or | xor | z::<1>();
    let _ = verify_group(group(a, b), device);

    // v6:

    let a = x::<0, 0>()  | and | xor | z::<1>();
    let b = x::<1, 43>() | and | or | xor | z::<1>();
    let _ = verify_group(a + b, device);

    // v7:

    let mut bad_wires = if true {
        // x00, y00 to z00
        let first_xz     = x::<0, 0>()  | xor                       | z::<0>();
        // xNN, yNN to zNN for NN > 0
        let xz           = x::<1, 44>() | xor |                 xor | z::<0>();
        // x00, y00 to z01
        let first_carry  = x::<0, 0>()  | and |                 xor | z::<1>();
        // xNN, yNN to zMM for MM - NN == 1
        let carry        = x::<1, 43>() | and |            or | xor | z::<1>();
        // x00, y00 to z02
        let first2_carry = x::<0, 0>()  | and |      and | or | xor | z::<2>();
        // xNN, yNN to zMM for MM - NN == 2
        let carry2       = x::<1, 42>() | and | or | and | or | xor | z::<2>();

        let errors = first_xz + xz + first_carry + carry + first2_carry + carry2 | verify(device);
        errors.collect::<Vec<_>>()
    } else {
        // previous form for historical reference

        // x00, y00 to z00
        let first_xz     = x::<0, 0>()  | xor                       | z::<0>() | verify(device);
        // xNN, yNN to zNN for NN > 0
        let xz           = x::<1, 44>() | xor |                 xor | z::<0>() | verify(device);
        // x00, y00 to z01
        let first_carry  = x::<0, 0>()  | and |                 xor | z::<1>() | verify(device);
        // xNN, yNN to zMM for MM - NN == 1
        let carry        = x::<1, 43>() | and |            or | xor | z::<1>() | verify(device);
        // x00, y00 to z02
        let first2_carry = x::<0, 0>()  | and |      and | or | xor | z::<2>() | verify(device);
        // xNN, yNN to zMM for MM - NN == 2
        let carry2       = x::<1, 42>() | and | or | and | or | xor | z::<2>() | verify(device);

        first_xz
            .chain(xz)
            .chain(first_carry)
            .chain(carry)
            .chain(first2_carry)
            .chain(carry2)
            .collect::<Vec<_>>()
    };

    bad_wires.sort();
    bad_wires.dedup();
    bad_wires.join(",")
}

fn parse_gate(line: &str) -> Gate {
    // x00 AND y00 -> z00
    let mut words = line.split(' ');
    let in_a = words.next().unwrap().to_string();
    let op = match words.next().unwrap() {
        "AND" => GateOp::And,
        "OR" => GateOp::Or,
        "XOR" => GateOp::Xor,
        _ => panic!()
    };
    let in_b = words.next().unwrap().to_string();
    words.next().unwrap().to_string();
    let out = words.next().unwrap().to_string();
    Gate { op, in_a, in_b, out }
}

fn parse(file: &str) -> Device {
    let (states, gates) = file.split_once("\n\n").unwrap();
    let states = states.lines()
        .map(|l| l.split_once(": ").unwrap())
        .map(|(a, b)| (a.to_string(), b == "1"))
        .collect();
    let gates = gates.lines()
        .map(parse_gate)
        .collect();
    Device { states, gates }
}

fn main() {
    let mut file = String::new();
    io::stdin().read_to_string(&mut file).unwrap();
    let device = parse(&file);
    if false {
        println!("digraph G {{");
        for g in &device.gates {
            println!("{} [label={} shape=circle]", g.in_a, g.in_a);
            println!("{} [label={} shape=circle]", g.in_b, g.in_b);
            println!("{} [label={} shape=circle]", g.out, g.out);
            println!("{}{:?}{} [label={:?} shape=rect]", g.in_a, g.op, g.in_b, g.op);
            println!("{} -> {}{:?}{}", g.in_a, g.in_a, g.op, g.in_b);
            println!("{} -> {}{:?}{}", g.in_b, g.in_a, g.op, g.in_b);
            println!("{}{:?}{} -> {}", g.in_a, g.op, g.in_b, g.out);
        }
        println!("}}");
    } else {
        println!("{}", simulate_z(&device));
        println!("{}", swapped_pairs(&device));
    }
}
