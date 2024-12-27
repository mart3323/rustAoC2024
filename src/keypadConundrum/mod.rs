pub mod simulator;
pub mod visualiser;

fn main() {
    // Note: If the previous and current numpad are both on the A key, the cheapest path to press X key is known
    // Note: The sequence to press a key on the current numpad always ends with all previous numpads returning to the A key
    //       because that is the only key that propagates through the chain
    //         HOWEVER: for any one press on the current numpad, the previous numpad may have to make multiple presses
    //                  without resetting in the meantime
    //                  For example, consider the following situation where we want to return to numpad A
    
    // TODO: Theory
    //       Assumption: A robot will never need to move in two opposite directions between A presses
    //                   because if it does, it would be more optimal to skip both the movements
    //       Therefore: we only need the costs for movements that don't contain opposites (DDD, DDR, but not DRU) 
    //       Plan:
    //         For each layer of keypad, starting from the first, precalculate the cost of pressing each of the following combinations
    //         L, R, U, D, UR, DR, DL, UL (for diagonals, test both orders)
    //           When doing DL to ←, only accept order DL, otherwise use DL or LD, whichever is shorter
    //           When doing UR from ←, only accept oder RU, otherwise use UR or RU, whichever is shorter
    //         Cache these results, For the next layer's calculations, simplify the path to two directions, use the cached value, and add back +1 for each omitted duplicate
    //
    // TODO: Test
    //             L  R  U  D  UR DR DL UL
    //  Numpad 0:  1  1  1  1  2  2  2  2    (path + return path + len + duplicates)
    //  Numpad 1:  7  3  3  5  6  7
    {
        //                       ┌─┬─┬─┐
        //                       │ │ │ │
        //                       ├─┼─┼─┤
        //                       │ │ │ │
        //    ┌─┬─┐  ┌─┬─┐  ┌─┬─┐├─┼─┼─┤
        //    │ │A│  │ │A│  │ │A││ │ │9│
        //  ┌─┼─┼─┤┌─┼─┼─┤┌─┼─┼─┤└─┼─┼─┤
        //  │ │ │ ││ │ │ ││ │ │ │  │ │ │
        //  └─┴─┴─┘└─┴─┴─┘└─┴─┴─┘  └─┴─┘
        //  After some moves
        //                       ┌─┬─┬─┐ Movement cost: nr of spaces + 6 if contains left + 4 if contains down or up right + 2 if contains right or up
        //                       │ │ │ │ (with return trip)
        //                       ├─┼─┼─┤ Movement cost 2: nr of spaces + 6+4 if contains left and/or down + 6+2 if contains down or up + 4+2 if contains right 
        //                       │ │ │ │ 
        //    ┌─┬─┐  ┌─┬─┐  ┌─┬─┐├─┼─┼─┤ Cost   A    LD/LU          RD           RU        U         R
        //    │ │A│  │ │ │  │ │A││ │ │9│        A    nA*6+nLD+nRU   nA*
        //  ┌─┼─┼─┤┌─┼─┼─┤┌─┼─┼─┤└─┼─┼─┤ 
        //  │ │ │ ││ │↓│ ││ │ │ │  │ │ │ 
        //  └─┴─┴─┘└─┴─┴─┘└─┴─┴─┘  └─┴─┘ 
        //                       ┌─┬─┬─┐
        //                       │ │ │ │
        //                       ├─┼─┼─┤
        //                       │ │ │ │
        //    ┌─┬─┐  ┌─┬─┐  ┌─┬─┐├─┼─┼─┤
        //    │ │A│  │ │ │  │ │ ││ │ │9│
        //  ┌─┼─┼─┤┌─┼─┼─┤┌─┼─┼─┤└─┼─┼─┤
        //  │ │ │ ││ │↓│ ││ │ │→│  │ │ │
        //  └─┴─┴─┘└─┴─┴─┘└─┴─┴─┘  └─┴─┘
        //  Note: Here numpad 2 will not return to A, because numpad 3 is still in the middle of its movement sequence
        // .                     ┌─┬─┬─┐
        //                       │ │ │ │
        //                       ├─┼─┼─┤
        //                       │ │ │ │
        //    ┌─┬─┐  ┌─┬─┐  ┌─┬─┐├─┼─┼─┤
        //    │ │ │  │ │ │  │ │ ││ │ │9│
        //  ┌─┼─┼─┤┌─┼─┼─┤┌─┼─┼─┤└─┼─┼─┤
        //  │←│ │ ││ │↓│ ││ │ │→│  │ │ │
        //  └─┴─┴─┘└─┴─┴─┘└─┴─┴─┘  └─┴─┘
        // .                     ┌─┬─┬─┐
        //                       │ │ │ │
        //                       ├─┼─┼─┤
        //                       │ │ │ │
        //    ┌─┬─┐  ┌─┬─┐  ┌─┬─┐├─┼─┼─┤
        //    │ │ │  │ │ │  │ │ ││ │ │9│
        //  ┌─┼─┼─┤┌─┼─┼─┤┌─┼─┼─┤└─┼─┼─┤
        //  │←│ │ ││←│ │ ││ │ │→│  │ │ │
        //  └─┴─┴─┘└─┴─┴─┘└─┴─┴─┘  └─┴─┘
        // .                     ┌─┬─┬─┐
        //                       │ │ │ │
        //                       ├─┼─┼─┤
        //                       │ │ │ │
        //    ┌─┬─┐  ┌─┬─┐  ┌─┬─┐├─┼─┼─┤
        //    │ │ │  │ │ │  │ │ ││ │ │ │
        //  ┌─┼─┼─┤┌─┼─┼─┤┌─┼─┼─┤└─┼─┼─┤
        //  │←│ │ ││←│ │ ││ │↓│ │  │ │A│
        //  └─┴─┴─┘└─┴─┴─┘└─┴─┴─┘  └─┴─┘
        // .                     ┌─┬─┬─┐
        //                       │ │ │ │
        //                       ├─┼─┼─┤
        //                       │ │ │ │
        //    ┌─┬─┐  ┌─┬─┐  ┌─┬─┐├─┼─┼─┤
        //    │ │ │  │ │ │  │ │ ││ │ │9│
        //  ┌─┼─┼─┤┌─┼─┼─┤┌─┼─┼─┤└─┼─┼─┤
        //  │ │↓│ ││←│ │ ││ │↓│ │  │ │ │
        //  └─┴─┴─┘└─┴─┴─┘└─┴─┴─┘  └─┴─┘       
        //                       ┌─┬─┬─┐
        //                       │ │ │ │
        //                       ├─┼─┼─┤
        //                       │ │ │ │
        //    ┌─┬─┐  ┌─┬─┐  ┌─┬─┐├─┼─┼─┤
        //    │ │ │  │ │ │  │ │ ││ │ │9│
        //  ┌─┼─┼─┤┌─┼─┼─┤┌─┼─┼─┤└─┼─┼─┤
        //  │ │ │→││ │↓│ ││ │↓│ │  │ │ │
        //  └─┴─┴─┘└─┴─┴─┘└─┴─┴─┘  └─┴─┘       
        //                       ┌─┬─┬─┐
        //                       │ │ │ │
        //                       ├─┼─┼─┤
        //                       │ │ │ │
        //    ┌─┬─┐  ┌─┬─┐  ┌─┬─┐├─┼─┼─┤
        //    │ │ │  │ │ │  │ │ ││ │ │9│
        //  ┌─┼─┼─┤┌─┼─┼─┤┌─┼─┼─┤└─┼─┼─┤
        //  │ │ │→││ │ │→││ │↓│ │  │ │ │
        //  └─┴─┴─┘└─┴─┴─┘└─┴─┴─┘  └─┴─┘       
        //                       ┌─┬─┬─┐
        //                       │ │ │ │
        //                       ├─┼─┼─┤
        //                       │ │ │ │
        //    ┌─┬─┐  ┌─┬─┐  ┌─┬─┐├─┼─┼─┤
        //    │ │A│  │ │ │  │ │ ││ │ │9│
        //  ┌─┼─┼─┤┌─┼─┼─┤┌─┼─┼─┤└─┼─┼─┤
        //  │ │ │ ││ │ │→││ │↓│ │  │ │ │
        //  └─┴─┴─┘└─┴─┴─┘└─┴─┴─┘  └─┴─┘       
        //                       ┌─┬─┬─┐
        //                       │ │ │ │
        //                       ├─┼─┼─┤
        //                       │ │ │ │
        //    ┌─┬─┐  ┌─┬─┐  ┌─┬─┐├─┼─┼─┤
        //    │↑│ │  │ │ │  │ │ ││ │ │9│
        //  ┌─┼─┼─┤┌─┼─┼─┤┌─┼─┼─┤└─┼─┼─┤
        //  │ │ │ ││ │ │→││ │↓│ │  │ │ │
        //  └─┴─┴─┘└─┴─┴─┘└─┴─┴─┘  └─┴─┘       
        //                       ┌─┬─┬─┐
        //                       │ │ │ │
        //                       ├─┼─┼─┤
        //                       │ │ │ │
        //    ┌─┬─┐  ┌─┬─┐  ┌─┬─┐├─┼─┼─┤
        //    │↑│ │  │ │A│  │ │ ││ │ │9│
        //  ┌─┼─┼─┤┌─┼─┼─┤┌─┼─┼─┤└─┼─┼─┤
        //  │ │ │ ││ │ │ ││ │↓│ │  │ │ │
        //  └─┴─┴─┘└─┴─┴─┘└─┴─┴─┘  └─┴─┘      
        //                       ┌─┬─┬─┐
        //                       │ │ │ │
        //                       ├─┼─┼─┤
        //                       │ │ │ │
        //    ┌─┬─┐  ┌─┬─┐  ┌─┬─┐├─┼─┼─┤
        //    │ │A│  │ │A│  │ │ ││ │ │9│
        //  ┌─┼─┼─┤┌─┼─┼─┤┌─┼─┼─┤└─┼─┼─┤
        //  │ │ │ ││ │ │ ││ │↓│ │  │ │ │
        //  └─┴─┴─┘└─┴─┴─┘└─┴─┴─┘  └─┴─┘  
        //                       ┌─┬─┬─┐
        //                       │ │ │ │
        //                       ├─┼─┼─┤
        //                       │ │ │ │
        //    ┌─┬─┐  ┌─┬─┐  ┌─┬─┐├─┼─┼─┤
        //    │ │A│  │ │A│  │ │ ││ │ │ │
        //  ┌─┼─┼─┤┌─┼─┼─┤┌─┼─┼─┤└─┼─┼─┤
        //  │ │ │ ││ │ │ ││ │↓│ │  │ │A│
        //  └─┴─┴─┘└─┴─┴─┘└─┴─┴─┘  └─┴─┘
    }
}