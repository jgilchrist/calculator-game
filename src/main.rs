extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "amount")]
enum Op {
    Add(i32),
    Subtract(i32),
    Multiply(i32),
    Divide(i32),
    Backspace,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProblemDefinition {
    start: i32,
    goal: i32,
    moves: i32,
    ops: Vec<Op>,
}

struct State {
    current: i32,
    moves_left: i32,
    ops_so_far: Vec<Op>,
}

fn apply_op(value: i32, op: Op) -> i32 {
    use Op::*;

    match op {
        Add(n) => value + n,
        Subtract(n) => value - n,
        Multiply(n) => value * n,
        Divide(n) => value / n,
        Backspace => value / 10,
    }
}

fn generate_successors() {}

fn get_problem_definition() -> ProblemDefinition {
    let filename = env::args().nth(1).unwrap();
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    toml::from_str(&contents).unwrap()
}

fn main() {
    let problem_definition = get_problem_definition();
    println!("Problem definition: {:?}", problem_definition);

    let initial_state = State {
        current: problem_definition.start,
        moves_left: problem_definition.moves,
        ops_so_far: vec![],
    };
}
