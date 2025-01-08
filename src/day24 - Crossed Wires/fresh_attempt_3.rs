
// The general problem does not seem to have any shortcuts to make it computable
// However, we know the most efficient way to implement adding two numbers is with a half adder and a bunch of full adders
//
// The input has 222 gates
// The first bit needs 1 gate:   (a xor b) -> z
//
// The second bit takes 3 gates:
//    1 gate for previous carry: ('a xor 'b) -> c
//    2 gates to calculate value: (a xor b) -> p
//                                (p xor c) -> z
//
// Every subsequent bit takes 5 gates;
//    3 gates for previous carry: ('a and 'b) -> c1 // Both previous bits
//                                ('p xor 'c) -> c2 // One previous bit and carry
//                                (c1  or c2) -> c
//    2 gates to calculate value: (a xor b) -> p
//                                (p xor c) -> z
//
// The highest input bit is 44, therefore at the minimum we need
//    1+3+(42*5) = 214 gates
// This means there may be up to 8 unnecessary gates...
// ...which is definitely enough to hide some gotchas in
//
// But by rendering out the network in plantuml, it looks pretty normal.