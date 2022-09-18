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

mod consts;
mod ioaddr;
mod ipaddr;
mod paddr;
mod tables;
mod vaddr;

pub use consts::*;
pub use ioaddr::IOAddr;
pub use ipaddr::IPAddr;
pub use paddr::PAddr;
pub use tables::{L0Descriptor, L0Table};
pub use tables::{L1Descriptor, L1DescriptorTable, L1DescriptorBlock, L1Table};
pub use tables::{L2Descriptor, L2DescriptorTable, L2DescriptorBlock, L2Table};
pub use tables::{L3Descriptor, L3Table};
pub use vaddr::VAddr;

/// A type wrapping a base page with a 4 KiB buffer.
pub struct Page([u8; BASE_PAGE_SIZE]);

/// A type wrapping a large page with a 2 MiB buffer.
pub struct LargePage([u8; LARGE_PAGE_SIZE]);

/// A type wrapping a huge page with a 1 GiB buffer.
pub struct HugePage([u8; HUGE_PAGE_SIZE]);
