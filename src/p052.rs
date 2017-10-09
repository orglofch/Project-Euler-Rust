// DONE: 142857
//
// Another opportunity to use our prime digit uniqueness trick.

const INDEX_PRIMES: [u32; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];

fn prime_index(num: u32) -> u32 {
    let mut index = 1;

    let mut rem = num;
    while rem > 0 {
        index *= INDEX_PRIMES[(rem % 10) as usize];
        rem /= 10;
    }
    return index;
}

fn smallest_permuted_multiple() -> u32 {
    let mut n = 1;
    loop {
        let index = prime_index(n);

        if (2..7).all(|m| prime_index(n * m) == index) {
            break;
        }

        n += 1;
    }
    return n;
}

fn main() {
    println!("Small permuted multiple is {}", smallest_permuted_multiple());
}
