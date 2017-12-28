use std::io::{BufRead};
use std::collections::{HashSet, HashMap};

#[derive(Debug)]
struct Process {
    pub pid: usize,
    pub pipes: HashSet<usize>
}

impl Process {
    fn new(pid: usize) -> Process {
        Process {
            pid: pid,
            pipes: HashSet::new(),
        }
    }
}

fn main() {
    let stdin = std::io::stdin();

    let procs = parse_input(stdin.lock());
    println!("{} procs", procs.len());

    let connected = connected_procs(&procs, 0);
    println!("connected: {}", connected.len());
}

fn parse_input<R: BufRead>(r: R) -> HashMap<usize, Process> {
    let mut m: HashMap<usize, Process> = HashMap::new();

    for line in r.split(b'\n') {
        let l = line.expect("line");
        let ll = String::from_utf8_lossy(&l);
        let mut split = ll.trim().split(" <-> ");
        let numstr = split.next().expect("proc number");
        let num = usize::from_str_radix(numstr, 10).expect("proc number");

        let mut process = Process::new(num);
        for s in split.next().expect("connections").split(", ") {
            let num = usize::from_str_radix(s, 10).expect("proc number");
            process.pipes.insert(num);
        }

        m.insert(num, process);
    }

    m
}

fn connected_procs(m: &HashMap<usize, Process>, pid: usize) -> HashSet<usize> {
    let root = m.get(&pid).expect("root proc");
    let mut s = HashSet::new();
    let mut q = Vec::new();

    s.insert(root.pid);
    for cpid in &root.pipes {
        q.push(cpid);
    }

    while let Some(pid) = q.pop() {
        let p = m.get(&pid).expect("root proc");
        if s.insert(p.pid) {
            for cpid in &p.pipes {
                q.push(cpid);
            }
        }
    }


    s
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    const SAMPLE_INPUT: &str = r#"0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5"#;

    #[test]
    fn test_sample() {
        let c = Cursor::new(SAMPLE_INPUT);
        let procs = super::parse_input(c);
        assert_eq!(7, procs.len());
        println!("procs {:?}", procs);

        let connected = super::connected_procs(&procs, 0);
        assert_eq!(6, connected.len());
        println!("connected {:?}", connected);

    }
}
