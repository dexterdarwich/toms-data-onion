use std::u64;

use anyhow::{anyhow, Result};

use crate::helpers;

/*
==[ Layer 2/6: Parity Bit ]=================================

Parity bits are used to detect when data is being corrupted
-- for example, by a faulty cable. If the receiver of the
data notices that a parity bit is not correct, that
indicates that the data is being changed somehow.

    ----------------------------------------------------

For each byte of the payload, the seven most significant
bits carry data, and the least significant bit is the parity
bit. Combine the seven data bits from each byte where the
parity bit is correct, discarding bytes where the parity bit
is incorrect.

To determine if the parity bit is correct, first count how
many '1' bits exist within the seven data bits. If the count
is odd, the parity bit should be '1'. If the count is even,
the parity bit should be '0'.

For example, here is the byte 0xA3 (163 in decimal):

  1 0 1 0 0 0 1 1 <-- Parity bit (least significant bit)
  ^ ^ ^ ^ ^ ^ ^
  | | | | | | |
    Data bits

Of the data bits above, three of them are '1's. This is an
odd number, so the '1' parity bit is correct.

To make this layer a little bit easier, the byte size of the
payload is guaranteed to be a multiple of eight. Every group
of eight bytes contains 64 bits total, including 8 parity
bits. Removing the 8 parity bits leaves behind 56 data
bits, which is exactly 7 bytes.
*/
pub(crate) fn decode(encoded: &str) -> Result<String> {
    let vec: Vec<u8> = helpers::decode(encoded)?;
    let unfiltered_size = vec.len();
    let filtered: Vec<u8> = vec
        .into_iter()
        .filter(|byte| -> bool {
            let actual_parity: u8 = *byte & 0x01;
            let calculated_parity: u8 = parity(*byte);
            actual_parity == calculated_parity
        })
        .collect();
    println!("Original size: {}", unfiltered_size);
    println!("Filtered size: {}", filtered.len());
    let chunks: std::slice::Chunks<u8> = filtered.chunks(8);
    let mut combined: Vec<u8> = Vec::new();
    let mut _value: u8 = 0;
    println!("Chunks size: {}", chunks.len());

    for chunk in chunks {
        let mut byte_chunk: u64 = 0;
        byte_chunk |= (chunk[0] >> 1) as u64;
        byte_chunk = (byte_chunk << 7) | ((chunk[1] >> 1) as u64);
        byte_chunk = (byte_chunk << 7) | ((chunk[2] >> 1) as u64);
        byte_chunk = (byte_chunk << 7) | ((chunk[3] >> 1) as u64);
        byte_chunk = (byte_chunk << 7) | ((chunk[4] >> 1) as u64);
        byte_chunk = (byte_chunk << 7) | ((chunk[5] >> 1) as u64);
        byte_chunk = (byte_chunk << 7) | ((chunk[6] >> 1) as u64);
        byte_chunk = (byte_chunk << 7) | ((chunk[7] >> 1) as u64);

        combined.push((byte_chunk >> 48) as u8);
        combined.push((byte_chunk >> 40) as u8);
        combined.push((byte_chunk >> 32) as u8);
        combined.push((byte_chunk >> 24) as u8);
        combined.push((byte_chunk >> 16) as u8);
        combined.push((byte_chunk >> 8) as u8);
        combined.push(byte_chunk as u8);
    }

    String::from_utf8(combined).map_err(|e| anyhow!(e.to_string()))
}

fn parity(byte: u8) -> u8 {
    (((byte & 0x02) >> 1)          // 0b00000010
        + ((byte & 0x04) >> 2)     // 0b00000100
        + ((byte & 0x08) >> 3)     // 0b00001000
        + ((byte & 0x10) >> 4)     // 0b00010000
        + ((byte & 0x20) >> 5)     // 0b00100000
        + ((byte & 0x40) >> 6)     // 0b01000000
        + ((byte & 0x80) >> 7))    // 0b10000000
        % 2
}

#[cfg(test)]
mod tests {
    use crate::layer_two::parity;

    #[test]
    fn parity_test() {
        assert_eq!(1, parity(0b01000001));
        assert_eq!(1, parity(0b01010011));
        assert_eq!(0, parity(0b00101001));
        assert_eq!(0, parity(0b11101001));
        assert_eq!(0, parity(0b00101111));
    }
}
