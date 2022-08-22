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
 * Generated on: 2022-08-22T15:51:28.525483
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
 * Register:    LORegion Start Address (EL1) (lorsa_el1)
 * Group:       Virtual memory control registers
 * Type:        64-bit Register
 * Description: Indicates whether the current LORegion descriptor selected by
 * File:        AArch64-lorsa_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the LORegion Start Address (EL1) value in memory
pub struct LorsaEl1(u64);

/// struct implementation for accessing the fields of register lorsa_el1
impl LorsaEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> LorsaEl1 {
        Self::default()
    }

    /// collects the modifications of LorsaEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> LorsaEl1 {
        LorsaEl1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> LorsaEl1 {
        let curval = Self::reg_rawrd() & 0xfffffffff0001;
        LorsaEl1(curval)
    }

    /// reading the LORegion Start Address (EL1) (lorsa_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, LORSA_EL1
            llvm_asm!("mrs $0, S3_0_C10_C4_0" : "=r"(regval));
        }
        return regval;
    }

    /// writing the LORegion Start Address (EL1) (lorsa_el1) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR LORSA_EL1, <Xt>
            llvm_asm!("msr S3_0_C10_C4_0, $0" : : "r"(val));
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0xfffffffff0001;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.val)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 4503599627304961;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: sa
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn sa_extract(&self) -> u64 {
        // bits 16..51
        self.val.get_bits(16..=51)
    }

    /// reads the current register value and extract field `sa` from it
    pub fn sa_read(&mut self) -> u64 {
        Self::with_reg_val().sa_extract()
    }

    /// inserts the given value `val` into the field `sa`
    pub fn sa_insert(&mut self, val: u64) -> &mut self {
        // bits 16..51
        self.val.set_bits(16..=51, val);
        self
    }

    /// reads the register, updates the `sa` field, and writes the updated value
    pub fn sa_write(&mut self, val: u64) {
        Self::with_reg_val().sa_insert(val).write();
    }

    /*
     * Field: valid
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn valid_extract(&self) -> u64 {
        // bits 0..0
        self.val.get_bits(0..=0)
    }

    /// reads the current register value and extract field `valid` from it
    pub fn valid_read(&mut self) -> u64 {
        Self::with_reg_val().valid_extract()
    }

    /// inserts the given value `val` into the field `valid`
    pub fn valid_insert(&mut self, val: u64) -> &mut self {
        // bits 0..0
        self.val.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `valid` field, and writes the updated value
    pub fn valid_write(&mut self, val: u64) {
        Self::with_reg_val().valid_insert(val).write();
    }
}

impl Default for LorsaEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> LorsaEl1 {
        LorsaEl1(0)
    }
}
