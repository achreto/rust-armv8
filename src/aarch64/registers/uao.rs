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

use bit_field::BitField;

/**************************************************************************************************
 *
 * !!!! WARNING: THIS FILE IS AUTO GENERATED. ANY CHANGES MAY BE OVERWRITTEN !!!!
 *
 * Generated on: 2022-08-22T15:51:28.539692
 * Version: Armv8.7-A-2020-09
 * Source: https://developer.arm.com/-/media/developer/products/architecture/armv8-a-architecture/2020-09/SysReg_xml_v87A-2020-09.tar.gz
 *
 * !!!! WARNING: THIS FILE IS AUTO GENERATED. ANY CHANGES MAY BE OVERWRITTEN !!!!
 *
 *************************************************************************************************/

/*
 * ================================================================================================
 * Register Information
 * ================================================================================================
 *
 * Register:    User Access Override (uao)
 * Group:       Process state registers
 * Type:        64-bit Register
 * Description: Allows access to the User Access Override bit.
 * File:        AArch64-uao.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the User Access Override value in memory
pub struct Uao(u64);

/// struct implementation for accessing the fields of register uao
impl Uao {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> Uao {
        Self::default()
    }

    /// collects the modifications of Uao and creates new object
    #[inline(always)]
    pub fn collect(&self) -> Uao {
        Uao(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> Uao {
        let curval = Self::reg_rawrd() & 0x800000;
        Uao(curval)
    }

    /// reading the User Access Override (uao) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, UAO
            llvm_asm!("mrs $0, S3_0_C4_C2_4" : "=r"(regval));
        }
        return regval;
    }

    /// writing the User Access Override (uao) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR UAO, <Xt>
            llvm_asm!("msr S3_0_C4_C2_4, $0" : : "r"(val));
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0x800000;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.val)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 8388608;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: uao
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn uao_extract(&self) -> u64 {
        // bits 23..23
        self.val.get_bits(23..=23)
    }

    /// reads the current register value and extract field `uao` from it
    pub fn uao_read(&mut self) -> u64 {
        Self::with_reg_val().uao_extract()
    }

    /// inserts the given value `val` into the field `uao`
    pub fn uao_insert(&mut self, val: u64) -> &mut self {
        // bits 23..23
        self.val.set_bits(23..=23, val);
        self
    }

    /// reads the register, updates the `uao` field, and writes the updated value
    pub fn uao_write(&mut self, val: u64) {
        Self::with_reg_val().uao_insert(val).write();
    }
}

impl Default for Uao {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> Uao {
        Uao(0)
    }
}
