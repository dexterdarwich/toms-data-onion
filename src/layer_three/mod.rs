use anyhow::Result;

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
    let _vec: Vec<u8> = helpers::decode(encoded)?;
    Ok("".to_string())
}

#[cfg(test)]
mod tests {}
