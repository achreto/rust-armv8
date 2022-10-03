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
 * Generated on: 2022-08-22T16:35:53.057340
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
 * Register:    Floating-point Control Register (fpcr)
 * Group:       Floating-point registers
 * Type:        64-bit Register
 * Description: Controls floating-point behavior.
 * File:        AArch64-fpcr.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Floating-point Control Register value in memory
pub struct Fpcr(u64);

/// struct implementation for accessing the fields of register fpcr
impl Fpcr {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> Fpcr {
        Self::default()
    }

    /// collects the modifications of Fpcr and creates new object
    #[inline(always)]
    pub fn collect(&self) -> Fpcr {
        Fpcr(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> Fpcr {
        let curval = Self::reg_rawrd() & 0x7ff9f07;
        Fpcr(curval)
    }

    /// reading the Floating-point Control Register (fpcr) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, FPCR
            asm!("mrs {}, fpcr", out(reg) regval);
        }
        return regval;
    }

    /// writing the Floating-point Control Register (fpcr) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR FPCR, <Xt>
            asm!("msr fpcr, {}", in(reg) val);
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0x7ff9f07;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 134192903;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: ahp
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ahp_extract(&self) -> u64 {
        // bits 26..26
        self.0.get_bits(26..=26)
    }

    /// reads the current register value and extract field `ahp` from it
    pub fn ahp_read() -> u64 {
        Self::with_reg_val().ahp_extract()
    }

    /// inserts the given value `val` into the field `ahp`
    pub fn ahp_insert(&mut self, val: u64) -> &mut Self {
        // bits 26..26
        self.0.set_bits(26..=26, val);
        self
    }

    /// reads the register, updates the `ahp` field, and writes the updated value
    pub fn ahp_write(val: u64) {
        Self::with_reg_val().ahp_insert(val).write();
    }

