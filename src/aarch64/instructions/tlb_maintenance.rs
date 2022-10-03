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
 * Generated on: 2022-08-22T16:35:53.085011
 * Version: Armv8.7-A-2020-09
 * Source: https://developer.arm.com/-/media/developer/products/architecture/armv8-a-architecture/2020-09/SysReg_xml_v87A-2020-09.tar.gz
 *
 * !!!! WARNING: THIS FILE IS AUTO GENERATED. ANY CHANGES MAY BE OVERWRITTEN !!!!
 *
 *************************************************************************************************/
use core::arch::asm;

/*
 * ================================================================================================
 * TLB Invalidate All, EL1
 * ================================================================================================
 */

/// TLB Invalidate All, EL1
#[inline(always)]
pub fn tlbi_alle1_tlbi_alle1nxs() {
    unsafe {
        asm!("tlbi alle1");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate All, EL1, Inner Shareable
 * ================================================================================================
 */

/// TLB Invalidate All, EL1, Inner Shareable
#[inline(always)]
pub fn tlbi_alle1is_tlbi_alle1isnxs() {
    unsafe {
        asm!("tlbi alle1is");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate All, EL1, Outer Shareable
 * ================================================================================================
 */

/// TLB Invalidate All, EL1, Outer Shareable
#[inline(always)]
pub fn tlbi_alle1os_tlbi_alle1osnxs() {
    unsafe {
        asm!("tlbi alle1os");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate All, EL2
 * ================================================================================================
 */

/// TLB Invalidate All, EL2
#[inline(always)]
pub fn tlbi_alle2_tlbi_alle2nxs() {
    unsafe {
        asm!("tlbi alle2");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate All, EL2, Inner Shareable
 * ================================================================================================
 */

/// TLB Invalidate All, EL2, Inner Shareable
#[inline(always)]
pub fn tlbi_alle2is_tlbi_alle2isnxs() {
    unsafe {
        asm!("tlbi alle2is");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate All, EL2, Outer Shareable
 * ================================================================================================
 */

/// TLB Invalidate All, EL2, Outer Shareable
#[inline(always)]
pub fn tlbi_alle2os_tlbi_alle2osnxs() {
    unsafe {
        asm!("tlbi alle2os");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate All, EL3
 * ================================================================================================
 */

/// TLB Invalidate All, EL3
#[inline(always)]
pub fn tlbi_alle3_tlbi_alle3nxs() {
    unsafe {
        asm!("tlbi alle3");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate All, EL3, Inner Shareable
 * ================================================================================================
 */

/// TLB Invalidate All, EL3, Inner Shareable
#[inline(always)]
pub fn tlbi_alle3is_tlbi_alle3isnxs() {
    unsafe {
        asm!("tlbi alle3is");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate All, EL3, Outer Shareable
 * ================================================================================================
 */

/// TLB Invalidate All, EL3, Outer Shareable
#[inline(always)]
pub fn tlbi_alle3os_tlbi_alle3osnxs() {
    unsafe {
        asm!("tlbi alle3os");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by ASID, EL1
 * ================================================================================================
 */

/// TLB Invalidate by ASID, EL1
#[inline(always)]
pub fn tlbi_aside1_tlbi_aside1nxs() {
    unsafe {
        asm!("tlbi aside1");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by ASID, EL1, Inner Shareable
 * ================================================================================================
 */

/// TLB Invalidate by ASID, EL1, Inner Shareable
#[inline(always)]
pub fn tlbi_aside1is_tlbi_aside1isnxs() {
    unsafe {
        asm!("tlbi aside1is");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by ASID, EL1, Outer Shareable
 * ================================================================================================
 */

/// TLB Invalidate by ASID, EL1, Outer Shareable
#[inline(always)]
pub fn tlbi_aside1os_tlbi_aside1osnxs() {
    unsafe {
        asm!("tlbi aside1os");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by Intermediate Physical Address, Stage 2, EL1
 * ================================================================================================
 */

/// TLB Invalidate by Intermediate Physical Address, Stage 2, EL1
#[inline(always)]
pub fn tlbi_ipas2e1_tlbi_ipas2e1nxs() {
    unsafe {
        asm!("tlbi ipas2e1");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by Intermediate Physical Address, Stage 2, EL1, Inner Shareable
 * ================================================================================================
 */

/// TLB Invalidate by Intermediate Physical Address, Stage 2, EL1, Inner Shareable
#[inline(always)]
pub fn tlbi_ipas2e1is_tlbi_ipas2e1isnxs() {
    unsafe {
        asm!("tlbi ipas2e1is");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by Intermediate Physical Address, Stage 2, EL1, Outer Shareable
 * ================================================================================================
 */

/// TLB Invalidate by Intermediate Physical Address, Stage 2, EL1, Outer Shareable
#[inline(always)]
pub fn tlbi_ipas2e1os_tlbi_ipas2e1osnxs() {
    unsafe {
        asm!("tlbi ipas2e1os");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by Intermediate Physical Address, Stage 2, Last level, EL1
 * ================================================================================================
 */

/// TLB Invalidate by Intermediate Physical Address, Stage 2, Last level, EL1
#[inline(always)]
pub fn tlbi_ipas2le1_tlbi_ipas2le1nxs() {
    unsafe {
        asm!("tlbi ipas2le1");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by Intermediate Physical Address, Stage 2, Last level, EL1, Inner Shareable
 * ================================================================================================
 */

/// TLB Invalidate by Intermediate Physical Address, Stage 2, Last level, EL1, Inner Shareable
#[inline(always)]
pub fn tlbi_ipas2le1is_tlbi_ipas2le1isnxs() {
    unsafe {
        asm!("tlbi ipas2le1is");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by Intermediate Physical Address, Stage 2, Last level, EL1, Outer Shareable
 * ================================================================================================
 */

/// TLB Invalidate by Intermediate Physical Address, Stage 2, Last level, EL1, Outer Shareable
#[inline(always)]
pub fn tlbi_ipas2le1os_tlbi_ipas2le1osnxs() {
    unsafe {
        asm!("tlbi ipas2le1os");
    }
}

/*
 * ================================================================================================
 * TLB Range Invalidate by Intermediate Physical Address, Stage 2, EL1
 * ================================================================================================
 */

/// TLB Range Invalidate by Intermediate Physical Address, Stage 2, EL1
#[inline(always)]
pub fn tlbi_ripas2e1_tlbi_ripas2e1nxs() {
    unsafe {
        asm!("tlbi ripas2e1");
    }
}

/*
 * ================================================================================================
 * TLB Range Invalidate by Intermediate Physical Address, Stage 2, EL1, Inner Shareable
 * ================================================================================================
 */

/// TLB Range Invalidate by Intermediate Physical Address, Stage 2, EL1, Inner Shareable
#[inline(always)]
pub fn tlbi_ripas2e1is_tlbi_ripas2e1isnxs() {
    unsafe {
        asm!("tlbi ripas2e1is");
    }
}

/*
 * ================================================================================================
 * TLB Range Invalidate by Intermediate Physical Address, Stage 2, EL1, Outer Shareable
 * ================================================================================================
 */

/// TLB Range Invalidate by Intermediate Physical Address, Stage 2, EL1, Outer Shareable
#[inline(always)]
pub fn tlbi_ripas2e1os_tlbi_ripas2e1osnxs() {
    unsafe {
        asm!("tlbi ripas2e1os");
    }
}

/*
 * ================================================================================================
 * TLB Range Invalidate by Intermediate Physical Address, Stage 2, Last level, EL1
 * ================================================================================================
 */

/// TLB Range Invalidate by Intermediate Physical Address, Stage 2, Last level, EL1
#[inline(always)]
pub fn tlbi_ripas2le1_tlbi_ripas2le1nxs() {
    unsafe {
        asm!("tlbi ripas2le1");
    }
}

/*
 * ================================================================================================
 * TLB Range Invalidate by Intermediate Physical Address, Stage 2, Last level, EL1, Inner Shareable
 * ================================================================================================
 */

/// TLB Range Invalidate by Intermediate Physical Address, Stage 2, Last level, EL1, Inner Shareable
#[inline(always)]
pub fn tlbi_ripas2le1is_tlbi_ripas2le1isnxs() {
    unsafe {
        asm!("tlbi ripas2le1is");
    }
}

/*
 * ================================================================================================
 * TLB Range Invalidate by Intermediate Physical Address, Stage 2, Last level, EL1, Outer Shareable
 * ================================================================================================
 */

/// TLB Range Invalidate by Intermediate Physical Address, Stage 2, Last level, EL1, Outer Shareable
#[inline(always)]
pub fn tlbi_ripas2le1os_tlbi_ripas2le1osnxs() {
    unsafe {
        asm!("tlbi ripas2le1os");
    }
}

/*
 * ================================================================================================
 * TLB Range Invalidate by VA, All ASID, EL1
 * ================================================================================================
 */

/// TLB Range Invalidate by VA, All ASID, EL1
#[inline(always)]
pub fn tlbi_rvaae1_tlbi_rvaae1nxs() {
    unsafe {
        asm!("tlbi rvaae1");
    }
}

/*
 * ================================================================================================
 * TLB Range Invalidate by VA, All ASID, EL1, Inner Shareable
 * ================================================================================================
 */

/// TLB Range Invalidate by VA, All ASID, EL1, Inner Shareable
#[inline(always)]
pub fn tlbi_rvaae1is_tlbi_rvaae1isnxs() {
    unsafe {
        asm!("tlbi rvaae1is");
    }
}

/*
 * ================================================================================================
 * TLB Range Invalidate by VA, All ASID, EL1, Outer Shareable
 * ================================================================================================
 */

/// TLB Range Invalidate by VA, All ASID, EL1, Outer Shareable
#[inline(always)]
pub fn tlbi_rvaae1os_tlbi_rvaae1osnxs() {
    unsafe {
        asm!("tlbi rvaae1os");
    }
}

/*
 * ================================================================================================
 * TLB Range Invalidate by VA, All ASID, Last level, EL1
 * ================================================================================================
 */

/// TLB Range Invalidate by VA, All ASID, Last level, EL1
#[inline(always)]
pub fn tlbi_rvaale1_tlbi_rvaale1nxs() {
    unsafe {
        asm!("tlbi rvaale1");
    }
}

/*
 * ================================================================================================
 * TLB Range Invalidate by VA, All ASID, Last Level, EL1, Inner Shareable
 * ================================================================================================
 */

/// TLB Range Invalidate by VA, All ASID, Last Level, EL1, Inner Shareable
#[inline(always)]
pub fn tlbi_rvaale1is_tlbi_rvaale1isnxs() {
    unsafe {
        asm!("tlbi rvaale1is");
    }
}

/*
 * ================================================================================================
 * TLB Range Invalidate by VA, All ASID, Last Level, EL1, Outer Shareable
 * ================================================================================================
 */

/// TLB Range Invalidate by VA, All ASID, Last Level, EL1, Outer Shareable
#[inline(always)]
pub fn tlbi_rvaale1os_tlbi_rvaale1osnxs() {
    unsafe {
        asm!("tlbi rvaale1os");
    }
}

/*
 * ================================================================================================
 * TLB Range Invalidate by VA, EL1
 * ================================================================================================
 */

/// TLB Range Invalidate by VA, EL1
#[inline(always)]
pub fn tlbi_rvae1_tlbi_rvae1nxs() {
    unsafe {
        asm!("tlbi rvae1");
    }
}

/*
 * ================================================================================================
 * TLB Range Invalidate by VA, EL1, Inner Shareable
 * ================================================================================================
 */

/// TLB Range Invalidate by VA, EL1, Inner Shareable
#[inline(always)]
pub fn tlbi_rvae1is_tlbi_rvae1isnxs() {
    unsafe {
        asm!("tlbi rvae1is");
    }
}

/*
 * ================================================================================================
 * TLB Range Invalidate by VA, EL1, Outer Shareable
 * ================================================================================================
 */

/// TLB Range Invalidate by VA, EL1, Outer Shareable
#[inline(always)]
pub fn tlbi_rvae1os_tlbi_rvae1osnxs() {
    unsafe {
        asm!("tlbi rvae1os");
    }
}

/*
 * ================================================================================================
 * TLB Range Invalidate by VA, EL2
 * ================================================================================================
 */

/// TLB Range Invalidate by VA, EL2
#[inline(always)]
pub fn tlbi_rvae2_tlbi_rvae2nxs() {
    unsafe {
        asm!("tlbi rvae2");
    }
}

/*
 * ================================================================================================
 * TLB Range Invalidate by VA, EL2, Inner Shareable
 * ================================================================================================
 */

/// TLB Range Invalidate by VA, EL2, Inner Shareable
#[inline(always)]
pub fn tlbi_rvae2is_tlbi_rvae2isnxs() {
    unsafe {
        asm!("tlbi rvae2is");
    }
}

/*
 * ================================================================================================
 * TLB Range Invalidate by VA, EL2, Outer Shareable
 * ================================================================================================
 */

/// TLB Range Invalidate by VA, EL2, Outer Shareable
#[inline(always)]
pub fn tlbi_rvae2os_tlbi_rvae2osnxs() {
    unsafe {
        asm!("tlbi rvae2os");
    }
}

/*
 * ================================================================================================
 * TLB Range Invalidate by VA, EL3
 * ================================================================================================
 */

/// TLB Range Invalidate by VA, EL3
#[inline(always)]
pub fn tlbi_rvae3_tlbi_rvae3nxs() {
    unsafe {
        asm!("tlbi rvae3");
    }
}

/*
 * ================================================================================================
 * TLB Range Invalidate by VA, EL3, Inner Shareable
 * ================================================================================================
 */

/// TLB Range Invalidate by VA, EL3, Inner Shareable
#[inline(always)]
pub fn tlbi_rvae3is_tlbi_rvae3isnxs() {
    unsafe {
        asm!("tlbi rvae3is");
    }
}

/*
 * ================================================================================================
 * TLB Range Invalidate by VA, EL3, Outer Shareable
 * ================================================================================================
 */

/// TLB Range Invalidate by VA, EL3, Outer Shareable
#[inline(always)]
pub fn tlbi_rvae3os_tlbi_rvae3osnxs() {
    unsafe {
        asm!("tlbi rvae3os");
    }
}

/*
 * ================================================================================================
 * TLB Range Invalidate by VA, Last level, EL1
 * ================================================================================================
 */

/// TLB Range Invalidate by VA, Last level, EL1
#[inline(always)]
pub fn tlbi_rvale1_tlbi_rvale1nxs() {
    unsafe {
        asm!("tlbi rvale1");
    }
}

/*
 * ================================================================================================
 * TLB Range Invalidate by VA, Last level, EL1, Inner Shareable
 * ================================================================================================
 */

/// TLB Range Invalidate by VA, Last level, EL1, Inner Shareable
#[inline(always)]
pub fn tlbi_rvale1is_tlbi_rvale1isnxs() {
    unsafe {
        asm!("tlbi rvale1is");
    }
}

/*
 * ================================================================================================
 * TLB Range Invalidate by VA, Last level, EL1, Outer Shareable
 * ================================================================================================
 */

/// TLB Range Invalidate by VA, Last level, EL1, Outer Shareable
#[inline(always)]
pub fn tlbi_rvale1os_tlbi_rvale1osnxs() {
    unsafe {
        asm!("tlbi rvale1os");
    }
}

/*
 * ================================================================================================
 * TLB Range Invalidate by VA, Last level, EL2
 * ================================================================================================
 */

/// TLB Range Invalidate by VA, Last level, EL2
#[inline(always)]
pub fn tlbi_rvale2_tlbi_rvale2nxs() {
    unsafe {
        asm!("tlbi rvale2");
    }
}

/*
 * ================================================================================================
 * TLB Range Invalidate by VA, Last level, EL2, Inner Shareable
 * ================================================================================================
 */

/// TLB Range Invalidate by VA, Last level, EL2, Inner Shareable
#[inline(always)]
pub fn tlbi_rvale2is_tlbi_rvale2isnxs() {
    unsafe {
        asm!("tlbi rvale2is");
    }
}

/*
 * ================================================================================================
 * TLB Range Invalidate by VA, Last level, EL2, Outer Shareable
 * ================================================================================================
 */

/// TLB Range Invalidate by VA, Last level, EL2, Outer Shareable
#[inline(always)]
pub fn tlbi_rvale2os_tlbi_rvale2osnxs() {
    unsafe {
        asm!("tlbi rvale2os");
    }
}

/*
 * ================================================================================================
 * TLB Range Invalidate by VA, Last level, EL3
 * ================================================================================================
 */

/// TLB Range Invalidate by VA, Last level, EL3
#[inline(always)]
pub fn tlbi_rvale3_tlbi_rvale3nxs() {
    unsafe {
        asm!("tlbi rvale3");
    }
}

/*
 * ================================================================================================
 * TLB Range Invalidate by VA, Last level, EL3, Inner Shareable
 * ================================================================================================
 */

/// TLB Range Invalidate by VA, Last level, EL3, Inner Shareable
#[inline(always)]
pub fn tlbi_rvale3is_tlbi_rvale3isnxs() {
    unsafe {
        asm!("tlbi rvale3is");
    }
}

/*
 * ================================================================================================
 * TLB Range Invalidate by VA, Last level, EL3, Outer Shareable
 * ================================================================================================
 */

/// TLB Range Invalidate by VA, Last level, EL3, Outer Shareable
#[inline(always)]
pub fn tlbi_rvale3os_tlbi_rvale3osnxs() {
    unsafe {
        asm!("tlbi rvale3os");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by VA, All ASID, EL1
 * ================================================================================================
 */

/// TLB Invalidate by VA, All ASID, EL1
#[inline(always)]
pub fn tlbi_vaae1_tlbi_vaae1nxs() {
    unsafe {
        asm!("tlbi vaae1");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by VA, All ASID, EL1, Inner Shareable
 * ================================================================================================
 */

/// TLB Invalidate by VA, All ASID, EL1, Inner Shareable
#[inline(always)]
pub fn tlbi_vaae1is_tlbi_vaae1isnxs() {
    unsafe {
        asm!("tlbi vaae1is");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by VA, All ASID, EL1, Outer Shareable
 * ================================================================================================
 */

/// TLB Invalidate by VA, All ASID, EL1, Outer Shareable
#[inline(always)]
pub fn tlbi_vaae1os_tlbi_vaae1osnxs() {
    unsafe {
        asm!("tlbi vaae1os");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by VA, All ASID, Last level, EL1
 * ================================================================================================
 */

/// TLB Invalidate by VA, All ASID, Last level, EL1
#[inline(always)]
pub fn tlbi_vaale1_tlbi_vaale1nxs() {
    unsafe {
        asm!("tlbi vaale1");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by VA, All ASID, Last Level, EL1, Inner Shareable
 * ================================================================================================
 */

/// TLB Invalidate by VA, All ASID, Last Level, EL1, Inner Shareable
#[inline(always)]
pub fn tlbi_vaale1is_tlbi_vaale1isnxs() {
    unsafe {
        asm!("tlbi vaale1is");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by VA, All ASID, Last Level, EL1, Outer Shareable
 * ================================================================================================
 */

/// TLB Invalidate by VA, All ASID, Last Level, EL1, Outer Shareable
#[inline(always)]
pub fn tlbi_vaale1os_tlbi_vaale1osnxs() {
    unsafe {
        asm!("tlbi vaale1os");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by VA, EL1
 * ================================================================================================
 */

/// TLB Invalidate by VA, EL1
#[inline(always)]
pub fn tlbi_vae1_tlbi_vae1nxs() {
    unsafe {
        asm!("tlbi vae1");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by VA, EL1, Inner Shareable
 * ================================================================================================
 */

/// TLB Invalidate by VA, EL1, Inner Shareable
#[inline(always)]
pub fn tlbi_vae1is_tlbi_vae1isnxs() {
    unsafe {
        asm!("tlbi vae1is");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by VA, EL1, Outer Shareable
 * ================================================================================================
 */

/// TLB Invalidate by VA, EL1, Outer Shareable
#[inline(always)]
pub fn tlbi_vae1os_tlbi_vae1osnxs() {
    unsafe {
        asm!("tlbi vae1os");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by VA, EL2
 * ================================================================================================
 */

/// TLB Invalidate by VA, EL2
#[inline(always)]
pub fn tlbi_vae2_tlbi_vae2nxs() {
    unsafe {
        asm!("tlbi vae2");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by VA, EL2, Inner Shareable
 * ================================================================================================
 */

/// TLB Invalidate by VA, EL2, Inner Shareable
#[inline(always)]
pub fn tlbi_vae2is_tlbi_vae2isnxs() {
    unsafe {
        asm!("tlbi vae2is");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by VA, EL2, Outer Shareable
 * ================================================================================================
 */

/// TLB Invalidate by VA, EL2, Outer Shareable
#[inline(always)]
pub fn tlbi_vae2os_tlbi_vae2osnxs() {
    unsafe {
        asm!("tlbi vae2os");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by VA, EL3
 * ================================================================================================
 */

/// TLB Invalidate by VA, EL3
#[inline(always)]
pub fn tlbi_vae3_tlbi_vae3nxs() {
    unsafe {
        asm!("tlbi vae3");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by VA, EL3, Inner Shareable
 * ================================================================================================
 */

/// TLB Invalidate by VA, EL3, Inner Shareable
#[inline(always)]
pub fn tlbi_vae3is_tlbi_vae3isnxs() {
    unsafe {
        asm!("tlbi vae3is");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by VA, EL3, Outer Shareable
 * ================================================================================================
 */

/// TLB Invalidate by VA, EL3, Outer Shareable
#[inline(always)]
pub fn tlbi_vae3os_tlbi_vae3osnxs() {
    unsafe {
        asm!("tlbi vae3os");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by VA, Last level, EL1
 * ================================================================================================
 */

/// TLB Invalidate by VA, Last level, EL1
#[inline(always)]
pub fn tlbi_vale1_tlbi_vale1nxs() {
    unsafe {
        asm!("tlbi vale1");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by VA, Last level, EL1, Inner Shareable
 * ================================================================================================
 */

/// TLB Invalidate by VA, Last level, EL1, Inner Shareable
#[inline(always)]
pub fn tlbi_vale1is_tlbi_vale1isnxs() {
    unsafe {
        asm!("tlbi vale1is");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by VA, Last level, EL1, Outer Shareable
 * ================================================================================================
 */

/// TLB Invalidate by VA, Last level, EL1, Outer Shareable
#[inline(always)]
pub fn tlbi_vale1os_tlbi_vale1osnxs() {
    unsafe {
        asm!("tlbi vale1os");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by VA, Last level, EL2
 * ================================================================================================
 */

/// TLB Invalidate by VA, Last level, EL2
#[inline(always)]
pub fn tlbi_vale2_tlbi_vale2nxs() {
    unsafe {
        asm!("tlbi vale2");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by VA, Last level, EL2, Inner Shareable
 * ================================================================================================
 */

/// TLB Invalidate by VA, Last level, EL2, Inner Shareable
#[inline(always)]
pub fn tlbi_vale2is_tlbi_vale2isnxs() {
    unsafe {
        asm!("tlbi vale2is");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by VA, Last level, EL2, Outer Shareable
 * ================================================================================================
 */

/// TLB Invalidate by VA, Last level, EL2, Outer Shareable
#[inline(always)]
pub fn tlbi_vale2os_tlbi_vale2osnxs() {
    unsafe {
        asm!("tlbi vale2os");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by VA, Last level, EL3
 * ================================================================================================
 */

/// TLB Invalidate by VA, Last level, EL3
#[inline(always)]
pub fn tlbi_vale3_tlbi_vale3nxs() {
    unsafe {
        asm!("tlbi vale3");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by VA, Last level, EL3, Inner Shareable
 * ================================================================================================
 */

/// TLB Invalidate by VA, Last level, EL3, Inner Shareable
#[inline(always)]
pub fn tlbi_vale3is_tlbi_vale3isnxs() {
    unsafe {
        asm!("tlbi vale3is");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by VA, Last level, EL3, Outer Shareable
 * ================================================================================================
 */

/// TLB Invalidate by VA, Last level, EL3, Outer Shareable
#[inline(always)]
pub fn tlbi_vale3os_tlbi_vale3osnxs() {
    unsafe {
        asm!("tlbi vale3os");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by VMID, All at stage 1, EL1
 * ================================================================================================
 */

/// TLB Invalidate by VMID, All at stage 1, EL1
#[inline(always)]
pub fn tlbi_vmalle1_tlbi_vmalle1nxs() {
    unsafe {
        asm!("tlbi vmalle1");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by VMID, All at stage 1, EL1, Inner Shareable
 * ================================================================================================
 */

/// TLB Invalidate by VMID, All at stage 1, EL1, Inner Shareable
#[inline(always)]
pub fn tlbi_vmalle1is_tlbi_vmalle1isnxs() {
    unsafe {
        asm!("tlbi vmalle1is");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by VMID, All at stage 1, EL1, Outer Shareable
 * ================================================================================================
 */

/// TLB Invalidate by VMID, All at stage 1, EL1, Outer Shareable
#[inline(always)]
pub fn tlbi_vmalle1os_tlbi_vmalle1osnxs() {
    unsafe {
        asm!("tlbi vmalle1os");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by VMID, All at Stage 1 and 2, EL1
 * ================================================================================================
 */

/// TLB Invalidate by VMID, All at Stage 1 and 2, EL1
#[inline(always)]
pub fn tlbi_vmalls12e1_tlbi_vmalls12e1nxs() {
    unsafe {
        asm!("tlbi vmalls12e1");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by VMID, All at Stage 1 and 2, EL1, Inner Shareable
 * ================================================================================================
 */

/// TLB Invalidate by VMID, All at Stage 1 and 2, EL1, Inner Shareable
#[inline(always)]
pub fn tlbi_vmalls12e1is_tlbi_vmalls12e1isnxs() {
    unsafe {
        asm!("tlbi vmalls12e1is");
    }
}

/*
 * ================================================================================================
 * TLB Invalidate by VMID, All at Stage 1 and 2, EL1, Outer Shareable
 * ================================================================================================
 */

/// TLB Invalidate by VMID, All at Stage 1 and 2, EL1, Outer Shareable
#[inline(always)]
pub fn tlbi_vmalls12e1os_tlbi_vmalls12e1osnxs() {
    unsafe {
        asm!("tlbi vmalls12e1os");
    }
}
