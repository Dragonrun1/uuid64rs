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

mod uuid4 {
    use std::convert::TryFrom;

    use crate::*;

    /// Common input data for encoder tests.
    fn test_inputs_array_data() -> Vec<[u8; 16]> {
        vec![
            [0; 16],
            [255; 16],
            [15; 16],
            [240; 16],
            [1, 3, 5, 9, 17, 33, 65, 129, 129, 65, 33, 17, 9, 5, 3, 1],
            [0, 62, 63, 64, 65, 66, 67, 68, 69, 70, 30, 31, 32, 33, 34, 35],
            [61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 30, 31, 32, 33, 34, 0],
            [0, 0, 63, 64, 65, 66, 67, 68, 69, 70, 30, 31, 32, 33, 34, 35],
            [
                0, 0, 99, 100, 101, 102, 103, 104, 105, 106, 48, 49, 50, 51,
                52, 53,
            ],
            [
                0, 99, 100, 101, 102, 103, 104, 105, 106, 48, 49, 50, 51, 52,
                53, 0,
            ],
        ]
    }
    /// Common expect data for decode tests.
    fn test_expects_array_data() -> Vec<u128> {
        vec![
            0x00000000000000800040000000000000u128,
            0xffffffffffffffbfff4fffffffffffff,
            0x0f0f0f0f0f0f0f8f0f4f0f0f0f0f0f0f,
            0xf0f0f0f0f0f0f0b0f040f0f0f0f0f0f0,
            0x01030509112141818141211109050301,
            0x232221201f1e468544434241403f3e00,
            0x002221201f1e468544434241403f3e3d,
            0x232221201f1e468544434241403f0000,
            0x3534333231306aa96847666564630000,
            0x00353433323130aa6948676665646300,
        ]
    }

