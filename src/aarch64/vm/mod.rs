// MIT License
//
// Copyright (c) 2022 Reto Achermann
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

pub mod granule16k;
pub mod granule4k;
pub mod granule64k;

/// Align address downwards.
///
/// Returns the greatest x with alignment `align` so that x <= addr.
/// The alignment must be a power of 2.
#[inline(always)]
pub fn align_down(addr: u64, align: u64) -> u64 {
    addr & !(align - 1)
}

/// Align address upwards.
///
/// Returns the smallest x with alignment `align` so that x >= addr.
/// The alignment must be a power of 2.
#[inline(always)]
pub fn align_up(addr: u64, align: u64) -> u64 {
    let align_mask = align - 1;
    if addr & align_mask == 0 {
        addr
    } else {
        (addr | align_mask) + 1
    }
}
