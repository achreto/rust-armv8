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
 * Generated on: 2022-08-22T15:51:28.528065
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
 * Register:    MPAM Virtual PARTID Mapping Register 7 (mpamvpm7_el2)
 * Group:       Memory Partitioning and Monitoring registers
 * Type:        64-bit Register
 * Description: MPAMVPM7_EL2 provides mappings from virtual PARTIDs 28 - 31 to physical PARTIDs.
 * File:        AArch64-mpamvpm7_el2.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the MPAM Virtual PARTID Mapping Register 7 value in memory
pub struct Mpamvpm7El2(u64);

/// struct implementation for accessing the fields of register mpamvpm7_el2
impl Mpamvpm7El2 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> Mpamvpm7El2 {
        Self::default()
    }

    /// collects the modifications of Mpamvpm7El2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> Mpamvpm7El2 {
        Mpamvpm7El2(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> Mpamvpm7El2 {
        let curval = Self::reg_rawrd() & 0xffffffffffffffff;
        Mpamvpm7El2(curval)
    }

    /// reading the MPAM Virtual PARTID Mapping Register 7 (mpamvpm7_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, MPAMVPM7_EL2
            llvm_asm!("mrs $0, S3_4_C10_C6_7" : "=r"(regval));
        }
        return regval;
    }

    /// writing the MPAM Virtual PARTID Mapping Register 7 (mpamvpm7_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR MPAMVPM7_EL2, <Xt>
            llvm_asm!("msr S3_4_C10_C6_7, $0" : : "r"(val));
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0xffffffffffffffff;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.val)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 18446744073709551615;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: phypartid31
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn phypartid31_extract(&self) -> u64 {
        // bits 48..63
        self.val.get_bits(48..=63)
    }

    /// reads the current register value and extract field `phypartid31` from it
    pub fn phypartid31_read(&mut self) -> u64 {
        Self::with_reg_val().phypartid31_extract()
    }

    /// inserts the given value `val` into the field `phypartid31`
    pub fn phypartid31_insert(&mut self, val: u64) -> &mut self {
        // bits 48..63
        self.val.set_bits(48..=63, val);
        self
    }

    /// reads the register, updates the `phypartid31` field, and writes the updated value
    pub fn phypartid31_write(&mut self, val: u64) {
        Self::with_reg_val().phypartid31_insert(val).write();
    }

    /*
     * Field: phypartid30
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn phypartid30_extract(&self) -> u64 {
        // bits 32..47
        self.val.get_bits(32..=47)
    }

    /// reads the current register value and extract field `phypartid30` from it
    pub fn phypartid30_read(&mut self) -> u64 {
        Self::with_reg_val().phypartid30_extract()
    }

    /// inserts the given value `val` into the field `phypartid30`
    pub fn phypartid30_insert(&mut self, val: u64) -> &mut self {
        // bits 32..47
        self.val.set_bits(32..=47, val);
        self
    }

    /// reads the register, updates the `phypartid30` field, and writes the updated value
    pub fn phypartid30_write(&mut self, val: u64) {
        Self::with_reg_val().phypartid30_insert(val).write();
    }

    /*
     * Field: phypartid29
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn phypartid29_extract(&self) -> u64 {
        // bits 16..31
        self.val.get_bits(16..=31)
    }

    /// reads the current register value and extract field `phypartid29` from it
    pub fn phypartid29_read(&mut self) -> u64 {
        Self::with_reg_val().phypartid29_extract()
    }

    /// inserts the given value `val` into the field `phypartid29`
    pub fn phypartid29_insert(&mut self, val: u64) -> &mut self {
        // bits 16..31
        self.val.set_bits(16..=31, val);
        self
    }

    /// reads the register, updates the `phypartid29` field, and writes the updated value
    pub fn phypartid29_write(&mut self, val: u64) {
        Self::with_reg_val().phypartid29_insert(val).write();
    }

    /*
     * Field: phypartid28
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn phypartid28_extract(&self) -> u64 {
        // bits 0..15
        self.val.get_bits(0..=15)
    }

    /// reads the current register value and extract field `phypartid28` from it
    pub fn phypartid28_read(&mut self) -> u64 {
        Self::with_reg_val().phypartid28_extract()
    }

    /// inserts the given value `val` into the field `phypartid28`
    pub fn phypartid28_insert(&mut self, val: u64) -> &mut self {
        // bits 0..15
        self.val.set_bits(0..=15, val);
        self
    }

    /// reads the register, updates the `phypartid28` field, and writes the updated value
    pub fn phypartid28_write(&mut self, val: u64) {
        Self::with_reg_val().phypartid28_insert(val).write();
    }
}

impl Default for Mpamvpm7El2 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> Mpamvpm7El2 {
        Mpamvpm7El2(0)
    }
}
