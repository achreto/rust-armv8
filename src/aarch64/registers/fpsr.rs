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
 * Generated on: 2022-08-22T16:35:53.057680
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
 * Register:    Floating-point Status Register (fpsr)
 * Group:       Floating-point registers
 * Type:        64-bit Register
 * Description: Provides floating-point system status information.
 * File:        AArch64-fpsr.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Floating-point Status Register value in memory
pub struct Fpsr(u64);

/// struct implementation for accessing the fields of register fpsr
impl Fpsr {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> Fpsr {
        Self::default()
    }

    /// collects the modifications of Fpsr and creates new object
    #[inline(always)]
    pub fn collect(&self) -> Fpsr {
        Fpsr(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> Fpsr {
        let curval = Self::reg_rawrd() & 0xf800009f;
        Fpsr(curval)
    }

    /// reading the Floating-point Status Register (fpsr) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, FPSR
            asm!("mrs {}, fpsr", out(reg) regval);
        }
        return regval;
    }

    /// writing the Floating-point Status Register (fpsr) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR FPSR, <Xt>
            asm!("msr fpsr, {}", in(reg) val);
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xf800009f;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 4160749727;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: n_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn n_1_extract(&self) -> u64 {
        // bits 31..31
        self.0.get_bits(31..=31)
    }

    /// reads the current register value and extract field `n_1` from it
    pub fn n_1_read() -> u64 {
        Self::with_reg_val().n_1_extract()
    }

