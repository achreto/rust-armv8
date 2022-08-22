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
 * Generated on: 2022-08-22T15:51:28.525130
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
 * Register:    LORegion Control (EL1) (lorc_el1)
 * Group:       Virtual memory control registers
 * Type:        64-bit Register
 * Description: Enables and disables LORegions, and selects the current LORegion descriptor.
 * File:        AArch64-lorc_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the LORegion Control (EL1) value in memory
pub struct LorcEl1(u64);

/// struct implementation for accessing the fields of register lorc_el1
impl LorcEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> LorcEl1 {
        Self::default()
    }

    /// collects the modifications of LorcEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> LorcEl1 {
        LorcEl1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> LorcEl1 {
        let curval = Self::reg_rawrd() & 0x3fd;
        LorcEl1(curval)
    }

    /// reading the LORegion Control (EL1) (lorc_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, LORC_EL1
            llvm_asm!("mrs $0, S3_0_C10_C4_3" : "=r"(regval));
        }
        return regval;
    }

    /// writing the LORegion Control (EL1) (lorc_el1) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR LORC_EL1, <Xt>
            llvm_asm!("msr S3_0_C10_C4_3, $0" : : "r"(val));
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0x3fd;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.val)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 1021;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: ds
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ds_extract(&self) -> u64 {
        // bits 2..9
        self.val.get_bits(2..=9)
    }

    /// reads the current register value and extract field `ds` from it
    pub fn ds_read(&mut self) -> u64 {
        Self::with_reg_val().ds_extract()
    }

    /// inserts the given value `val` into the field `ds`
    pub fn ds_insert(&mut self, val: u64) -> &mut self {
        // bits 2..9
        self.val.set_bits(2..=9, val);
        self
    }

    /// reads the register, updates the `ds` field, and writes the updated value
    pub fn ds_write(&mut self, val: u64) {
        Self::with_reg_val().ds_insert(val).write();
    }

    /*
     * Field: en
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn en_extract(&self) -> u64 {
        // bits 0..0
        self.val.get_bits(0..=0)
    }

    /// reads the current register value and extract field `en` from it
    pub fn en_read(&mut self) -> u64 {
        Self::with_reg_val().en_extract()
    }

    /// inserts the given value `val` into the field `en`
    pub fn en_insert(&mut self, val: u64) -> &mut self {
        // bits 0..0
        self.val.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `en` field, and writes the updated value
    pub fn en_write(&mut self, val: u64) {
        Self::with_reg_val().en_insert(val).write();
    }
}

impl Default for LorcEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> LorcEl1 {
        LorcEl1(0)
    }
}
