const MAPMEM: &[i64] = &[
    31,10,7,30,32,67,8,24,11,62,6,11,19,78,16,20,8,80,14,19,63,8,40,36,65,34,59,23,33,29,79,19,47,28,54,8,11,41,33,
    57,85,25,56,48,16,90,74,39,11,79,68,18,46,33,74,47,25,60,1,23,78,69,5,55,12,28,73,22,80,30,26,55,2,6,96,21,57,34,
    33,10,91,72,61,31,2,24,29,94,24,12,43,60,72,79,27,24,21,95,59,15,53,34,9,36,82,83,4,67,30,62,5,70,94,1,81,75,6,
    18,68,9,26,38,31,1,98,57,97,63,8,60,35,5,48,36,59,75,4,88,23,21,39,10,99,13,36,53,66,73,28,33,80,28,78,23,7,30,
    27,77,28,69,69,1,65,78,17,17,2,16,27,91,43,27,72,93,6,5,92,12,55,79,94,98,60,19,15,36,35,55,9,62,84,27,74,56,25,
    9,60,72,15,34,59,15,31,58,76,24,81,62,99,35,31,14,39,25,60,3,5,46,24,48,22,1,73,99,96,27,46,48,5,65,26,6,48,11,
    13,69,12,33,22,95,11,72,28,42,28,88,5,31,56,50,72,30,49,84,52,32,11,45,7,54,60,12,72,33,38,62,18,54,31,8,92,53,34,
    4,76,21,46,81,53,81,21,10,63,12,75,22,62,87,32,23,30,40,29,24,61,6,88,70,14,18,99,13,14,4,72,5,22,54,90,75,35,1,
    10,49,17,7,98,8,81,13,47,59,13,80,70,9,26,73,22,77,3,22,73,99,74,11,10,60,4,27,86,46,67,30,94,29,93,26,66,25,8,
    14,92,24,45,78,24,23,97,31,9,25,25,61,44,35,31,73,52,80,35,96,32,43,8,66,57,87,31,85,12,50,74,7,23,61,12,7,78,1,
    1,53,14,54,18,18,63,41,25,90,1,85,24,22,98,62,35,14,19,50,80,20,7,73,21,14,81,19,89,11,31,84,7,53,9,54,20,90,72,
    31,70,54,17,31,59,18,8,69,83,58,78,12,98,20,81,26,50,95,19,25,54,31,80,67,6,3,87,6,99,93,22,75,73,34,52,58,22,32,
    52,34,30,85,54,58,75,14,22,97,12,36,53,67,32,99,54,15,4,66,69,7,48,87,25,17,41,57,10,63,35,24,43,5,57,25,93,22,71,
    7,36,63,84,26,4,7,78,26,68,77,35,9,70,17,12,59,41,78,18,54,18,80,18,86,93,19,35,73,34,53,97,23,2,95,30,32,85,21,
    21,79,19,18,85,57,23,85,35,34,61,30,66,29,19,76,30,17,46,1,16,98,26,25,91,15,47,54,75,26,17,36,74,60,33,28,49,53,15,
    13,45,6,90,26,73,17,87,4,68,18,30,22,96,92,97,14,40,24,50,96,15,49,55,79,8,16,1,50,5,60,55,14,41,67,25,26,71,18,
    26,89,70,14,6,51,11,94,68,69,22,73,63,6,33,88,36,51,20,6,44,26,71,17,31,11,86,81,23,31,80,18,87,26,12,91,8,41,6,
    18,9,33,90,1,59,56,32,29,54,50,34,12,74,97,10,39,87,41,9,52,67,21,22,38,61,57,1,87,4,35,98,61,16,95,78,65,17,31,
    9,71,9,52,52,9,8,73,40,36,16,48,52,9,26,39,4,17,42,1,35,80,93,4,40,23,13,66,7,28,84,73,22,31,76,31,21,39,4,
    83,84,41,27,66,34,88,15,50,65,45,22,65,26,78,15,50,40,79,31,38,9,60,2,51,24,46,99,42,27,45,1,71,20,78,86,95,9,81
];

pub struct OxygenMap {
    currx: i64,
    curry: i64,
    oddx: bool,
    halfy: i64,
    oddy: bool,
}

impl OxygenMap {
    pub fn new() -> Self {
        OxygenMap {
            currx: 21,
            curry: 21,
            oddx: true,
            halfy: 10,
            oddy: true,
        }
    }

    pub fn run(&mut self, inp: i64) -> Option<i64> {
        let nx;
        let ny;
        let noddx;
        let noddy;
        let nhalfy;
        let outval;

        if inp == 1 {
            // north
            nx = self.currx;
            noddx = self.oddx;
            ny = self.curry - 1;
            noddy = !self.oddy;
            nhalfy = self.halfy - (!self.oddy) as i64;
        } else if inp == 2 {
            // south
            nx = self.currx;
            noddx = self.oddx;
            ny = self.curry + 1;
            noddy = !self.oddy;
            nhalfy = self.halfy + self.oddy as i64;
        } else if inp == 3 {
            // west
            nx = self.currx - 1;
            noddx = !self.oddx;
            ny = self.curry;
            noddy = self.oddy;
            nhalfy = self.halfy;
        } else if inp == 4 {
            // east
            nx = self.currx + 1;
            noddx = !self.oddx;
            ny = self.curry;
            noddy = self.oddy;
            nhalfy = self.halfy;
        } else {
            return None;
        }

        if nx != 0 && ny != 0 && nx != 40 && ny != 40 {
            if nx == 33 && ny == 35 {
                outval = 2;
            } else if noddx && noddy {
                outval = 1;
            } else if noddx || noddy {
                let off = (nhalfy + (noddy as i64) - 1) * 39 + nx - 1;
                outval = (MAPMEM[off as usize] < 37) as i64;
            } else {
                outval = 0;
            }
        } else {
            outval = 0;
        }

        if outval != 0 {
            self.currx = nx;
            self.curry = ny;
            self.oddx = noddx;
            self.oddy = noddy;
            self.halfy = nhalfy;
        }

        Some(outval)
    }
}