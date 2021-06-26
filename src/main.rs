use std::{str, fs};
use std::string::String;

use anyhow::{Result, anyhow};

fn main() {
    let payload_file = "payload".to_string();
    println!("Reading from input file: {}", payload_file);
    let payload = fs::read_to_string(payload_file)
        .expect("Cannot read from input file");

    let index: usize = get_layer_start_index(&*payload);
    let layer_0 = &payload[index..];

    println!("Content of layer 0:");
    println!("{}", layer_0);
    let layer_1 = decode_layer_zero(&*layer_0)
        .expect("Cannot decode layer zero");

    println!("Content of layer 1:");
    println!("{}", layer_1);
}

/*
    Takes in an ASCII85 encoded string slice and returns a Result<Vec<u8>>
*/
fn decode(encoded: &str) -> Result<Vec<u8>> {
    ascii85::decode(encoded)
        .map_err(|e| anyhow!(e.to_string()))
}

/*
    Get the index of the start of the layer's payload.
    The index is the start of ASCII85 '<~'
*/
fn get_layer_start_index(string: &str) -> usize {
    string.find("<~")
        .expect("Cannot find the start of the layer '<~' in payload")
}

/*
Layer 0 instructions, payload file.

==[ Layer 0/6: ASCII85 ]====================================

ASCII85 is a binary-to-text encoding. These encodings are
useful when you need to send arbitrary binary data as text,
such as sending an image as an email attachment, or
embedding obfuscated data in a text file. It takes four
bytes of binary data, and converts them into five printable
ASCII characters. The encoding only uses 85 "safe" ASCII
characters, hence its name.

    ----------------------------------------------------

This payload has been encoded with Adobe-flavoured ASCII85.
All subsequent layers are ASCII85 encoded just like this
one, but they require additional processing in order to be
solved.

Decode the payload below to proceed!
 */
fn decode_layer_zero(encoded: &str) -> Result<String> {
    decode(encoded)
        .and_then(|vec| String::from_utf8(vec)
            .map_err(|e| anyhow!(e.to_string())))
}
