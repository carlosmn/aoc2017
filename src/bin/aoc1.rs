use std::env;

fn main() {
    let input = env::args().nth(1).expect("a string on argv");

    let numbers = input
        .chars()
        .enumerate()
        .map(|(i, _ch)| u64::from_str_radix(&input[i..i+1], 10).expect("an integer"))
        .collect::<Vec<u64>>();

    // let numbers = Vec::new();
    // for (i, _ch) in input.as_str().chars().enumerate() {
    //     let n = u64::from_str_radix(&input[i..i+1], 10).expect("an integer");
    //     numbers.push(n);
    // }

    let mut acc = 0u64;
    for (i, _) in numbers.iter().enumerate() {
        let next_i = if i == numbers.len()-1 { 0 } else { i+1 };
        if numbers[i] == numbers[next_i] {
            acc += numbers[i];
        }
    }
    println!("answer 1 {}", acc);

    acc = 0;
    for (i, _) in numbers.iter().enumerate() {
        let next_i = (i + (numbers.len() / 2)) % numbers.len();
        if numbers[i] == numbers[next_i] {
            acc += numbers[i];
        }
    }
    println!("answer 2 {}", acc);

}
