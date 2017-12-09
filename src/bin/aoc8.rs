extern crate regex;

use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
struct InvalidCondition;

impl From<std::num::ParseIntError> for InvalidCondition {
    fn from(_e: std::num::ParseIntError) -> InvalidCondition {
        InvalidCondition
    }
}

#[derive(Debug)]
struct Condition {
    reg: String,
    op: Operator,
    value: i64,
}

#[derive(Debug)]
enum Operator {
    GreaterThan,
    GreaterThanOrEqualTo,
    SmallerThan,
    SmallerThanOrEqualTo,
    EqualTo,
    NotEqualTo,
}

impl Condition {
    fn from_strs(reg_s: &str, op_s: &str, size_s: &str, ) -> Result<Self, InvalidCondition> {
        let reg = reg_s.into();
        let size = i64::from_str_radix(size_s, 10)?;

        let op = match op_s {
            ">"  => Operator::GreaterThan,
            ">=" => Operator::GreaterThanOrEqualTo,
            "<"  => Operator::SmallerThan,
            "<=" => Operator::SmallerThanOrEqualTo,
            "==" => Operator::EqualTo,
            "!=" => Operator::NotEqualTo,
            _ => return Err(InvalidCondition),
        };

        Ok(Condition{
            reg: reg,
            op: op,
            value: size,
        })
    }
}

#[derive(Debug)]
struct InvalidOperation;

impl From<std::num::ParseIntError> for InvalidOperation {
    fn from(_e: std::num::ParseIntError) -> InvalidOperation {
        InvalidOperation
    }
}

#[derive(Debug)]
enum Operation {
    Inc(i64),
    Dec(i64),
}

impl Operation {
    fn from_strs(op_s: &str, size_s: &str) -> Result<Self, InvalidOperation> {
        let size = i64::from_str_radix(size_s, 10)?;

        match op_s {
            "inc" => Ok(Operation::Inc(size)),
            "dec" => Ok(Operation::Dec(size)),
            _ => Err(InvalidOperation),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    register: String,
    op: Operation,
    cond: Condition,
}

type Registers = HashMap<String, i64>;

fn main() {
    let mut registers: Registers = HashMap::new();

    let stdin = std::io::stdin();
    let insns = read_instructions(stdin.lock());

    // If everything's negative this wouldn't work but let's punt on that.
    let mut max_value = 0i64;

    for ins in &insns {
        execute(ins, &mut registers);
        if let Some((_, &new_max)) = registers.iter().max_by_key(|&(_k, v)| v) {
            if new_max > max_value {
                max_value = new_max;
            }
        }
    }

    println!("Largest value {:?}", registers.iter().max_by_key(|&(_k, v)| v));
    println!("Largest value at any point {:?}", max_value);
}

fn execute(i: &Instruction, regs: &mut Registers) {
    if !condition_matches(&i.cond, regs) {
        return;
    }

    let v = regs.entry(i.register.clone()).or_insert(0);
    match i.op {
        Operation::Inc(d) => *v += d,
        Operation::Dec(d) => *v -= d,
    }
}

fn condition_matches(c: &Condition, regs: &Registers) -> bool {
    let rv = match regs.get(&c.reg) {
        Some(v) => *v,
        None => 0,
    };
    let cv = c.value;

    match c.op {
        Operator::GreaterThan => rv > cv,
        Operator::GreaterThanOrEqualTo => rv >= cv,
        Operator::SmallerThan => rv < cv,
        Operator::SmallerThanOrEqualTo => rv <= cv,
        Operator::EqualTo => rv == cv,
        Operator::NotEqualTo => rv != cv,
    }
}

fn read_instructions<BR: std::io::BufRead>(r: BR) -> Vec<Instruction> {
    // I bet this is how Intel does it
    let expr = Regex::new(r"^(\w+) (\w+) (-?\d+) if (\w+) (.+) (-?\d+)$").expect("regex");

    r.lines().map(|line| {
        let line = &line.expect("line");
        let caps = expr.captures(line).expect("caps");

        let reg: String = caps.get(1).expect("1").as_str().into();
        let op = Operation::from_strs(caps.get(2).expect("2").as_str(),
                                      caps.get(3).expect("3").as_str()).expect("operation");
        let cond = Condition::from_strs(caps.get(4).expect("4").as_str(),
                                        caps.get(5).expect("5").as_str(),
                                        caps.get(6).expect("6").as_str()).expect("condition");

        Instruction{
            register: reg,
            op: op,
            cond: cond,
        }
    }).collect()
}
