fn main() {
    let stdin = std::io::stdin();
    let (mut count, mut count_ext) = (0u64, 0u64);
    loop {
        let mut line = String::new();
        let n = stdin.read_line(&mut line).expect("OK");
        if n == 0 {
            break;
        }

        if is_valid(&line) {
            count += 1;
        }

        if is_valid_extended(&line) {
            count_ext += 1;
        }
    }

    println!("count {}, extended {}", count, count_ext);
}

fn is_valid(s: &str) -> bool {
    let words: Vec<&str> = s.split_whitespace().collect();
    for (i, w) in words.iter().enumerate() {
        for j in i+1..words.len() {
            if w == &words[j] {
                return false;
            }
        }
    }

    true
}

fn is_valid_extended(s: &str) -> bool {
    let words: Vec<&str> = s.split_whitespace().collect();
    for (i, w) in words.iter().enumerate() {
        for j in i+1..words.len() {
            let mut letters = w.chars().collect::<Vec<char>>();
            letters.sort();

            let mut letters2 = words[j].chars().collect::<Vec<char>>();
            letters2.sort();

            if letters == letters2 {
                return false;
            }
        }
    }

    true
}
