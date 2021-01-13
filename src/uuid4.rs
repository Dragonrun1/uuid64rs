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

pub use crate::error::*;
use crate::Uuid;
use diesel::{
    backend::Backend,
    deserialize::{self, FromSql},
    serialize::{self, ToSql},
};
use diesel_derives::{AsExpression, FromSqlRow, SqlType};
use rand::{rngs::ThreadRng, Rng};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
    fmt,
    io::Write,
};

/// Minimum structure for implementing core trait.
///
/// It implements a lot of From and TryFrom traits to allow easy
/// interfacing with most any code and easy conversions between formats.
#[derive(
    AsExpression,
    Debug,
    Deserialize,
    Eq,
    FromSqlRow,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[sql_type = "Uuid4Proxy"]
pub struct Uuid4(u128);

impl Uuid4 {
    /// Construct a new random instance.
    ///
    /// ## Arguments
    /// * `rng` - Optional random number generator to save startup overhead when
    /// generating lots of new UUIDs or other custom needs.
    pub fn new<'a, TR>(rng: TR) -> Self
    where
        TR: Into<Option<&'a mut ThreadRng>>,
    {
        let mut v: u128;
        match rng.into() {
            Some(r) => v = r.gen(),
            None => v = rand::random(),
        }
        v &= 0xffffffffffffff3fff0fffffffffffff;
        v |= 0x00000000000000800040000000000000;
        Self(v)
    }
}

impl Uuid for Uuid4 {
    #[inline]
    fn uuid0(&self) -> u128 {
        self.0
    }
    #[inline]
    fn set_uuid0(&mut self, v: u128) {
        self.0 = v;
    }
}

impl Default for Uuid4 {
    fn default() -> Self {
        Self(0x00000000000000800040000000000000)
    }
}

impl fmt::Binary for Uuid4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.0;
        fmt::Binary::fmt(&val, f)
    }
}

impl fmt::LowerHex for Uuid4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.0;
        fmt::LowerHex::fmt(&val, f)
    }
}

impl fmt::UpperHex for Uuid4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.0;
        fmt::UpperHex::fmt(&val, f)
    }
}

impl From<u128> for Uuid4 {
    fn from(v: u128) -> Self {
        let mut v = v;
        v &= 0xffffffffffffff3fff0fffffffffffff;
        v |= 0x00000000000000800040000000000000;
        Self(v)
    }
}

impl From<&[u8; 16]> for Uuid4 {
    fn from(bytes: &[u8; 16]) -> Self {
        let mut result = u128::from_le_bytes(bytes.to_owned());
        result &= 0xffffffffffffff3fff0fffffffffffff;
        result |= 0x00000000000000800040000000000000;
        Self(result)
    }
}

impl TryFrom<&str> for Uuid4 {
    type Error = U64Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Uuid4::try_from(value.as_bytes())
    }
}

impl TryFrom<&[u8]> for Uuid4 {
    type Error = U64Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match value.len() {
            16 => {
                let val: &[u8; 16] = value[..16].try_into()?;
                Ok(val.into())
            }
            22 => {
                let val: &[u8; 22] = value[..22].try_into()?;
                Ok(val.try_into()?)
            }
            32 => {
                let val: &[u8; 32] = value[..32].try_into()?;
                Ok(val.try_into()?)
            }
            36 => {
                let val: &[u8; 36] = value[..36].try_into()?;
                Ok(val.try_into()?)
            }
            n => Err(U64Error::InvalidStrLength(n)),
        }
    }
}

impl TryFrom<&[u8; 22]> for Uuid4 {
    type Error = U64Error;
    fn try_from(value: &[u8; 22]) -> Result<Self, Self::Error> {
        let mut map = HashMap::with_capacity(64);
        for (v, k) in Self::BASE64.iter() {
            map.insert(*k, *v);
        }
        let mut bin = String::new();
        for char in value.iter() {
            let char = &char.to_owned().into();
            match map.get(char) {
                Some(n) => {
                    bin.push_str(*n);
                }
                None => return Err(U64Error::InvalidBase64String),
            }
        }
        // Drop the 4 fill bits that were add to have 22 chars.
        bin = bin.split_off(4);
        let mut result = u128::from_str_radix(&*bin, 2)
            .map_err(|_| U64Error::InvalidBinString)?;
        result = result.to_le();
        result &= 0xffffffffffffff3fff0fffffffffffff;
        result |= 0x00000000000000800040000000000000;
        Ok(Self(result))
    }
}

impl TryFrom<&[u8; 32]> for Uuid4 {
    type Error = U64Error;

    /// Converts an utf-8 hexadecimal byte array into a uuid4 value.
    ///
    /// __NOTE:__ _This function does NOT do any additional validating above
    /// what Rust needs to parse the bytes as a hexadecimal string._
    fn try_from(value: &[u8; 32]) -> Result<Self, Self::Error> {
        let utf = std::str::from_utf8(value)
            .map_err(|_| U64Error::InvalidUtf8String)?;
        let mut result = u128::from_str_radix(utf, 16)
            .map_err(|_| U64Error::InvalidHexString)?;
        result &= 0xffffffffffffff3fff0fffffffffffff;
        result |= 0x00000000000000800040000000000000;
        Ok(Self(result))
    }
}

impl TryFrom<&[u8; 36]> for Uuid4 {
    type Error = U64Error;

    /// Converts an utf-8 hexadecimal byte array into a uuid4 value.
    ///
    /// the first 4 '-' characters found in the `value` will be removed.
    ///
    /// __NOTE:__ _This function does NOT do any additional validating above
    /// what Rust needs to parse the bytes as a hexadecimal string._
    fn try_from(value: &[u8; 36]) -> Result<Self, Self::Error> {
        let utf = std::str::from_utf8(value)
            .map_err(|_| U64Error::InvalidUtf8String)?
            .replacen('-', "", 4);
        let mut result = u128::from_str_radix(&*utf, 16)
            .map_err(|_| U64Error::InvalidUuidString)?;
        result &= 0xffffffffffffff3fff0fffffffffffff;
        result |= 0x00000000000000800040000000000000;
        Ok(Self(result))
    }
}

impl<DB> FromSql<Uuid4Proxy, DB> for Uuid4
where
    DB: Backend<RawValue = [u8]>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        match bytes {
            Some(bytes) => Ok(bytes.try_into()?),
            None => Err(Box::new(diesel::result::UnexpectedNullError)),
        }
    }
}

impl<DB> ToSql<Uuid4Proxy, DB> for Uuid4
where
    DB: Backend,
    String: ToSql<Uuid4Proxy, DB>,
{
    fn to_sql<W: Write>(
        &self,
        out: &mut serialize::Output<W, DB>,
    ) -> serialize::Result {
        self.as_base64().to_sql(out)
    }
}

#[derive(SqlType)]
pub struct Uuid4Proxy;
