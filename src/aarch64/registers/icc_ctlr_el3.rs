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
 * Generated on: 2022-08-22T16:35:53.061479
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
 * Register:    Interrupt Controller Control Register (EL3) (icc_ctlr_el3)
 * Group:       GIC control registers
 * Type:        64-bit Register
 * Description: Controls aspects of the behavior of the GIC CPU interface and provides information about the features implemented.
 * File:        AArch64-icc_ctlr_el3.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Interrupt Controller Control Register (EL3) value in memory
pub struct IccCtlrEl3(u64);

/// struct implementation for accessing the fields of register icc_ctlr_el3
impl IccCtlrEl3 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> IccCtlrEl3 {
        Self::default()
    }

    /// collects the modifications of IccCtlrEl3 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> IccCtlrEl3 {
        IccCtlrEl3(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> IccCtlrEl3 {
        let curval = Self::reg_rawrd() & 0xeff7f;
        IccCtlrEl3(curval)
    }

    /// reading the Interrupt Controller Control Register (EL3) (icc_ctlr_el3) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ICC_CTLR_EL3
            asm!("mrs {}, icc_ctlr_el3", out(reg) regval);
        }
        return regval;
    }

    /// writing the Interrupt Controller Control Register (EL3) (icc_ctlr_el3) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR ICC_CTLR_EL3, <Xt>
            asm!("msr icc_ctlr_el3, {}", in(reg) val);
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xeff7f;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 982911;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: extrange
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn extrange_extract(&self) -> u64 {
        // bits 19..19
        self.0.get_bits(19..=19)
    }

    /// reads the current register value and extract field `extrange` from it
    pub fn extrange_read() -> u64 {
        Self::with_reg_val().extrange_extract()
    }

    /// inserts the given value `val` into the field `extrange`
    pub fn extrange_insert(&mut self, val: u64) -> &mut Self {
        // bits 19..19
        self.0.set_bits(19..=19, val);
        self
    }

    /// reads the register, updates the `extrange` field, and writes the updated value
    pub fn extrange_write(val: u64) {
        Self::with_reg_val().extrange_insert(val).write();
    }

    /*
     * Field: rss
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn rss_extract(&self) -> u64 {
        // bits 18..18
        self.0.get_bits(18..=18)
    }

    /// reads the current register value and extract field `rss` from it
    pub fn rss_read() -> u64 {
        Self::with_reg_val().rss_extract()
    }

    /// inserts the given value `val` into the field `rss`
    pub fn rss_insert(&mut self, val: u64) -> &mut Self {
        // bits 18..18
        self.0.set_bits(18..=18, val);
        self
    }

    /// reads the register, updates the `rss` field, and writes the updated value
    pub fn rss_write(val: u64) {
        Self::with_reg_val().rss_insert(val).write();
    }

    /*
     * Field: nds
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn nds_extract(&self) -> u64 {
        // bits 17..17
        self.0.get_bits(17..=17)
    }

    /// reads the current register value and extract field `nds` from it
    pub fn nds_read() -> u64 {
        Self::with_reg_val().nds_extract()
    }

    /// inserts the given value `val` into the field `nds`
    pub fn nds_insert(&mut self, val: u64) -> &mut Self {
        // bits 17..17
        self.0.set_bits(17..=17, val);
        self
    }

    /// reads the register, updates the `nds` field, and writes the updated value
    pub fn nds_write(val: u64) {
        Self::with_reg_val().nds_insert(val).write();
    }

    /*
     * Field: a3v
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn a3v_extract(&self) -> u64 {
        // bits 15..15
        self.0.get_bits(15..=15)
    }

    /// reads the current register value and extract field `a3v` from it
    pub fn a3v_read() -> u64 {
        Self::with_reg_val().a3v_extract()
    }

    /// inserts the given value `val` into the field `a3v`
    pub fn a3v_insert(&mut self, val: u64) -> &mut Self {
        // bits 15..15
        self.0.set_bits(15..=15, val);
        self
    }

    /// reads the register, updates the `a3v` field, and writes the updated value
    pub fn a3v_write(val: u64) {
        Self::with_reg_val().a3v_insert(val).write();
    }

    /*
     * Field: seis
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn seis_extract(&self) -> u64 {
        // bits 14..14
        self.0.get_bits(14..=14)
    }

    /// reads the current register value and extract field `seis` from it
    pub fn seis_read() -> u64 {
        Self::with_reg_val().seis_extract()
    }

    /// inserts the given value `val` into the field `seis`
    pub fn seis_insert(&mut self, val: u64) -> &mut Self {
        // bits 14..14
        self.0.set_bits(14..=14, val);
        self
    }

    /// reads the register, updates the `seis` field, and writes the updated value
    pub fn seis_write(val: u64) {
        Self::with_reg_val().seis_insert(val).write();
    }

    /*
     * Field: idbits
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn idbits_extract(&self) -> u64 {
        // bits 11..13
        self.0.get_bits(11..=13)
    }

    /// reads the current register value and extract field `idbits` from it
    pub fn idbits_read() -> u64 {
        Self::with_reg_val().idbits_extract()
    }

    /// inserts the given value `val` into the field `idbits`
    pub fn idbits_insert(&mut self, val: u64) -> &mut Self {
        // bits 11..13
        self.0.set_bits(11..=13, val);
        self
    }

    /// reads the register, updates the `idbits` field, and writes the updated value
    pub fn idbits_write(val: u64) {
        Self::with_reg_val().idbits_insert(val).write();
    }

    /*
     * Field: pribits
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn pribits_extract(&self) -> u64 {
        // bits 8..10
        self.0.get_bits(8..=10)
    }

    /// reads the current register value and extract field `pribits` from it
    pub fn pribits_read() -> u64 {
        Self::with_reg_val().pribits_extract()
    }

    /// inserts the given value `val` into the field `pribits`
    pub fn pribits_insert(&mut self, val: u64) -> &mut Self {
        // bits 8..10
        self.0.set_bits(8..=10, val);
        self
    }

    /// reads the register, updates the `pribits` field, and writes the updated value
    pub fn pribits_write(val: u64) {
        Self::with_reg_val().pribits_insert(val).write();
    }

    /*
     * Field: pmhe
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn pmhe_extract(&self) -> u64 {
        // bits 6..6
        self.0.get_bits(6..=6)
    }

    /// reads the current register value and extract field `pmhe` from it
    pub fn pmhe_read() -> u64 {
        Self::with_reg_val().pmhe_extract()
    }

    /// inserts the given value `val` into the field `pmhe`
    pub fn pmhe_insert(&mut self, val: u64) -> &mut Self {
        // bits 6..6
        self.0.set_bits(6..=6, val);
        self
    }

    /// reads the register, updates the `pmhe` field, and writes the updated value
    pub fn pmhe_write(val: u64) {
        Self::with_reg_val().pmhe_insert(val).write();
    }

    /*
     * Field: rm
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn rm_extract(&self) -> u64 {
        // bits 5..5
        self.0.get_bits(5..=5)
    }

    /// reads the current register value and extract field `rm` from it
    pub fn rm_read() -> u64 {
        Self::with_reg_val().rm_extract()
    }

    /// inserts the given value `val` into the field `rm`
    pub fn rm_insert(&mut self, val: u64) -> &mut Self {
        // bits 5..5
        self.0.set_bits(5..=5, val);
        self
    }

    /// reads the register, updates the `rm` field, and writes the updated value
    pub fn rm_write(val: u64) {
        Self::with_reg_val().rm_insert(val).write();
    }

    /*
     * Field: eoimode_el1ns
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn eoimode_el1ns_extract(&self) -> u64 {
        // bits 4..4
        self.0.get_bits(4..=4)
    }

    /// reads the current register value and extract field `eoimode_el1ns` from it
    pub fn eoimode_el1ns_read() -> u64 {
        Self::with_reg_val().eoimode_el1ns_extract()
    }

    /// inserts the given value `val` into the field `eoimode_el1ns`
    pub fn eoimode_el1ns_insert(&mut self, val: u64) -> &mut Self {
        // bits 4..4
        self.0.set_bits(4..=4, val);
        self
    }

    /// reads the register, updates the `eoimode_el1ns` field, and writes the updated value
    pub fn eoimode_el1ns_write(val: u64) {
        Self::with_reg_val().eoimode_el1ns_insert(val).write();
    }

    /*
     * Field: eoimode_el1s
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn eoimode_el1s_extract(&self) -> u64 {
        // bits 3..3
        self.0.get_bits(3..=3)
    }

    /// reads the current register value and extract field `eoimode_el1s` from it
    pub fn eoimode_el1s_read() -> u64 {
        Self::with_reg_val().eoimode_el1s_extract()
    }

    /// inserts the given value `val` into the field `eoimode_el1s`
    pub fn eoimode_el1s_insert(&mut self, val: u64) -> &mut Self {
        // bits 3..3
        self.0.set_bits(3..=3, val);
        self
    }

    /// reads the register, updates the `eoimode_el1s` field, and writes the updated value
    pub fn eoimode_el1s_write(val: u64) {
        Self::with_reg_val().eoimode_el1s_insert(val).write();
    }

    /*
     * Field: eoimode_el3
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn eoimode_el3_extract(&self) -> u64 {
        // bits 2..2
        self.0.get_bits(2..=2)
    }

    /// reads the current register value and extract field `eoimode_el3` from it
    pub fn eoimode_el3_read() -> u64 {
        Self::with_reg_val().eoimode_el3_extract()
    }

    /// inserts the given value `val` into the field `eoimode_el3`
    pub fn eoimode_el3_insert(&mut self, val: u64) -> &mut Self {
        // bits 2..2
        self.0.set_bits(2..=2, val);
        self
    }

    /// reads the register, updates the `eoimode_el3` field, and writes the updated value
    pub fn eoimode_el3_write(val: u64) {
        Self::with_reg_val().eoimode_el3_insert(val).write();
    }

    /*
     * Field: cbpr_el1ns
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn cbpr_el1ns_extract(&self) -> u64 {
        // bits 1..1
        self.0.get_bits(1..=1)
    }

    /// reads the current register value and extract field `cbpr_el1ns` from it
    pub fn cbpr_el1ns_read() -> u64 {
        Self::with_reg_val().cbpr_el1ns_extract()
    }

    /// inserts the given value `val` into the field `cbpr_el1ns`
    pub fn cbpr_el1ns_insert(&mut self, val: u64) -> &mut Self {
        // bits 1..1
        self.0.set_bits(1..=1, val);
        self
    }

    /// reads the register, updates the `cbpr_el1ns` field, and writes the updated value
    pub fn cbpr_el1ns_write(val: u64) {
        Self::with_reg_val().cbpr_el1ns_insert(val).write();
    }

    /*
     * Field: cbpr_el1s
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn cbpr_el1s_extract(&self) -> u64 {
        // bits 0..0
        self.0.get_bits(0..=0)
    }

    /// reads the current register value and extract field `cbpr_el1s` from it
    pub fn cbpr_el1s_read() -> u64 {
        Self::with_reg_val().cbpr_el1s_extract()
    }

    /// inserts the given value `val` into the field `cbpr_el1s`
    pub fn cbpr_el1s_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..0
        self.0.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `cbpr_el1s` field, and writes the updated value
    pub fn cbpr_el1s_write(val: u64) {
        Self::with_reg_val().cbpr_el1s_insert(val).write();
    }
}

impl Default for IccCtlrEl3 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> IccCtlrEl3 {
        IccCtlrEl3(0)
    }
}
