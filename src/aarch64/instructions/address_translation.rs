/*
 * MIT License
 *
 * Copyright (c) 2022 Reto Achermann
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 * SPDX-License-Identifier: MIT
 */

/**************************************************************************************************
 *
 * !!!! WARNING: THIS FILE IS AUTO GENERATED. ANY CHANGES MAY BE OVERWRITTEN !!!!
 *
 * Generated on: 2022-08-22T16:35:53.084734
 * Version: Armv8.7-A-2020-09
 * Source: https://developer.arm.com/-/media/developer/products/architecture/armv8-a-architecture/2020-09/SysReg_xml_v87A-2020-09.tar.gz
 *
 * !!!! WARNING: THIS FILE IS AUTO GENERATED. ANY CHANGES MAY BE OVERWRITTEN !!!!
 *
 *************************************************************************************************/
use core::arch::asm;

/*
 * ================================================================================================
 * Address Translate Stages 1 and 2 EL0 Read
 * ================================================================================================
 */

/// Address Translate Stages 1 and 2 EL0 Read
#[inline(always)]
pub fn at_s12e0r(arg: u64) {
    unsafe {
        asm!("at s12e0r, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Address Translate Stages 1 and 2 EL0 Write
 * ================================================================================================
 */

/// Address Translate Stages 1 and 2 EL0 Write
#[inline(always)]
pub fn at_s12e0w(arg: u64) {
    unsafe {
        asm!("at s12e0w, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Address Translate Stages 1 and 2 EL1 Read
 * ================================================================================================
 */

/// Address Translate Stages 1 and 2 EL1 Read
#[inline(always)]
pub fn at_s12e1r(arg: u64) {
    unsafe {
        asm!("at s12e1r, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Address Translate Stages 1 and 2 EL1 Write
 * ================================================================================================
 */

/// Address Translate Stages 1 and 2 EL1 Write
#[inline(always)]
pub fn at_s12e1w(arg: u64) {
    unsafe {
        asm!("at s12e1w, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Address Translate Stage 1 EL0 Read
 * ================================================================================================
 */

/// Address Translate Stage 1 EL0 Read
#[inline(always)]
pub fn at_s1e0r(arg: u64) {
    unsafe {
        asm!("at s1e0r, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Address Translate Stage 1 EL0 Write
 * ================================================================================================
 */

/// Address Translate Stage 1 EL0 Write
#[inline(always)]
pub fn at_s1e0w(arg: u64) {
    unsafe {
        asm!("at s1e0w, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Address Translate Stage 1 EL1 Read
 * ================================================================================================
 */

/// Address Translate Stage 1 EL1 Read
#[inline(always)]
pub fn at_s1e1r(arg: u64) {
    unsafe {
        asm!("at s1e1r, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Address Translate Stage 1 EL1 Read PAN
 * ================================================================================================
 */

/// Address Translate Stage 1 EL1 Read PAN
#[inline(always)]
pub fn at_s1e1rp(arg: u64) {
    unsafe {
        asm!("at s1e1rp, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Address Translate Stage 1 EL1 Write
 * ================================================================================================
 */

/// Address Translate Stage 1 EL1 Write
#[inline(always)]
pub fn at_s1e1w(arg: u64) {
    unsafe {
        asm!("at s1e1w, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Address Translate Stage 1 EL1 Write PAN
 * ================================================================================================
 */

/// Address Translate Stage 1 EL1 Write PAN
#[inline(always)]
pub fn at_s1e1wp(arg: u64) {
    unsafe {
        asm!("at s1e1wp, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Address Translate Stage 1 EL2 Read
 * ================================================================================================
 */

/// Address Translate Stage 1 EL2 Read
#[inline(always)]
pub fn at_s1e2r(arg: u64) {
    unsafe {
        asm!("at s1e2r, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Address Translate Stage 1 EL2 Write
 * ================================================================================================
 */

/// Address Translate Stage 1 EL2 Write
#[inline(always)]
pub fn at_s1e2w(arg: u64) {
    unsafe {
        asm!("at s1e2w, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Address Translate Stage 1 EL3 Read
 * ================================================================================================
 */

/// Address Translate Stage 1 EL3 Read
#[inline(always)]
pub fn at_s1e3r(arg: u64) {
    unsafe {
        asm!("at s1e3r, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Address Translate Stage 1 EL3 Write
 * ================================================================================================
 */

/// Address Translate Stage 1 EL3 Write
#[inline(always)]
pub fn at_s1e3w(arg: u64) {
    unsafe {
        asm!("at s1e3w, $0 {}", in(reg) arg);
    }
}
