/// We have a hex grid, but that doesn't work too well with x and y coords, so
/// we overlay a square grid on top with each square being half the size of a
/// hexagon. Each row then represents alternating rows of hexagons, whose centre
/// points are a height apart.
///
/// Thus, moving n is going two up, (+2 on y) and moving nw is one in each
/// direction.


use std::str::FromStr;

/// Cardinal directions, except that we cannot move east or west in our
/// hexagonal grid pattern.
#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    N,
    NE,
    SE,
    S,
    SW,
    NW,
}

#[derive(Debug)]
struct InvalidDirection;

impl FromStr for Direction {
    type Err = InvalidDirection;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let d = match s {
            "n"  => Direction::N,
            "ne" => Direction::NE,
            "se" => Direction::SE,
            "s"  => Direction::S,
            "sw" => Direction::SW,
            "nw" => Direction::NW,
            _ => return Err(InvalidDirection),
        };

        Ok(d)
    }
}

/// A point on the overlay grid
#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0, y: 0 }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut raw_input = String::new();
    stdin.read_line(&mut raw_input).expect("input line");
    let line = raw_input.trim();

    let directions = line
        .split(',')
        .map(|d| Direction::from_str(d).expect("direction"))
        .collect::<Vec<Direction>>();

    let end = directions.iter().fold(Point::origin(), |acc, &x| move_in_direction(acc, x));
    let moves = moves_to_reach(end);

    println!("end pos: {:?}", end);
    println!("moves: {}", moves);
}

/// Calculate how many movements it would take to reach the given point from the
/// origin.
fn moves_to_reach(target: Point) -> u64 {
    let mut count = 0u64;
    let mut p = Point::origin();

    while p != target {
        // Choose a direction in which to travel. If both x and y disagree, move
        // diagnoally, otherwise move vertically.
        let d = if target.x < p.x && target.y < p.y {
            Direction::SW
        } else if target.x < p.x && target.y > p.y {
            Direction::SE
        } else if target.x > p.x && target.y < p.y {
            Direction::NW
        } else if target.x > p.x && target.y > p.y {
            Direction::NE
        } else if target.x > p.x {
            Direction::N
        } else if target.x < p.x {
            Direction::S
        } else {
            // Here we're already aligned horizontally, so all we can do is
            // nw,sw or ne,se sequences to move horizontally. At this point, we
            // now that we need to add twice the delta between our y and the
            // target's.
            let delta = if target.y > p.y { target.y - p.y } else { p.y - target.y } as u64;
            return count + delta;

        };

        p = move_in_direction(p, d);
        count += 1;
    }

    count
}

fn move_in_direction(p: Point, d: Direction) -> Point {
    match d {
        Direction::N  => Point { x: p.x+2, y: p.y },
        Direction::NE => Point { x: p.x+1, y: p.y+1 },
        Direction::SE => Point { x: p.x-1, y: p.y+1 },
        Direction::S  => Point { x: p.x-2, y: p.y },
        Direction::SW => Point { x: p.x-1, y: p.y-1 },
        Direction::NW => Point { x: p.x+1, y: p.y-1 },
    }
}
