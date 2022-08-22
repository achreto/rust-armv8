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
 * Generated on: 2022-08-22T15:51:28.526443
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
 * Register:    Monitor Debug System Control Register (mdscr_el1)
 * Group:       Debug registers
 * Type:        64-bit Register
 * Description: Main control register for the debug implementation.
 * File:        AArch64-mdscr_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Monitor Debug System Control Register value in memory
pub struct MdscrEl1(u64);

/// struct implementation for accessing the fields of register mdscr_el1
impl MdscrEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> MdscrEl1 {
        Self::default()
    }

    /// collects the modifications of MdscrEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> MdscrEl1 {
        MdscrEl1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> MdscrEl1 {
        let curval = Self::reg_rawrd() & 0xece8f041;
        MdscrEl1(curval)
    }

    /// reading the Monitor Debug System Control Register (mdscr_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, MDSCR_EL1
            llvm_asm!("mrs $0, mdscr_el1" : "=r"(regval));
        }
        return regval;
    }

    /// writing the Monitor Debug System Control Register (mdscr_el1) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR MDSCR_EL1, <Xt>
            llvm_asm!("msr mdscr_el1, $0" : : "r"(val));
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0xece8f041;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.val)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 3974688833;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: tfo_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tfo_1_extract(&self) -> u64 {
        // bits 31..31
        self.val.get_bits(31..=31)
    }

    /// reads the current register value and extract field `tfo_1` from it
    pub fn tfo_1_read(&mut self) -> u64 {
        Self::with_reg_val().tfo_1_extract()
    }

    /// inserts the given value `val` into the field `tfo_1`
    pub fn tfo_1_insert(&mut self, val: u64) -> &mut self {
        // bits 31..31
        self.val.set_bits(31..=31, val);
        self
    }

    /// reads the register, updates the `tfo_1` field, and writes the updated value
    pub fn tfo_1_write(&mut self, val: u64) {
        Self::with_reg_val().tfo_1_insert(val).write();
    }

    /*
     * Field: rxfull
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn rxfull_extract(&self) -> u64 {
        // bits 30..30
        self.val.get_bits(30..=30)
    }

    /// reads the current register value and extract field `rxfull` from it
    pub fn rxfull_read(&mut self) -> u64 {
        Self::with_reg_val().rxfull_extract()
    }

    /// inserts the given value `val` into the field `rxfull`
    pub fn rxfull_insert(&mut self, val: u64) -> &mut self {
        // bits 30..30
        self.val.set_bits(30..=30, val);
        self
    }

    /// reads the register, updates the `rxfull` field, and writes the updated value
    pub fn rxfull_write(&mut self, val: u64) {
        Self::with_reg_val().rxfull_insert(val).write();
    }

    /*
     * Field: txfull
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn txfull_extract(&self) -> u64 {
        // bits 29..29
        self.val.get_bits(29..=29)
    }

    /// reads the current register value and extract field `txfull` from it
    pub fn txfull_read(&mut self) -> u64 {
        Self::with_reg_val().txfull_extract()
    }

    /// inserts the given value `val` into the field `txfull`
    pub fn txfull_insert(&mut self, val: u64) -> &mut self {
        // bits 29..29
        self.val.set_bits(29..=29, val);
        self
    }

    /// reads the register, updates the `txfull` field, and writes the updated value
    pub fn txfull_write(&mut self, val: u64) {
        Self::with_reg_val().txfull_insert(val).write();
    }

    /*
     * Field: rxo
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn rxo_extract(&self) -> u64 {
        // bits 27..27
        self.val.get_bits(27..=27)
    }

    /// reads the current register value and extract field `rxo` from it
    pub fn rxo_read(&mut self) -> u64 {
        Self::with_reg_val().rxo_extract()
    }

    /// inserts the given value `val` into the field `rxo`
    pub fn rxo_insert(&mut self, val: u64) -> &mut self {
        // bits 27..27
        self.val.set_bits(27..=27, val);
        self
    }

    /// reads the register, updates the `rxo` field, and writes the updated value
    pub fn rxo_write(&mut self, val: u64) {
        Self::with_reg_val().rxo_insert(val).write();
    }

    /*
     * Field: txu
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn txu_extract(&self) -> u64 {
        // bits 26..26
        self.val.get_bits(26..=26)
    }

    /// reads the current register value and extract field `txu` from it
    pub fn txu_read(&mut self) -> u64 {
        Self::with_reg_val().txu_extract()
    }

    /// inserts the given value `val` into the field `txu`
    pub fn txu_insert(&mut self, val: u64) -> &mut self {
        // bits 26..26
        self.val.set_bits(26..=26, val);
        self
    }

    /// reads the register, updates the `txu` field, and writes the updated value
    pub fn txu_write(&mut self, val: u64) {
        Self::with_reg_val().txu_insert(val).write();
    }

    /*
     * Field: intdis
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn intdis_extract(&self) -> u64 {
        // bits 22..23
        self.val.get_bits(22..=23)
    }

    /// reads the current register value and extract field `intdis` from it
    pub fn intdis_read(&mut self) -> u64 {
        Self::with_reg_val().intdis_extract()
    }

    /// inserts the given value `val` into the field `intdis`
    pub fn intdis_insert(&mut self, val: u64) -> &mut self {
        // bits 22..23
        self.val.set_bits(22..=23, val);
        self
    }

    /// reads the register, updates the `intdis` field, and writes the updated value
    pub fn intdis_write(&mut self, val: u64) {
        Self::with_reg_val().intdis_insert(val).write();
    }

    /*
     * Field: tda
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tda_extract(&self) -> u64 {
        // bits 21..21
        self.val.get_bits(21..=21)
    }

    /// reads the current register value and extract field `tda` from it
    pub fn tda_read(&mut self) -> u64 {
        Self::with_reg_val().tda_extract()
    }

    /// inserts the given value `val` into the field `tda`
    pub fn tda_insert(&mut self, val: u64) -> &mut self {
        // bits 21..21
        self.val.set_bits(21..=21, val);
        self
    }

    /// reads the register, updates the `tda` field, and writes the updated value
    pub fn tda_write(&mut self, val: u64) {
        Self::with_reg_val().tda_insert(val).write();
    }

    /*
     * Field: sc2_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn sc2_1_extract(&self) -> u64 {
        // bits 19..19
        self.val.get_bits(19..=19)
    }

    /// reads the current register value and extract field `sc2_1` from it
    pub fn sc2_1_read(&mut self) -> u64 {
        Self::with_reg_val().sc2_1_extract()
    }

    /// inserts the given value `val` into the field `sc2_1`
    pub fn sc2_1_insert(&mut self, val: u64) -> &mut self {
        // bits 19..19
        self.val.set_bits(19..=19, val);
        self
    }

    /// reads the register, updates the `sc2_1` field, and writes the updated value
    pub fn sc2_1_write(&mut self, val: u64) {
        Self::with_reg_val().sc2_1_insert(val).write();
    }

    /*
     * Field: mde
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn mde_extract(&self) -> u64 {
        // bits 15..15
        self.val.get_bits(15..=15)
    }

    /// reads the current register value and extract field `mde` from it
    pub fn mde_read(&mut self) -> u64 {
        Self::with_reg_val().mde_extract()
    }

    /// inserts the given value `val` into the field `mde`
    pub fn mde_insert(&mut self, val: u64) -> &mut self {
        // bits 15..15
        self.val.set_bits(15..=15, val);
        self
    }

    /// reads the register, updates the `mde` field, and writes the updated value
    pub fn mde_write(&mut self, val: u64) {
        Self::with_reg_val().mde_insert(val).write();
    }

    /*
     * Field: hde
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn hde_extract(&self) -> u64 {
        // bits 14..14
        self.val.get_bits(14..=14)
    }

    /// reads the current register value and extract field `hde` from it
    pub fn hde_read(&mut self) -> u64 {
        Self::with_reg_val().hde_extract()
    }

    /// inserts the given value `val` into the field `hde`
    pub fn hde_insert(&mut self, val: u64) -> &mut self {
        // bits 14..14
        self.val.set_bits(14..=14, val);
        self
    }

    /// reads the register, updates the `hde` field, and writes the updated value
    pub fn hde_write(&mut self, val: u64) {
        Self::with_reg_val().hde_insert(val).write();
    }

    /*
     * Field: kde
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn kde_extract(&self) -> u64 {
        // bits 13..13
        self.val.get_bits(13..=13)
    }

    /// reads the current register value and extract field `kde` from it
    pub fn kde_read(&mut self) -> u64 {
        Self::with_reg_val().kde_extract()
    }

    /// inserts the given value `val` into the field `kde`
    pub fn kde_insert(&mut self, val: u64) -> &mut self {
        // bits 13..13
        self.val.set_bits(13..=13, val);
        self
    }

    /// reads the register, updates the `kde` field, and writes the updated value
    pub fn kde_write(&mut self, val: u64) {
        Self::with_reg_val().kde_insert(val).write();
    }

    /*
     * Field: tdcc
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tdcc_extract(&self) -> u64 {
        // bits 12..12
        self.val.get_bits(12..=12)
    }

    /// reads the current register value and extract field `tdcc` from it
    pub fn tdcc_read(&mut self) -> u64 {
        Self::with_reg_val().tdcc_extract()
    }

    /// inserts the given value `val` into the field `tdcc`
    pub fn tdcc_insert(&mut self, val: u64) -> &mut self {
        // bits 12..12
        self.val.set_bits(12..=12, val);
        self
    }

    /// reads the register, updates the `tdcc` field, and writes the updated value
    pub fn tdcc_write(&mut self, val: u64) {
        Self::with_reg_val().tdcc_insert(val).write();
    }

    /*
     * Field: err
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn err_extract(&self) -> u64 {
        // bits 6..6
        self.val.get_bits(6..=6)
    }

    /// reads the current register value and extract field `err` from it
    pub fn err_read(&mut self) -> u64 {
        Self::with_reg_val().err_extract()
    }

    /// inserts the given value `val` into the field `err`
    pub fn err_insert(&mut self, val: u64) -> &mut self {
        // bits 6..6
        self.val.set_bits(6..=6, val);
        self
    }

    /// reads the register, updates the `err` field, and writes the updated value
    pub fn err_write(&mut self, val: u64) {
        Self::with_reg_val().err_insert(val).write();
    }

    /*
     * Field: ss
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ss_extract(&self) -> u64 {
        // bits 0..0
        self.val.get_bits(0..=0)
    }

    /// reads the current register value and extract field `ss` from it
    pub fn ss_read(&mut self) -> u64 {
        Self::with_reg_val().ss_extract()
    }

    /// inserts the given value `val` into the field `ss`
    pub fn ss_insert(&mut self, val: u64) -> &mut self {
        // bits 0..0
        self.val.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `ss` field, and writes the updated value
    pub fn ss_write(&mut self, val: u64) {
        Self::with_reg_val().ss_insert(val).write();
    }
}

impl Default for MdscrEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> MdscrEl1 {
        MdscrEl1(0)
    }
}
