use binpacking_exercise::ryanfoster::RyanFoster;
use russcip::prelude::*;
use russcip::*;
use std::collections::HashMap;
use std::hash::Hash;
use binpacking_exercise::bnp::solve_binpacking;

fn main() {
    let capacity = 15.0;
    let item_sizes = &[6.0, 5.0, 4.0, 2.0, 3.0, 7.0, 5.0, 8.0, 4.0, 5.0];
    
    // Generate a random bin packing instance
    // let item_sizes = binpacking_exercise::generator::generate_binpacking(10, capacity);

    let (patterns, obj_val) = solve_binpacking(item_sizes, capacity);

    println!("Objective value: {}", obj_val);
    for pattern in &patterns {
        println!("Pattern: {:?}", pattern);
    }
}