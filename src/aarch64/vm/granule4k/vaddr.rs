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
#[cfg(feature = "unstable")]
use core::iter::Step;

use super::consts::{BASE_PAGE_SIZE, HUGE_PAGE_SIZE, LARGE_PAGE_SIZE};
use crate::aarch64::vm::granule4k::VADDR_MAX;
use crate::aarch64::vm::{align_down, align_up};

/// A wrapper for a virtual address.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct VAddr(pub u64);

impl VAddr {
    /// Convert to `u64`
    pub const fn as_u64(self) -> u64 {
        self.0
    }

    /// Convert to `usize`
    pub const fn as_usize(self) -> usize {
        self.0 as usize
    }

    /// Convert to mutable pointer.
    pub fn as_mut_ptr<T>(self) -> *mut T {
        self.0 as *mut T
    }

    /// Convert to pointer.
    pub fn as_ptr<T>(self) -> *const T {
        self.0 as *const T
    }

    /// Virtual Address zero.
    pub const fn zero() -> Self {
        VAddr(0)
    }

    /// Is zero?
    pub fn is_zero(self) -> bool {
        self == VAddr::zero()
    }

    fn align_up<U>(self, align: U) -> Self
    where
        U: Into<u64>,
    {
        VAddr(align_up(self.0, align.into()))
    }

    fn align_down<U>(self, align: U) -> Self
    where
        U: Into<u64>,
    {
        VAddr(align_down(self.0, align.into()))
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

impl From<u64> for VAddr {
    fn from(num: u64) -> Self {
        VAddr(num)
    }
}

impl From<i32> for VAddr {
    fn from(num: i32) -> Self {
        assert!(num >= 0);
        VAddr(num as u64)
    }
}

#[allow(clippy::clippy::from_over_into)]
impl Into<u64> for VAddr {
    fn into(self) -> u64 {
        self.0
    }
}

impl From<usize> for VAddr {
    fn from(num: usize) -> Self {
        VAddr::from(num as u64)
    }
}

#[allow(clippy::clippy::from_over_into)]
impl Into<usize> for VAddr {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl ops::Add for VAddr {
    type Output = VAddr;

    fn add(self, rhs: VAddr) -> Self::Output {
        VAddr::from(self.0 + rhs.0)
    }
}

impl ops::Add<u64> for VAddr {
    type Output = VAddr;

    fn add(self, rhs: u64) -> Self::Output {
        VAddr::from(self.0 + rhs)
    }
}

impl ops::Add<usize> for VAddr {
    type Output = VAddr;

    fn add(self, rhs: usize) -> Self::Output {
        VAddr::from(self.0 + rhs as u64)
    }
}

impl ops::AddAssign for VAddr {
    fn add_assign(&mut self, other: VAddr) {
        *self = VAddr::from(self.0 + other.0);
    }
}

impl ops::AddAssign<u64> for VAddr {
    fn add_assign(&mut self, offset: u64) {
        *self = VAddr::from(self.0 + offset);
    }
}

impl ops::AddAssign<usize> for VAddr {
    fn add_assign(&mut self, offset: usize) {
        *self = VAddr::from(self.0 + offset as u64);
    }
}

impl ops::Sub for VAddr {
    type Output = VAddr;

    fn sub(self, rhs: VAddr) -> Self::Output {
        VAddr::from(self.0 - rhs.0)
    }
}

impl ops::Sub<u64> for VAddr {
    type Output = VAddr;

    fn sub(self, rhs: u64) -> Self::Output {
        VAddr::from(self.0 - rhs)
    }
}

impl ops::Sub<usize> for VAddr {
    type Output = VAddr;

    fn sub(self, rhs: usize) -> Self::Output {
        VAddr::from(self.0 - rhs as u64)
    }
}

impl ops::Rem for VAddr {
    type Output = VAddr;

    fn rem(self, rhs: VAddr) -> Self::Output {
        VAddr::from(self.0 % rhs.0)
    }
}

impl ops::Rem<u64> for VAddr {
    type Output = u64;

    fn rem(self, rhs: Self::Output) -> Self::Output {
        self.0 % rhs
    }
}

impl ops::Rem<usize> for VAddr {
    type Output = usize;

    fn rem(self, rhs: Self::Output) -> Self::Output {
        self.as_usize() % rhs
    }
}

impl ops::BitAnd for VAddr {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        VAddr::from(self.0 & rhs.0)
    }
}

impl ops::BitAnd<u64> for VAddr {
    type Output = VAddr;

    fn bitand(self, rhs: u64) -> Self::Output {
        VAddr::from(self.0 & rhs)
    }
}

impl ops::BitAnd<usize> for VAddr {
    type Output = VAddr;

    fn bitand(self, rhs: usize) -> Self::Output {
        VAddr::from(self.0 & rhs as u64)
    }
}

impl ops::BitAnd<i32> for VAddr {
    type Output = VAddr;

    fn bitand(self, rhs: i32) -> Self::Output {
        VAddr::from(self.0 & rhs as u64)
    }
}

impl ops::BitOr for VAddr {
    type Output = VAddr;

    fn bitor(self, rhs: VAddr) -> VAddr {
        VAddr::from(self.0 | rhs.0)
    }
}

impl ops::BitOr<u64> for VAddr {
    type Output = VAddr;

    fn bitor(self, rhs: u64) -> Self::Output {
        VAddr::from(self.0 | rhs)
    }
}

impl ops::BitOr<usize> for VAddr {
    type Output = VAddr;

    fn bitor(self, rhs: usize) -> Self::Output {
        VAddr::from(self.0 | rhs as u64)
    }
}

impl ops::Shr<u64> for VAddr {
    type Output = u64;

    fn shr(self, rhs: u64) -> Self::Output {
        self.0 >> rhs as u64
    }
}

impl ops::Shr<usize> for VAddr {
    type Output = u64;

    fn shr(self, rhs: usize) -> Self::Output {
        self.0 >> rhs as u64
    }
}

impl ops::Shr<i32> for VAddr {
    type Output = u64;

    fn shr(self, rhs: i32) -> Self::Output {
        self.0 >> rhs as u64
    }
}

impl fmt::Binary for VAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for VAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl fmt::Debug for VAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl fmt::LowerHex for VAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Octal for VAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::UpperHex for VAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Pointer for VAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use core::fmt::LowerHex;
        self.0.fmt(f)
    }
}

#[allow(clippy::clippy::derive_hash_xor_eq)]
impl Hash for VAddr {
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