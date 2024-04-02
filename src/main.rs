fn sum_u32(nums: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;

    for &num in nums {
        match sum.checked_add(num) {
            Option::Some(result) => {
                sum = result;
            }
            Option::None => {
                return Option::None;
            }
        }
    }
    Option::Some(sum)
}

fn main() {
    let nums = [2, 4, 6, 8];
    let max = [u32::MAX; 2];

    match sum_u32(&nums) {
        Option::Some(result) => println!("Sum: {}", result),
        Option::None => println!("Overflow"),
    }

    match sum_u32(&max) {
        Option::Some(result) => println!("Sum: {}", result),
        Option::None => println!("Overflow"),
    }

    println!("Hello, world!");
}