    /*
     * Field: dn
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dn_extract(&self) -> u64 {
        // bits 25..25
        self.0.get_bits(25..=25)
    }

    /// reads the current register value and extract field `dn` from it
    pub fn dn_read() -> u64 {
        Self::with_reg_val().dn_extract()
    }

    /// inserts the given value `val` into the field `dn`
    pub fn dn_insert(&mut self, val: u64) -> &mut Self {
        // bits 25..25
        self.0.set_bits(25..=25, val);
        self
    }

    /// reads the register, updates the `dn` field, and writes the updated value
    pub fn dn_write(val: u64) {
        Self::with_reg_val().dn_insert(val).write();
    }

    /*
     * Field: fz
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn fz_extract(&self) -> u64 {
        // bits 24..24
        self.0.get_bits(24..=24)
    }

    /// reads the current register value and extract field `fz` from it
    pub fn fz_read() -> u64 {
        Self::with_reg_val().fz_extract()
    }

    /// inserts the given value `val` into the field `fz`
    pub fn fz_insert(&mut self, val: u64) -> &mut Self {
        // bits 24..24
        self.0.set_bits(24..=24, val);
        self
    }

    /// reads the register, updates the `fz` field, and writes the updated value
    pub fn fz_write(val: u64) {
        Self::with_reg_val().fz_insert(val).write();
    }

    /*
     * Field: rmode
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn rmode_extract(&self) -> u64 {
        // bits 22..23
        self.0.get_bits(22..=23)
    }

    /// reads the current register value and extract field `rmode` from it
    pub fn rmode_read() -> u64 {
        Self::with_reg_val().rmode_extract()
    }

    /// inserts the given value `val` into the field `rmode`
    pub fn rmode_insert(&mut self, val: u64) -> &mut Self {
        // bits 22..23
        self.0.set_bits(22..=23, val);
        self
    }

    /// reads the register, updates the `rmode` field, and writes the updated value
    pub fn rmode_write(val: u64) {
        Self::with_reg_val().rmode_insert(val).write();
    }

    /*
     * Field: stride
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn stride_extract(&self) -> u64 {
        // bits 20..21
        self.0.get_bits(20..=21)
    }

    /// reads the current register value and extract field `stride` from it
    pub fn stride_read() -> u64 {
        Self::with_reg_val().stride_extract()
    }

    /// inserts the given value `val` into the field `stride`
    pub fn stride_insert(&mut self, val: u64) -> &mut Self {
        // bits 20..21
        self.0.set_bits(20..=21, val);
        self
    }

    /// reads the register, updates the `stride` field, and writes the updated value
    pub fn stride_write(val: u64) {
        Self::with_reg_val().stride_insert(val).write();
    }

    /*
     * Field: fz16_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn fz16_1_extract(&self) -> u64 {
        // bits 19..19
        self.0.get_bits(19..=19)
    }

    /// reads the current register value and extract field `fz16_1` from it
    pub fn fz16_1_read() -> u64 {
        Self::with_reg_val().fz16_1_extract()
    }

    /// inserts the given value `val` into the field `fz16_1`
    pub fn fz16_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 19..19
        self.0.set_bits(19..=19, val);
        self
    }

    /// reads the register, updates the `fz16_1` field, and writes the updated value
    pub fn fz16_1_write(val: u64) {
        Self::with_reg_val().fz16_1_insert(val).write();
    }

    /*
     * Field: len
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn len_extract(&self) -> u64 {
        // bits 16..18
        self.0.get_bits(16..=18)
    }

    /// reads the current register value and extract field `len` from it
    pub fn len_read() -> u64 {
        Self::with_reg_val().len_extract()
    }

    /// inserts the given value `val` into the field `len`
    pub fn len_insert(&mut self, val: u64) -> &mut Self {
        // bits 16..18
        self.0.set_bits(16..=18, val);
        self
    }

    /// reads the register, updates the `len` field, and writes the updated value
    pub fn len_write(val: u64) {
        Self::with_reg_val().len_insert(val).write();
    }

    /*
     * Field: ide
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ide_extract(&self) -> u64 {
        // bits 15..15
        self.0.get_bits(15..=15)
    }

    /// reads the current register value and extract field `ide` from it
    pub fn ide_read() -> u64 {
        Self::with_reg_val().ide_extract()
    }

    /// inserts the given value `val` into the field `ide`
    pub fn ide_insert(&mut self, val: u64) -> &mut Self {
        // bits 15..15
        self.0.set_bits(15..=15, val);
        self
    }

    /// reads the register, updates the `ide` field, and writes the updated value
    pub fn ide_write(val: u64) {
        Self::with_reg_val().ide_insert(val).write();
    }

    /*
     * Field: ixe
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ixe_extract(&self) -> u64 {
        // bits 12..12
        self.0.get_bits(12..=12)
    }

    /// reads the current register value and extract field `ixe` from it
    pub fn ixe_read() -> u64 {
        Self::with_reg_val().ixe_extract()
    }

    /// inserts the given value `val` into the field `ixe`
    pub fn ixe_insert(&mut self, val: u64) -> &mut Self {
        // bits 12..12
        self.0.set_bits(12..=12, val);
        self
    }

    /// reads the register, updates the `ixe` field, and writes the updated value
    pub fn ixe_write(val: u64) {
        Self::with_reg_val().ixe_insert(val).write();
    }

    /*
     * Field: ufe
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ufe_extract(&self) -> u64 {
        // bits 11..11
        self.0.get_bits(11..=11)
    }

    /// reads the current register value and extract field `ufe` from it
    pub fn ufe_read() -> u64 {
        Self::with_reg_val().ufe_extract()
    }

    /// inserts the given value `val` into the field `ufe`
    pub fn ufe_insert(&mut self, val: u64) -> &mut Self {
        // bits 11..11
        self.0.set_bits(11..=11, val);
        self
    }

    /// reads the register, updates the `ufe` field, and writes the updated value
    pub fn ufe_write(val: u64) {
        Self::with_reg_val().ufe_insert(val).write();
    }

    /*
     * Field: ofe
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ofe_extract(&self) -> u64 {
        // bits 10..10
        self.0.get_bits(10..=10)
    }

    /// reads the current register value and extract field `ofe` from it
    pub fn ofe_read() -> u64 {
        Self::with_reg_val().ofe_extract()
    }

    /// inserts the given value `val` into the field `ofe`
    pub fn ofe_insert(&mut self, val: u64) -> &mut Self {
        // bits 10..10
        self.0.set_bits(10..=10, val);
        self
    }

    /// reads the register, updates the `ofe` field, and writes the updated value
    pub fn ofe_write(val: u64) {
        Self::with_reg_val().ofe_insert(val).write();
    }

    /*
     * Field: dze
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dze_extract(&self) -> u64 {
        // bits 9..9
        self.0.get_bits(9..=9)
    }

    /// reads the current register value and extract field `dze` from it
    pub fn dze_read() -> u64 {
        Self::with_reg_val().dze_extract()
    }

    /// inserts the given value `val` into the field `dze`
    pub fn dze_insert(&mut self, val: u64) -> &mut Self {
        // bits 9..9
        self.0.set_bits(9..=9, val);
        self
    }

    /// reads the register, updates the `dze` field, and writes the updated value
    pub fn dze_write(val: u64) {
        Self::with_reg_val().dze_insert(val).write();
    }

    /*
     * Field: ioe
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ioe_extract(&self) -> u64 {
        // bits 8..8
        self.0.get_bits(8..=8)
    }

    /// reads the current register value and extract field `ioe` from it
    pub fn ioe_read() -> u64 {
        Self::with_reg_val().ioe_extract()
    }

    /// inserts the given value `val` into the field `ioe`
    pub fn ioe_insert(&mut self, val: u64) -> &mut Self {
        // bits 8..8
        self.0.set_bits(8..=8, val);
        self
    }

    /// reads the register, updates the `ioe` field, and writes the updated value
    pub fn ioe_write(val: u64) {
        Self::with_reg_val().ioe_insert(val).write();
    }

    /*
     * Field: nep_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn nep_1_extract(&self) -> u64 {
        // bits 2..2
        self.0.get_bits(2..=2)
    }

    /// reads the current register value and extract field `nep_1` from it
    pub fn nep_1_read() -> u64 {
        Self::with_reg_val().nep_1_extract()
    }

    /// inserts the given value `val` into the field `nep_1`
    pub fn nep_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 2..2
        self.0.set_bits(2..=2, val);
        self
    }

    /// reads the register, updates the `nep_1` field, and writes the updated value
    pub fn nep_1_write(val: u64) {
        Self::with_reg_val().nep_1_insert(val).write();
    }

    /*
     * Field: ah_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ah_1_extract(&self) -> u64 {
        // bits 1..1
        self.0.get_bits(1..=1)
    }

    /// reads the current register value and extract field `ah_1` from it
    pub fn ah_1_read() -> u64 {
        Self::with_reg_val().ah_1_extract()
    }

    /// inserts the given value `val` into the field `ah_1`
    pub fn ah_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 1..1
        self.0.set_bits(1..=1, val);
        self
    }

    /// reads the register, updates the `ah_1` field, and writes the updated value
    pub fn ah_1_write(val: u64) {
        Self::with_reg_val().ah_1_insert(val).write();
    }

    /*
     * Field: fiz_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn fiz_1_extract(&self) -> u64 {
        // bits 0..0
        self.0.get_bits(0..=0)
    }

    /// reads the current register value and extract field `fiz_1` from it
    pub fn fiz_1_read() -> u64 {
        Self::with_reg_val().fiz_1_extract()
    }

    /// inserts the given value `val` into the field `fiz_1`
    pub fn fiz_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..0
        self.0.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `fiz_1` field, and writes the updated value
    pub fn fiz_1_write(val: u64) {
        Self::with_reg_val().fiz_1_insert(val).write();
    }
}

impl Default for Fpcr {
    /// creates a new default value
    #[inline(always)]
    fn default() -> Fpcr {
        Fpcr(0)
    }
}
