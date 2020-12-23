// Copyright © 2020-present, Michael Cummings
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// MIT License
//
// Copyright © 2020-present, Michael Cummings <mgcummings@yahoo.com>.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

pub use crate::{error::*, uuid4::*};
use std::{collections::HashMap, convert::TryInto};

mod error;
#[cfg(test)]
mod tests;
mod uuid4;

/// Core trait for the library.
pub trait Uuid {
    /// Provides direct access to the internal value of the uuid.
    ///
    /// This allows the other trait methods to access the value without knowing
    /// how or where it is actual kept.
    fn uuid0(&self) -> u128;
    /// Provides direct access to set the internal value of the uuid.
    ///
    /// This allows the other trait methods to write the value without knowing
    /// how or where it is actual kept.
    fn set_uuid0(&mut self, v: u128);
    /// Generate a custom base 64 encoded UUID v4 (random).
    fn as_base64(&self) -> Result<String, U64Error> {
        let mut map = HashMap::with_capacity(64);
        for (k, v) in Self::BASE64.iter() {
            map.insert(*k, *v);
        }
        let mut binary = String::from("0000");
        binary += &*format!("{:0>128b}", self.uuid0());
        let mut result = String::new();
        while binary.len() >= 6 {
            let (bits, remaining) = binary.split_at(6);
            match map.get(bits) {
                Some(v) => {
                    result.push(*v);
                    binary = remaining.to_string();
                }
                None => {
                    return Err(U64Error::UnknownBitPattern(bits.to_string()))
                }
            }
        }
        Ok(result)
    }
    /// Generate a hexadecimal encoded UUID v4 (random).
    fn as_hex_string(&self) -> String {
        format!("{:0>32x}", self.uuid0())
    }
    /// Generate a standard UUID v4 (random).
    ///
    /// The original PHP code for the function was found in answer at
    /// https://stackoverflow.com/questions/2040240/php-function-to-generate-v4-uuid
    /// by Arie which is based on a function found at
    /// http://php.net/manual/en/function.com-create-guid.php
    /// by pavel.volyntsev(at)gmail.
    ///
    /// There have been many other changes since the above code especially with
    /// translation to Rust.
    fn as_uuid(&self) -> Result<String, U64Error> {
        let byte_array: [u8; 32] =
            self.as_hex_string().as_bytes().try_into()?;
        let mut result = String::new();
        // 8, 4, 4, 4, 12 hex offsets into indexes
        // Trailing 0 insures no more matches to index and prevents '-' at end.
        let mut hyphens = [7usize, 11, 15, 19, 0].iter();
        let mut hyphen = *hyphens.next().unwrap();
        for (idx, byte) in byte_array.iter().enumerate() {
            result.push((*byte).into());
            if idx == hyphen {
                result.push('-');
                hyphen = *hyphens.next().unwrap();
            }
        }
        Ok(result)
    }
    /// An array use when decoding/encoding base64.
    const BASE64: [(&'static str, char); 64] = [
        ("000000", 'A'),
        ("000001", 'B'),
        ("000010", 'C'),
        ("000011", 'D'),
        ("000100", 'E'),
        ("000101", 'F'),
        ("000110", 'G'),
        ("000111", 'H'),
        ("001000", 'I'),
        ("001001", 'J'),
        ("001010", 'K'),
        ("001011", 'L'),
        ("001100", 'M'),
        ("001101", 'N'),
        ("001110", 'O'),
        ("001111", 'P'),
        ("010000", 'Q'),
        ("010001", 'R'),
        ("010010", 'S'),
        ("010011", 'T'),
        ("010100", 'U'),
        ("010101", 'V'),
        ("010110", 'W'),
        ("010111", 'X'),
        ("011000", 'Y'),
        ("011001", 'Z'),
        ("011010", 'a'),
        ("011011", 'b'),
        ("011100", 'c'),
        ("011101", 'd'),
        ("011110", 'e'),
        ("011111", 'f'),
        ("100000", 'g'),
        ("100001", 'h'),
        ("100010", 'i'),
        ("100011", 'j'),
        ("100100", 'k'),
        ("100101", 'l'),
        ("100110", 'm'),
        ("100111", 'n'),
        ("101000", 'o'),
        ("101001", 'p'),
        ("101010", 'q'),
        ("101011", 'r'),
        ("101100", 's'),
        ("101101", 't'),
        ("101110", 'u'),
        ("101111", 'v'),
        ("110000", 'w'),
        ("110001", 'x'),
        ("110010", 'y'),
        ("110011", 'z'),
        ("110100", '0'),
        ("110101", '1'),
        ("110110", '2'),
        ("110111", '3'),
        ("111000", '4'),
        ("111001", '5'),
        ("111010", '6'),
        ("111011", '7'),
        ("111100", '8'),
        ("111101", '9'),
        ("111110", '-'),
        ("111111", '_'),
    ];
}
