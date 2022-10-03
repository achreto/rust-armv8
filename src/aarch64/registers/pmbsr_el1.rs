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
use core::arch::asm;

/**************************************************************************************************
 *
 * !!!! WARNING: THIS FILE IS AUTO GENERATED. ANY CHANGES MAY BE OVERWRITTEN !!!!
 *
 * Generated on: 2022-08-22T16:35:53.073197
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
 * Register:    Profiling Buffer Status/syndrome Register (pmbsr_el1)
 * Group:       Statistical Profiling Extension registers
 * Type:        64-bit Register
 * Description: Provides syndrome information to software when the buffer is disabled because the management interrupt has been raised.
 * File:        AArch64-pmbsr_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Profiling Buffer Status/syndrome Register value in memory
pub struct PmbsrEl1(u64);

/// struct implementation for accessing the fields of register pmbsr_el1
impl PmbsrEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> PmbsrEl1 {
        Self::default()
    }

    /// collects the modifications of PmbsrEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> PmbsrEl1 {
        PmbsrEl1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> PmbsrEl1 {
        let curval = Self::reg_rawrd() & 0xfc0fffff;
        PmbsrEl1(curval)
    }

    /// reading the Profiling Buffer Status/syndrome Register (pmbsr_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, PMBSR_EL1
            asm!("mrs {}, S3_0_C9_C10_3", out(reg) regval);
        }
        return regval;
    }

    /// writing the Profiling Buffer Status/syndrome Register (pmbsr_el1) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR PMBSR_EL1, <Xt>
            asm!("msr S3_0_C9_C10_3, {}", in(reg) val);
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xfc0fffff;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 4228907007;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: ec
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ec_extract(&self) -> u64 {
        // bits 26..31
        self.0.get_bits(26..=31)
    }

    /// reads the current register value and extract field `ec` from it
    pub fn ec_read() -> u64 {
        Self::with_reg_val().ec_extract()
    }

    /// inserts the given value `val` into the field `ec`
    pub fn ec_insert(&mut self, val: u64) -> &mut Self {
        // bits 26..31
        self.0.set_bits(26..=31, val);
        self
    }

    /// reads the register, updates the `ec` field, and writes the updated value
    pub fn ec_write(val: u64) {
        Self::with_reg_val().ec_insert(val).write();
    }

    /*
     * Field: dl
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dl_extract(&self) -> u64 {
        // bits 19..19
        self.0.get_bits(19..=19)
    }

    /// reads the current register value and extract field `dl` from it
    pub fn dl_read() -> u64 {
        Self::with_reg_val().dl_extract()
    }

    /// inserts the given value `val` into the field `dl`
    pub fn dl_insert(&mut self, val: u64) -> &mut Self {
        // bits 19..19
        self.0.set_bits(19..=19, val);
        self
    }

    /// reads the register, updates the `dl` field, and writes the updated value
    pub fn dl_write(val: u64) {
        Self::with_reg_val().dl_insert(val).write();
    }

    /*
     * Field: ea
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ea_extract(&self) -> u64 {
        // bits 18..18
        self.0.get_bits(18..=18)
    }

    /// reads the current register value and extract field `ea` from it
    pub fn ea_read() -> u64 {
        Self::with_reg_val().ea_extract()
    }

    /// inserts the given value `val` into the field `ea`
    pub fn ea_insert(&mut self, val: u64) -> &mut Self {
        // bits 18..18
        self.0.set_bits(18..=18, val);
        self
    }

    /// reads the register, updates the `ea` field, and writes the updated value
    pub fn ea_write(val: u64) {
        Self::with_reg_val().ea_insert(val).write();
    }

    /*
     * Field: s
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn s_extract(&self) -> u64 {
        // bits 17..17
        self.0.get_bits(17..=17)
    }

    /// reads the current register value and extract field `s` from it
    pub fn s_read() -> u64 {
        Self::with_reg_val().s_extract()
    }

    /// inserts the given value `val` into the field `s`
    pub fn s_insert(&mut self, val: u64) -> &mut Self {
        // bits 17..17
        self.0.set_bits(17..=17, val);
        self
    }

    /// reads the register, updates the `s` field, and writes the updated value
    pub fn s_write(val: u64) {
        Self::with_reg_val().s_insert(val).write();
    }

    /*
     * Field: coll
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn coll_extract(&self) -> u64 {
        // bits 16..16
        self.0.get_bits(16..=16)
    }

    /// reads the current register value and extract field `coll` from it
    pub fn coll_read() -> u64 {
        Self::with_reg_val().coll_extract()
    }

    /// inserts the given value `val` into the field `coll`
    pub fn coll_insert(&mut self, val: u64) -> &mut Self {
        // bits 16..16
        self.0.set_bits(16..=16, val);
        self
    }

    /// reads the register, updates the `coll` field, and writes the updated value
    pub fn coll_write(val: u64) {
        Self::with_reg_val().coll_insert(val).write();
    }

    /*
     * Field: mss
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn mss_extract(&self) -> u64 {
        // bits 0..15
        self.0.get_bits(0..=15)
    }

    /// reads the current register value and extract field `mss` from it
    pub fn mss_read() -> u64 {
        Self::with_reg_val().mss_extract()
    }

    /// inserts the given value `val` into the field `mss`
    pub fn mss_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..15
        self.0.set_bits(0..=15, val);
        self
    }

    /// reads the register, updates the `mss` field, and writes the updated value
    pub fn mss_write(val: u64) {
        Self::with_reg_val().mss_insert(val).write();
    }
}

impl Default for PmbsrEl1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> PmbsrEl1 {
        PmbsrEl1(0)
    }
}
