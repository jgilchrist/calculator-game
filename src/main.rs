extern crate colored;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use colored::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "amount")]
enum Op {
    Add(i32),
    Subtract(i32),
    Multiply(i32),
    Divide(i32),
    Insert(i32),
    Backspace,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProblemDefinition {
    start: i32,
    goal: i32,
    moves: i32,
    ops: Vec<Op>,
}

#[derive(Clone, Debug)]
struct State {
    value: i32,
    moves_left: i32,
    ops_so_far: Vec<PastState>,
}

#[derive(Clone, Debug)]
struct PastState(Op, i32);

fn apply_op(value: i32, op: &Op) -> i32 {
    use Op::*;

    match *op {
        Add(n) => value + n,
        Subtract(n) => value - n,
        Multiply(n) => value * n,
        Divide(n) => value / n,
        Insert(n) => value * 10 + n,
        Backspace => value / 10,
    }
}

fn generate_next_state(state: &State, op: &Op) -> State {
    let value = apply_op(state.value, op);
    let moves_left = state.moves_left - 1;

    let mut ops_so_far = state.ops_so_far.clone();
    ops_so_far.push(PastState(op.clone(), value));

    State {
        value,
        moves_left,
        ops_so_far,
    }
}

fn generate_successors(state: &State, problem_definition: &ProblemDefinition) -> Vec<State> {
    problem_definition.ops.iter().map(|op| generate_next_state(state, op)).collect()
}

fn search(state: &State, problem_definition: &ProblemDefinition) {
    if state.value == problem_definition.goal {
        print_result(state, problem_definition);
    }
    if state.moves_left == 0 {
        return;
    }

    let states = generate_successors(state, problem_definition);
    for state in &states {
        search(state, problem_definition);
    }
}

fn get_problem_definition() -> ProblemDefinition {
    let filename = env::args()
        .nth(1)
        .expect("please specify a problem definition file");
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    toml::from_str(&contents).unwrap()
}

fn print_result(state: &State, problem_definition: &ProblemDefinition) {
    use Op::*;

    println!();
    println!(
        "{} can be reached from {} in {} moves",
        problem_definition.goal.to_string().blue(),
        problem_definition.start.to_string().blue(),
        state.ops_so_far.len().to_string().red(),
    );

    for past_state in &state.ops_so_far {
        let &PastState(ref op, ref value) = past_state;

        let formatted_op = match *op {
            Add(n) => format!("Add {}", n.to_string().green()),
            Subtract(n) => format!("Subtract {}", n.to_string().green()),
            Multiply(n) => format!("Multiply {}", n.to_string().green()),
            Divide(n) => format!("Divide {}", n.to_string().green()),
            Insert(n) => format!("Insert {}", n.to_string().green()),
            Backspace => format!("{}", "Backspace".green()),
        };

        println!("  - {:20} => {}", formatted_op, value.to_string().yellow());
    }

    println!();
}

fn main() {
    let problem_definition = get_problem_definition();
    println!("Problem definition: {:?}", problem_definition);

    let initial_state = State {
        value: problem_definition.start,
        moves_left: problem_definition.moves,
        ops_so_far: vec![],
    };
    println!("Initial state: {:?}", initial_state);

    search(&initial_state, &problem_definition);
}
