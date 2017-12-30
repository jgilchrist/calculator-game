extern crate colored;
extern crate serde;
extern crate toml;

#[macro_use] extern crate serde_derive;

use std::env;
use std::fs::File;
use std::fmt::Display;
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
    Transform(i32, i32),
    Exponent(u32),
    Negate,
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
        Insert(n) => format!("{}{}", value, n).parse::<i32>().unwrap(),
        Transform(n, m) => str::replace(&value.to_string(), &n.to_string(), &m.to_string())
            .parse::<i32>()
            .unwrap(),
        Exponent(n) => value.pow(n),
        Negate => -value,
        Backspace => value / 10,
    }
}

fn generate_next_state(state: &State, op: &Op) -> State {
    let value = apply_op(state.value, op);

    let mut ops_so_far = state.ops_so_far.clone();
    ops_so_far.push(PastState(op.clone(), value));

    State {
        value,
        ops_so_far,
    }
}

fn generate_successors(state: &State, problem_definition: &ProblemDefinition) -> Vec<State> {
    problem_definition
        .ops
        .iter()
        .map(|op| generate_next_state(state, op))
        .collect()
}

fn search(state: &State, problem_definition: &ProblemDefinition, moves_left: i32) {
    if state.value == problem_definition.goal {
        print_result(state, problem_definition);
    }

    if moves_left == 0 {
        return;
    }

    let states = generate_successors(state, problem_definition);
    for state in &states {
        search(state, problem_definition, moves_left - 1);
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

fn format_action<T: Display>(action: &str, arg: T) -> String {
    format!("{}{}", action, arg.to_string().green())
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
            Add(n) => format_action("Add ", n),
            Subtract(n) => format_action("Subtract ", n),
            Multiply(n) => format_action("Multiply ", n),
            Divide(n) => format_action("Divide ", n),
            Insert(n) => format_action("Insert ", n),
            Transform(n, m) => format!(
                "Transform {} -> {}",
                n.to_string().green(),
                m.to_string().green()
            ),
            Exponent(n) => format_action("Exponent", n),
            Negate => format_action("", "Negate"),
            Backspace => format_action("", "Backspace"),
        };

        println!("  - {:20} => {}", formatted_op, value.to_string().yellow());
    }
}

fn main() {
    let problem_definition = get_problem_definition();
    println!("Problem definition: {:?}", problem_definition);

    let initial_state = State {
        value: problem_definition.start,
        ops_so_far: vec![],
    };

    println!("Initial state: {:?}", initial_state);

    search(&initial_state, &problem_definition, problem_definition.moves);
}
