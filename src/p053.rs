// DONE: 4075
//
// We can take advantage of 2 simple identities to cut the work down significantly.
// 1) nCk = (n - 1)Ck + (n - 1)C(k - 1) - The dynamic programming recurrence.
// 2) nCk = nC(n - k) - The combination are symmetric, cutting the work in half.
//
// We can iterate from the center upwards and stop once we've
// hit a value < 1,000,000 since
// n!/((k - 1)!(n - k + 1)!) = (k * n!) / (k!(n - k)!(n - k + 1))
//                           = k/(n - k + 1) * nCk
// Which is <= nCk if k/(n - k + 1) <= 1
//
// k <= (n - k + 1)
// 2k <= n + 1, which is always true for k <= n / 2
//
// Note, however, we can't take advantage of both so we'll just use the recurrence
// relation and go from there if need be.

fn combinations_greater_than_1_million() -> u32 {
    // Note, truncation is fine since we don't are about higher order bits.
    let mut cache = [[0u32; 101]; 101];

    let mut count = 0;
    for n in 1..101 {
        cache[n][0] = 1;
        cache[n][n] = 1;
        cache[n][1] = n as u32;
        cache[n][n - 1] = n as u32;
        for k in 2..((n as f32 / 2.0).floor() as usize + 1) {
            let comb = cache[n - 1][k] + cache[n - 1][k - 1];
            cache[n][k] = comb;
            cache[n][n - k] = comb;

            if comb > 1_000_000 {
                count += if n - k == k { 1 } else { 2 };
            }
        }
    }
    return count;
}

fn main() {
    println!("There are {} combinations > 1,000,000", combinations_greater_than_1_million());
}
