use anyhow::{anyhow, Result};

use crate::helpers;

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
pub(crate) fn decode(encoded: &str) -> Result<String> {
    helpers::decode(encoded)
        .and_then(|vec| String::from_utf8(vec).map_err(|e| anyhow!(e.to_string())))
}
