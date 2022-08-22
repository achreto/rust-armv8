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
 * Generated on: 2022-08-22T15:51:28.520661
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
 * Register:    Interrupt Controller Virtual Control Register (icv_ctlr_el1)
 * Group:       Virtualization registers
 * Type:        64-bit Register
 * Description: Controls aspects of the behavior of the GIC virtual CPU interface and provides information about the features implemented.
 * File:        AArch64-icv_ctlr_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Interrupt Controller Virtual Control Register value in memory
pub struct IcvCtlrEl1(u64);

/// struct implementation for accessing the fields of register icv_ctlr_el1
impl IcvCtlrEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> IcvCtlrEl1 {
        Self::default()
    }

    /// collects the modifications of IcvCtlrEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> IcvCtlrEl1 {
        IcvCtlrEl1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> IcvCtlrEl1 {
        let curval = Self::reg_rawrd() & 0xcff03;
        IcvCtlrEl1(curval)
    }

    /// reading the Interrupt Controller Virtual Control Register (icv_ctlr_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ICC_CTLR_EL1
            llvm_asm!("mrs $0, S3_0_C12_C12_4" : "=r"(regval));
        }
        return regval;
    }

    /// writing the Interrupt Controller Virtual Control Register (icv_ctlr_el1) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR ICC_CTLR_EL1, <Xt>
            llvm_asm!("msr S3_0_C12_C12_4, $0" : : "r"(val));
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0xcff03;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.val)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 851715;
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
        self.val.get_bits(19..=19)
    }

    /// reads the current register value and extract field `extrange` from it
    pub fn extrange_read(&mut self) -> u64 {
        Self::with_reg_val().extrange_extract()
    }

    /// inserts the given value `val` into the field `extrange`
    pub fn extrange_insert(&mut self, val: u64) -> &mut self {
        // bits 19..19
        self.val.set_bits(19..=19, val);
        self
    }

    /// reads the register, updates the `extrange` field, and writes the updated value
    pub fn extrange_write(&mut self, val: u64) {
        Self::with_reg_val().extrange_insert(val).write();
    }

    /*
     * Field: rss
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn rss_extract(&self) -> u64 {
        // bits 18..18
        self.val.get_bits(18..=18)
    }

    /// reads the current register value and extract field `rss` from it
    pub fn rss_read(&mut self) -> u64 {
        Self::with_reg_val().rss_extract()
    }

    /// inserts the given value `val` into the field `rss`
    pub fn rss_insert(&mut self, val: u64) -> &mut self {
        // bits 18..18
        self.val.set_bits(18..=18, val);
        self
    }

    /// reads the register, updates the `rss` field, and writes the updated value
    pub fn rss_write(&mut self, val: u64) {
        Self::with_reg_val().rss_insert(val).write();
    }

    /*
     * Field: a3v
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn a3v_extract(&self) -> u64 {
        // bits 15..15
        self.val.get_bits(15..=15)
    }

    /// reads the current register value and extract field `a3v` from it
    pub fn a3v_read(&mut self) -> u64 {
        Self::with_reg_val().a3v_extract()
    }

    /// inserts the given value `val` into the field `a3v`
    pub fn a3v_insert(&mut self, val: u64) -> &mut self {
        // bits 15..15
        self.val.set_bits(15..=15, val);
        self
    }

    /// reads the register, updates the `a3v` field, and writes the updated value
    pub fn a3v_write(&mut self, val: u64) {
        Self::with_reg_val().a3v_insert(val).write();
    }

    /*
     * Field: seis
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn seis_extract(&self) -> u64 {
        // bits 14..14
        self.val.get_bits(14..=14)
    }

    /// reads the current register value and extract field `seis` from it
    pub fn seis_read(&mut self) -> u64 {
        Self::with_reg_val().seis_extract()
    }

    /// inserts the given value `val` into the field `seis`
    pub fn seis_insert(&mut self, val: u64) -> &mut self {
        // bits 14..14
        self.val.set_bits(14..=14, val);
        self
    }

    /// reads the register, updates the `seis` field, and writes the updated value
    pub fn seis_write(&mut self, val: u64) {
        Self::with_reg_val().seis_insert(val).write();
    }

    /*
     * Field: idbits
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn idbits_extract(&self) -> u64 {
        // bits 11..13
        self.val.get_bits(11..=13)
    }

    /// reads the current register value and extract field `idbits` from it
    pub fn idbits_read(&mut self) -> u64 {
        Self::with_reg_val().idbits_extract()
    }

    /// inserts the given value `val` into the field `idbits`
    pub fn idbits_insert(&mut self, val: u64) -> &mut self {
        // bits 11..13
        self.val.set_bits(11..=13, val);
        self
    }

    /// reads the register, updates the `idbits` field, and writes the updated value
    pub fn idbits_write(&mut self, val: u64) {
        Self::with_reg_val().idbits_insert(val).write();
    }

    /*
     * Field: pribits
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn pribits_extract(&self) -> u64 {
        // bits 8..10
        self.val.get_bits(8..=10)
    }

    /// reads the current register value and extract field `pribits` from it
    pub fn pribits_read(&mut self) -> u64 {
        Self::with_reg_val().pribits_extract()
    }

    /// inserts the given value `val` into the field `pribits`
    pub fn pribits_insert(&mut self, val: u64) -> &mut self {
        // bits 8..10
        self.val.set_bits(8..=10, val);
        self
    }

    /// reads the register, updates the `pribits` field, and writes the updated value
    pub fn pribits_write(&mut self, val: u64) {
        Self::with_reg_val().pribits_insert(val).write();
    }

    /*
     * Field: eoimode
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn eoimode_extract(&self) -> u64 {
        // bits 1..1
        self.val.get_bits(1..=1)
    }

    /// reads the current register value and extract field `eoimode` from it
    pub fn eoimode_read(&mut self) -> u64 {
        Self::with_reg_val().eoimode_extract()
    }

    /// inserts the given value `val` into the field `eoimode`
    pub fn eoimode_insert(&mut self, val: u64) -> &mut self {
        // bits 1..1
        self.val.set_bits(1..=1, val);
        self
    }

    /// reads the register, updates the `eoimode` field, and writes the updated value
    pub fn eoimode_write(&mut self, val: u64) {
        Self::with_reg_val().eoimode_insert(val).write();
    }

    /*
     * Field: cbpr
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn cbpr_extract(&self) -> u64 {
        // bits 0..0
        self.val.get_bits(0..=0)
    }

    /// reads the current register value and extract field `cbpr` from it
    pub fn cbpr_read(&mut self) -> u64 {
        Self::with_reg_val().cbpr_extract()
    }

    /// inserts the given value `val` into the field `cbpr`
    pub fn cbpr_insert(&mut self, val: u64) -> &mut self {
        // bits 0..0
        self.val.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `cbpr` field, and writes the updated value
    pub fn cbpr_write(&mut self, val: u64) {
        Self::with_reg_val().cbpr_insert(val).write();
    }
}

impl Default for IcvCtlrEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> IcvCtlrEl1 {
        IcvCtlrEl1(0)
    }
}
