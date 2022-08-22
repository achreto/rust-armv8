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
 * Generated on: 2022-08-22T15:51:28.531647
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
 * Register:    Sampling Filter Control Register (pmsfcr_el1)
 * Group:       Statistical Profiling Extension registers
 * Type:        64-bit Register
 * Description: Controls sample filtering. The filter is the logical AND of the FL, FT and FE bits. For example, if FE == 1 and FT == 1 only samples including the selected operation types and the selected events will be recorded
 * File:        AArch64-pmsfcr_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Sampling Filter Control Register value in memory
pub struct PmsfcrEl1(u64);

/// struct implementation for accessing the fields of register pmsfcr_el1
impl PmsfcrEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> PmsfcrEl1 {
        Self::default()
    }

    /// collects the modifications of PmsfcrEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> PmsfcrEl1 {
        PmsfcrEl1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> PmsfcrEl1 {
        let curval = Self::reg_rawrd() & 0x7000f;
        PmsfcrEl1(curval)
    }

    /// reading the Sampling Filter Control Register (pmsfcr_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, PMSFCR_EL1
            llvm_asm!("mrs $0, S3_0_C9_C9_4" : "=r"(regval));
        }
        return regval;
    }

    /// writing the Sampling Filter Control Register (pmsfcr_el1) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR PMSFCR_EL1, <Xt>
            llvm_asm!("msr S3_0_C9_C9_4, $0" : : "r"(val));
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0x7000f;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.val)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 458767;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: st
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn st_extract(&self) -> u64 {
        // bits 18..18
        self.val.get_bits(18..=18)
    }

    /// reads the current register value and extract field `st` from it
    pub fn st_read(&mut self) -> u64 {
        Self::with_reg_val().st_extract()
    }

    /// inserts the given value `val` into the field `st`
    pub fn st_insert(&mut self, val: u64) -> &mut self {
        // bits 18..18
        self.val.set_bits(18..=18, val);
        self
    }

    /// reads the register, updates the `st` field, and writes the updated value
    pub fn st_write(&mut self, val: u64) {
        Self::with_reg_val().st_insert(val).write();
    }

    /*
     * Field: ld
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ld_extract(&self) -> u64 {
        // bits 17..17
        self.val.get_bits(17..=17)
    }

    /// reads the current register value and extract field `ld` from it
    pub fn ld_read(&mut self) -> u64 {
        Self::with_reg_val().ld_extract()
    }

    /// inserts the given value `val` into the field `ld`
    pub fn ld_insert(&mut self, val: u64) -> &mut self {
        // bits 17..17
        self.val.set_bits(17..=17, val);
        self
    }

    /// reads the register, updates the `ld` field, and writes the updated value
    pub fn ld_write(&mut self, val: u64) {
        Self::with_reg_val().ld_insert(val).write();
    }

    /*
     * Field: b
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn b_extract(&self) -> u64 {
        // bits 16..16
        self.val.get_bits(16..=16)
    }

    /// reads the current register value and extract field `b` from it
    pub fn b_read(&mut self) -> u64 {
        Self::with_reg_val().b_extract()
    }

    /// inserts the given value `val` into the field `b`
    pub fn b_insert(&mut self, val: u64) -> &mut self {
        // bits 16..16
        self.val.set_bits(16..=16, val);
        self
    }

    /// reads the register, updates the `b` field, and writes the updated value
    pub fn b_write(&mut self, val: u64) {
        Self::with_reg_val().b_insert(val).write();
    }

    /*
     * Field: fne_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn fne_1_extract(&self) -> u64 {
        // bits 3..3
        self.val.get_bits(3..=3)
    }

    /// reads the current register value and extract field `fne_1` from it
    pub fn fne_1_read(&mut self) -> u64 {
        Self::with_reg_val().fne_1_extract()
    }

    /// inserts the given value `val` into the field `fne_1`
    pub fn fne_1_insert(&mut self, val: u64) -> &mut self {
        // bits 3..3
        self.val.set_bits(3..=3, val);
        self
    }

    /// reads the register, updates the `fne_1` field, and writes the updated value
    pub fn fne_1_write(&mut self, val: u64) {
        Self::with_reg_val().fne_1_insert(val).write();
    }

    /*
     * Field: fl
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn fl_extract(&self) -> u64 {
        // bits 2..2
        self.val.get_bits(2..=2)
    }

    /// reads the current register value and extract field `fl` from it
    pub fn fl_read(&mut self) -> u64 {
        Self::with_reg_val().fl_extract()
    }

    /// inserts the given value `val` into the field `fl`
    pub fn fl_insert(&mut self, val: u64) -> &mut self {
        // bits 2..2
        self.val.set_bits(2..=2, val);
        self
    }

    /// reads the register, updates the `fl` field, and writes the updated value
    pub fn fl_write(&mut self, val: u64) {
        Self::with_reg_val().fl_insert(val).write();
    }

    /*
     * Field: ft
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ft_extract(&self) -> u64 {
        // bits 1..1
        self.val.get_bits(1..=1)
    }

    /// reads the current register value and extract field `ft` from it
    pub fn ft_read(&mut self) -> u64 {
        Self::with_reg_val().ft_extract()
    }

    /// inserts the given value `val` into the field `ft`
    pub fn ft_insert(&mut self, val: u64) -> &mut self {
        // bits 1..1
        self.val.set_bits(1..=1, val);
        self
    }

    /// reads the register, updates the `ft` field, and writes the updated value
    pub fn ft_write(&mut self, val: u64) {
        Self::with_reg_val().ft_insert(val).write();
    }

    /*
     * Field: fe
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn fe_extract(&self) -> u64 {
        // bits 0..0
        self.val.get_bits(0..=0)
    }

    /// reads the current register value and extract field `fe` from it
    pub fn fe_read(&mut self) -> u64 {
        Self::with_reg_val().fe_extract()
    }

    /// inserts the given value `val` into the field `fe`
    pub fn fe_insert(&mut self, val: u64) -> &mut self {
        // bits 0..0
        self.val.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `fe` field, and writes the updated value
    pub fn fe_write(&mut self, val: u64) {
        Self::with_reg_val().fe_insert(val).write();
    }
}

impl Default for PmsfcrEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> PmsfcrEl1 {
        PmsfcrEl1(0)
    }
}
