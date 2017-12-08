extern crate regex;

use regex::Regex;
use std::collections::{HashSet, HashMap};
use std::cell::RefCell;

#[derive(Debug)]
struct Process {
    name: String,
    weight: u64,
    above: HashSet<String>,
    // So we can modify the process while the overall hashtable is borrowed. It
    // might not be the best option.
    below: RefCell<Option<String>>,
}

type ProcMap = HashMap<String, Process>;

fn main() {
    let procs = read_procs(std::io::stdin());

    // Now that we have the processes and have pointers between them and we
    // assume that all processes are in the one tree, we can find the one at the
    // bottom by starting from any of them, and following the "below" pointers
    // until there are no more.
    //
    // Or as an alternative, we can go through the list until we find an empty
    // "below".
    for p in procs.values() {
        if p.below.borrow().is_none() {
            println!("The bottom process is {}", p.name);
        }
    }
}

/// Read the processes from stdin and return them with filled dependencies.
fn read_procs(stdin: std::io::Stdin) -> ProcMap {
    let expr = Regex::new(r"^(\w+?) \((\d+)\)(?: -> (.*))?$").expect("regex");

    let mut procs: ProcMap = ProcMap::new();
    let mut input = String::new();
    loop {
        input.clear();
        let read = stdin.read_line(&mut input).expect("stdin");
        if read == 0 {
            break;
        }

        let s = input.trim();

        let caps = expr.captures(s).expect("captures");

        let name = caps.get(1).expect("name").as_str().to_owned();
        let weight = u64::from_str_radix(caps.get(2).expect("weight").as_str(), 10).expect("weight");
        let mut above = HashSet::new();
        if let Some(m) = caps.get(3) {
            for name in m.as_str().split(", ") {
                above.insert(name.into());
            }
        }

        let p = Process {
            name: name.clone(),
            weight: weight,
            above: above,
            below: RefCell::new(None),
        };

        procs.insert(name, p);
    }

    fill_belows(procs)
}

/// Take the process map and insert all those processes which are below
fn fill_belows(procs: ProcMap) -> ProcMap {
    {
        let names = procs.keys().map(|s| s.as_str()).collect::<Vec<&str>>();
        for name in names {
            let p = procs.get(name).expect("proc");
            for ref above in &p.above {
                procs.get(above.as_str()).expect("proc").below.borrow_mut().get_or_insert(name.into());
            }
        }
    }

    procs
}
