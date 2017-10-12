#[macro_use]
extern crate text_io;

use std::str::FromStr;

fn main() {
    loop {
        println!("enter space separated ops (\n\tadd:\t[+n]\tsub:\t[-n]\n\tmul:\t[xn]\tdiv:\t[/n]\n\tlshift:\t[<]\t\trshift:\t[>n]\n\treplace:[:a,b]\tsquare:\t[^]\n\tneg:\t[$]\n):");
        let ops_string: String = read!("{}\n");
        println!("read: {}", ops_string);
        let ops = op_vec_from_str(&ops_string);
        println!("enter goal:");
        let goal = read!();
        println!("enter moves:");
        let moves = read!();
        println!("enter starting value:");
        let starting_value = read!();
        let iter_limit = 100_000_000;

        match solve(ops, starting_value, moves, goal, iter_limit) {
            Some(mut path) => println!("Solution: {:?}", path),
            None => println!("no solution found")
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Add(f32),
    Sub(f32),
    Mul(f32),
    Div(f32),
    RShift(f32),
    LShift,
    Replace(f32, f32),
    Raise(i32),
    Negate
}

impl Op {
    fn apply(&self, arg: f32) -> f32 {
        use Op::*;
        match *self {
            Add(rhs) => arg + rhs,
            Sub(rhs) => arg - rhs,
            Mul(rhs) => arg * rhs,
            Div(rhs) => arg / rhs,
            LShift => {
                let mut string: String = arg.to_string();
                string.pop();
                string.parse().unwrap_or(0.0)
            }
            RShift(rhs) => {
                format!("{}{}", arg, rhs).parse().unwrap_or(0.0)
            }
            Replace(x, y) => {
                let string: String = arg.to_string();
                let string = string.replace(&x.to_string(), &y.to_string());
                string.parse().unwrap()
            },
            Raise(exponent) => arg.powi(exponent),
            Negate => -arg
        }
    }
}

fn op_from_str(s: &str) -> Op {
    println!("{}", s);
    use Op::*;
    let first_char = s.chars().next().unwrap();
    let remainder: String = s.chars().skip(1).collect();
    match first_char {
        '+' => Add(remainder.parse().unwrap()),
        '-' => Sub(remainder.parse().unwrap()),
        'x' => Mul(remainder.parse().unwrap()),
        '/' => Div(remainder.parse().unwrap()),
        '<' => LShift,
        '>' => RShift(remainder.parse().unwrap()),
        ':' => {
            let xy: Vec<&str> = remainder.split(',').collect();
            let x = xy[0].parse().unwrap();
            let y = xy[1].parse().unwrap();
            println!("{}, {}", x, y);
            Replace(x, y)
        },
        '^' => Raise(2),
        '$' => Negate,
        _ => panic!()
    }
}

fn op_vec_from_str(s: &str) -> Vec<Op> {
    s.split(' ').map(op_from_str).collect()
}

#[derive(Debug, Clone)]
struct StackState {
    moves_remaining: i32,
    value: f32,
    op: Op,
    parent: Option<Box<StackState>>
}

fn solve(ops: Vec<Op>, starting_value: f32, max_moves: i32, goal: f32, iter_limit: i32) -> Option<Vec<(f32, Op)>> {
    let mut stack: Vec<StackState> = Vec::new();

    for op in ops.iter() {
        stack.push(StackState {
            moves_remaining: max_moves,
            value: starting_value,
            op: *op,
            parent: None
        });
    }

    for i in 0..iter_limit {
        let current = match stack.pop() {
            Some(state) => state,
            None => {
                println!("exhaustive search complete. {} iters", i);
                break
            }
        };

        if current.moves_remaining == 0 {
            if current.value == goal {
                let mut solution = Vec::new();
                let mut head = Box::new(current);
                while head.parent.is_some() {
                    solution.push((head.value,head.op));
                    head = head.parent.unwrap();
                }
                println!("finished in {} iterations", i);return Some(solution);
            } else {
                continue;
            }
        }

        for op in ops.iter() {
            stack.push(StackState {
                moves_remaining: current.moves_remaining - 1,
                value: op.apply(current.value),
                op: *op,
                parent: Some(Box::new(current.clone()))
            });
        }
    }

    None
}
