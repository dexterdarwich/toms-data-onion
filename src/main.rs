use std::fs;

use helpers::get_layer_start_index;

mod helpers;
mod layer_one;
mod layer_three;
mod layer_two;
mod layer_zero;

fn main() {
    let payload_file = "payload".to_string();
    println!("Reading from input file: {}", payload_file);
    let layer_0 = fs::read_to_string(payload_file).expect("Cannot read from input file");

    let index: usize = get_layer_start_index(&*layer_0);
    let layer_0_data = &layer_0[index..];

    //println!("Content of layer 0:");
    //println!("{}", layer_0);
    let layer_1 = layer_zero::decode(&*layer_0_data).expect("Cannot decode layer zero");

    //println!("Content of layer 1:");
    //println!("{}", layer_1);
    let index_layer1: usize = get_layer_start_index(&*layer_1);
    let layer_1_data = &layer_1[index_layer1..];
    let layer_2 = layer_one::decode(&*layer_1_data).expect("Cannot decode layer one");

    //println!("Content of layer 2:");
    //println!("{}", layer_2);
    let index_layer2: usize = get_layer_start_index(&*layer_2);
    let layer_2_data = &layer_2[index_layer2..];
    let layer_3 = layer_two::decode(&*layer_2_data).expect("Cannot decode layer two");

    //println!("Content of layer 3:");
    //println!("'{}'", layer_3);
    //println!("Layer3 as bytes: {:?}", layer_3.as_bytes());
    let index_layer3: usize = get_layer_start_index(&*layer_3);
    let layer_3_data = &layer_3[index_layer3..];
    let layer_4 = layer_three::decode(&*layer_3_data).expect("Cannot decode layer three");

    println!("Content of layer 4:");
    println!("'{}'", layer_4);
}
