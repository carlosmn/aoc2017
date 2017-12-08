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
    let bottom: &str = (|| {
        for p in procs.values() {
            if p.below.borrow().is_none() {
                return &p.name;
            }
        }
        unreachable!();
    })();

    println!("The bottom process is {}", bottom);

    // Now we're tasked to find a single process with the wrong weight to be
    // balanced. We start from the bottom process and see whether all of the
    // ones above are balanced. If not, we go into the different one and see
    // whether the ones above it are unbalanced. If they are balanced, we know
    // it's that base process the one that's unbalanced.
    let mut pn = bottom;
    let mut last_common_weight = 0;
    loop {
        println!("Looking at {}", pn);
        println!("Combined weight {}", combined_weight(pn, &procs));

        let above = &procs.get(pn).expect("proc").above;
        match odd_weight(above, &procs) {
            Some((odd_one, w)) => {
                println!("Odd one out {}", odd_one);
                pn = odd_one;
                last_common_weight = w;
                continue;
            }
            None => {
                println!("Above is balanced");
                break;
            }
        }
    }

    // last_common_weight is what we need pn's combined weight to be. The
    // difference between these values is how much we need to change its weight
    // by.
    let unbalanced_w = combined_weight(pn, &procs);
    let d = (last_common_weight as i64) - (unbalanced_w as i64);
    println!("Unabalanced {}, should weigh {}", pn, procs.get(pn).expect("proc").weight as i64 + d);

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

/// Returns the combined weight of the given process together with any above it.
fn combined_weight(pn: &str, procs: &ProcMap) -> u64 {
    let p = procs.get(pn).expect("proc");
    let mut w = p.weight;

    for name in &p.above {
        w += combined_weight(name, procs);
    }

    w
}

/// Returns the name of the process with a different weight than the rest, plus
/// the common weight. It assumes there is at most a single odd one out and the
/// rest have the same weight.
///
/// If they're all the same, it returns None.
fn odd_weight<'a>(names: &'a HashSet<String>, procs: &ProcMap) -> Option<(&'a str, u64)> {
    for name in names {
        let w = combined_weight(name, procs);
        let mut common = 0;
        let mut common_weight = 0;

        for pn in names {
            if pn == name {
                continue;
            }

            let ww = combined_weight(pn, procs);
            println!("comparing {} ({}) <=> {} ({})", name, w, pn, ww);
            if ww == w {
                common += 1;
            } else {
                common_weight = ww;
            }
        }

        if common == 0 {
            return Some((name, common_weight));
        }
    }

    None
}
