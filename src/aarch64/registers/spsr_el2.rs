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
 * Generated on: 2022-08-22T16:35:53.079350
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
 * Register:    Saved Program Status Register (EL2) (spsr_el2)
 * Group:       Special-purpose registers
 * Type:        64-bit Register
 * Description: Holds the saved process state when an exception is taken to EL2.
 * File:        AArch64-spsr_el2.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Saved Program Status Register (EL2) value in memory
pub struct SpsrEl2(u64);

/// struct implementation for accessing the fields of register spsr_el2
impl SpsrEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> SpsrEl2 {
        Self::default()
    }

    /// collects the modifications of SpsrEl2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> SpsrEl2 {
        SpsrEl2(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> SpsrEl2 {
        let curval = Self::reg_rawrd() & 0xf3f01fdf;
        SpsrEl2(curval)
    }

    /// reading the Saved Program Status Register (EL2) (spsr_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, SPSR_EL2
            asm!("mrs {}, spsr_el2", out(reg) regval);
        }
        return regval;
    }

    /// writing the Saved Program Status Register (EL2) (spsr_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR SPSR_EL2, <Xt>
            asm!("msr spsr_el2, {}", in(reg) val);
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xf3f01fdf;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 4092600287;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: n
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn n_extract(&self) -> u64 {
        // bits 31..31
        self.0.get_bits(31..=31)
    }

    /// reads the current register value and extract field `n` from it
    pub fn n_read() -> u64 {
        Self::with_reg_val().n_extract()
    }

    /// inserts the given value `val` into the field `n`
    pub fn n_insert(&mut self, val: u64) -> &mut Self {
        // bits 31..31
        self.0.set_bits(31..=31, val);
        self
    }

    /// reads the register, updates the `n` field, and writes the updated value
    pub fn n_write(val: u64) {
        Self::with_reg_val().n_insert(val).write();
    }

    /*
     * Field: z
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn z_extract(&self) -> u64 {
        // bits 30..30
        self.0.get_bits(30..=30)
    }

    /// reads the current register value and extract field `z` from it
    pub fn z_read() -> u64 {
        Self::with_reg_val().z_extract()
    }

    /// inserts the given value `val` into the field `z`
    pub fn z_insert(&mut self, val: u64) -> &mut Self {
        // bits 30..30
        self.0.set_bits(30..=30, val);
        self
    }

    /// reads the register, updates the `z` field, and writes the updated value
    pub fn z_write(val: u64) {
        Self::with_reg_val().z_insert(val).write();
    }

    /*
     * Field: c
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn c_extract(&self) -> u64 {
        // bits 29..29
        self.0.get_bits(29..=29)
    }

    /// reads the current register value and extract field `c` from it
    pub fn c_read() -> u64 {
        Self::with_reg_val().c_extract()
    }

    /// inserts the given value `val` into the field `c`
    pub fn c_insert(&mut self, val: u64) -> &mut Self {
        // bits 29..29
        self.0.set_bits(29..=29, val);
        self
    }

    /// reads the register, updates the `c` field, and writes the updated value
    pub fn c_write(val: u64) {
        Self::with_reg_val().c_insert(val).write();
    }

    /*
     * Field: v
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn v_extract(&self) -> u64 {
        // bits 28..28
        self.0.get_bits(28..=28)
    }

    /// reads the current register value and extract field `v` from it
    pub fn v_read() -> u64 {
        Self::with_reg_val().v_extract()
    }

    /// inserts the given value `val` into the field `v`
    pub fn v_insert(&mut self, val: u64) -> &mut Self {
        // bits 28..28
        self.0.set_bits(28..=28, val);
        self
    }

    /// reads the register, updates the `v` field, and writes the updated value
    pub fn v_write(val: u64) {
        Self::with_reg_val().v_insert(val).write();
    }

    /*
     * Field: tco_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tco_1_extract(&self) -> u64 {
        // bits 25..25
        self.0.get_bits(25..=25)
    }

    /// reads the current register value and extract field `tco_1` from it
    pub fn tco_1_read() -> u64 {
        Self::with_reg_val().tco_1_extract()
    }

    /// inserts the given value `val` into the field `tco_1`
    pub fn tco_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 25..25
        self.0.set_bits(25..=25, val);
        self
    }

    /// reads the register, updates the `tco_1` field, and writes the updated value
    pub fn tco_1_write(val: u64) {
        Self::with_reg_val().tco_1_insert(val).write();
    }

    /*
     * Field: dit_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dit_1_extract(&self) -> u64 {
        // bits 24..24
        self.0.get_bits(24..=24)
    }

    /// reads the current register value and extract field `dit_1` from it
    pub fn dit_1_read() -> u64 {
        Self::with_reg_val().dit_1_extract()
    }

    /// inserts the given value `val` into the field `dit_1`
    pub fn dit_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 24..24
        self.0.set_bits(24..=24, val);
        self
    }

    /// reads the register, updates the `dit_1` field, and writes the updated value
    pub fn dit_1_write(val: u64) {
        Self::with_reg_val().dit_1_insert(val).write();
    }

    /*
     * Field: uao_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn uao_1_extract(&self) -> u64 {
        // bits 23..23
        self.0.get_bits(23..=23)
    }

    /// reads the current register value and extract field `uao_1` from it
    pub fn uao_1_read() -> u64 {
        Self::with_reg_val().uao_1_extract()
    }

    /// inserts the given value `val` into the field `uao_1`
    pub fn uao_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 23..23
        self.0.set_bits(23..=23, val);
        self
    }

    /// reads the register, updates the `uao_1` field, and writes the updated value
    pub fn uao_1_write(val: u64) {
        Self::with_reg_val().uao_1_insert(val).write();
    }

    /*
     * Field: pan_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn pan_1_extract(&self) -> u64 {
        // bits 22..22
        self.0.get_bits(22..=22)
    }

    /// reads the current register value and extract field `pan_1` from it
    pub fn pan_1_read() -> u64 {
        Self::with_reg_val().pan_1_extract()
    }

    /// inserts the given value `val` into the field `pan_1`
    pub fn pan_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 22..22
        self.0.set_bits(22..=22, val);
        self
    }

    /// reads the register, updates the `pan_1` field, and writes the updated value
    pub fn pan_1_write(val: u64) {
        Self::with_reg_val().pan_1_insert(val).write();
    }

    /*
     * Field: ss
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ss_extract(&self) -> u64 {
        // bits 21..21
        self.0.get_bits(21..=21)
    }

    /// reads the current register value and extract field `ss` from it
    pub fn ss_read() -> u64 {
        Self::with_reg_val().ss_extract()
    }

    /// inserts the given value `val` into the field `ss`
    pub fn ss_insert(&mut self, val: u64) -> &mut Self {
        // bits 21..21
        self.0.set_bits(21..=21, val);
        self
    }

    /// reads the register, updates the `ss` field, and writes the updated value
    pub fn ss_write(val: u64) {
        Self::with_reg_val().ss_insert(val).write();
    }

    /*
     * Field: il
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn il_extract(&self) -> u64 {
        // bits 20..20
        self.0.get_bits(20..=20)
    }

    /// reads the current register value and extract field `il` from it
    pub fn il_read() -> u64 {
        Self::with_reg_val().il_extract()
    }

    /// inserts the given value `val` into the field `il`
    pub fn il_insert(&mut self, val: u64) -> &mut Self {
        // bits 20..20
        self.0.set_bits(20..=20, val);
        self
    }

    /// reads the register, updates the `il` field, and writes the updated value
    pub fn il_write(val: u64) {
        Self::with_reg_val().il_insert(val).write();
    }

    /*
     * Field: ssbs_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ssbs_1_extract(&self) -> u64 {
        // bits 12..12
        self.0.get_bits(12..=12)
    }

    /// reads the current register value and extract field `ssbs_1` from it
    pub fn ssbs_1_read() -> u64 {
        Self::with_reg_val().ssbs_1_extract()
    }

    /// inserts the given value `val` into the field `ssbs_1`
    pub fn ssbs_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 12..12
        self.0.set_bits(12..=12, val);
        self
    }

    /// reads the register, updates the `ssbs_1` field, and writes the updated value
    pub fn ssbs_1_write(val: u64) {
        Self::with_reg_val().ssbs_1_insert(val).write();
    }

    /*
     * Field: btype_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn btype_1_extract(&self) -> u64 {
        // bits 10..11
        self.0.get_bits(10..=11)
    }

    /// reads the current register value and extract field `btype_1` from it
    pub fn btype_1_read() -> u64 {
        Self::with_reg_val().btype_1_extract()
    }

    /// inserts the given value `val` into the field `btype_1`
    pub fn btype_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 10..11
        self.0.set_bits(10..=11, val);
        self
    }

    /// reads the register, updates the `btype_1` field, and writes the updated value
    pub fn btype_1_write(val: u64) {
        Self::with_reg_val().btype_1_insert(val).write();
    }

    /*
     * Field: d
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn d_extract(&self) -> u64 {
        // bits 9..9
        self.0.get_bits(9..=9)
    }

    /// reads the current register value and extract field `d` from it
    pub fn d_read() -> u64 {
        Self::with_reg_val().d_extract()
    }

    /// inserts the given value `val` into the field `d`
    pub fn d_insert(&mut self, val: u64) -> &mut Self {
        // bits 9..9
        self.0.set_bits(9..=9, val);
        self
    }

    /// reads the register, updates the `d` field, and writes the updated value
    pub fn d_write(val: u64) {
        Self::with_reg_val().d_insert(val).write();
    }

    /*
     * Field: a
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn a_extract(&self) -> u64 {
        // bits 8..8
        self.0.get_bits(8..=8)
    }

    /// reads the current register value and extract field `a` from it
    pub fn a_read() -> u64 {
        Self::with_reg_val().a_extract()
    }

    /// inserts the given value `val` into the field `a`
    pub fn a_insert(&mut self, val: u64) -> &mut Self {
        // bits 8..8
        self.0.set_bits(8..=8, val);
        self
    }

    /// reads the register, updates the `a` field, and writes the updated value
    pub fn a_write(val: u64) {
        Self::with_reg_val().a_insert(val).write();
    }

    /*
     * Field: i
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn i_extract(&self) -> u64 {
        // bits 7..7
        self.0.get_bits(7..=7)
    }

    /// reads the current register value and extract field `i` from it
    pub fn i_read() -> u64 {
        Self::with_reg_val().i_extract()
    }

    /// inserts the given value `val` into the field `i`
    pub fn i_insert(&mut self, val: u64) -> &mut Self {
        // bits 7..7
        self.0.set_bits(7..=7, val);
        self
    }

    /// reads the register, updates the `i` field, and writes the updated value
    pub fn i_write(val: u64) {
        Self::with_reg_val().i_insert(val).write();
    }

    /*
     * Field: f
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn f_extract(&self) -> u64 {
        // bits 6..6
        self.0.get_bits(6..=6)
    }

    /// reads the current register value and extract field `f` from it
    pub fn f_read() -> u64 {
        Self::with_reg_val().f_extract()
    }

    /// inserts the given value `val` into the field `f`
    pub fn f_insert(&mut self, val: u64) -> &mut Self {
        // bits 6..6
        self.0.set_bits(6..=6, val);
        self
    }

    /// reads the register, updates the `f` field, and writes the updated value
    pub fn f_write(val: u64) {
        Self::with_reg_val().f_insert(val).write();
    }

    /*
     * Field: m4
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn m4_extract(&self) -> u64 {
        // bits 4..4
        self.0.get_bits(4..=4)
    }

    /// reads the current register value and extract field `m4` from it
    pub fn m4_read() -> u64 {
        Self::with_reg_val().m4_extract()
    }

    /// inserts the given value `val` into the field `m4`
    pub fn m4_insert(&mut self, val: u64) -> &mut Self {
        // bits 4..4
        self.0.set_bits(4..=4, val);
        self
    }

    /// reads the register, updates the `m4` field, and writes the updated value
    pub fn m4_write(val: u64) {
        Self::with_reg_val().m4_insert(val).write();
    }

    /*
     * Field: m30
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn m30_extract(&self) -> u64 {
        // bits 0..3
        self.0.get_bits(0..=3)
    }

    /// reads the current register value and extract field `m30` from it
    pub fn m30_read() -> u64 {
        Self::with_reg_val().m30_extract()
    }

    /// inserts the given value `val` into the field `m30`
    pub fn m30_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..3
        self.0.set_bits(0..=3, val);
        self
    }

    /// reads the register, updates the `m30` field, and writes the updated value
    pub fn m30_write(val: u64) {
        Self::with_reg_val().m30_insert(val).write();
    }
}

impl Default for SpsrEl2 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> SpsrEl2 {
        SpsrEl2(0)
    }
}