    #[test]
    fn it_should_create_a_valid_uuid_in_new_with_none() {
        let sut = Uuid4::new(None);
        let uuid = sut.as_uuid();
        let internal = sut.uuid0();
        assert_eq!(uuid.len(), 36);
        let sut = Uuid4::try_from(&*uuid).unwrap();
        assert_eq!(sut.uuid0(), internal)
    }
    #[test]
    fn it_should_correctly_decode_base64_from_str() {
        let inputs = vec![
            "AAAAAAAAAAgABAAAAAAAAA",
            "D_________v_9P________",
            "APDw8PDw8Pjw9PDw8PDw8P",
            "Dw8PDw8PDwsPBA8PDw8PDw",
            "ABAwUJESFBgYFBIREJBQMB",
            "AjIiEgHx5GhURDQkFAPz4A",
            "AAIiEgHx5GhURDQkFAPz49",
            "AjIiEgHx5GhURDQkFAPwAA",
            "A1NDMyMTBqqWhHZmVkYwAA",
            "AANTQzMjEwqmlIZ2ZlZGMA",
        ];
        let expects = test_expects_array_data();
        for (input, expected) in inputs.iter().zip(expects) {
            let sut = Uuid4::try_from(*input).unwrap();
            assert_eq!(sut.uuid0(), expected);
        }
    }
    #[test]
    fn it_should_correctly_decode_hex_string_from_str() {
        let inputs = vec![
            "00000000000000800040000000000000",
            "ffffffffffffffbfff4fffffffffffff",
            "0f0f0f0f0f0f0f8f0f4f0f0f0f0f0f0f",
            "f0f0f0f0f0f0f0b0f040f0f0f0f0f0f0",
            "01030509112141818141211109050301",
            "232221201f1e468544434241403f3e00",
            "002221201f1e468544434241403f3e3d",
            "232221201f1e468544434241403f0000",
            "3534333231306aa96847666564630000",
            "00353433323130aa6948676665646300",
        ];
        let expects = test_expects_array_data();
        for (input, expected) in inputs.iter().zip(expects) {
            let sut = Uuid4::try_from(*input).unwrap();
            assert_eq!(sut.uuid0(), expected);
        }
    }
    #[test]
    fn it_should_correctly_decode_uuid_from_str() {
        let inputs = vec![
            "00000000-0000-0080-0040-000000000000",
            "ffffffff-ffff-ffbf-ff4f-ffffffffffff",
            "0f0f0f0f-0f0f-0f8f-0f4f-0f0f0f0f0f0f",
            "f0f0f0f0-f0f0-f0b0-f040-f0f0f0f0f0f0",
            "01030509-1121-4181-8141-211109050301",
            "23222120-1f1e-4685-4443-4241403f3e00",
            "00222120-1f1e-4685-4443-4241403f3e3d",
            "23222120-1f1e-4685-4443-4241403f0000",
            "35343332-3130-6aa9-6847-666564630000",
            "00353433-3231-30aa-6948-676665646300",
        ];
        let expects = test_expects_array_data();
        for (input, expected) in inputs.iter().zip(expects) {
            let sut = Uuid4::try_from(*input).unwrap();
            assert_eq!(sut.uuid0(), expected);
        }
    }
    #[test]
    fn it_should_correctly_encode_based64() {
        let expects = vec![
            "AAAAAAAAAAgABAAAAAAAAA",
            "D_________v_9P________",
            "APDw8PDw8Pjw9PDw8PDw8P",
            "Dw8PDw8PDwsPBA8PDw8PDw",
            "ABAwUJESFBgYFBIREJBQMB",
            "AjIiEgHx5GhURDQkFAPz4A",
            "AAIiEgHx5GhURDQkFAPz49",
            "AjIiEgHx5GhURDQkFAPwAA",
            "A1NDMyMTBqqWhHZmVkYwAA",
            "AANTQzMjEwqmlIZ2ZlZGMA",
        ];
        let inputs = test_inputs_array_data();
        for (input, expected) in inputs.iter().zip(expects) {
            let sut: Uuid4 = input.into();
            let expected = String::from(expected);
            assert_eq!(sut.as_base64(), expected)
        }
    }
    #[test]
    fn it_should_correctly_encode_hex_string() {
        let expects = vec![
            "00000000000000800040000000000000",
            "ffffffffffffffbfff4fffffffffffff",
            "0f0f0f0f0f0f0f8f0f4f0f0f0f0f0f0f",
            "f0f0f0f0f0f0f0b0f040f0f0f0f0f0f0",
            "01030509112141818141211109050301",
            "232221201f1e468544434241403f3e00",
            "002221201f1e468544434241403f3e3d",
            "232221201f1e468544434241403f0000",
            "3534333231306aa96847666564630000",
            "00353433323130aa6948676665646300",
        ];
        let inputs = test_inputs_array_data();
        for (input, expected) in inputs.iter().zip(expects) {
            let sut: Uuid4 = input.into();
            let expected = String::from(expected);
            assert_eq!(sut.as_hex_string(), expected)
        }
    }
    #[test]
    fn it_should_correctly_encode_uuid() {
        let expects = vec![
            "00000000-0000-0080-0040-000000000000",
            "ffffffff-ffff-ffbf-ff4f-ffffffffffff",
            "0f0f0f0f-0f0f-0f8f-0f4f-0f0f0f0f0f0f",
            "f0f0f0f0-f0f0-f0b0-f040-f0f0f0f0f0f0",
            "01030509-1121-4181-8141-211109050301",
            "23222120-1f1e-4685-4443-4241403f3e00",
            "00222120-1f1e-4685-4443-4241403f3e3d",
            "23222120-1f1e-4685-4443-4241403f0000",
            "35343332-3130-6aa9-6847-666564630000",
            "00353433-3231-30aa-6948-676665646300",
        ];
        let inputs = test_inputs_array_data();
        for (input, expected) in inputs.iter().zip(expects) {
            let sut: Uuid4 = input.into();
            let expected = String::from(expected);
            assert_eq!(sut.as_uuid(), expected)
        }
    }
    #[test]
    fn it_should_have_valid_default() {
        let expected = "00000000-0000-0080-0040-000000000000";
        let sut = Uuid4::default();
        assert_eq!(sut.as_uuid(), expected)
    }
    #[test]
    fn it_should_return_error_when_decoding_bad_base64_str() {
        let input = "AAAAAAAAAAgABAAAAAAAA+";
        let expected = U64Error::InvalidBase64String;
        let sut = Uuid4::try_from(input).unwrap_err();
        assert_eq!(sut, expected);
    }
    #[test]
    fn it_should_return_error_when_decoding_bad_hex_str() {
        let input = "0000000000000080004000000000000Z";
        let expected = U64Error::InvalidHexString;
        let sut = Uuid4::try_from(input).unwrap_err();
        assert_eq!(sut, expected);
    }
    #[test]
    fn it_should_return_error_when_decoding_bad_uuid_str() {
        let input = "00000000-0000-0080-0040-00000000000Z";
        let expected = U64Error::InvalidUuidString;
        let sut = Uuid4::try_from(input).unwrap_err();
        assert_eq!(sut, expected);
    }
    #[test]
    fn it_should_junk() {
        let sut = Uuid4::default();
        let ser = serde_json::to_string(&sut).unwrap();
        let deser: Uuid4 = serde_json::from_str(&ser).unwrap();
        eprintln!("serialized = {}", ser);
        println!("deserialized = {:?}", deser);
    }
}
#[cfg(feature = "experimental")]
mod uuid4gen {
    use crate::*;

    #[test]
    fn it_should_generate_correct_length_values() {
        let sut = Uuid4Gen::new();
        assert_eq!(sut.gen_base64().len(), 22);
        assert_eq!(sut.gen_hex_string().len(), 32);
        assert_eq!(sut.gen_uuid().len(), 36);
    }
}
