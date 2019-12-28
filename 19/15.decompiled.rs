boot:
//01032: [   0,    0,   21,   21,    1,   10,    1,    0,    0,    0]
//01042: [   0,    0,    0]
tmp = 0;
inp = 0;
currx = 21;
curry = 21;
oddx = 1;
halfy = 10;
oddy = 1;
nx = 0;
ny = 0;
noddx = 0;
nhalfy = 0;
noddy = 0;
outval = 0;

loop {
    //00000: [   3, 1033]              in    inp                    [ 249]
    //00002: [1008, 1033,    1, 1032]  eq    inp      1 =>     tmp   |
    //00006: [1005, 1032,   31]        jnz     tmp     31             |
    input(inp);
    if inp == 1 {
        // north
        tmp = inp == 1;
        //00031: [ 102,    1, 1034, 1039]  mul      1     currx =>     nx   [   6]
        //00035: [1002, 1036,    1, 1041]  mul     oddx      1 =>     noddx   |
        //00039: [1001, 1035,   -1, 1040]  add     curry     -1 =>     ny   |
        //00043: [1008, 1038,    0, 1043]  eq      oddy      0 =>     noddy   |
        //00047: [ 102,   -1, 1043, 1032]  mul     -1     noddy =>     tmp   |
        //00051: [   1, 1037, 1032, 1042]  add     halfy     tmp =>     nhalfy   |
        //00055: [1106,    0,  124]        jz       0    124             |
        nx = currx;
        noddx = oddx;
        ny = curry - 1;
        noddy = oddy == 0;
        tmp = -(oddy == 0);
        // shared halfy: (0,1), (2,3), ..
        // halfy decreases if moving up to odd
        nhalfy = halfy - (oddy == 0);
        goto l_124;
    //00009: [1008, 1033,    2, 1032]  eq    inp      2 =>     tmp   |
    //00013: [1005, 1032,   58]        jnz     tmp     58             |
    } else if inp == 2 {
        // south
        tmp = inp == 2;
        //00058: [1002, 1034,    1, 1039]  mul     currx      1 =>     nx   [  13]
        //00062: [ 102,    1, 1036, 1041]  mul      1     oddx =>     noddx   |
        //00066: [1001, 1035,    1, 1040]  add     curry      1 =>     ny   |
        //00070: [1008, 1038,    0, 1043]  eq      oddy      0 =>     noddy   |
        //00074: [   1, 1037, 1038, 1042]  add     halfy     oddy =>     nhalfy   |
        //00078: [1105,    1,  124]        jnz      1    124             |
        nx = currx;
        noddx = oddx;
        ny = curry + 1;
        // shared halfy: (0,1), (2,3), ..
        // halfy increases if moving down from odd
        noddy = oddy == 0;
        nhalfy = halfy + oddy;
    //00016: [1008, 1033,    3, 1032]  eq    inp      3 =>     tmp   |
    //00020: [1005, 1032,   81]        jnz     tmp     81             |
    } else if inp == 3 {
        // west
        tmp = inp == 3;
        //00081: [1001, 1034,   -1, 1039]  add     currx     -1 =>     nx   [  20]
        //00085: [1008, 1036,    0, 1041]  eq      oddx      0 =>     noddx   |
        //00089: [ 102,    1, 1035, 1040]  mul      1     curry =>     ny   |
        //00093: [ 102,    1, 1038, 1043]  mul      1     oddy =>     noddy   |
        //00097: [1002, 1037,    1, 1042]  mul     halfy      1 =>     nhalfy   |
        //00101: [1105,    1,  124]        jnz      1    124             |
        nx = currx - 1;
        noddx = oddx == 0;
        ny = curry;
        noddy = oddy;
        nhalfy = halfy;
    } else if inp == 4 {
        // east
        tmp = inp == 4;
        //00104: [1001, 1034,    1, 1039]  add     currx      1 =>     nx   [  27]
        //00108: [1008, 1036,    0, 1041]  eq      oddx      0 =>     noddx   |
        //00112: [ 102,    1, 1035, 1040]  mul      1     curry =>     ny   |
        //00116: [ 102,    1, 1038, 1043]  mul      1     oddy =>     noddy   |
        //00120: [1002, 1037,    1, 1042]  mul     halfy      1 =>     nhalfy   |
        nx = currx + 1;
        noddx = oddx == 0;
        ny = curry;
        noddy = oddy;
        nhalfy = halfy;
    } else {
        tmp = 0;
        stop;
    }

l_124:
    //00124: [1006, 1039,  217]        jz      nx    217             [  55,   78,  101]
    if nx != 0 {
        //00127: [1006, 1040,  217]        jz      ny    217             |
        if ny != 0 {
            //00130: [1008, 1039,   40, 1032]  eq      nx     40 =>     tmp   |
            //00134: [1005, 1032,  217]        jnz     tmp    217             |
            tmp = nx == 40;
            if nx != 40 {
                //00137: [1008, 1040,   40, 1032]  eq      ny     40 =>     tmp   |
                //00141: [1005, 1032,  217]        jnz     tmp    217             |
                tmp = ny == 40;
                if ny != 40 {
                    //00144: [1008, 1039,   33, 1032]  eq      nx     33 =>     tmp   |
                    //00148: [1006, 1032,  165]        jz      tmp    165             |
                    tmp = nx == 33;
                    if nx == 33 {
                        //00151: [1008, 1040,   35, 1032]  eq      ny     35 =>     tmp   |
                        //00155: [1006, 1032,  165]        jz      tmp    165             |
                        tmp = ny == 35;
                        if ny == 35 {
                            //00158: [1101,    2,    0, 1044]  add      2      0 =>     outval   |
                            //00162: [1106,    0,  224]        jz       0    224             |
                            // oxygen is at 33,35
                            outval = 2;
                            goto l_224;
                        } else {
                            goto l_165;
                        }
                    } else {
                        goto l_165;
                    }
l_165:
                    //00165: [   2, 1041, 1043, 1032]  mul     noddx     noddy =>     tmp   [ 148,  155]
                    //00169: [1006, 1032,  179]        jz      tmp    179             |
                    tmp = noddx * noddy;
                    if noddx * noddy != 0 {
                        //00172: [1101,    1,    0, 1044]  add      1      0 =>     outval   |
                        //00176: [1106,    0,  224]        jz       0    224             |
                        // moved one step
                        outval = 1;
                        goto l_224;
                    } else {
                        //00179: [   1, 1041, 1043, 1032]  add     noddx     noddy =>     tmp   [ 169]
                        //00183: [1006, 1032,  217]        jz      tmp    217             |
                        tmp = noddx + noddy;
                        if noddx + noddy != 0 {
                            //00186: [   1, 1042, 1043, 1032]  add     nhalfy     noddy =>     tmp   |
                            //00190: [1001, 1032,   -1, 1032]  add     tmp     -1 =>     tmp   |
                            //00194: [1002, 1032,   39, 1032]  mul     tmp     39 =>     tmp   |
                            //00198: [   1, 1032, 1039, 1032]  add     tmp     nx =>     tmp   |
                            //00202: [ 101,   -1, 1032, 1032]  add     -1     tmp =>     tmp   |
                            //00206: [ 101,  252, 1032,  211]  add    252     tmp => [ 211]   |
                            //00210: [1007,    0,   37, 1044]  lt  [   0]     37 =>     outval   |
                            //00214: [1105,    1,  224]        jnz      1    224             |
                            /*
                            tmp = nhalfy + noddy;
                            tmp -= 1;
                            tmp *= 39;
                            tmp += nx;
                            tmp -= 1;
                            a211 = tmp + 252;
                            outval = mem[tmp] < 37;
                            */
                            // either x or y is odd
                            // map corner starts at (1, 1) because implicit borders
                            // y position:       1), (2, 3), (4, 5), ..
                            // nhalfy:           0,   1, 1,   2, 2, 
                            // odd y:            1,   0, 1,   0, 1, 
                            // half + odd - 1:   0,   0, 1,   1, 2
                            tmp = (nhalfy + noddy - 1) * 39 + nx - 1;
                            a211 = 252 + tmp;
                            outval = mem_252[tmp] < 37;
                            goto l_224;
                        } else {
                            goto l_217;
                        }
                    }
                } else {
                    goto l_217;
                }
            } else {
                goto l_217;
            }
        } else {
            goto l_217;
        }
    } else {
        goto l_217;
    }

l_217:
    outval = 0;
    goto l_224;
l_224:
//00224: [1006, 1044,  247]        jz      outval    247             [ 162,  176,  214,  221]
    if outval != 0 {
        // didn't hit a wall, move
        //00227: [ 101,    0, 1039, 1034]  add      0     nx =>     currx   |
        //00231: [ 101,    0, 1040, 1035]  add      0     ny =>     curry   |
        //00235: [ 102,    1, 1041, 1036]  mul      1     noddx =>     oddx   |
        //00239: [1001, 1043,    0, 1038]  add     noddy      0 =>     oddy   |
        //00243: [1002, 1042,    1, 1037]  mul     nhalfy      1 =>     halfy   |
        currx = nx;
        curry = ny;
        oddx = noddx;
        oddy = noddy;
        halfy = nhalfy;
    }
l_247:
    output(outval);
}


