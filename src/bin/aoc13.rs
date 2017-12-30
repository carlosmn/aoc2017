use std::io::BufRead;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
enum Direction {
    Forward,
    Backward,
}

/// A layer of the firewall
#[derive(Debug)]
struct Layer {
    /// Its depth
    depth: usize,
    /// Its range
    range: usize,
    /// The current position of the scanner
    pos: usize,
    /// Going forward or back?
    dir: Direction,
}

impl Layer {
    fn new(depth: usize, range: usize) -> Self {
        Layer {
            depth: depth,
            range: range,
            pos: 0,
            dir: Direction::Forward,
        }
    }
}

impl FromStr for Layer {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s.split(": ").collect::<Vec<&str>>();
        let depth = nums[0].parse::<usize>()?;
        let range = nums[1].parse::<usize>()?;

        Ok(Self::new(depth, range))
    }
}

type Firewall = HashMap<usize, Layer>;

fn main() {
    let stdin = std::io::stdin();
    let mut f = parse_input(stdin.lock());
    let max_depth = *f.keys().max().expect("max number");

    let mut severity = 0;
    for pos in 0..max_depth {
        severity += severity_at(&f, pos);
        tick(&mut f);
    }

    println!("severity: {}", severity);
}


fn parse_input<R: BufRead>(r: R) -> Firewall {
    let mut f: Firewall = Firewall::new();

    for mline in r.lines() {
        if let Ok(ref line) = mline {
            let layer = Layer::from_str(line).expect("a layer");
            f.insert(layer.depth, layer);
        }
    }

    f
}

fn severity_at(f: &Firewall, pos: usize) -> usize {
    if let Some(layer) = f.get(&pos) {
        if layer.pos == 0 {
            return layer.depth * layer.range;
        }
    }

    0
}

/// Tick over time. This advances each scanner one position
fn tick(f: &mut Firewall) {
    for (_k, v) in f {
        match v.dir {
            Direction::Forward if v.pos == v.range - 1 => {
                v.pos -= 1;
                v.dir = Direction::Backward;
            },
            Direction::Forward => v.pos += 1,
            Direction::Backward if v.pos == 0 => {
                v.pos += 1;
                v.dir = Direction::Forward;
            }
            Direction::Backward => v.pos -= 1,
        }
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    const SAMPLE_INPUT: &'static str = r#"0: 3
1: 2
4: 4
6: 4"#;

    #[test]
    fn sample_input() {
        let c = Cursor::new(SAMPLE_INPUT);
        let mut f = super::parse_input(c);
        assert_eq!(4, f.len());
        assert_eq!(3, f.get(&0).unwrap().range);
        assert_eq!(2, f.get(&1).unwrap().range);
        assert_eq!(4, f.get(&4).unwrap().range);
        assert_eq!(4, f.get(&6).unwrap().range);

        assert_eq!(0, super::severity_at(&f, 0));
        assert_eq!(0, super::severity_at(&f, 2));
        assert_eq!(24, super::severity_at(&f, 6));

        let mut severity = 0;
        for pos in 0..10 {
            let add = super::severity_at(&f, pos);
            severity += add;
            super::tick(&mut f);
        }

        assert_eq!(24, severity);
    }
}
