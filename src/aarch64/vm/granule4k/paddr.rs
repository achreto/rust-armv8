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

use core::convert::{From, Into};
use core::fmt;
use core::hash::{Hash, Hasher};
#[cfg(feature = "unstable")]
use core::iter::Step;
use core::marker::Copy;
use core::ops;

use super::consts::{BASE_PAGE_SIZE, HUGE_PAGE_SIZE, LARGE_PAGE_SIZE, PADDR_MAX};
use crate::aarch64::vm::granule4k::VAddr;
use crate::aarch64::vm::KERNEL_OFFSET;
use crate::aarch64::vm::{align_down, align_up};

/// A wrapper for a physical address.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct PAddr(pub u64);

impl PAddr {
    /// Convert to `u64`
    pub fn as_u64(self) -> u64 {
        self.0
    }

    /// Convert to `usize`
    pub fn as_usize(self) -> usize {
        self.0 as usize
    }

    /// Physical Address zero.
    pub const fn zero() -> Self {
        PAddr(0)
    }

    /// Is zero?
    pub fn is_zero(self) -> bool {
        self == PAddr::zero()
    }

    /// Split `PAddr` into lower and higher 32-bits.
    pub fn split(&self) -> (u32, u32) {
        (self.0 as u32, (self.0 >> 32) as u32)
    }

    fn align_up<U>(self, align: U) -> Self
    where
        U: Into<u64>,
    {
        PAddr(align_up(self.0, align.into()))
    }

    fn align_down<U>(self, align: U) -> Self
    where
        U: Into<u64>,
    {
        PAddr(align_down(self.0, align.into()))
    }

    /// Offset within the 4 KiB page.
    pub fn base_page_offset(self) -> u64 {
        self.0 & (BASE_PAGE_SIZE as u64 - 1)
    }

    /// Offset within the 2 MiB page.
    pub fn large_page_offset(self) -> u64 {
        self.0 & (LARGE_PAGE_SIZE as u64 - 1)
    }

    /// Offset within the 1 GiB page.
    pub fn huge_page_offset(self) -> u64 {
        self.0 & (HUGE_PAGE_SIZE as u64 - 1)
    }

    /// Return address of nearest 4 KiB page (lower or equal than self).
    pub fn align_down_to_base_page(self) -> Self {
        self.align_down(BASE_PAGE_SIZE as u64)
    }

    /// Return address of nearest 2 MiB page (lower or equal than self).
    pub fn align_down_to_large_page(self) -> Self {
        self.align_down(LARGE_PAGE_SIZE as u64)
    }

    /// Return address of nearest 1 GiB page (lower or equal than self).
    pub fn align_down_to_huge_page(self) -> Self {
        self.align_down(HUGE_PAGE_SIZE as u64)
    }

    /// Return address of nearest 4 KiB page (higher or equal than self).
    pub fn align_up_to_base_page(self) -> Self {
        self.align_up(BASE_PAGE_SIZE as u64)
    }

    /// Return address of nearest 2 MiB page (higher or equal than self).
    pub fn align_up_to_large_page(self) -> Self {
        self.align_up(LARGE_PAGE_SIZE as u64)
    }

    /// Return address of nearest 1 GiB page (higher or equal than self).
    pub fn align_up_to_huge_page(self) -> Self {
        self.align_up(HUGE_PAGE_SIZE as u64)
    }

    /// Is this address aligned to a 4 KiB page?
    pub fn is_base_page_aligned(self) -> bool {
        self.align_down(BASE_PAGE_SIZE as u64) == self
    }

    /// Is this address aligned to a 2 MiB page?
    pub fn is_large_page_aligned(self) -> bool {
        self.align_down(LARGE_PAGE_SIZE as u64) == self
    }

    /// Is this address aligned to a 1 GiB page?
    pub fn is_huge_page_aligned(self) -> bool {
        self.align_down(HUGE_PAGE_SIZE as u64) == self
    }

    /// Is this address aligned to `align`?
    ///
    /// # Note
    /// `align` must be a power of two.
    pub fn is_aligned<U>(self, align: U) -> bool
    where
        U: Into<u64> + Copy,
    {
        if !align.into().is_power_of_two() {
            return false;
        }

        self.align_down(align) == self
    }
}

impl From<u64> for PAddr {
    fn from(num: u64) -> Self {
        PAddr(num & PADDR_MAX)
    }
}

impl From<usize> for PAddr {
    fn from(num: usize) -> Self {
        PAddr(num as u64 & PADDR_MAX)
    }
}

