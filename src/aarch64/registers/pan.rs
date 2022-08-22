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
 * Generated on: 2022-08-22T15:51:28.529207
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
 * Register:    Privileged Access Never (pan)
 * Group:       Process state registers
 * Type:        64-bit Register
 * Description: Allows access to the Privileged Access Never bit.
 * File:        AArch64-pan.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Privileged Access Never value in memory
pub struct Pan(u64);

/// struct implementation for accessing the fields of register pan
impl Pan {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> Pan {
        Self::default()
    }

    /// collects the modifications of Pan and creates new object
    #[inline(always)]
    pub fn collect(&self) -> Pan {
        Pan(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> Pan {
        let curval = Self::reg_rawrd() & 0x400000;
        Pan(curval)
    }

    /// reading the Privileged Access Never (pan) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, PAN
            llvm_asm!("mrs $0, S3_0_C4_C2_3" : "=r"(regval));
        }
        return regval;
    }

    /// writing the Privileged Access Never (pan) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR PAN, <Xt>
            llvm_asm!("msr S3_0_C4_C2_3, $0" : : "r"(val));
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0x400000;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.val)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 4194304;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: pan
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn pan_extract(&self) -> u64 {
        // bits 22..22
        self.val.get_bits(22..=22)
    }

    /// reads the current register value and extract field `pan` from it
    pub fn pan_read(&mut self) -> u64 {
        Self::with_reg_val().pan_extract()
    }

    /// inserts the given value `val` into the field `pan`
    pub fn pan_insert(&mut self, val: u64) -> &mut self {
        // bits 22..22
        self.val.set_bits(22..=22, val);
        self
    }

    /// reads the register, updates the `pan` field, and writes the updated value
    pub fn pan_write(&mut self, val: u64) {
        Self::with_reg_val().pan_insert(val).write();
    }
}

impl Default for Pan {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> Pan {
        Pan(0)
    }
}
