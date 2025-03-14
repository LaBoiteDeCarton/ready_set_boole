pub fn gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}

/*
0000
    1
0001       0000 -> 0010  0011 -> 0010
    2
0010
    1
0011
    3
0100
    1
0101
    2
0110
    1
0111


0000    0
    1
0001    1
    1
0011    2
    1
0010    3
    1
0110    4
    1
0111    5
    1
0101    6
    1
0100    7


0000
0001
0011
0010
0110
0111
0101
0100
1100
1101
1111
1
1
1
1
1


*/
