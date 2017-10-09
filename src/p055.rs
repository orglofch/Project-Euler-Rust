// DONE: 3603
//
// It seems like a number will maintain it's palindromic properties
// as long as there isn't any carry. If there is carry then we
// know we'll need to proceed to the next iteration under most
// circumstances, however, 47 is still a coutner example to that
// idea.
//
// We can skip numbers with >= 5 digits.
// We can ski[ numbers with == 4 digits when the most significant
// 10's digit + the least significant 10's digit > 1.
//
// We can skip numbers for which we've already found the answer.

fn is_lychrel(num: u32) -> bool {
    let mut sum = num;
    for _ in 0..50 {
        let reversed = reverse(sum);
        if reversed == sum {
            // It's a palindrome.
            return true;
        }
        sum = sum + reversed;
        if sum > 9900 {
            return false;
        }
    }
    return false;
}

fn reverse(num: u32) -> u32 {
    let mut ans = 0;
    let mut place = 1;
    let mut rem = num;
    while rem > 0 {
        ans = ans * 10 + (rem % 10);
        rem = rem / 10;
        place = place * 10;
    }
    return ans;
}

fn main() {
    let mut lychrel_count = 0;
    for i in 1..9900 {
        if is_lychrel(i) {
            println!("{} is lychrel", i);
            lychrel_count = lychrel_count + 1;
        }
    }
    println!("There are {} lychrel numbers below 10,000", lychrel_count);
}
