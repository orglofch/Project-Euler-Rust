// DONE: 2783915460
//
// This one can just be done by hand very easily.
//
// We know that the first 9! (362880) numbers start with a 0.
// The second 9! (362880) numbers start with a 1.
// Which means we can narrow the number we're looking for to
// the 274240 number which starts with 2.
//
// Numbers Taken: [2]
// The first 8! (40320) of those numbers starts with a 0.
// ... Same logic
// Which means we're looking for the 32320 number which
// starts with 27.
//
// Numbers Taken: [2, 7]
// The first 7! (5040) of those numbers starts with a 0.
// ...
// Which means we're looking for the 2080 number which
// starts with 278.
//
// Numbers Taken: [2, 7, 8]
// The first 6! (720) of those numbers starts with a 0.
// ...
// Which means we're looking for the 640 number which
// starts with 2783.
//
// Numbers Taken: [2, 7, 8, 3]
// The first 5! (120) of those numbers starts with a 0.
// ...
// Which means we're looking for the 40 number which
// starts with 27839.
//
// Numbers Taken: [2, 7, 8, 3, 9]
// The first 4! (24) of those numbers starts with a 0.
// ...
// Which means we're looking for the 16 number which
// starts with 278391.
//
// Numbers Taken: [2, 7, 8, 3, 9, 1]
// The first 3! (6) of those numbers starts with a 0.
// ...
// Which means we're looking for the 4 number which
// starts with 2783915.
//
// Numbers Taken: [2, 7, 8, 3, 9, 1, 5]
// ...
// Which means we're looking for the last lexographic number
// which starts with 27391454.
//
// Leaving us with 27839154-60.

fn main() {
    println!("The 1,000,000th lexographic permutation is {}", 2783915460);
}
