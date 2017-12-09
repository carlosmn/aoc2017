use std::io::Read;

fn main() {
    let stdin = std::io::stdin();
    let (score, noncancelled) = count_score(stdin.lock());

    println!("Score: {}, noncancelled: {}", score, noncancelled);
}

fn count_score<R: Read>(r: R) -> (u64, u64) {
    let mut score = 0u64;
    let mut open_groups = 0u64;
    let mut skip_next = false;
    let mut in_garbage = false;
    let mut noncancelled = 0u64;

    for mb in r.bytes() {
        let b = mb.expect("mb");
        if skip_next {
            skip_next = false;
            continue;
        }

        match b {
            b'<' if !in_garbage => in_garbage = true,
            b'>' => in_garbage = false,
            b'!' => skip_next = true,
            b'{' if !in_garbage => open_groups += 1,
            b'}' if !in_garbage => {
                score += open_groups;
                open_groups -= 1;
            },
            _ if in_garbage => noncancelled += 1,
            _ => {},
        }

        //println!("b {}, score {}, open_groups {}", b as char, score, open_groups);
    }

    (score, noncancelled)
}

#[cfg(test)]
macro_rules! count_string {
    ($x:expr, $y:expr) => {
        assert_eq!($x, count_score(Cursor::new($y.as_bytes())).0)
    }
}

#[cfg(test)]
macro_rules! count_noncancelled_string {
    ($x:expr, $y:expr) => {
        assert_eq!($x, count_score(Cursor::new($y.as_bytes())).1)
    }
}

#[test]
fn test_count_score() {
    use std::io::Cursor;
    count_string!(1, "{}");
    count_string!(6, "{{{}}}");
    count_string!(16, "{{{},{},{{}}}}");
    count_string!(1, "{<a>,<a>,<a>,<a>}");
    count_string!(9, "{{<ab>},{<ab>},{<ab>},{<ab>}}");
    count_string!(9, "{{<!!>},{<!!>},{<!!>},{<!!>}}");
    count_string!(3, "{{<a!>},{<a!>},{<a!>},{<ab>}}");
}

#[test]
fn test_noncancelled() {
    use std::io::Cursor;
    count_noncancelled_string!(0, "<>");
    count_noncancelled_string!(17, "<random characters>");
    count_noncancelled_string!(3, "<<<<>");
    count_noncancelled_string!(2, "<{!>}>");
    count_noncancelled_string!(0, "<!!>");
    count_noncancelled_string!(0, "<!!!!>>");
    count_noncancelled_string!(10, r#"<{o"i!a,<{i<a>"#);
}
