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
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//! A common set of error and result type used in the library.

use std::array::TryFromSliceError;
use thiserror::Error;

/// Provides a shared set of error types.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum U64Error {
    #[error("Expected data array length of 16 but was given length: {0}")]
    InvalidArrayLength(usize),
    #[error("The given binary string contained one or more invalid digits")]
    InvalidBinString,
    #[error(
        "The given hexadecimal string contained one or more invalid digits"
    )]
    InvalidHexString,
    #[error(transparent)]
    InvalidSliceLength(#[from] TryFromSliceError),
    #[error("The given string contained one or more invalid UTF-8 characters")]
    InvalidUtf8String,
    #[error(
        "The given uuid64 string contained one or more invalid characters"
    )]
    InvalidUuid64String,
    #[error("Can not convert a string with length of: {0}")]
    InvalidStrLength(usize),
    #[error("Received unknown bit pattern: {0}")]
    UnknownBitPattern(String),
}

impl From<U64Error> for std::num::ParseIntError {
    fn from(ue: U64Error) -> Self {
        ue.into()
    }
}

impl From<U64Error> for std::str::Utf8Error {
    fn from(ue: U64Error) -> Self {
        ue.into()
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
}
