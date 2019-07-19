use glob::glob;
use std::fs::File;
use std::{fs, env};
use std::io::{BufReader, BufRead, Error};
use std::process;
use std::collections::HashMap;

fn state(path: &str) -> String {
    fs::read_to_string(path).expect("oops").split(" ").nth(2).unwrap().to_string()
}

fn stack(pid: u32) -> String {
    fs::read_to_string(format!("/proc/{}/stack", pid)).expect("Ooops")
}

fn top_stacks() -> HashMap<String, u32> {
    let mut stacks = HashMap::new();

    for entry in glob("/proc/[0-9]*").unwrap() {
        // TODO clean up this mess
        let entry = entry.unwrap();
        let pid = entry.to_str().unwrap().split("/proc/").nth(1).expect("pid");

        let path = format!("{}/stat", entry.display());
        let state = state(&path);
        // TODO: move this to state()
        // println!("path {:?} {:?}", state, path);
        if state == "D" {
            let current_stack = stack(pid.parse::<u32>().unwrap());
            let val = stacks.entry(current_stack).or_insert(0);
            *val += 1;
        }
    }
    stacks
}

fn states_stats() -> HashMap<String, u32> {
    let mut counter = HashMap::new();

    for entry in glob("/proc/[0-9]*").unwrap() {
        let entry = entry.unwrap();
        let path = format!("{}/stat", entry.display());
        let state = state(&path);

        let val = counter.entry(state).or_insert(0);
        *val += 1;
    }

    counter
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 || args[1] == "top" {
        println!("{:#?}", top_stacks());
    } else if args[1] == "stats" {
        println!("{:#?}", states_stats());
    }
}
