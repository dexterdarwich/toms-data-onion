use anyhow::{anyhow, Result};

use crate::helpers;

/*
==[ Layer 3/6: XOR Encryption ]=============================

Exclusive Or (XOR) is another bitwise operation. It's often
used in cryptography to combine two sources of binary data
-- for example, to combine binary data with a secret key,
resulting in scrambled output data.

What makes XOR useful, compared to other bitwise operations
such as AND or OR, is that it can be reversed without losing
any information. If you know the output and one of the
inputs, you can determine what the other input was. It
enables encryption algorithms to be undone, so that data can
be decrypted back to its original state.

For example, let's say we have two input bytes, A and B, and
the result of XOR'ing these two bytes together is another
byte C:

    A XOR B == C

If we have bytes C and B, we're able to determine what A was
by XOR'ing together C and B:

    C XOR B == A

Likewise, if we have bytes C and A, XOR'ing them together
will produce B:

    C XOR A == B

Using XOR by itself to encrypt data is very, very insecure,
as you're about to discover. Good encryption algorithms
still use XOR at certain points, but they have many steps
with various different data transformations.

    ----------------------------------------------------

The payload has been encrypted by XOR'ing each byte with a
secret, cycling key. The key is 32 bytes of random data,
which I'm not going to give you. You will need to use your
hacker skills to discover what the key is, in order to
decrypt the payload.

For example, if it were a three byte secret key:

    Key = 0xAA 0xBB 0xCC

And the original data was seven bytes long:

    Original = 0x11 0x22 0x33 0x44 0x55 0x66 0x77

Then the key would be repeated (cycled) to match the length
of the payload, and then each byte from the key and the
payload would be XOR'd together to create the encrypted
payload.

     Original    Cycled Key    Encrypted
    -------------------------------------
       0x11   XOR   0xAA    ==    0xBB
       0x22   XOR   0xBB    ==    0x99
       0x33   XOR   0xCC    ==    0xFF
       0x44   XOR   0xAA    ==    0xEE
       0x55   XOR   0xBB    ==    0xEE
       0x66   XOR   0xCC    ==    0xAA
       0x77   XOR   0xAA    ==    0xDD


==[ Payload ]===============================================
*/
pub(crate) fn decode(encoded: &str) -> Result<String> {
    let encrypted: Vec<u8> = helpers::decode(encoded)?;

    // Got the key from the test below
    let key: Vec<u8> = vec![
        108, 36, 132, 142, 66, 25, 168, 225, 197, 219, 87, 101, 185, 198, 20, 158, 165, 25, 53,
        150, 59, 57, 127, 165, 101, 209, 254, 1, 133, 125, 217, 76,
    ];

    let mut decrypted_bytes: Vec<u8> = Vec::new();
    for i in 0..encrypted.len() {
        decrypted_bytes.push(encrypted[i] ^ key[i % 32]);
    }

    String::from_utf8(decrypted_bytes).map_err(|e| anyhow!(e.to_string()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn decrypt() {
        // First 62 bytes of the encrypted data that should look something similar to this:
        // "==[ Layer 3/6: XOR Encryption ]=============================\n\n"
        //  ^              ^                               ^
        //  |              |                               |________________
        //  |              |                               The second part of the key
        //  |              |_______________
        //  |               We do not know what text is in the next layer yet.
        //  |_____________  From the pattern, we can assume that the first decrypted part is:
        // "==[ Layer 4/6: " which is the first 15 chars (bytes) of the first line.
        //
        let encrypted: Vec<u8> = vec![
            81, 25, 223, 174, 14, 120, 209, 132, 183, 251, 99, 74, 143, 252, 52, 208, 192, 109, 66,
            249, 73, 82, 95, 241, 23, 176, 152, 103, 236, 30, 249, 17, 81, 25, 185, 179, 127, 36,
            149, 220, 248, 230, 106, 88, 132, 251, 41, 163, 152, 36, 8, 171, 6, 4, 66, 152, 88,
            236, 195, 60, 143, 119,
        ];

        // This is the infered first 15 bytes of the next layer
        let decrypted1 = "==[ Layer 4/6: ".to_string();
        let decrypted_bytes1 = decrypted1.as_bytes();
        // This is the infered last 15 bytes of the first line in the next layer
        // The reason we chose the last 15 bytes is because we have to take into consideration
        // the 32 bytes key alignment.
        let decrypted2 = "=============\n\n".to_string();
        let decrypted_bytes2 = decrypted2.as_bytes();

        // A ^ B = C EQ B ^ C = A EQ A ^ C = B

        let mut key: Vec<u8> = Vec::new();
        for i in 0..decrypted1.len() {
            key.push(encrypted[i] ^ decrypted_bytes1[i]);
        }
        for i in 0..decrypted2.len() {
            key.push(encrypted[i + 47] ^ decrypted_bytes2[i]);
        }

        assert_eq!(30, key.len());

        let mut decrypted_bytes: Vec<u8> = Vec::new();
        for i in 0..30 {
            decrypted_bytes.push(encrypted[i] ^ key[i % 32]);
        }
        let decrypted_string = String::from_utf8(decrypted_bytes).unwrap();
        assert_eq!(
            "==[ Layer 4/6: Network Traffic".to_string(),
            decrypted_string
        );

        // We still need to know what are the last two bytes of the key. So will try to
        // use ' ]' (a space and a square bracket) at the end of the first 30 characters
        // of the decrypted string and try if we can encrypt the whole data.
        let decrypted_from_test = "==[ Layer 4/6: Network Traffic ]".to_string();
        let decrypted_from_test_bytes = decrypted_from_test.as_bytes();
        let mut key: Vec<u8> = Vec::new();
        // A ^ B = C EQ B ^ C = A EQ A ^ C = B

        for i in 0..decrypted_from_test_bytes.len() {
            key.push(encrypted[i] ^ decrypted_from_test_bytes[i]);
        }

        // Use the output from this in the function above to decrypt the layer
        println!("Key: {:?}", key);

        let mut decrypted_bytes: Vec<u8> = Vec::new();
        for i in 0..encrypted.len() {
            decrypted_bytes.push(encrypted[i] ^ key[i % 32]);
        }
        let decrypted = String::from_utf8(decrypted_bytes).unwrap();
        println!("Decrypted: '{}'", decrypted);
    }
}
