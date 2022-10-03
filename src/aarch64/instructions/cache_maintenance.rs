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
 * Generated on: 2022-08-22T16:35:53.084881
 * Version: Armv8.7-A-2020-09
 * Source: https://developer.arm.com/-/media/developer/products/architecture/armv8-a-architecture/2020-09/SysReg_xml_v87A-2020-09.tar.gz
 *
 * !!!! WARNING: THIS FILE IS AUTO GENERATED. ANY CHANGES MAY BE OVERWRITTEN !!!!
 *
 *************************************************************************************************/
use core::arch::asm;

/*
 * ================================================================================================
 * Clean of Data and Allocation Tags by Set/Way
 * ================================================================================================
 */

/// Clean of Data and Allocation Tags by Set/Way
#[inline(always)]
pub fn dc_cgdsw(arg: u64) {
    unsafe {
        asm!("dc cgdsw, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Clean of Data and Allocation Tags by VA to PoC
 * ================================================================================================
 */

/// Clean of Data and Allocation Tags by VA to PoC
#[inline(always)]
pub fn dc_cgdvac(arg: u64) {
    unsafe {
        asm!("dc cgdvac, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Clean of Data and Allocation Tags by VA to PoDP
 * ================================================================================================
 */

/// Clean of Data and Allocation Tags by VA to PoDP
#[inline(always)]
pub fn dc_cgdvadp(arg: u64) {
    unsafe {
        asm!("dc cgdvadp, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Clean of Data and Allocation Tags by VA to PoP
 * ================================================================================================
 */

/// Clean of Data and Allocation Tags by VA to PoP
#[inline(always)]
pub fn dc_cgdvap(arg: u64) {
    unsafe {
        asm!("dc cgdvap, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Clean of Allocation Tags by Set/Way
 * ================================================================================================
 */

/// Clean of Allocation Tags by Set/Way
#[inline(always)]
pub fn dc_cgsw(arg: u64) {
    unsafe {
        asm!("dc cgsw, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Clean of Allocation Tags by VA to PoC
 * ================================================================================================
 */

/// Clean of Allocation Tags by VA to PoC
#[inline(always)]
pub fn dc_cgvac(arg: u64) {
    unsafe {
        asm!("dc cgvac, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Clean of Allocation Tags by VA to PoDP
 * ================================================================================================
 */

/// Clean of Allocation Tags by VA to PoDP
#[inline(always)]
pub fn dc_cgvadp(arg: u64) {
    unsafe {
        asm!("dc cgvadp, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Clean of Allocation Tags by VA to PoP
 * ================================================================================================
 */

/// Clean of Allocation Tags by VA to PoP
#[inline(always)]
pub fn dc_cgvap(arg: u64) {
    unsafe {
        asm!("dc cgvap, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Clean and Invalidate of Data and Allocation Tags by Set/Way
 * ================================================================================================
 */

/// Clean and Invalidate of Data and Allocation Tags by Set/Way
#[inline(always)]
pub fn dc_cigdsw(arg: u64) {
    unsafe {
        asm!("dc cigdsw, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Clean and Invalidate of Data and Allocation Tags by VA to PoC
 * ================================================================================================
 */

/// Clean and Invalidate of Data and Allocation Tags by VA to PoC
#[inline(always)]
pub fn dc_cigdvac(arg: u64) {
    unsafe {
        asm!("dc cigdvac, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Clean and Invalidate of Allocation Tags by Set/Way
 * ================================================================================================
 */

/// Clean and Invalidate of Allocation Tags by Set/Way
#[inline(always)]
pub fn dc_cigsw(arg: u64) {
    unsafe {
        asm!("dc cigsw, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Clean and Invalidate of Allocation Tags by VA to PoC
 * ================================================================================================
 */

/// Clean and Invalidate of Allocation Tags by VA to PoC
#[inline(always)]
pub fn dc_cigvac(arg: u64) {
    unsafe {
        asm!("dc cigvac, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Data or unified Cache line Clean and Invalidate by Set/Way
 * ================================================================================================
 */

/// Data or unified Cache line Clean and Invalidate by Set/Way
#[inline(always)]
pub fn dc_cisw(arg: u64) {
    unsafe {
        asm!("dc cisw, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Data or unified Cache line Clean and Invalidate by VA to PoC
 * ================================================================================================
 */

/// Data or unified Cache line Clean and Invalidate by VA to PoC
#[inline(always)]
pub fn dc_civac(arg: u64) {
    unsafe {
        asm!("dc civac, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Data or unified Cache line Clean by Set/Way
 * ================================================================================================
 */

/// Data or unified Cache line Clean by Set/Way
#[inline(always)]
pub fn dc_csw(arg: u64) {
    unsafe {
        asm!("dc csw, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Data or unified Cache line Clean by VA to PoC
 * ================================================================================================
 */

/// Data or unified Cache line Clean by VA to PoC
#[inline(always)]
pub fn dc_cvac(arg: u64) {
    unsafe {
        asm!("dc cvac, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Data or unified Cache line Clean by VA to PoDP
 * ================================================================================================
 */

/// Data or unified Cache line Clean by VA to PoDP
#[inline(always)]
pub fn dc_cvadp(arg: u64) {
    unsafe {
        asm!("dc cvadp, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Data or unified Cache line Clean by VA to PoP
 * ================================================================================================
 */

/// Data or unified Cache line Clean by VA to PoP
#[inline(always)]
pub fn dc_cvap(arg: u64) {
    unsafe {
        asm!("dc cvap, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Data or unified Cache line Clean by VA to PoU
 * ================================================================================================
 */

/// Data or unified Cache line Clean by VA to PoU
#[inline(always)]
pub fn dc_cvau(arg: u64) {
    unsafe {
        asm!("dc cvau, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Data Cache set Allocation Tag by VA
 * ================================================================================================
 */

/// Data Cache set Allocation Tag by VA
#[inline(always)]
pub fn dc_gva(arg: u64) {
    unsafe {
        asm!("dc gva, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Data Cache set Allocation Tags and Zero by VA
 * ================================================================================================
 */

/// Data Cache set Allocation Tags and Zero by VA
#[inline(always)]
pub fn dc_gzva(arg: u64) {
    unsafe {
        asm!("dc gzva, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Invalidate of Data and Allocation Tags by Set/Way
 * ================================================================================================
 */

/// Invalidate of Data and Allocation Tags by Set/Way
#[inline(always)]
pub fn dc_igdsw(arg: u64) {
    unsafe {
        asm!("dc igdsw, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Invalidate of Data and Allocation Tags by VA to PoC
 * ================================================================================================
 */

/// Invalidate of Data and Allocation Tags by VA to PoC
#[inline(always)]
pub fn dc_igdvac(arg: u64) {
    unsafe {
        asm!("dc igdvac, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Invalidate of Allocation Tags by Set/Way
 * ================================================================================================
 */

/// Invalidate of Allocation Tags by Set/Way
#[inline(always)]
pub fn dc_igsw(arg: u64) {
    unsafe {
        asm!("dc igsw, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Invalidate of Allocation Tags by VA to PoC
 * ================================================================================================
 */

/// Invalidate of Allocation Tags by VA to PoC
#[inline(always)]
pub fn dc_igvac(arg: u64) {
    unsafe {
        asm!("dc igvac, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Data or unified Cache line Invalidate by Set/Way
 * ================================================================================================
 */

/// Data or unified Cache line Invalidate by Set/Way
#[inline(always)]
pub fn dc_isw(arg: u64) {
    unsafe {
        asm!("dc isw, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Data or unified Cache line Invalidate by VA to PoC
 * ================================================================================================
 */

/// Data or unified Cache line Invalidate by VA to PoC
#[inline(always)]
pub fn dc_ivac(arg: u64) {
    unsafe {
        asm!("dc ivac, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Data Cache Zero by VA
 * ================================================================================================
 */

/// Data Cache Zero by VA
#[inline(always)]
pub fn dc_zva(arg: u64) {
    unsafe {
        asm!("dc zva, $0 {}", in(reg) arg);
    }
}

/*
 * ================================================================================================
 * Instruction Cache Invalidate All to PoU
 * ================================================================================================
 */

/// Instruction Cache Invalidate All to PoU
#[inline(always)]
pub fn ic_iallu() {
    unsafe {
        asm!("ic iallu");
    }
}

/*
 * ================================================================================================
 * Instruction Cache Invalidate All to PoU, Inner Shareable
 * ================================================================================================
 */

/// Instruction Cache Invalidate All to PoU, Inner Shareable
#[inline(always)]
pub fn ic_ialluis() {
    unsafe {
        asm!("ic ialluis");
    }
}

/*
 * ================================================================================================
 * Instruction Cache line Invalidate by VA to PoU
 * ================================================================================================
 */

/// Instruction Cache line Invalidate by VA to PoU
#[inline(always)]
pub fn ic_ivau() {
    unsafe {
        asm!("ic ivau");
    }
}
