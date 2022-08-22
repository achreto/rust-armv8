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
 * Generated on: 2022-08-22T15:51:28.534521
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
 * Register:    System Control Register (EL2) (sctlr_el2)
 * Group:       Virtualization registers
 * Type:        64-bit Register
 * Description: Provides top level control of the system, including its memory system, at EL2.
 * File:        AArch64-sctlr_el2.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the System Control Register (EL2) value in memory
pub struct SctlrEl2(u64);

/// struct implementation for accessing the fields of register sctlr_el2
impl SctlrEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> SctlrEl2 {
        Self::default()
    }

    /// collects the modifications of SctlrEl2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> SctlrEl2 {
        SctlrEl2(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> SctlrEl2 {
        let curval = Self::reg_rawrd() & 0x1b30ca68384f;
        SctlrEl2(curval)
    }

    /// reading the System Control Register (EL2) (sctlr_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, SCTLR_EL2
            llvm_asm!("mrs $0, sctlr_el2" : "=r"(regval));
        }
        return regval;
    }

    /// writing the System Control Register (EL2) (sctlr_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR SCTLR_EL2, <Xt>
            llvm_asm!("msr sctlr_el2, $0" : : "r"(val));
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0x1b30ca68384f;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.val)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 29896368207951;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: dssbs_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dssbs_1_extract(&self) -> u64 {
        // bits 44..44
        self.val.get_bits(44..=44)
    }

    /// reads the current register value and extract field `dssbs_1` from it
    pub fn dssbs_1_read(&mut self) -> u64 {
        Self::with_reg_val().dssbs_1_extract()
    }

    /// inserts the given value `val` into the field `dssbs_1`
    pub fn dssbs_1_insert(&mut self, val: u64) -> &mut self {
        // bits 44..44
        self.val.set_bits(44..=44, val);
        self
    }

    /// reads the register, updates the `dssbs_1` field, and writes the updated value
    pub fn dssbs_1_write(&mut self, val: u64) {
        Self::with_reg_val().dssbs_1_insert(val).write();
    }

    /*
     * Field: ata_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ata_1_extract(&self) -> u64 {
        // bits 43..43
        self.val.get_bits(43..=43)
    }

    /// reads the current register value and extract field `ata_1` from it
    pub fn ata_1_read(&mut self) -> u64 {
        Self::with_reg_val().ata_1_extract()
    }

    /// inserts the given value `val` into the field `ata_1`
    pub fn ata_1_insert(&mut self, val: u64) -> &mut self {
        // bits 43..43
        self.val.set_bits(43..=43, val);
        self
    }

    /// reads the register, updates the `ata_1` field, and writes the updated value
    pub fn ata_1_write(&mut self, val: u64) {
        Self::with_reg_val().ata_1_insert(val).write();
    }

    /*
     * Field: tcf_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tcf_1_extract(&self) -> u64 {
        // bits 40..41
        self.val.get_bits(40..=41)
    }

    /// reads the current register value and extract field `tcf_1` from it
    pub fn tcf_1_read(&mut self) -> u64 {
        Self::with_reg_val().tcf_1_extract()
    }

    /// inserts the given value `val` into the field `tcf_1`
    pub fn tcf_1_insert(&mut self, val: u64) -> &mut self {
        // bits 40..41
        self.val.set_bits(40..=41, val);
        self
    }

    /// reads the register, updates the `tcf_1` field, and writes the updated value
    pub fn tcf_1_write(&mut self, val: u64) {
        Self::with_reg_val().tcf_1_insert(val).write();
    }

    /*
     * Field: itfsb_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn itfsb_1_extract(&self) -> u64 {
        // bits 37..37
        self.val.get_bits(37..=37)
    }

    /// reads the current register value and extract field `itfsb_1` from it
    pub fn itfsb_1_read(&mut self) -> u64 {
        Self::with_reg_val().itfsb_1_extract()
    }

    /// inserts the given value `val` into the field `itfsb_1`
    pub fn itfsb_1_insert(&mut self, val: u64) -> &mut self {
        // bits 37..37
        self.val.set_bits(37..=37, val);
        self
    }

    /// reads the register, updates the `itfsb_1` field, and writes the updated value
    pub fn itfsb_1_write(&mut self, val: u64) {
        Self::with_reg_val().itfsb_1_insert(val).write();
    }

    /*
     * Field: bt_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn bt_1_extract(&self) -> u64 {
        // bits 36..36
        self.val.get_bits(36..=36)
    }

    /// reads the current register value and extract field `bt_1` from it
    pub fn bt_1_read(&mut self) -> u64 {
        Self::with_reg_val().bt_1_extract()
    }

    /// inserts the given value `val` into the field `bt_1`
    pub fn bt_1_insert(&mut self, val: u64) -> &mut self {
        // bits 36..36
        self.val.set_bits(36..=36, val);
        self
    }

    /// reads the register, updates the `bt_1` field, and writes the updated value
    pub fn bt_1_write(&mut self, val: u64) {
        Self::with_reg_val().bt_1_insert(val).write();
    }

    /*
     * Field: enia_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn enia_1_extract(&self) -> u64 {
        // bits 31..31
        self.val.get_bits(31..=31)
    }

    /// reads the current register value and extract field `enia_1` from it
    pub fn enia_1_read(&mut self) -> u64 {
        Self::with_reg_val().enia_1_extract()
    }

    /// inserts the given value `val` into the field `enia_1`
    pub fn enia_1_insert(&mut self, val: u64) -> &mut self {
        // bits 31..31
        self.val.set_bits(31..=31, val);
        self
    }

    /// reads the register, updates the `enia_1` field, and writes the updated value
    pub fn enia_1_write(&mut self, val: u64) {
        Self::with_reg_val().enia_1_insert(val).write();
    }

    /*
     * Field: enib_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn enib_1_extract(&self) -> u64 {
        // bits 30..30
        self.val.get_bits(30..=30)
    }

    /// reads the current register value and extract field `enib_1` from it
    pub fn enib_1_read(&mut self) -> u64 {
        Self::with_reg_val().enib_1_extract()
    }

    /// inserts the given value `val` into the field `enib_1`
    pub fn enib_1_insert(&mut self, val: u64) -> &mut self {
        // bits 30..30
        self.val.set_bits(30..=30, val);
        self
    }

    /// reads the register, updates the `enib_1` field, and writes the updated value
    pub fn enib_1_write(&mut self, val: u64) {
        Self::with_reg_val().enib_1_insert(val).write();
    }

    /*
     * Field: enda_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn enda_1_extract(&self) -> u64 {
        // bits 27..27
        self.val.get_bits(27..=27)
    }

    /// reads the current register value and extract field `enda_1` from it
    pub fn enda_1_read(&mut self) -> u64 {
        Self::with_reg_val().enda_1_extract()
    }

    /// inserts the given value `val` into the field `enda_1`
    pub fn enda_1_insert(&mut self, val: u64) -> &mut self {
        // bits 27..27
        self.val.set_bits(27..=27, val);
        self
    }

    /// reads the register, updates the `enda_1` field, and writes the updated value
    pub fn enda_1_write(&mut self, val: u64) {
        Self::with_reg_val().enda_1_insert(val).write();
    }

    /*
     * Field: ee
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ee_extract(&self) -> u64 {
        // bits 25..25
        self.val.get_bits(25..=25)
    }

    /// reads the current register value and extract field `ee` from it
    pub fn ee_read(&mut self) -> u64 {
        Self::with_reg_val().ee_extract()
    }

    /// inserts the given value `val` into the field `ee`
    pub fn ee_insert(&mut self, val: u64) -> &mut self {
        // bits 25..25
        self.val.set_bits(25..=25, val);
        self
    }

    /// reads the register, updates the `ee` field, and writes the updated value
    pub fn ee_write(&mut self, val: u64) {
        Self::with_reg_val().ee_insert(val).write();
    }

    /*
     * Field: eis_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn eis_1_extract(&self) -> u64 {
        // bits 22..22
        self.val.get_bits(22..=22)
    }

    /// reads the current register value and extract field `eis_1` from it
    pub fn eis_1_read(&mut self) -> u64 {
        Self::with_reg_val().eis_1_extract()
    }

    /// inserts the given value `val` into the field `eis_1`
    pub fn eis_1_insert(&mut self, val: u64) -> &mut self {
        // bits 22..22
        self.val.set_bits(22..=22, val);
        self
    }

    /// reads the register, updates the `eis_1` field, and writes the updated value
    pub fn eis_1_write(&mut self, val: u64) {
        Self::with_reg_val().eis_1_insert(val).write();
    }

    /*
     * Field: iesb_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn iesb_1_extract(&self) -> u64 {
        // bits 21..21
        self.val.get_bits(21..=21)
    }

    /// reads the current register value and extract field `iesb_1` from it
    pub fn iesb_1_read(&mut self) -> u64 {
        Self::with_reg_val().iesb_1_extract()
    }

    /// inserts the given value `val` into the field `iesb_1`
    pub fn iesb_1_insert(&mut self, val: u64) -> &mut self {
        // bits 21..21
        self.val.set_bits(21..=21, val);
        self
    }

    /// reads the register, updates the `iesb_1` field, and writes the updated value
    pub fn iesb_1_write(&mut self, val: u64) {
        Self::with_reg_val().iesb_1_insert(val).write();
    }

    /*
     * Field: wxn
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn wxn_extract(&self) -> u64 {
        // bits 19..19
        self.val.get_bits(19..=19)
    }

    /// reads the current register value and extract field `wxn` from it
    pub fn wxn_read(&mut self) -> u64 {
        Self::with_reg_val().wxn_extract()
    }

    /// inserts the given value `val` into the field `wxn`
    pub fn wxn_insert(&mut self, val: u64) -> &mut self {
        // bits 19..19
        self.val.set_bits(19..=19, val);
        self
    }

    /// reads the register, updates the `wxn` field, and writes the updated value
    pub fn wxn_write(&mut self, val: u64) {
        Self::with_reg_val().wxn_insert(val).write();
    }

    /*
     * Field: endb_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn endb_1_extract(&self) -> u64 {
        // bits 13..13
        self.val.get_bits(13..=13)
    }

    /// reads the current register value and extract field `endb_1` from it
    pub fn endb_1_read(&mut self) -> u64 {
        Self::with_reg_val().endb_1_extract()
    }

    /// inserts the given value `val` into the field `endb_1`
    pub fn endb_1_insert(&mut self, val: u64) -> &mut self {
        // bits 13..13
        self.val.set_bits(13..=13, val);
        self
    }

    /// reads the register, updates the `endb_1` field, and writes the updated value
    pub fn endb_1_write(&mut self, val: u64) {
        Self::with_reg_val().endb_1_insert(val).write();
    }

    /*
     * Field: i
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn i_extract(&self) -> u64 {
        // bits 12..12
        self.val.get_bits(12..=12)
    }

    /// reads the current register value and extract field `i` from it
    pub fn i_read(&mut self) -> u64 {
        Self::with_reg_val().i_extract()
    }

    /// inserts the given value `val` into the field `i`
    pub fn i_insert(&mut self, val: u64) -> &mut self {
        // bits 12..12
        self.val.set_bits(12..=12, val);
        self
    }

    /// reads the register, updates the `i` field, and writes the updated value
    pub fn i_write(&mut self, val: u64) {
        Self::with_reg_val().i_insert(val).write();
    }

    /*
     * Field: eos_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn eos_1_extract(&self) -> u64 {
        // bits 11..11
        self.val.get_bits(11..=11)
    }

    /// reads the current register value and extract field `eos_1` from it
    pub fn eos_1_read(&mut self) -> u64 {
        Self::with_reg_val().eos_1_extract()
    }

    /// inserts the given value `val` into the field `eos_1`
    pub fn eos_1_insert(&mut self, val: u64) -> &mut self {
        // bits 11..11
        self.val.set_bits(11..=11, val);
        self
    }

    /// reads the register, updates the `eos_1` field, and writes the updated value
    pub fn eos_1_write(&mut self, val: u64) {
        Self::with_reg_val().eos_1_insert(val).write();
    }

    /*
     * Field: naa_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn naa_1_extract(&self) -> u64 {
        // bits 6..6
        self.val.get_bits(6..=6)
    }

    /// reads the current register value and extract field `naa_1` from it
    pub fn naa_1_read(&mut self) -> u64 {
        Self::with_reg_val().naa_1_extract()
    }

    /// inserts the given value `val` into the field `naa_1`
    pub fn naa_1_insert(&mut self, val: u64) -> &mut self {
        // bits 6..6
        self.val.set_bits(6..=6, val);
        self
    }

    /// reads the register, updates the `naa_1` field, and writes the updated value
    pub fn naa_1_write(&mut self, val: u64) {
        Self::with_reg_val().naa_1_insert(val).write();
    }

    /*
     * Field: sa
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn sa_extract(&self) -> u64 {
        // bits 3..3
        self.val.get_bits(3..=3)
    }

    /// reads the current register value and extract field `sa` from it
    pub fn sa_read(&mut self) -> u64 {
        Self::with_reg_val().sa_extract()
    }

    /// inserts the given value `val` into the field `sa`
    pub fn sa_insert(&mut self, val: u64) -> &mut self {
        // bits 3..3
        self.val.set_bits(3..=3, val);
        self
    }

    /// reads the register, updates the `sa` field, and writes the updated value
    pub fn sa_write(&mut self, val: u64) {
        Self::with_reg_val().sa_insert(val).write();
    }

    /*
     * Field: c
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn c_extract(&self) -> u64 {
        // bits 2..2
        self.val.get_bits(2..=2)
    }

    /// reads the current register value and extract field `c` from it
    pub fn c_read(&mut self) -> u64 {
        Self::with_reg_val().c_extract()
    }

    /// inserts the given value `val` into the field `c`
    pub fn c_insert(&mut self, val: u64) -> &mut self {
        // bits 2..2
        self.val.set_bits(2..=2, val);
        self
    }

    /// reads the register, updates the `c` field, and writes the updated value
    pub fn c_write(&mut self, val: u64) {
        Self::with_reg_val().c_insert(val).write();
    }

    /*
     * Field: a
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn a_extract(&self) -> u64 {
        // bits 1..1
        self.val.get_bits(1..=1)
    }

    /// reads the current register value and extract field `a` from it
    pub fn a_read(&mut self) -> u64 {
        Self::with_reg_val().a_extract()
    }

    /// inserts the given value `val` into the field `a`
    pub fn a_insert(&mut self, val: u64) -> &mut self {
        // bits 1..1
        self.val.set_bits(1..=1, val);
        self
    }

    /// reads the register, updates the `a` field, and writes the updated value
    pub fn a_write(&mut self, val: u64) {
        Self::with_reg_val().a_insert(val).write();
    }

    /*
     * Field: m
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn m_extract(&self) -> u64 {
        // bits 0..0
        self.val.get_bits(0..=0)
    }

    /// reads the current register value and extract field `m` from it
    pub fn m_read(&mut self) -> u64 {
        Self::with_reg_val().m_extract()
    }

    /// inserts the given value `val` into the field `m`
    pub fn m_insert(&mut self, val: u64) -> &mut self {
        // bits 0..0
        self.val.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `m` field, and writes the updated value
    pub fn m_write(&mut self, val: u64) {
        Self::with_reg_val().m_insert(val).write();
    }
}

impl Default for SctlrEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> SctlrEl2 {
        SctlrEl2(0)
    }
}