impl From<i32> for PAddr {
    fn from(num: i32) -> Self {
        PAddr(num as u64)
    }
}

impl From<VAddr> for PAddr {
    fn from(num: VAddr) -> Self {
        // this assumes that we have a 1:1 mapping between virtual and physical addresses
        // if the address is in the high-addresses then we subtract the kernel offset
        // other wise we just keep the address as-is.
        let vaddr = num.as_u64();
        unsafe {
            if vaddr < KERNEL_OFFSET {
                PAddr(vaddr)
            } else {
                PAddr(vaddr - KERNEL_OFFSET)
            }
        }
    }
}

#[allow(clippy::clippy::from_over_into)]
impl Into<u64> for PAddr {
    fn into(self) -> u64 {
        self.0
    }
}

#[allow(clippy::clippy::from_over_into)]
impl Into<usize> for PAddr {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl ops::Add for PAddr {
    type Output = PAddr;

    fn add(self, rhs: PAddr) -> Self::Output {
        PAddr::from(self.0 + rhs.0)
    }
}

impl ops::Add<u64> for PAddr {
    type Output = PAddr;

    fn add(self, rhs: u64) -> Self::Output {
        PAddr::from(self.0 + rhs)
    }
}

impl ops::Add<usize> for PAddr {
    type Output = PAddr;

    fn add(self, rhs: usize) -> Self::Output {
        PAddr::from(self.0 + rhs as u64)
    }
}

impl ops::AddAssign for PAddr {
    fn add_assign(&mut self, other: PAddr) {
        *self = PAddr::from(self.0 + other.0);
    }
}

impl ops::AddAssign<u64> for PAddr {
    fn add_assign(&mut self, offset: u64) {
        *self = PAddr::from(self.0 + offset);
    }
}

impl ops::Sub for PAddr {
    type Output = PAddr;

    fn sub(self, rhs: PAddr) -> Self::Output {
        PAddr::from(self.0 - rhs.0)
    }
}

impl ops::Sub<u64> for PAddr {
    type Output = PAddr;

    fn sub(self, rhs: u64) -> Self::Output {
        PAddr::from(self.0 - rhs)
    }
}

impl ops::Sub<usize> for PAddr {
    type Output = PAddr;

    fn sub(self, rhs: usize) -> Self::Output {
        PAddr::from(self.0 - rhs as u64)
    }
}

impl ops::Rem for PAddr {
    type Output = PAddr;

    fn rem(self, rhs: PAddr) -> Self::Output {
        PAddr::from(self.0 % rhs.0)
    }
}

impl ops::Rem<u64> for PAddr {
    type Output = u64;

    fn rem(self, rhs: u64) -> Self::Output {
        self.0 % rhs
    }
}

impl ops::Rem<usize> for PAddr {
    type Output = u64;

    fn rem(self, rhs: usize) -> Self::Output {
        self.0 % (rhs as u64)
    }
}

impl ops::BitAnd for PAddr {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        PAddr::from(self.0 & rhs.0)
    }
}

impl ops::BitAnd<u64> for PAddr {
    type Output = u64;

    fn bitand(self, rhs: u64) -> Self::Output {
        Into::<u64>::into(self) & rhs
    }
}

impl ops::BitOr for PAddr {
    type Output = PAddr;

    fn bitor(self, rhs: PAddr) -> Self::Output {
        PAddr::from(self.0 | rhs.0)
    }
}

impl ops::BitOr<u64> for PAddr {
    type Output = u64;

    fn bitor(self, rhs: u64) -> Self::Output {
        self.0 | rhs
    }
}

impl ops::Shr<u64> for PAddr {
    type Output = u64;

    fn shr(self, rhs: u64) -> Self::Output {
        self.0 >> rhs
    }
}

impl fmt::Binary for PAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for PAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Debug for PAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl fmt::LowerHex for PAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Octal for PAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::UpperHex for PAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Pointer for PAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use core::fmt::LowerHex;
        self.0.fmt(f)
    }
}

#[allow(clippy::clippy::derive_hash_xor_eq)]
impl Hash for PAddr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[cfg(feature = "unstable")]
impl Step for PAddr {
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        <u64 as Step>::steps_between(&start.0, &end.0)
    }
    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        <u64 as Step>::forward_checked(start.0, count).map(|v| PAddr(v))
    }
    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        <u64 as Step>::backward_checked(start.0, count).map(|v| PAddr(v))
    }
}
