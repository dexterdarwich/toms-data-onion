use anyhow::{anyhow, Result};

/*
    Takes in an ASCII85 encoded string slice and returns a Result<Vec<u8>>
*/
pub(crate) fn decode(encoded: &str) -> Result<Vec<u8>> {
    ascii85::decode(encoded).map_err(|e| anyhow!(e.to_string()))
}

/*
    Get the index of the start of the layer's payload.
    The index is the start of ASCII85 '<~'
*/
pub(crate) fn get_layer_start_index(string: &str) -> usize {
    string
        .find("<~")
        .expect("Cannot find the start of the layer '<~' in payload")
}
