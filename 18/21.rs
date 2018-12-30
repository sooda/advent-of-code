use std::collections::HashSet;

#[allow(dead_code)]
fn l6_orig(r0: i64) {
    let mut r1: i64 = 0;
    let mut r2: i64 = 0;
    // let mut r3: i64 = 0; // (IP)
    let mut r4: i64 = 0;
    let mut r5: i64 = 0;
    loop {
        // those if conditions are silly so that this print would match with the interpreter log
        println!("[{}, {}, {}, {}, {}, {}]", r0, r1, r2, 6, r4, r5);
        // 6-7
        r2 = r4 | 0x10000;
        r4 = 6152285;
        loop {
            // 8-16
            r1 = r2 & 0xff;
            r4 += r1;
            r4 &= 0xffffff;
            r4 *= 65899;
            r4 &= 0xffffff;
            r1 = if 0x100 > r2 { 1 } else { 0 };
            if r1 == 1 /* 0x100 > r2 */ { // jgt 28 via 16
                // 28-30
                r1 = if r4 == r0 { 1 } else { 0 };
                if r1 == 1 /* r4 == r0 */ { // jeq 31
                    return;
                } else { // jmp 6
                    break;
                }
            } else { // jmp 17
                // 17
                r1 = 0;
                loop {
                    // 18-23
                    r5 = r1 + 1;
                    r5 *= 0x100;
                    r5 = if r5 > r2 { 1 } else { 0 };
                    if r5 == 1 /* r5 > r2 */ { // jgt 26 via 23
                        // 26-27
                        r2 = r1;
                        break; // jmp 8
                    } else { // jmp 24
                        // 24-25
                        r1 += 1;
                        // jmp 18;
                    }
                }
            }
        }
    }
}

// slightly easier to read without the asm-level annotations
#[allow(dead_code)]
fn l6(r0: i64) {
    let mut r1: i64 = 0;
    let mut r2: i64 = 0;
    // let mut r3: i64 = 0; // (IP)
    let mut r4: i64 = 0;
    let mut r5: i64 = 0;
    loop {
        println!("[{}, {}, {}, {}, {}, {}]", r0, r1, r2, 6, r4, r5);
        // r4 is at most 0xffffff from previous iteration
        r2 = r4 | 0x10000;
        r4 = 6152285; // 0x5de05d
        loop {
            r4 += r2 & 0xff;
            r4 &= 0xffffff;
            r4 *= 65899; // 0x1016b
            r4 &= 0xffffff;
            // this can never happen on the first two iterations (0x1_00_00 shifts by 8 first)
            if r2 < 0x100 {
                // r2 has only something in low 8 bits
                println!("  [{}, {}, should be 0..255: {}, {}, r0 would stop: {}, {}]", r0, r1, r2, 6, r4, r5);
                if r4 == r0 {
                    return;
                }
                break;
            }
            // find smallest r1 for: ((r1 + 1) << 8) > r2, store in r2.
            // in other words: this shifts r2 right by 8 bits (r2 = r2 >> 8)
            r1 = 0;
            loop {
                r5 = r1 + 1;
                r5 *= 0x100;
                if r5 > r2 {
                    r2 = r1;
                    break;
                }
                r1 += 1;
            }
            println!("  r2 became {} = 0x{:x}", r2, r2);
        }
    }
}

fn find_cycle() -> i64 {
    let mut r2: i64;
    let mut r4: i64 = 0;
    let mut seen = HashSet::new();
    let mut prev = 0;

    for _i in 0.. {
        if seen.contains(&r4) {
            // starts to repeat here; the previous value must be the end of the cycle
            return prev;
        } else {
            seen.insert(r4);
        }
        prev = r4;

        r2 = r4 | 0x10000;
        r4 = 6152285;

        r4 += r2 & 0xff;
        r4 &= 0xffffff;
        r4 *= 65899;
        r4 &= 0xffffff;

        assert!(r2 > 0xff);
        r2 >>= 8;

        r4 += r2 & 0xff;
        r4 &= 0xffffff;
        r4 *= 65899;
        r4 &= 0xffffff;

        assert!(r2 > 0xff);
        r2 >>= 8;

        r4 += r2 & 0xff;
        r4 &= 0xffffff;
        r4 *= 65899;
        r4 &= 0xffffff;
    }
    unreachable!()
}

fn main() {
    println!("{}", find_cycle());
}
