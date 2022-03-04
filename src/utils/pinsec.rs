
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
    let previous = 10;
    for digit in digits {
        if digit == previous {
            s *= 2.0;
        }
        if digit < 5 {
            s += 0.2;
        }
        if digit == 0 {
            s -= 0.5;
        }
    }
    s
}