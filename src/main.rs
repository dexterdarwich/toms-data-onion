use std::fs;

use helpers::get_layer_start_index;

mod helpers;
mod layer_one;
mod layer_zero;

fn main() {
    let payload_file = "payload".to_string();
    println!("Reading from input file: {}", payload_file);
    let payload = fs::read_to_string(payload_file).expect("Cannot read from input file");

    let index: usize = get_layer_start_index(&*payload);
    let layer_0 = &payload[index..];

    println!("Content of layer 0:");
    println!("{}", layer_0);
    let layer_1 = layer_zero::decode(&*layer_0).expect("Cannot decode layer zero");

    println!("Content of layer 1:");
    println!("{}", layer_1);
    let layer_2 = layer_one::decode(&*layer_1).expect("Cannot decode layer one");

    println!("Content of layer 2:");
    println!("{}", layer_2);
}
