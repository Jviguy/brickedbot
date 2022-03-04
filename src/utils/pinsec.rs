
//A simple algorithm for determine the guess heurestic
pub fn score(pin: i32) -> f32 {
    let mut s = 1.0;
    let mut digits: Vec<i32> = vec![(pin % 10)];
    digits.append(&mut (0..)
        .scan(pin, |num, _| {
            *num /= 10;
            Some(*num)
        })
        .take_while(|num| *num > 0)
        .map(|num| num % 10)
        .collect::<Vec<i32>>());
    let mut previous = 10;
    for digit in digits {
        // A 4 digit pin grows exponentially easier to guess if theres many repeated digits
        if digit == previous {
            s *= 2.0;
        }
        // Lower digits are guessed first in most 10 digit code prompts
        if digit < 5 {
            s += 0.2;
        }
        // The digit 0 adds complexity to codes based on its position in code prompts
        if digit == 0 {
            s -= 0.5;
        }
        previous = digit;
    }
    if s > 10.0 {
        s = 10.0;
    }
    s
}