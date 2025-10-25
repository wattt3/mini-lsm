// Copyright (c) 2022-2025 Alex Chi Z
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

#![allow(unused_variables)] // TODO(you): remove this lint after implementing this mod
#![allow(dead_code)] // TODO(you): remove this lint after implementing this mod

mod builder;
mod iterator;

pub use builder::BlockBuilder;
use bytes::{Buf, BufMut, Bytes};
pub use iterator::BlockIterator;

pub(crate) const SIZE_OF_U16: usize = size_of::<u16>();

/// A block is the smallest unit of read and caching in LSM tree. It is a collection of sorted key-value pairs.
pub struct Block {
    pub(crate) data: Vec<u8>,
    pub(crate) offsets: Vec<u16>,
}

impl Block {
    // data + offset + the number of elements
    pub fn encode(&self) -> Bytes {
        let mut buf = self.data.clone();
        for &offset in &self.offsets {
            buf.put_u16(offset);
        }

        buf.put_u16(self.offsets.len() as u16);
        buf.into()
    }

    /// Decode from the data layout, transform the input `data` to a single `Block`
    pub fn decode(data: &[u8]) -> Self {
        let num_of_elements = (&data[data.len() - SIZE_OF_U16..]).get_u16() as usize;

        let offsets_offset = data.len() - SIZE_OF_U16 - num_of_elements * SIZE_OF_U16;
        let offsets_u8 = &data[offsets_offset..offsets_offset + num_of_elements * SIZE_OF_U16];
        let offsets = offsets_u8
            .chunks(SIZE_OF_U16)
            .map(|mut x| x.get_u16())
            .collect();

        let data = data[..offsets_offset].to_vec();

        Self { data, offsets }
    }
}
