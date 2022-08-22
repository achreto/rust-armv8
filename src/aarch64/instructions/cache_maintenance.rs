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
 * Generated on: 2022-08-22T15:51:28.541893
 * Version: Armv8.7-A-2020-09
 * Source: https://developer.arm.com/-/media/developer/products/architecture/armv8-a-architecture/2020-09/SysReg_xml_v87A-2020-09.tar.gz
 *
 * !!!! WARNING: THIS FILE IS AUTO GENERATED. ANY CHANGES MAY BE OVERWRITTEN !!!!
 *
 *************************************************************************************************/

/*
 * ================================================================================================
 * Clean of Data and Allocation Tags by Set/Way
 * ================================================================================================
 */

/// Clean of Data and Allocation Tags by Set/Way
#[inline(always)]
pub fn dc_cgdsw(arg: u64) {
    unsafe {
        llvm_asm!("dc cgdsw, $0" : : "r"(arg));
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
        llvm_asm!("dc cgdvac, $0" : : "r"(arg));
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
        llvm_asm!("dc cgdvadp, $0" : : "r"(arg));
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
        llvm_asm!("dc cgdvap, $0" : : "r"(arg));
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
        llvm_asm!("dc cgsw, $0" : : "r"(arg));
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
        llvm_asm!("dc cgvac, $0" : : "r"(arg));
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
        llvm_asm!("dc cgvadp, $0" : : "r"(arg));
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
        llvm_asm!("dc cgvap, $0" : : "r"(arg));
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
        llvm_asm!("dc cigdsw, $0" : : "r"(arg));
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
        llvm_asm!("dc cigdvac, $0" : : "r"(arg));
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
        llvm_asm!("dc cigsw, $0" : : "r"(arg));
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
        llvm_asm!("dc cigvac, $0" : : "r"(arg));
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
        llvm_asm!("dc cisw, $0" : : "r"(arg));
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
        llvm_asm!("dc civac, $0" : : "r"(arg));
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
        llvm_asm!("dc csw, $0" : : "r"(arg));
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
        llvm_asm!("dc cvac, $0" : : "r"(arg));
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
        llvm_asm!("dc cvadp, $0" : : "r"(arg));
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
        llvm_asm!("dc cvap, $0" : : "r"(arg));
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
        llvm_asm!("dc cvau, $0" : : "r"(arg));
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
        llvm_asm!("dc gva, $0" : : "r"(arg));
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
        llvm_asm!("dc gzva, $0" : : "r"(arg));
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
        llvm_asm!("dc igdsw, $0" : : "r"(arg));
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
        llvm_asm!("dc igdvac, $0" : : "r"(arg));
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
        llvm_asm!("dc igsw, $0" : : "r"(arg));
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
        llvm_asm!("dc igvac, $0" : : "r"(arg));
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
        llvm_asm!("dc isw, $0" : : "r"(arg));
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
        llvm_asm!("dc ivac, $0" : : "r"(arg));
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
        llvm_asm!("dc zva, $0" : : "r"(arg));
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
        llvm_asm!("ic iallu");
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
        llvm_asm!("ic ialluis");
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
        llvm_asm!("ic ivau");
    }
}