    /// inserts the given value `val` into the field `n_1`
    pub fn n_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 31..31
        self.0.set_bits(31..=31, val);
        self
    }

    /// reads the register, updates the `n_1` field, and writes the updated value
    pub fn n_1_write(val: u64) {
        Self::with_reg_val().n_1_insert(val).write();
    }

    /*
     * Field: z_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn z_1_extract(&self) -> u64 {
        // bits 30..30
        self.0.get_bits(30..=30)
    }

    /// reads the current register value and extract field `z_1` from it
    pub fn z_1_read() -> u64 {
        Self::with_reg_val().z_1_extract()
    }

    /// inserts the given value `val` into the field `z_1`
    pub fn z_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 30..30
        self.0.set_bits(30..=30, val);
        self
    }

    /// reads the register, updates the `z_1` field, and writes the updated value
    pub fn z_1_write(val: u64) {
        Self::with_reg_val().z_1_insert(val).write();
    }

    /*
     * Field: c_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn c_1_extract(&self) -> u64 {
        // bits 29..29
        self.0.get_bits(29..=29)
    }

    /// reads the current register value and extract field `c_1` from it
    pub fn c_1_read() -> u64 {
        Self::with_reg_val().c_1_extract()
    }

    /// inserts the given value `val` into the field `c_1`
    pub fn c_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 29..29
        self.0.set_bits(29..=29, val);
        self
    }

    /// reads the register, updates the `c_1` field, and writes the updated value
    pub fn c_1_write(val: u64) {
        Self::with_reg_val().c_1_insert(val).write();
    }

    /*
     * Field: v_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn v_1_extract(&self) -> u64 {
        // bits 28..28
        self.0.get_bits(28..=28)
    }

    /// reads the current register value and extract field `v_1` from it
    pub fn v_1_read() -> u64 {
        Self::with_reg_val().v_1_extract()
    }

    /// inserts the given value `val` into the field `v_1`
    pub fn v_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 28..28
        self.0.set_bits(28..=28, val);
        self
    }

    /// reads the register, updates the `v_1` field, and writes the updated value
    pub fn v_1_write(val: u64) {
        Self::with_reg_val().v_1_insert(val).write();
    }

    /*
     * Field: qc
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn qc_extract(&self) -> u64 {
        // bits 27..27
        self.0.get_bits(27..=27)
    }

    /// reads the current register value and extract field `qc` from it
    pub fn qc_read() -> u64 {
        Self::with_reg_val().qc_extract()
    }

    /// inserts the given value `val` into the field `qc`
    pub fn qc_insert(&mut self, val: u64) -> &mut Self {
        // bits 27..27
        self.0.set_bits(27..=27, val);
        self
    }

    /// reads the register, updates the `qc` field, and writes the updated value
    pub fn qc_write(val: u64) {
        Self::with_reg_val().qc_insert(val).write();
    }

    /*
     * Field: idc
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn idc_extract(&self) -> u64 {
        // bits 7..7
        self.0.get_bits(7..=7)
    }

    /// reads the current register value and extract field `idc` from it
    pub fn idc_read() -> u64 {
        Self::with_reg_val().idc_extract()
    }

    /// inserts the given value `val` into the field `idc`
    pub fn idc_insert(&mut self, val: u64) -> &mut Self {
        // bits 7..7
        self.0.set_bits(7..=7, val);
        self
    }

    /// reads the register, updates the `idc` field, and writes the updated value
    pub fn idc_write(val: u64) {
        Self::with_reg_val().idc_insert(val).write();
    }

    /*
     * Field: ixc
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ixc_extract(&self) -> u64 {
        // bits 4..4
        self.0.get_bits(4..=4)
    }

    /// reads the current register value and extract field `ixc` from it
    pub fn ixc_read() -> u64 {
        Self::with_reg_val().ixc_extract()
    }

    /// inserts the given value `val` into the field `ixc`
    pub fn ixc_insert(&mut self, val: u64) -> &mut Self {
        // bits 4..4
        self.0.set_bits(4..=4, val);
        self
    }

    /// reads the register, updates the `ixc` field, and writes the updated value
    pub fn ixc_write(val: u64) {
        Self::with_reg_val().ixc_insert(val).write();
    }

    /*
     * Field: ufc
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ufc_extract(&self) -> u64 {
        // bits 3..3
        self.0.get_bits(3..=3)
    }

    /// reads the current register value and extract field `ufc` from it
    pub fn ufc_read() -> u64 {
        Self::with_reg_val().ufc_extract()
    }

    /// inserts the given value `val` into the field `ufc`
    pub fn ufc_insert(&mut self, val: u64) -> &mut Self {
        // bits 3..3
        self.0.set_bits(3..=3, val);
        self
    }

    /// reads the register, updates the `ufc` field, and writes the updated value
    pub fn ufc_write(val: u64) {
        Self::with_reg_val().ufc_insert(val).write();
    }

    /*
     * Field: ofc
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ofc_extract(&self) -> u64 {
        // bits 2..2
        self.0.get_bits(2..=2)
    }

    /// reads the current register value and extract field `ofc` from it
    pub fn ofc_read() -> u64 {
        Self::with_reg_val().ofc_extract()
    }

    /// inserts the given value `val` into the field `ofc`
    pub fn ofc_insert(&mut self, val: u64) -> &mut Self {
        // bits 2..2
        self.0.set_bits(2..=2, val);
        self
    }

    /// reads the register, updates the `ofc` field, and writes the updated value
    pub fn ofc_write(val: u64) {
        Self::with_reg_val().ofc_insert(val).write();
    }

    /*
     * Field: dzc
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dzc_extract(&self) -> u64 {
        // bits 1..1
        self.0.get_bits(1..=1)
    }

    /// reads the current register value and extract field `dzc` from it
    pub fn dzc_read() -> u64 {
        Self::with_reg_val().dzc_extract()
    }

    /// inserts the given value `val` into the field `dzc`
    pub fn dzc_insert(&mut self, val: u64) -> &mut Self {
        // bits 1..1
        self.0.set_bits(1..=1, val);
        self
    }

    /// reads the register, updates the `dzc` field, and writes the updated value
    pub fn dzc_write(val: u64) {
        Self::with_reg_val().dzc_insert(val).write();
    }

    /*
     * Field: ioc
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ioc_extract(&self) -> u64 {
        // bits 0..0
        self.0.get_bits(0..=0)
    }

    /// reads the current register value and extract field `ioc` from it
    pub fn ioc_read() -> u64 {
        Self::with_reg_val().ioc_extract()
    }

    /// inserts the given value `val` into the field `ioc`
    pub fn ioc_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..0
        self.0.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `ioc` field, and writes the updated value
    pub fn ioc_write(val: u64) {
        Self::with_reg_val().ioc_insert(val).write();
    }
}

impl Default for Fpsr {
    /// creates a new default value
    #[inline(always)]
    fn default() -> Fpsr {
        Fpsr(0)
    }
}
