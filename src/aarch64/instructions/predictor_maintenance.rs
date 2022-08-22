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
 * Generated on: 2022-08-22T15:51:28.541829
 * Version: Armv8.7-A-2020-09
 * Source: https://developer.arm.com/-/media/developer/products/architecture/armv8-a-architecture/2020-09/SysReg_xml_v87A-2020-09.tar.gz
 *
 * !!!! WARNING: THIS FILE IS AUTO GENERATED. ANY CHANGES MAY BE OVERWRITTEN !!!!
 *
 *************************************************************************************************/

/*
 * ================================================================================================
 * Control Flow Prediction Restriction by Context
 * ================================================================================================
 */

/// Control Flow Prediction Restriction by Context
#[inline(always)]
pub fn cfp_rctx(arg: u64) {
    unsafe {
        llvm_asm!("cfp rctx, $0" : : "r"(arg));
    }
}

/*
 * ================================================================================================
 * Cache Prefetch Prediction Restriction by Context
 * ================================================================================================
 */

/// Cache Prefetch Prediction Restriction by Context
#[inline(always)]
pub fn cpp_rctx(arg: u64) {
    unsafe {
        llvm_asm!("cpp rctx, $0" : : "r"(arg));
    }
}

/*
 * ================================================================================================
 * Data Value Prediction Restriction by Context
 * ================================================================================================
 */

/// Data Value Prediction Restriction by Context
#[inline(always)]
pub fn dvp_rctx(arg: u64) {
    unsafe {
        llvm_asm!("dvp rctx, $0" : : "r"(arg));
    }
}
