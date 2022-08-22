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

use core::arch::asm;
use bit_field::BitField;


/**************************************************************************************************
 *
 * !!!! WARNING: THIS FILE IS AUTO GENERATED. ANY CHANGES MAY BE OVERWRITTEN !!!!
 *
 * Generated on: 2022-08-22T16:25:59.099023
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
 * Register:    Translation Control Register (EL1) (tcr_el1)
 * Group:       Virtual memory control registers
 * Type:        64-bit Register
 * Description: The control register for stage 1 of the EL1&0 translation regime.
 * File:        AArch64-tcr_el1.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Translation Control Register (EL1) value in memory
pub struct TcrEl1(u64);

/// struct implementation for accessing the fields of register tcr_el1
impl TcrEl1 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> TcrEl1 {
        Self::default()
    }

    /// collects the modifications of TcrEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> TcrEl1 {
        TcrEl1(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  TcrEl1 {
        let curval = Self::reg_rawrd() & 0xffffff7ffffffbf;
        TcrEl1(curval)
    }


    
    /// reading the Translation Control Register (EL1) (tcr_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, TCR_EL1
            asm!("mrs {}, tcr_el1", out(reg) regval);
        }
        return regval;
    }


    /// writing the Translation Control Register (EL1) (tcr_el1) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR TCR_EL1, <Xt>
            asm!("msr tcr_el1, {}", in(reg) val);
        }
    }



    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xffffff7ffffffbf;
        self
    }

    
    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }


    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 1152921470247108543;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: ds_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn ds_1_extract(&self) -> u64 {
        // bits 59..59
        self.0.get_bits(59..=59)
    }

    /// reads the current register value and extract field `ds_1` from it
    pub fn ds_1_read(&mut self) -> u64 {
        Self::with_reg_val().ds_1_extract()
    }

    /// inserts the given value `val` into the field `ds_1`
    pub fn ds_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 59..59
        self.0.set_bits(59..=59, val);
        self
    }

    /// reads the register, updates the `ds_1` field, and writes the updated value
    pub fn ds_1_write(&mut self, val: u64) {
        Self::with_reg_val().ds_1_insert(val).write();
    }

    /*
     * Field: tcma1_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn tcma1_1_extract(&self) -> u64 {
        // bits 58..58
        self.0.get_bits(58..=58)
    }

    /// reads the current register value and extract field `tcma1_1` from it
    pub fn tcma1_1_read(&mut self) -> u64 {
        Self::with_reg_val().tcma1_1_extract()
    }

    /// inserts the given value `val` into the field `tcma1_1`
    pub fn tcma1_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 58..58
        self.0.set_bits(58..=58, val);
        self
    }

    /// reads the register, updates the `tcma1_1` field, and writes the updated value
    pub fn tcma1_1_write(&mut self, val: u64) {
        Self::with_reg_val().tcma1_1_insert(val).write();
    }

    /*
     * Field: tcma0_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn tcma0_1_extract(&self) -> u64 {
        // bits 57..57
        self.0.get_bits(57..=57)
    }

    /// reads the current register value and extract field `tcma0_1` from it
    pub fn tcma0_1_read(&mut self) -> u64 {
        Self::with_reg_val().tcma0_1_extract()
    }

    /// inserts the given value `val` into the field `tcma0_1`
    pub fn tcma0_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 57..57
        self.0.set_bits(57..=57, val);
        self
    }

    /// reads the register, updates the `tcma0_1` field, and writes the updated value
    pub fn tcma0_1_write(&mut self, val: u64) {
        Self::with_reg_val().tcma0_1_insert(val).write();
    }

    /*
     * Field: e0pd1_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e0pd1_1_extract(&self) -> u64 {
        // bits 56..56
        self.0.get_bits(56..=56)
    }

    /// reads the current register value and extract field `e0pd1_1` from it
    pub fn e0pd1_1_read(&mut self) -> u64 {
        Self::with_reg_val().e0pd1_1_extract()
    }

    /// inserts the given value `val` into the field `e0pd1_1`
    pub fn e0pd1_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 56..56
        self.0.set_bits(56..=56, val);
        self
    }

    /// reads the register, updates the `e0pd1_1` field, and writes the updated value
    pub fn e0pd1_1_write(&mut self, val: u64) {
        Self::with_reg_val().e0pd1_1_insert(val).write();
    }

    /*
     * Field: e0pd0_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e0pd0_1_extract(&self) -> u64 {
        // bits 55..55
        self.0.get_bits(55..=55)
    }

    /// reads the current register value and extract field `e0pd0_1` from it
    pub fn e0pd0_1_read(&mut self) -> u64 {
        Self::with_reg_val().e0pd0_1_extract()
    }

    /// inserts the given value `val` into the field `e0pd0_1`
    pub fn e0pd0_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 55..55
        self.0.set_bits(55..=55, val);
        self
    }

    /// reads the register, updates the `e0pd0_1` field, and writes the updated value
    pub fn e0pd0_1_write(&mut self, val: u64) {
        Self::with_reg_val().e0pd0_1_insert(val).write();
    }

    /*
     * Field: nfd1_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn nfd1_1_extract(&self) -> u64 {
        // bits 54..54
        self.0.get_bits(54..=54)
    }

    /// reads the current register value and extract field `nfd1_1` from it
    pub fn nfd1_1_read(&mut self) -> u64 {
        Self::with_reg_val().nfd1_1_extract()
    }

    /// inserts the given value `val` into the field `nfd1_1`
    pub fn nfd1_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 54..54
        self.0.set_bits(54..=54, val);
        self
    }

    /// reads the register, updates the `nfd1_1` field, and writes the updated value
    pub fn nfd1_1_write(&mut self, val: u64) {
        Self::with_reg_val().nfd1_1_insert(val).write();
    }

    /*
     * Field: nfd0_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn nfd0_1_extract(&self) -> u64 {
        // bits 53..53
        self.0.get_bits(53..=53)
    }

    /// reads the current register value and extract field `nfd0_1` from it
    pub fn nfd0_1_read(&mut self) -> u64 {
        Self::with_reg_val().nfd0_1_extract()
    }

    /// inserts the given value `val` into the field `nfd0_1`
    pub fn nfd0_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 53..53
        self.0.set_bits(53..=53, val);
        self
    }

    /// reads the register, updates the `nfd0_1` field, and writes the updated value
    pub fn nfd0_1_write(&mut self, val: u64) {
        Self::with_reg_val().nfd0_1_insert(val).write();
    }

    /*
     * Field: tbid1_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn tbid1_1_extract(&self) -> u64 {
        // bits 52..52
        self.0.get_bits(52..=52)
    }

    /// reads the current register value and extract field `tbid1_1` from it
    pub fn tbid1_1_read(&mut self) -> u64 {
        Self::with_reg_val().tbid1_1_extract()
    }

    /// inserts the given value `val` into the field `tbid1_1`
    pub fn tbid1_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 52..52
        self.0.set_bits(52..=52, val);
        self
    }

    /// reads the register, updates the `tbid1_1` field, and writes the updated value
    pub fn tbid1_1_write(&mut self, val: u64) {
        Self::with_reg_val().tbid1_1_insert(val).write();
    }

    /*
     * Field: tbid0_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn tbid0_1_extract(&self) -> u64 {
        // bits 51..51
        self.0.get_bits(51..=51)
    }

    /// reads the current register value and extract field `tbid0_1` from it
    pub fn tbid0_1_read(&mut self) -> u64 {
        Self::with_reg_val().tbid0_1_extract()
    }

    /// inserts the given value `val` into the field `tbid0_1`
    pub fn tbid0_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 51..51
        self.0.set_bits(51..=51, val);
        self
    }

    /// reads the register, updates the `tbid0_1` field, and writes the updated value
    pub fn tbid0_1_write(&mut self, val: u64) {
        Self::with_reg_val().tbid0_1_insert(val).write();
    }

    /*
     * Field: hwu162_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn hwu162_1_extract(&self) -> u64 {
        // bits 50..50
        self.0.get_bits(50..=50)
    }

    /// reads the current register value and extract field `hwu162_1` from it
    pub fn hwu162_1_read(&mut self) -> u64 {
        Self::with_reg_val().hwu162_1_extract()
    }

    /// inserts the given value `val` into the field `hwu162_1`
    pub fn hwu162_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 50..50
        self.0.set_bits(50..=50, val);
        self
    }

    /// reads the register, updates the `hwu162_1` field, and writes the updated value
    pub fn hwu162_1_write(&mut self, val: u64) {
        Self::with_reg_val().hwu162_1_insert(val).write();
    }

    /*
     * Field: hwu161_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn hwu161_1_extract(&self) -> u64 {
        // bits 49..49
        self.0.get_bits(49..=49)
    }

    /// reads the current register value and extract field `hwu161_1` from it
    pub fn hwu161_1_read(&mut self) -> u64 {
        Self::with_reg_val().hwu161_1_extract()
    }

    /// inserts the given value `val` into the field `hwu161_1`
    pub fn hwu161_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 49..49
        self.0.set_bits(49..=49, val);
        self
    }

    /// reads the register, updates the `hwu161_1` field, and writes the updated value
    pub fn hwu161_1_write(&mut self, val: u64) {
        Self::with_reg_val().hwu161_1_insert(val).write();
    }

    /*
     * Field: hwu160_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn hwu160_1_extract(&self) -> u64 {
        // bits 48..48
        self.0.get_bits(48..=48)
    }

    /// reads the current register value and extract field `hwu160_1` from it
    pub fn hwu160_1_read(&mut self) -> u64 {
        Self::with_reg_val().hwu160_1_extract()
    }

    /// inserts the given value `val` into the field `hwu160_1`
    pub fn hwu160_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 48..48
        self.0.set_bits(48..=48, val);
        self
    }

    /// reads the register, updates the `hwu160_1` field, and writes the updated value
    pub fn hwu160_1_write(&mut self, val: u64) {
        Self::with_reg_val().hwu160_1_insert(val).write();
    }

    /*
     * Field: hwu159_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn hwu159_1_extract(&self) -> u64 {
        // bits 47..47
        self.0.get_bits(47..=47)
    }

    /// reads the current register value and extract field `hwu159_1` from it
    pub fn hwu159_1_read(&mut self) -> u64 {
        Self::with_reg_val().hwu159_1_extract()
    }

    /// inserts the given value `val` into the field `hwu159_1`
    pub fn hwu159_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 47..47
        self.0.set_bits(47..=47, val);
        self
    }

    /// reads the register, updates the `hwu159_1` field, and writes the updated value
    pub fn hwu159_1_write(&mut self, val: u64) {
        Self::with_reg_val().hwu159_1_insert(val).write();
    }

    /*
     * Field: hwu062_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn hwu062_1_extract(&self) -> u64 {
        // bits 46..46
        self.0.get_bits(46..=46)
    }

    /// reads the current register value and extract field `hwu062_1` from it
    pub fn hwu062_1_read(&mut self) -> u64 {
        Self::with_reg_val().hwu062_1_extract()
    }

    /// inserts the given value `val` into the field `hwu062_1`
    pub fn hwu062_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 46..46
        self.0.set_bits(46..=46, val);
        self
    }

    /// reads the register, updates the `hwu062_1` field, and writes the updated value
    pub fn hwu062_1_write(&mut self, val: u64) {
        Self::with_reg_val().hwu062_1_insert(val).write();
    }

    /*
     * Field: hwu061_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn hwu061_1_extract(&self) -> u64 {
        // bits 45..45
        self.0.get_bits(45..=45)
    }

    /// reads the current register value and extract field `hwu061_1` from it
    pub fn hwu061_1_read(&mut self) -> u64 {
        Self::with_reg_val().hwu061_1_extract()
    }

    /// inserts the given value `val` into the field `hwu061_1`
    pub fn hwu061_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 45..45
        self.0.set_bits(45..=45, val);
        self
    }

    /// reads the register, updates the `hwu061_1` field, and writes the updated value
    pub fn hwu061_1_write(&mut self, val: u64) {
        Self::with_reg_val().hwu061_1_insert(val).write();
    }

    /*
     * Field: hwu060_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn hwu060_1_extract(&self) -> u64 {
        // bits 44..44
        self.0.get_bits(44..=44)
    }

    /// reads the current register value and extract field `hwu060_1` from it
    pub fn hwu060_1_read(&mut self) -> u64 {
        Self::with_reg_val().hwu060_1_extract()
    }

    /// inserts the given value `val` into the field `hwu060_1`
    pub fn hwu060_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 44..44
        self.0.set_bits(44..=44, val);
        self
    }

    /// reads the register, updates the `hwu060_1` field, and writes the updated value
    pub fn hwu060_1_write(&mut self, val: u64) {
        Self::with_reg_val().hwu060_1_insert(val).write();
    }

    /*
     * Field: hwu059_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn hwu059_1_extract(&self) -> u64 {
        // bits 43..43
        self.0.get_bits(43..=43)
    }

    /// reads the current register value and extract field `hwu059_1` from it
    pub fn hwu059_1_read(&mut self) -> u64 {
        Self::with_reg_val().hwu059_1_extract()
    }

    /// inserts the given value `val` into the field `hwu059_1`
    pub fn hwu059_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 43..43
        self.0.set_bits(43..=43, val);
        self
    }

    /// reads the register, updates the `hwu059_1` field, and writes the updated value
    pub fn hwu059_1_write(&mut self, val: u64) {
        Self::with_reg_val().hwu059_1_insert(val).write();
    }

    /*
     * Field: hpd1_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn hpd1_1_extract(&self) -> u64 {
        // bits 42..42
        self.0.get_bits(42..=42)
    }

    /// reads the current register value and extract field `hpd1_1` from it
    pub fn hpd1_1_read(&mut self) -> u64 {
        Self::with_reg_val().hpd1_1_extract()
    }

    /// inserts the given value `val` into the field `hpd1_1`
    pub fn hpd1_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 42..42
        self.0.set_bits(42..=42, val);
        self
    }

    /// reads the register, updates the `hpd1_1` field, and writes the updated value
    pub fn hpd1_1_write(&mut self, val: u64) {
        Self::with_reg_val().hpd1_1_insert(val).write();
    }

    /*
     * Field: hpd0_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn hpd0_1_extract(&self) -> u64 {
        // bits 41..41
        self.0.get_bits(41..=41)
    }

    /// reads the current register value and extract field `hpd0_1` from it
    pub fn hpd0_1_read(&mut self) -> u64 {
        Self::with_reg_val().hpd0_1_extract()
    }

    /// inserts the given value `val` into the field `hpd0_1`
    pub fn hpd0_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 41..41
        self.0.set_bits(41..=41, val);
        self
    }

    /// reads the register, updates the `hpd0_1` field, and writes the updated value
    pub fn hpd0_1_write(&mut self, val: u64) {
        Self::with_reg_val().hpd0_1_insert(val).write();
    }

    /*
     * Field: hd_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn hd_1_extract(&self) -> u64 {
        // bits 40..40
        self.0.get_bits(40..=40)
    }

    /// reads the current register value and extract field `hd_1` from it
    pub fn hd_1_read(&mut self) -> u64 {
        Self::with_reg_val().hd_1_extract()
    }

    /// inserts the given value `val` into the field `hd_1`
    pub fn hd_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 40..40
        self.0.set_bits(40..=40, val);
        self
    }

    /// reads the register, updates the `hd_1` field, and writes the updated value
    pub fn hd_1_write(&mut self, val: u64) {
        Self::with_reg_val().hd_1_insert(val).write();
    }

    /*
     * Field: ha_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn ha_1_extract(&self) -> u64 {
        // bits 39..39
        self.0.get_bits(39..=39)
    }

    /// reads the current register value and extract field `ha_1` from it
    pub fn ha_1_read(&mut self) -> u64 {
        Self::with_reg_val().ha_1_extract()
    }

    /// inserts the given value `val` into the field `ha_1`
    pub fn ha_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 39..39
        self.0.set_bits(39..=39, val);
        self
    }

    /// reads the register, updates the `ha_1` field, and writes the updated value
    pub fn ha_1_write(&mut self, val: u64) {
        Self::with_reg_val().ha_1_insert(val).write();
    }

    /*
     * Field: tbi1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn tbi1_extract(&self) -> u64 {
        // bits 38..38
        self.0.get_bits(38..=38)
    }

    /// reads the current register value and extract field `tbi1` from it
    pub fn tbi1_read(&mut self) -> u64 {
        Self::with_reg_val().tbi1_extract()
    }

    /// inserts the given value `val` into the field `tbi1`
    pub fn tbi1_insert(&mut self, val: u64) -> &mut Self {
        // bits 38..38
        self.0.set_bits(38..=38, val);
        self
    }

    /// reads the register, updates the `tbi1` field, and writes the updated value
    pub fn tbi1_write(&mut self, val: u64) {
        Self::with_reg_val().tbi1_insert(val).write();
    }

    /*
     * Field: tbi0
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn tbi0_extract(&self) -> u64 {
        // bits 37..37
        self.0.get_bits(37..=37)
    }

    /// reads the current register value and extract field `tbi0` from it
    pub fn tbi0_read(&mut self) -> u64 {
        Self::with_reg_val().tbi0_extract()
    }

    /// inserts the given value `val` into the field `tbi0`
    pub fn tbi0_insert(&mut self, val: u64) -> &mut Self {
        // bits 37..37
        self.0.set_bits(37..=37, val);
        self
    }

    /// reads the register, updates the `tbi0` field, and writes the updated value
    pub fn tbi0_write(&mut self, val: u64) {
        Self::with_reg_val().tbi0_insert(val).write();
    }

    /*
     * Field: as
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn as_extract(&self) -> u64 {
        // bits 36..36
        self.0.get_bits(36..=36)
    }

    /// reads the current register value and extract field `as` from it
    pub fn as_read(&mut self) -> u64 {
        Self::with_reg_val().as_extract()
    }

    /// inserts the given value `val` into the field `as`
    pub fn as_insert(&mut self, val: u64) -> &mut Self {
        // bits 36..36
        self.0.set_bits(36..=36, val);
        self
    }

    /// reads the register, updates the `as` field, and writes the updated value
    pub fn as_write(&mut self, val: u64) {
        Self::with_reg_val().as_insert(val).write();
    }

    /*
     * Field: ips
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn ips_extract(&self) -> u64 {
        // bits 32..34
        self.0.get_bits(32..=34)
    }

    /// reads the current register value and extract field `ips` from it
    pub fn ips_read(&mut self) -> u64 {
        Self::with_reg_val().ips_extract()
    }

    /// inserts the given value `val` into the field `ips`
    pub fn ips_insert(&mut self, val: u64) -> &mut Self {
        // bits 32..34
        self.0.set_bits(32..=34, val);
        self
    }

    /// reads the register, updates the `ips` field, and writes the updated value
    pub fn ips_write(&mut self, val: u64) {
        Self::with_reg_val().ips_insert(val).write();
    }

    /*
     * Field: tg1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn tg1_extract(&self) -> u64 {
        // bits 30..31
        self.0.get_bits(30..=31)
    }

    /// reads the current register value and extract field `tg1` from it
    pub fn tg1_read(&mut self) -> u64 {
        Self::with_reg_val().tg1_extract()
    }

    /// inserts the given value `val` into the field `tg1`
    pub fn tg1_insert(&mut self, val: u64) -> &mut Self {
        // bits 30..31
        self.0.set_bits(30..=31, val);
        self
    }

    /// reads the register, updates the `tg1` field, and writes the updated value
    pub fn tg1_write(&mut self, val: u64) {
        Self::with_reg_val().tg1_insert(val).write();
    }

    /*
     * Field: sh1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn sh1_extract(&self) -> u64 {
        // bits 28..29
        self.0.get_bits(28..=29)
    }

    /// reads the current register value and extract field `sh1` from it
    pub fn sh1_read(&mut self) -> u64 {
        Self::with_reg_val().sh1_extract()
    }

    /// inserts the given value `val` into the field `sh1`
    pub fn sh1_insert(&mut self, val: u64) -> &mut Self {
        // bits 28..29
        self.0.set_bits(28..=29, val);
        self
    }

    /// reads the register, updates the `sh1` field, and writes the updated value
    pub fn sh1_write(&mut self, val: u64) {
        Self::with_reg_val().sh1_insert(val).write();
    }

    /*
     * Field: orgn1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn orgn1_extract(&self) -> u64 {
        // bits 26..27
        self.0.get_bits(26..=27)
    }

    /// reads the current register value and extract field `orgn1` from it
    pub fn orgn1_read(&mut self) -> u64 {
        Self::with_reg_val().orgn1_extract()
    }

    /// inserts the given value `val` into the field `orgn1`
    pub fn orgn1_insert(&mut self, val: u64) -> &mut Self {
        // bits 26..27
        self.0.set_bits(26..=27, val);
        self
    }

    /// reads the register, updates the `orgn1` field, and writes the updated value
    pub fn orgn1_write(&mut self, val: u64) {
        Self::with_reg_val().orgn1_insert(val).write();
    }

    /*
     * Field: irgn1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn irgn1_extract(&self) -> u64 {
        // bits 24..25
        self.0.get_bits(24..=25)
    }

    /// reads the current register value and extract field `irgn1` from it
    pub fn irgn1_read(&mut self) -> u64 {
        Self::with_reg_val().irgn1_extract()
    }

    /// inserts the given value `val` into the field `irgn1`
    pub fn irgn1_insert(&mut self, val: u64) -> &mut Self {
        // bits 24..25
        self.0.set_bits(24..=25, val);
        self
    }

    /// reads the register, updates the `irgn1` field, and writes the updated value
    pub fn irgn1_write(&mut self, val: u64) {
        Self::with_reg_val().irgn1_insert(val).write();
    }

    /*
     * Field: epd1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn epd1_extract(&self) -> u64 {
        // bits 23..23
        self.0.get_bits(23..=23)
    }

    /// reads the current register value and extract field `epd1` from it
    pub fn epd1_read(&mut self) -> u64 {
        Self::with_reg_val().epd1_extract()
    }

    /// inserts the given value `val` into the field `epd1`
    pub fn epd1_insert(&mut self, val: u64) -> &mut Self {
        // bits 23..23
        self.0.set_bits(23..=23, val);
        self
    }

    /// reads the register, updates the `epd1` field, and writes the updated value
    pub fn epd1_write(&mut self, val: u64) {
        Self::with_reg_val().epd1_insert(val).write();
    }

    /*
     * Field: a1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn a1_extract(&self) -> u64 {
        // bits 22..22
        self.0.get_bits(22..=22)
    }

    /// reads the current register value and extract field `a1` from it
    pub fn a1_read(&mut self) -> u64 {
        Self::with_reg_val().a1_extract()
    }

    /// inserts the given value `val` into the field `a1`
    pub fn a1_insert(&mut self, val: u64) -> &mut Self {
        // bits 22..22
        self.0.set_bits(22..=22, val);
        self
    }

    /// reads the register, updates the `a1` field, and writes the updated value
    pub fn a1_write(&mut self, val: u64) {
        Self::with_reg_val().a1_insert(val).write();
    }

    /*
     * Field: t1sz
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn t1sz_extract(&self) -> u64 {
        // bits 16..21
        self.0.get_bits(16..=21)
    }

    /// reads the current register value and extract field `t1sz` from it
    pub fn t1sz_read(&mut self) -> u64 {
        Self::with_reg_val().t1sz_extract()
    }

    /// inserts the given value `val` into the field `t1sz`
    pub fn t1sz_insert(&mut self, val: u64) -> &mut Self {
        // bits 16..21
        self.0.set_bits(16..=21, val);
        self
    }

    /// reads the register, updates the `t1sz` field, and writes the updated value
    pub fn t1sz_write(&mut self, val: u64) {
        Self::with_reg_val().t1sz_insert(val).write();
    }

    /*
     * Field: tg0
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn tg0_extract(&self) -> u64 {
        // bits 14..15
        self.0.get_bits(14..=15)
    }

    /// reads the current register value and extract field `tg0` from it
    pub fn tg0_read(&mut self) -> u64 {
        Self::with_reg_val().tg0_extract()
    }

    /// inserts the given value `val` into the field `tg0`
    pub fn tg0_insert(&mut self, val: u64) -> &mut Self {
        // bits 14..15
        self.0.set_bits(14..=15, val);
        self
    }

    /// reads the register, updates the `tg0` field, and writes the updated value
    pub fn tg0_write(&mut self, val: u64) {
        Self::with_reg_val().tg0_insert(val).write();
    }

    /*
     * Field: sh0
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn sh0_extract(&self) -> u64 {
        // bits 12..13
        self.0.get_bits(12..=13)
    }

    /// reads the current register value and extract field `sh0` from it
    pub fn sh0_read(&mut self) -> u64 {
        Self::with_reg_val().sh0_extract()
    }

    /// inserts the given value `val` into the field `sh0`
    pub fn sh0_insert(&mut self, val: u64) -> &mut Self {
        // bits 12..13
        self.0.set_bits(12..=13, val);
        self
    }

    /// reads the register, updates the `sh0` field, and writes the updated value
    pub fn sh0_write(&mut self, val: u64) {
        Self::with_reg_val().sh0_insert(val).write();
    }

    /*
     * Field: orgn0
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn orgn0_extract(&self) -> u64 {
        // bits 10..11
        self.0.get_bits(10..=11)
    }

    /// reads the current register value and extract field `orgn0` from it
    pub fn orgn0_read(&mut self) -> u64 {
        Self::with_reg_val().orgn0_extract()
    }

    /// inserts the given value `val` into the field `orgn0`
    pub fn orgn0_insert(&mut self, val: u64) -> &mut Self {
        // bits 10..11
        self.0.set_bits(10..=11, val);
        self
    }

    /// reads the register, updates the `orgn0` field, and writes the updated value
    pub fn orgn0_write(&mut self, val: u64) {
        Self::with_reg_val().orgn0_insert(val).write();
    }

    /*
     * Field: irgn0
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn irgn0_extract(&self) -> u64 {
        // bits 8..9
        self.0.get_bits(8..=9)
    }

    /// reads the current register value and extract field `irgn0` from it
    pub fn irgn0_read(&mut self) -> u64 {
        Self::with_reg_val().irgn0_extract()
    }

    /// inserts the given value `val` into the field `irgn0`
    pub fn irgn0_insert(&mut self, val: u64) -> &mut Self {
        // bits 8..9
        self.0.set_bits(8..=9, val);
        self
    }

    /// reads the register, updates the `irgn0` field, and writes the updated value
    pub fn irgn0_write(&mut self, val: u64) {
        Self::with_reg_val().irgn0_insert(val).write();
    }

    /*
     * Field: epd0
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn epd0_extract(&self) -> u64 {
        // bits 7..7
        self.0.get_bits(7..=7)
    }

    /// reads the current register value and extract field `epd0` from it
    pub fn epd0_read(&mut self) -> u64 {
        Self::with_reg_val().epd0_extract()
    }

    /// inserts the given value `val` into the field `epd0`
    pub fn epd0_insert(&mut self, val: u64) -> &mut Self {
        // bits 7..7
        self.0.set_bits(7..=7, val);
        self
    }

    /// reads the register, updates the `epd0` field, and writes the updated value
    pub fn epd0_write(&mut self, val: u64) {
        Self::with_reg_val().epd0_insert(val).write();
    }

    /*
     * Field: t0sz
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn t0sz_extract(&self) -> u64 {
        // bits 0..5
        self.0.get_bits(0..=5)
    }

    /// reads the current register value and extract field `t0sz` from it
    pub fn t0sz_read(&mut self) -> u64 {
        Self::with_reg_val().t0sz_extract()
    }

    /// inserts the given value `val` into the field `t0sz`
    pub fn t0sz_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..5
        self.0.set_bits(0..=5, val);
        self
    }

    /// reads the register, updates the `t0sz` field, and writes the updated value
    pub fn t0sz_write(&mut self, val: u64) {
        Self::with_reg_val().t0sz_insert(val).write();
    }

}

impl Default for TcrEl1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> TcrEl1 {
        TcrEl1(0)
    }
}
