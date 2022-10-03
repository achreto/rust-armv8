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
 * Generated on: 2022-08-22T16:35:53.077296
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
 * Register:    System Control Register (EL1) (sctlr_el1)
 * Group:       Other system control registers
 * Type:        64-bit Register
 * Description: Provides top level control of the system, including its memory system, at EL1 and EL0.
 * File:        AArch64-sctlr_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the System Control Register (EL1) value in memory
pub struct SctlrEl1(u64);

/// struct implementation for accessing the fields of register sctlr_el1
impl SctlrEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> SctlrEl1 {
        Self::default()
    }

    /// collects the modifications of SctlrEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> SctlrEl1 {
        SctlrEl1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> SctlrEl1 {
        let curval = Self::reg_rawrd() & 0x3c3fff8fffdffff;
        SctlrEl1(curval)
    }

    /// reading the System Control Register (EL1) (sctlr_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, SCTLR_EL1
            asm!("mrs {}, sctlr_el1", out(reg) regval);
        }
        return regval;
    }

    /// writing the System Control Register (EL1) (sctlr_el1) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR SCTLR_EL1, <Xt>
            asm!("msr sctlr_el1, {}", in(reg) val);
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0x3c3fff8fffdffff;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 271341847484170239;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: epan_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn epan_1_extract(&self) -> u64 {
        // bits 57..57
        self.0.get_bits(57..=57)
    }

    /// reads the current register value and extract field `epan_1` from it
    pub fn epan_1_read() -> u64 {
        Self::with_reg_val().epan_1_extract()
    }

    /// inserts the given value `val` into the field `epan_1`
    pub fn epan_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 57..57
        self.0.set_bits(57..=57, val);
        self
    }

    /// reads the register, updates the `epan_1` field, and writes the updated value
    pub fn epan_1_write(val: u64) {
        Self::with_reg_val().epan_1_insert(val).write();
    }

    /*
     * Field: enals_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn enals_1_extract(&self) -> u64 {
        // bits 56..56
        self.0.get_bits(56..=56)
    }

    /// reads the current register value and extract field `enals_1` from it
    pub fn enals_1_read() -> u64 {
        Self::with_reg_val().enals_1_extract()
    }

    /// inserts the given value `val` into the field `enals_1`
    pub fn enals_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 56..56
        self.0.set_bits(56..=56, val);
        self
    }

    /// reads the register, updates the `enals_1` field, and writes the updated value
    pub fn enals_1_write(val: u64) {
        Self::with_reg_val().enals_1_insert(val).write();
    }

    /*
     * Field: enas0_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn enas0_1_extract(&self) -> u64 {
        // bits 55..55
        self.0.get_bits(55..=55)
    }

    /// reads the current register value and extract field `enas0_1` from it
    pub fn enas0_1_read() -> u64 {
        Self::with_reg_val().enas0_1_extract()
    }

    /// inserts the given value `val` into the field `enas0_1`
    pub fn enas0_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 55..55
        self.0.set_bits(55..=55, val);
        self
    }

    /// reads the register, updates the `enas0_1` field, and writes the updated value
    pub fn enas0_1_write(val: u64) {
        Self::with_reg_val().enas0_1_insert(val).write();
    }

    /*
     * Field: enasr_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn enasr_1_extract(&self) -> u64 {
        // bits 54..54
        self.0.get_bits(54..=54)
    }

    /// reads the current register value and extract field `enasr_1` from it
    pub fn enasr_1_read() -> u64 {
        Self::with_reg_val().enasr_1_extract()
    }

    /// inserts the given value `val` into the field `enasr_1`
    pub fn enasr_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 54..54
        self.0.set_bits(54..=54, val);
        self
    }

    /// reads the register, updates the `enasr_1` field, and writes the updated value
    pub fn enasr_1_write(val: u64) {
        Self::with_reg_val().enasr_1_insert(val).write();
    }

    /*
     * Field: twedel_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn twedel_1_extract(&self) -> u64 {
        // bits 46..49
        self.0.get_bits(46..=49)
    }

    /// reads the current register value and extract field `twedel_1` from it
    pub fn twedel_1_read() -> u64 {
        Self::with_reg_val().twedel_1_extract()
    }

    /// inserts the given value `val` into the field `twedel_1`
    pub fn twedel_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 46..49
        self.0.set_bits(46..=49, val);
        self
    }

    /// reads the register, updates the `twedel_1` field, and writes the updated value
    pub fn twedel_1_write(val: u64) {
        Self::with_reg_val().twedel_1_insert(val).write();
    }

    /*
     * Field: tweden_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tweden_1_extract(&self) -> u64 {
        // bits 45..45
        self.0.get_bits(45..=45)
    }

    /// reads the current register value and extract field `tweden_1` from it
    pub fn tweden_1_read() -> u64 {
        Self::with_reg_val().tweden_1_extract()
    }

    /// inserts the given value `val` into the field `tweden_1`
    pub fn tweden_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 45..45
        self.0.set_bits(45..=45, val);
        self
    }

    /// reads the register, updates the `tweden_1` field, and writes the updated value
    pub fn tweden_1_write(val: u64) {
        Self::with_reg_val().tweden_1_insert(val).write();
    }

    /*
     * Field: dssbs_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dssbs_1_extract(&self) -> u64 {
        // bits 44..44
        self.0.get_bits(44..=44)
    }

    /// reads the current register value and extract field `dssbs_1` from it
    pub fn dssbs_1_read() -> u64 {
        Self::with_reg_val().dssbs_1_extract()
    }

    /// inserts the given value `val` into the field `dssbs_1`
    pub fn dssbs_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 44..44
        self.0.set_bits(44..=44, val);
        self
    }

    /// reads the register, updates the `dssbs_1` field, and writes the updated value
    pub fn dssbs_1_write(val: u64) {
        Self::with_reg_val().dssbs_1_insert(val).write();
    }

    /*
     * Field: ata_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ata_1_extract(&self) -> u64 {
        // bits 43..43
        self.0.get_bits(43..=43)
    }

    /// reads the current register value and extract field `ata_1` from it
    pub fn ata_1_read() -> u64 {
        Self::with_reg_val().ata_1_extract()
    }

    /// inserts the given value `val` into the field `ata_1`
    pub fn ata_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 43..43
        self.0.set_bits(43..=43, val);
        self
    }

    /// reads the register, updates the `ata_1` field, and writes the updated value
    pub fn ata_1_write(val: u64) {
        Self::with_reg_val().ata_1_insert(val).write();
    }

    /*
     * Field: ata0_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ata0_1_extract(&self) -> u64 {
        // bits 42..42
        self.0.get_bits(42..=42)
    }

    /// reads the current register value and extract field `ata0_1` from it
    pub fn ata0_1_read() -> u64 {
        Self::with_reg_val().ata0_1_extract()
    }

    /// inserts the given value `val` into the field `ata0_1`
    pub fn ata0_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 42..42
        self.0.set_bits(42..=42, val);
        self
    }

    /// reads the register, updates the `ata0_1` field, and writes the updated value
    pub fn ata0_1_write(val: u64) {
        Self::with_reg_val().ata0_1_insert(val).write();
    }

    /*
     * Field: tcf_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tcf_1_extract(&self) -> u64 {
        // bits 40..41
        self.0.get_bits(40..=41)
    }

    /// reads the current register value and extract field `tcf_1` from it
    pub fn tcf_1_read() -> u64 {
        Self::with_reg_val().tcf_1_extract()
    }

    /// inserts the given value `val` into the field `tcf_1`
    pub fn tcf_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 40..41
        self.0.set_bits(40..=41, val);
        self
    }

    /// reads the register, updates the `tcf_1` field, and writes the updated value
    pub fn tcf_1_write(val: u64) {
        Self::with_reg_val().tcf_1_insert(val).write();
    }

    /*
     * Field: tcf0_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tcf0_1_extract(&self) -> u64 {
        // bits 38..39
        self.0.get_bits(38..=39)
    }

    /// reads the current register value and extract field `tcf0_1` from it
    pub fn tcf0_1_read() -> u64 {
        Self::with_reg_val().tcf0_1_extract()
    }

    /// inserts the given value `val` into the field `tcf0_1`
    pub fn tcf0_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 38..39
        self.0.set_bits(38..=39, val);
        self
    }

    /// reads the register, updates the `tcf0_1` field, and writes the updated value
    pub fn tcf0_1_write(val: u64) {
        Self::with_reg_val().tcf0_1_insert(val).write();
    }

    /*
     * Field: itfsb_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn itfsb_1_extract(&self) -> u64 {
        // bits 37..37
        self.0.get_bits(37..=37)
    }

    /// reads the current register value and extract field `itfsb_1` from it
    pub fn itfsb_1_read() -> u64 {
        Self::with_reg_val().itfsb_1_extract()
    }

    /// inserts the given value `val` into the field `itfsb_1`
    pub fn itfsb_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 37..37
        self.0.set_bits(37..=37, val);
        self
    }

    /// reads the register, updates the `itfsb_1` field, and writes the updated value
    pub fn itfsb_1_write(val: u64) {
        Self::with_reg_val().itfsb_1_insert(val).write();
    }

    /*
     * Field: bt1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn bt1_1_extract(&self) -> u64 {
        // bits 36..36
        self.0.get_bits(36..=36)
    }

    /// reads the current register value and extract field `bt1_1` from it
    pub fn bt1_1_read() -> u64 {
        Self::with_reg_val().bt1_1_extract()
    }

    /// inserts the given value `val` into the field `bt1_1`
    pub fn bt1_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 36..36
        self.0.set_bits(36..=36, val);
        self
    }

    /// reads the register, updates the `bt1_1` field, and writes the updated value
    pub fn bt1_1_write(val: u64) {
        Self::with_reg_val().bt1_1_insert(val).write();
    }

    /*
     * Field: bt0_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn bt0_1_extract(&self) -> u64 {
        // bits 35..35
        self.0.get_bits(35..=35)
    }

    /// reads the current register value and extract field `bt0_1` from it
    pub fn bt0_1_read() -> u64 {
        Self::with_reg_val().bt0_1_extract()
    }

    /// inserts the given value `val` into the field `bt0_1`
    pub fn bt0_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 35..35
        self.0.set_bits(35..=35, val);
        self
    }

    /// reads the register, updates the `bt0_1` field, and writes the updated value
    pub fn bt0_1_write(val: u64) {
        Self::with_reg_val().bt0_1_insert(val).write();
    }

    /*
     * Field: enia_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn enia_1_extract(&self) -> u64 {
        // bits 31..31
        self.0.get_bits(31..=31)
    }

    /// reads the current register value and extract field `enia_1` from it
    pub fn enia_1_read() -> u64 {
        Self::with_reg_val().enia_1_extract()
    }

    /// inserts the given value `val` into the field `enia_1`
    pub fn enia_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 31..31
        self.0.set_bits(31..=31, val);
        self
    }

    /// reads the register, updates the `enia_1` field, and writes the updated value
    pub fn enia_1_write(val: u64) {
        Self::with_reg_val().enia_1_insert(val).write();
    }

    /*
     * Field: enib_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn enib_1_extract(&self) -> u64 {
        // bits 30..30
        self.0.get_bits(30..=30)
    }

    /// reads the current register value and extract field `enib_1` from it
    pub fn enib_1_read() -> u64 {
        Self::with_reg_val().enib_1_extract()
    }

    /// inserts the given value `val` into the field `enib_1`
    pub fn enib_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 30..30
        self.0.set_bits(30..=30, val);
        self
    }

    /// reads the register, updates the `enib_1` field, and writes the updated value
    pub fn enib_1_write(val: u64) {
        Self::with_reg_val().enib_1_insert(val).write();
    }

    /*
     * Field: lsmaoe_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn lsmaoe_1_extract(&self) -> u64 {
        // bits 29..29
        self.0.get_bits(29..=29)
    }

    /// reads the current register value and extract field `lsmaoe_1` from it
    pub fn lsmaoe_1_read() -> u64 {
        Self::with_reg_val().lsmaoe_1_extract()
    }

    /// inserts the given value `val` into the field `lsmaoe_1`
    pub fn lsmaoe_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 29..29
        self.0.set_bits(29..=29, val);
        self
    }

    /// reads the register, updates the `lsmaoe_1` field, and writes the updated value
    pub fn lsmaoe_1_write(val: u64) {
        Self::with_reg_val().lsmaoe_1_insert(val).write();
    }

    /*
     * Field: ntlsmd_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ntlsmd_1_extract(&self) -> u64 {
        // bits 28..28
        self.0.get_bits(28..=28)
    }

    /// reads the current register value and extract field `ntlsmd_1` from it
    pub fn ntlsmd_1_read() -> u64 {
        Self::with_reg_val().ntlsmd_1_extract()
    }

    /// inserts the given value `val` into the field `ntlsmd_1`
    pub fn ntlsmd_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 28..28
        self.0.set_bits(28..=28, val);
        self
    }

    /// reads the register, updates the `ntlsmd_1` field, and writes the updated value
    pub fn ntlsmd_1_write(val: u64) {
        Self::with_reg_val().ntlsmd_1_insert(val).write();
    }

    /*
     * Field: enda_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn enda_1_extract(&self) -> u64 {
        // bits 27..27
        self.0.get_bits(27..=27)
    }

    /// reads the current register value and extract field `enda_1` from it
    pub fn enda_1_read() -> u64 {
        Self::with_reg_val().enda_1_extract()
    }

    /// inserts the given value `val` into the field `enda_1`
    pub fn enda_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 27..27
        self.0.set_bits(27..=27, val);
        self
    }

    /// reads the register, updates the `enda_1` field, and writes the updated value
    pub fn enda_1_write(val: u64) {
        Self::with_reg_val().enda_1_insert(val).write();
    }

    /*
     * Field: uci
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn uci_extract(&self) -> u64 {
        // bits 26..26
        self.0.get_bits(26..=26)
    }

    /// reads the current register value and extract field `uci` from it
    pub fn uci_read() -> u64 {
        Self::with_reg_val().uci_extract()
    }

    /// inserts the given value `val` into the field `uci`
    pub fn uci_insert(&mut self, val: u64) -> &mut Self {
        // bits 26..26
        self.0.set_bits(26..=26, val);
        self
    }

    /// reads the register, updates the `uci` field, and writes the updated value
    pub fn uci_write(val: u64) {
        Self::with_reg_val().uci_insert(val).write();
    }

    /*
     * Field: ee
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ee_extract(&self) -> u64 {
        // bits 25..25
        self.0.get_bits(25..=25)
    }

    /// reads the current register value and extract field `ee` from it
    pub fn ee_read() -> u64 {
        Self::with_reg_val().ee_extract()
    }

    /// inserts the given value `val` into the field `ee`
    pub fn ee_insert(&mut self, val: u64) -> &mut Self {
        // bits 25..25
        self.0.set_bits(25..=25, val);
        self
    }

    /// reads the register, updates the `ee` field, and writes the updated value
    pub fn ee_write(val: u64) {
        Self::with_reg_val().ee_insert(val).write();
    }

    /*
     * Field: e0e
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn e0e_extract(&self) -> u64 {
        // bits 24..24
        self.0.get_bits(24..=24)
    }

    /// reads the current register value and extract field `e0e` from it
    pub fn e0e_read() -> u64 {
        Self::with_reg_val().e0e_extract()
    }

    /// inserts the given value `val` into the field `e0e`
    pub fn e0e_insert(&mut self, val: u64) -> &mut Self {
        // bits 24..24
        self.0.set_bits(24..=24, val);
        self
    }

    /// reads the register, updates the `e0e` field, and writes the updated value
    pub fn e0e_write(val: u64) {
        Self::with_reg_val().e0e_insert(val).write();
    }

    /*
     * Field: span_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn span_1_extract(&self) -> u64 {
        // bits 23..23
        self.0.get_bits(23..=23)
    }

    /// reads the current register value and extract field `span_1` from it
    pub fn span_1_read() -> u64 {
        Self::with_reg_val().span_1_extract()
    }

    /// inserts the given value `val` into the field `span_1`
    pub fn span_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 23..23
        self.0.set_bits(23..=23, val);
        self
    }

    /// reads the register, updates the `span_1` field, and writes the updated value
    pub fn span_1_write(val: u64) {
        Self::with_reg_val().span_1_insert(val).write();
    }

    /*
     * Field: eis_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn eis_1_extract(&self) -> u64 {
        // bits 22..22
        self.0.get_bits(22..=22)
    }

    /// reads the current register value and extract field `eis_1` from it
    pub fn eis_1_read() -> u64 {
        Self::with_reg_val().eis_1_extract()
    }

    /// inserts the given value `val` into the field `eis_1`
    pub fn eis_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 22..22
        self.0.set_bits(22..=22, val);
        self
    }

    /// reads the register, updates the `eis_1` field, and writes the updated value
    pub fn eis_1_write(val: u64) {
        Self::with_reg_val().eis_1_insert(val).write();
    }

    /*
     * Field: iesb_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn iesb_1_extract(&self) -> u64 {
        // bits 21..21
        self.0.get_bits(21..=21)
    }

    /// reads the current register value and extract field `iesb_1` from it
    pub fn iesb_1_read() -> u64 {
        Self::with_reg_val().iesb_1_extract()
    }

    /// inserts the given value `val` into the field `iesb_1`
    pub fn iesb_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 21..21
        self.0.set_bits(21..=21, val);
        self
    }

    /// reads the register, updates the `iesb_1` field, and writes the updated value
    pub fn iesb_1_write(val: u64) {
        Self::with_reg_val().iesb_1_insert(val).write();
    }

    /*
     * Field: tscxt_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tscxt_1_extract(&self) -> u64 {
        // bits 20..20
        self.0.get_bits(20..=20)
    }

    /// reads the current register value and extract field `tscxt_1` from it
    pub fn tscxt_1_read() -> u64 {
        Self::with_reg_val().tscxt_1_extract()
    }

    /// inserts the given value `val` into the field `tscxt_1`
    pub fn tscxt_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 20..20
        self.0.set_bits(20..=20, val);
        self
    }

    /// reads the register, updates the `tscxt_1` field, and writes the updated value
    pub fn tscxt_1_write(val: u64) {
        Self::with_reg_val().tscxt_1_insert(val).write();
    }

    /*
     * Field: wxn
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn wxn_extract(&self) -> u64 {
        // bits 19..19
        self.0.get_bits(19..=19)
    }

    /// reads the current register value and extract field `wxn` from it
    pub fn wxn_read() -> u64 {
        Self::with_reg_val().wxn_extract()
    }

    /// inserts the given value `val` into the field `wxn`
    pub fn wxn_insert(&mut self, val: u64) -> &mut Self {
        // bits 19..19
        self.0.set_bits(19..=19, val);
        self
    }

    /// reads the register, updates the `wxn` field, and writes the updated value
    pub fn wxn_write(val: u64) {
        Self::with_reg_val().wxn_insert(val).write();
    }

    /*
     * Field: ntwe
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ntwe_extract(&self) -> u64 {
        // bits 18..18
        self.0.get_bits(18..=18)
    }

    /// reads the current register value and extract field `ntwe` from it
    pub fn ntwe_read() -> u64 {
        Self::with_reg_val().ntwe_extract()
    }

    /// inserts the given value `val` into the field `ntwe`
    pub fn ntwe_insert(&mut self, val: u64) -> &mut Self {
        // bits 18..18
        self.0.set_bits(18..=18, val);
        self
    }

    /// reads the register, updates the `ntwe` field, and writes the updated value
    pub fn ntwe_write(val: u64) {
        Self::with_reg_val().ntwe_insert(val).write();
    }

    /*
     * Field: ntwi
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ntwi_extract(&self) -> u64 {
        // bits 16..16
        self.0.get_bits(16..=16)
    }

    /// reads the current register value and extract field `ntwi` from it
    pub fn ntwi_read() -> u64 {
        Self::with_reg_val().ntwi_extract()
    }

    /// inserts the given value `val` into the field `ntwi`
    pub fn ntwi_insert(&mut self, val: u64) -> &mut Self {
        // bits 16..16
        self.0.set_bits(16..=16, val);
        self
    }

    /// reads the register, updates the `ntwi` field, and writes the updated value
    pub fn ntwi_write(val: u64) {
        Self::with_reg_val().ntwi_insert(val).write();
    }

    /*
     * Field: uct
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn uct_extract(&self) -> u64 {
        // bits 15..15
        self.0.get_bits(15..=15)
    }

    /// reads the current register value and extract field `uct` from it
    pub fn uct_read() -> u64 {
        Self::with_reg_val().uct_extract()
    }

    /// inserts the given value `val` into the field `uct`
    pub fn uct_insert(&mut self, val: u64) -> &mut Self {
        // bits 15..15
        self.0.set_bits(15..=15, val);
        self
    }

    /// reads the register, updates the `uct` field, and writes the updated value
    pub fn uct_write(val: u64) {
        Self::with_reg_val().uct_insert(val).write();
    }

    /*
     * Field: dze
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dze_extract(&self) -> u64 {
        // bits 14..14
        self.0.get_bits(14..=14)
    }

    /// reads the current register value and extract field `dze` from it
    pub fn dze_read() -> u64 {
        Self::with_reg_val().dze_extract()
    }

    /// inserts the given value `val` into the field `dze`
    pub fn dze_insert(&mut self, val: u64) -> &mut Self {
        // bits 14..14
        self.0.set_bits(14..=14, val);
        self
    }

    /// reads the register, updates the `dze` field, and writes the updated value
    pub fn dze_write(val: u64) {
        Self::with_reg_val().dze_insert(val).write();
    }

    /*
     * Field: endb_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn endb_1_extract(&self) -> u64 {
        // bits 13..13
        self.0.get_bits(13..=13)
    }

    /// reads the current register value and extract field `endb_1` from it
    pub fn endb_1_read() -> u64 {
        Self::with_reg_val().endb_1_extract()
    }

    /// inserts the given value `val` into the field `endb_1`
    pub fn endb_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 13..13
        self.0.set_bits(13..=13, val);
        self
    }

    /// reads the register, updates the `endb_1` field, and writes the updated value
    pub fn endb_1_write(val: u64) {
        Self::with_reg_val().endb_1_insert(val).write();
    }

    /*
     * Field: i
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn i_extract(&self) -> u64 {
        // bits 12..12
        self.0.get_bits(12..=12)
    }

    /// reads the current register value and extract field `i` from it
    pub fn i_read() -> u64 {
        Self::with_reg_val().i_extract()
    }

    /// inserts the given value `val` into the field `i`
    pub fn i_insert(&mut self, val: u64) -> &mut Self {
        // bits 12..12
        self.0.set_bits(12..=12, val);
        self
    }

    /// reads the register, updates the `i` field, and writes the updated value
    pub fn i_write(val: u64) {
        Self::with_reg_val().i_insert(val).write();
    }

    /*
     * Field: eos_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn eos_1_extract(&self) -> u64 {
        // bits 11..11
        self.0.get_bits(11..=11)
    }

    /// reads the current register value and extract field `eos_1` from it
    pub fn eos_1_read() -> u64 {
        Self::with_reg_val().eos_1_extract()
    }

    /// inserts the given value `val` into the field `eos_1`
    pub fn eos_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 11..11
        self.0.set_bits(11..=11, val);
        self
    }

    /// reads the register, updates the `eos_1` field, and writes the updated value
    pub fn eos_1_write(val: u64) {
        Self::with_reg_val().eos_1_insert(val).write();
    }

    /*
     * Field: enrctx_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn enrctx_1_extract(&self) -> u64 {
        // bits 10..10
        self.0.get_bits(10..=10)
    }

    /// reads the current register value and extract field `enrctx_1` from it
    pub fn enrctx_1_read() -> u64 {
        Self::with_reg_val().enrctx_1_extract()
    }

    /// inserts the given value `val` into the field `enrctx_1`
    pub fn enrctx_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 10..10
        self.0.set_bits(10..=10, val);
        self
    }

    /// reads the register, updates the `enrctx_1` field, and writes the updated value
    pub fn enrctx_1_write(val: u64) {
        Self::with_reg_val().enrctx_1_insert(val).write();
    }

    /*
     * Field: uma
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn uma_extract(&self) -> u64 {
        // bits 9..9
        self.0.get_bits(9..=9)
    }

    /// reads the current register value and extract field `uma` from it
    pub fn uma_read() -> u64 {
        Self::with_reg_val().uma_extract()
    }

    /// inserts the given value `val` into the field `uma`
    pub fn uma_insert(&mut self, val: u64) -> &mut Self {
        // bits 9..9
        self.0.set_bits(9..=9, val);
        self
    }

    /// reads the register, updates the `uma` field, and writes the updated value
    pub fn uma_write(val: u64) {
        Self::with_reg_val().uma_insert(val).write();
    }

    /*
     * Field: sed_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn sed_1_extract(&self) -> u64 {
        // bits 8..8
        self.0.get_bits(8..=8)
    }

    /// reads the current register value and extract field `sed_1` from it
    pub fn sed_1_read() -> u64 {
        Self::with_reg_val().sed_1_extract()
    }

    /// inserts the given value `val` into the field `sed_1`
    pub fn sed_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 8..8
        self.0.set_bits(8..=8, val);
        self
    }

    /// reads the register, updates the `sed_1` field, and writes the updated value
    pub fn sed_1_write(val: u64) {
        Self::with_reg_val().sed_1_insert(val).write();
    }

    /*
     * Field: itd_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn itd_1_extract(&self) -> u64 {
        // bits 7..7
        self.0.get_bits(7..=7)
    }

    /// reads the current register value and extract field `itd_1` from it
    pub fn itd_1_read() -> u64 {
        Self::with_reg_val().itd_1_extract()
    }

    /// inserts the given value `val` into the field `itd_1`
    pub fn itd_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 7..7
        self.0.set_bits(7..=7, val);
        self
    }

    /// reads the register, updates the `itd_1` field, and writes the updated value
    pub fn itd_1_write(val: u64) {
        Self::with_reg_val().itd_1_insert(val).write();
    }

    /*
     * Field: naa_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn naa_1_extract(&self) -> u64 {
        // bits 6..6
        self.0.get_bits(6..=6)
    }

    /// reads the current register value and extract field `naa_1` from it
    pub fn naa_1_read() -> u64 {
        Self::with_reg_val().naa_1_extract()
    }

    /// inserts the given value `val` into the field `naa_1`
    pub fn naa_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 6..6
        self.0.set_bits(6..=6, val);
        self
    }

    /// reads the register, updates the `naa_1` field, and writes the updated value
    pub fn naa_1_write(val: u64) {
        Self::with_reg_val().naa_1_insert(val).write();
    }

    /*
     * Field: cp15ben_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn cp15ben_1_extract(&self) -> u64 {
        // bits 5..5
        self.0.get_bits(5..=5)
    }

    /// reads the current register value and extract field `cp15ben_1` from it
    pub fn cp15ben_1_read() -> u64 {
        Self::with_reg_val().cp15ben_1_extract()
    }

    /// inserts the given value `val` into the field `cp15ben_1`
    pub fn cp15ben_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 5..5
        self.0.set_bits(5..=5, val);
        self
    }

    /// reads the register, updates the `cp15ben_1` field, and writes the updated value
    pub fn cp15ben_1_write(val: u64) {
        Self::with_reg_val().cp15ben_1_insert(val).write();
    }

    /*
     * Field: sa0
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn sa0_extract(&self) -> u64 {
        // bits 4..4
        self.0.get_bits(4..=4)
    }

    /// reads the current register value and extract field `sa0` from it
    pub fn sa0_read() -> u64 {
        Self::with_reg_val().sa0_extract()
    }

    /// inserts the given value `val` into the field `sa0`
    pub fn sa0_insert(&mut self, val: u64) -> &mut Self {
        // bits 4..4
        self.0.set_bits(4..=4, val);
        self
    }

    /// reads the register, updates the `sa0` field, and writes the updated value
    pub fn sa0_write(val: u64) {
        Self::with_reg_val().sa0_insert(val).write();
    }

    /*
     * Field: sa
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn sa_extract(&self) -> u64 {
        // bits 3..3
        self.0.get_bits(3..=3)
    }

    /// reads the current register value and extract field `sa` from it
    pub fn sa_read() -> u64 {
        Self::with_reg_val().sa_extract()
    }

    /// inserts the given value `val` into the field `sa`
    pub fn sa_insert(&mut self, val: u64) -> &mut Self {
        // bits 3..3
        self.0.set_bits(3..=3, val);
        self
    }

    /// reads the register, updates the `sa` field, and writes the updated value
    pub fn sa_write(val: u64) {
        Self::with_reg_val().sa_insert(val).write();
    }

    /*
     * Field: c
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn c_extract(&self) -> u64 {
        // bits 2..2
        self.0.get_bits(2..=2)
    }

    /// reads the current register value and extract field `c` from it
    pub fn c_read() -> u64 {
        Self::with_reg_val().c_extract()
    }

    /// inserts the given value `val` into the field `c`
    pub fn c_insert(&mut self, val: u64) -> &mut Self {
        // bits 2..2
        self.0.set_bits(2..=2, val);
        self
    }

    /// reads the register, updates the `c` field, and writes the updated value
    pub fn c_write(val: u64) {
        Self::with_reg_val().c_insert(val).write();
    }

    /*
     * Field: a
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn a_extract(&self) -> u64 {
        // bits 1..1
        self.0.get_bits(1..=1)
    }

    /// reads the current register value and extract field `a` from it
    pub fn a_read() -> u64 {
        Self::with_reg_val().a_extract()
    }

    /// inserts the given value `val` into the field `a`
    pub fn a_insert(&mut self, val: u64) -> &mut Self {
        // bits 1..1
        self.0.set_bits(1..=1, val);
        self
    }

    /// reads the register, updates the `a` field, and writes the updated value
    pub fn a_write(val: u64) {
        Self::with_reg_val().a_insert(val).write();
    }

    /*
     * Field: m
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn m_extract(&self) -> u64 {
        // bits 0..0
        self.0.get_bits(0..=0)
    }

    /// reads the current register value and extract field `m` from it
    pub fn m_read() -> u64 {
        Self::with_reg_val().m_extract()
    }

    /// inserts the given value `val` into the field `m`
    pub fn m_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..0
        self.0.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `m` field, and writes the updated value
    pub fn m_write(val: u64) {
        Self::with_reg_val().m_insert(val).write();
    }
}

impl Default for SctlrEl1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> SctlrEl1 {
        SctlrEl1(0)
    }
}
