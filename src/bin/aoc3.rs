/// Ofsset in (x, y) directions.
type Offset = (u64, u64);

fn main() {
    let input_str = std::env::args().nth(1).expect("an input number");
    let input = u64::from_str_radix(&input_str, 10).expect("an input number");

    let offset = calculate_offset(input);
    println!("offset ({}, {}), distance {}", offset.0, offset.1, offset.0+offset.1);
}

/// Calculate the offset from the centre cell for a given position p.
///
/// Let p be out input number and n,m ∈ ℕ the values at the top-bottom corner
/// for the immediately-smaller right and p's right resp.
///
/// The bottom-right values are given by f(x) = (2x+1)² for the xth ring
/// (starting at zero with the ring which only contains the 1). Its inverse
/// f⁻¹(x) = (√x - 1)/2 lets us go from a bottom-right corner value to the ring's
/// number.
///
/// Once we've found n < p ≤ m we know our position is in ring √l and can
/// calculate the values at all four corners of the ring. This will help us
/// figure where the ring our position lies.
///
/// Let r be the ring number and l = √m be the length of the ring in which the
/// position lies (which matches the ring's number). The offset from the centre
/// is however off = l-1 as we consider the distance from the central ring.
///
///     c_tr = n + l - 1
///     c_tl = c_tr + l - 1
///     c_bl = m - (l - 1)
///     c_br = m
///
/// We can now also figure out the centres of each side of the ring (as much as
/// square rings have centres).
///
///     c_r = c_tr - r
///     c_t = c_tl - r
///     c_l = c_bl - r
///     c_b = c_br - r
///
/// The live at offset (r, 0) or (0, r). For the other values one of the
/// directions is fixed at r but the other one depends on the difference between
/// it and the centre. We define
///
///     δ(a, b) | a < b = b - a
///             | else  = a - b
///
/// to give us this difference. We're finally ready to represent the offsets
/// from the centre cell.
///
/// 0. p ∈ {c_tr, c_tl, c_bl, c_br}: offset (r, r)
/// 1.    n < p < c_tr: offset (r, δ(c_r, p))
/// 2. c_tr < p < c_tl: offset (δ(c_t, p), r)
/// 3. c_tl < p < c_bl: offset (r, δ(c_l, p))
/// 4. c_bl < p < c_br: offset (δ(c_b, p), r)
pub fn calculate_offset(p: u64) -> Offset {
    let (n, m) = bottom_right_values(p);

    let l = (m as f64).sqrt() as u64;
    let r = ring_number(m);

    let c_tr = n + (l - 1);
    let c_tl = c_tr + (l - 1);
    let c_bl = m - (l - 1);
    let c_br = m;

    let c_r = c_tr - r;
    let c_t = c_tl - r;
    let c_l = c_bl - r;
    let c_b = c_br - r;

    if p == c_tr || p == c_tl || p == c_bl || p == c_br {
        return (r, r);
    }

    if n < p && p < c_tr {
        return (r, delta(c_r, p));
    }

    if c_tr < p && p < c_tl {
        return (delta(c_t, p), r)
    }

    if c_tl < p && p < c_bl {
        return (r, delta(c_l, p));
    }

    if c_bl < p && p < c_br {
        return (delta(c_b, p), r);
    }

    unreachable!();
}

fn bottom_right_values(p: u64) -> (u64, u64) {
    let mut i = 1;
    loop {
        if i*i >= p {
            return ((i-2)*(i-2), i*i)
        }

        // Skip the even numbers
        i += 2;
    }
}

/// This is f⁻¹(x) = (√x - 1)/2 described above
fn ring_number(x: u64) -> u64 {
    let root = (x as f64).sqrt() as u64;
    (root - 1)/ 2 as u64
}

/// This is δ(a, b) defined above
fn delta(a: u64, b: u64) -> u64 {
    if a < b {
        return b - a
    } else {
        return a - b
    }
}
