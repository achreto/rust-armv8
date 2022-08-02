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
use core::marker::Copy;
use core::ops;

use super::consts::{BASE_PAGE_SIZE, HUGE_PAGE_SIZE, LARGE_PAGE_SIZE};
use crate::aarch64::vm::{align_down, align_up};

/// A wrapper for an IO address (IOVA / DMA Address for devices)
#[repr(transparent)]
#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct IOAddr(pub u64);

impl IOAddr {
    /// Convert to `u64`
    pub fn as_u64(self) -> u64 {
        self.0
    }

    /// Convert to `usize`
    pub fn as_usize(self) -> usize {
        self.0 as usize
    }

    /// IO Address zero.
    pub const fn zero() -> Self {
        IOAddr(0)
    }

    /// Is zero?
    pub fn is_zero(self) -> bool {
        self == IOAddr::zero()
    }

    /// Split `IOAddr` into lower and higher 32-bits.
    pub fn split(&self) -> (u32, u32) {
        (self.0 as u32, (self.0 >> 32) as u32)
    }

    fn align_up<U>(self, align: U) -> Self
    where
        U: Into<u64>,
    {
        IOAddr(align_up(self.0, align.into()))
    }

    fn align_down<U>(self, align: U) -> Self
    where
        U: Into<u64>,
    {
        IOAddr(align_down(self.0, align.into()))
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

impl From<u64> for IOAddr {
    fn from(num: u64) -> Self {
        IOAddr(num)
    }
}

impl From<usize> for IOAddr {
    fn from(num: usize) -> Self {
        IOAddr(num as u64)
    }
}

impl From<i32> for IOAddr {
    fn from(num: i32) -> Self {
        IOAddr(num as u64)
    }
}

#[allow(clippy::clippy::from_over_into)]
impl Into<u64> for IOAddr {
    fn into(self) -> u64 {
        self.0
    }
}

#[allow(clippy::clippy::from_over_into)]
impl Into<usize> for IOAddr {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl ops::Add for IOAddr {
    type Output = IOAddr;

    fn add(self, rhs: IOAddr) -> Self::Output {
        IOAddr(self.0 + rhs.0)
    }
}

impl ops::Add<u64> for IOAddr {
    type Output = IOAddr;

    fn add(self, rhs: u64) -> Self::Output {
        IOAddr::from(self.0 + rhs)
    }
}

impl ops::Add<usize> for IOAddr {
    type Output = IOAddr;

    fn add(self, rhs: usize) -> Self::Output {
        IOAddr::from(self.0 + rhs as u64)
    }
}

impl ops::AddAssign for IOAddr {
    fn add_assign(&mut self, other: IOAddr) {
        *self = IOAddr::from(self.0 + other.0);
    }
}

impl ops::AddAssign<u64> for IOAddr {
    fn add_assign(&mut self, offset: u64) {
        *self = IOAddr::from(self.0 + offset);
    }
}

impl ops::Sub for IOAddr {
    type Output = IOAddr;

    fn sub(self, rhs: IOAddr) -> Self::Output {
        IOAddr::from(self.0 - rhs.0)
    }
}

impl ops::Sub<u64> for IOAddr {
    type Output = IOAddr;

    fn sub(self, rhs: u64) -> Self::Output {
        IOAddr::from(self.0 - rhs)
    }
}

impl ops::Sub<usize> for IOAddr {
    type Output = IOAddr;

    fn sub(self, rhs: usize) -> Self::Output {
        IOAddr::from(self.0 - rhs as u64)
    }
}

impl ops::Rem for IOAddr {
    type Output = IOAddr;

    fn rem(self, rhs: IOAddr) -> Self::Output {
        IOAddr(self.0 % rhs.0)
    }
}

impl ops::Rem<u64> for IOAddr {
    type Output = u64;

    fn rem(self, rhs: u64) -> Self::Output {
        self.0 % rhs
    }
}

impl ops::Rem<usize> for IOAddr {
    type Output = u64;

    fn rem(self, rhs: usize) -> Self::Output {
        self.0 % (rhs as u64)
    }
}

impl ops::BitAnd for IOAddr {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        IOAddr(self.0 & rhs.0)
    }
}

impl ops::BitAnd<u64> for IOAddr {
    type Output = u64;

    fn bitand(self, rhs: u64) -> Self::Output {
        Into::<u64>::into(self) & rhs
    }
}

impl ops::BitOr for IOAddr {
    type Output = IOAddr;

    fn bitor(self, rhs: IOAddr) -> Self::Output {
        IOAddr(self.0 | rhs.0)
    }
}

impl ops::BitOr<u64> for IOAddr {
    type Output = u64;

    fn bitor(self, rhs: u64) -> Self::Output {
        self.0 | rhs
    }
}

impl ops::Shr<u64> for IOAddr {
    type Output = u64;

    fn shr(self, rhs: u64) -> Self::Output {
        self.0 >> rhs
    }
}

impl fmt::Binary for IOAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for IOAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Debug for IOAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl fmt::LowerHex for IOAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Octal for IOAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::UpperHex for IOAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Pointer for IOAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use core::fmt::LowerHex;
        self.0.fmt(f)
    }
}

#[allow(clippy::clippy::derive_hash_xor_eq)]
impl Hash for IOAddr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
