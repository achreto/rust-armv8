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

use core::arch::asm;

pub fn sp() -> u64 {
    let mut spval: u64 = 0;
    unsafe {
        asm!("mov {spval}, sp\n\t", spval  = out(reg) spval);
    }
    return spval;
}

pub fn fp() -> u64 {
    let mut fpval: u64 = 0;
    unsafe {
        asm!("mov {fpval}, x29\n\t", fpval  = out(reg) fpval);
    }
    return fpval;
}

pub fn lr() -> u64 {
    let mut lrval: u64 = 0;
    unsafe {
        asm!("mov {lrval}, x30\n\t",  lrval = out(reg) lrval);
    }
    return lrval;
}
