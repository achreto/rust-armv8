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
 * Generated on: 2022-08-22T16:35:53.083861
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
 * Register:    Virtualization Translation Control Register (vtcr_el2)
 * Group:       Virtual memory control registers
 * Type:        64-bit Register
 * Description: The control register for stage 2 of the EL1&0 translation regime.
 * File:        AArch64-vtcr_el2.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Virtualization Translation Control Register value in memory
pub struct VtcrEl2(u64);

/// struct implementation for accessing the fields of register vtcr_el2
impl VtcrEl2 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> VtcrEl2 {
        Self::default()
    }

    /// collects the modifications of VtcrEl2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> VtcrEl2 {
        VtcrEl2(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  VtcrEl2 {
        let curval = Self::reg_rawrd() & 0x37e6fffff;
        VtcrEl2(curval)
    }


    
    /// reading the Virtualization Translation Control Register (vtcr_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, VTCR_EL2
            asm!("mrs {}, vtcr_el2", out(reg) regval);
        }
        return regval;
    }


    /// writing the Virtualization Translation Control Register (vtcr_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR VTCR_EL2, <Xt>
            asm!("msr vtcr_el2, {}", in(reg) val);
        }
    }



    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0x37e6fffff;
        self
    }

    
    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }


    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 15006171135;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: sl2_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn sl2_1_extract(&self) -> u64 {
        // bits 33..33
        self.0.get_bits(33..=33)
    }

    /// reads the current register value and extract field `sl2_1` from it
    pub fn sl2_1_read() -> u64 {
        Self::with_reg_val().sl2_1_extract()
    }

    /// inserts the given value `val` into the field `sl2_1`
    pub fn sl2_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 33..33
        self.0.set_bits(33..=33, val);
        self
    }

    /// reads the register, updates the `sl2_1` field, and writes the updated value
    pub fn sl2_1_write(val: u64) {
        Self::with_reg_val().sl2_1_insert(val).write();
    }

    /*
     * Field: ds_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn ds_1_extract(&self) -> u64 {
        // bits 32..32
        self.0.get_bits(32..=32)
    }

    /// reads the current register value and extract field `ds_1` from it
    pub fn ds_1_read() -> u64 {
        Self::with_reg_val().ds_1_extract()
    }

    /// inserts the given value `val` into the field `ds_1`
    pub fn ds_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 32..32
        self.0.set_bits(32..=32, val);
        self
    }

    /// reads the register, updates the `ds_1` field, and writes the updated value
    pub fn ds_1_write(val: u64) {
        Self::with_reg_val().ds_1_insert(val).write();
    }

    /*
     * Field: nsa_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn nsa_1_extract(&self) -> u64 {
        // bits 30..30
        self.0.get_bits(30..=30)
    }

    /// reads the current register value and extract field `nsa_1` from it
    pub fn nsa_1_read() -> u64 {
        Self::with_reg_val().nsa_1_extract()
    }

    /// inserts the given value `val` into the field `nsa_1`
    pub fn nsa_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 30..30
        self.0.set_bits(30..=30, val);
        self
    }

    /// reads the register, updates the `nsa_1` field, and writes the updated value
    pub fn nsa_1_write(val: u64) {
        Self::with_reg_val().nsa_1_insert(val).write();
    }

    /*
     * Field: nsw_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn nsw_1_extract(&self) -> u64 {
        // bits 29..29
        self.0.get_bits(29..=29)
    }

    /// reads the current register value and extract field `nsw_1` from it
    pub fn nsw_1_read() -> u64 {
        Self::with_reg_val().nsw_1_extract()
    }

    /// inserts the given value `val` into the field `nsw_1`
    pub fn nsw_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 29..29
        self.0.set_bits(29..=29, val);
        self
    }

    /// reads the register, updates the `nsw_1` field, and writes the updated value
    pub fn nsw_1_write(val: u64) {
        Self::with_reg_val().nsw_1_insert(val).write();
    }

    /*
     * Field: hwu62_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn hwu62_1_extract(&self) -> u64 {
        // bits 28..28
        self.0.get_bits(28..=28)
    }

    /// reads the current register value and extract field `hwu62_1` from it
    pub fn hwu62_1_read() -> u64 {
        Self::with_reg_val().hwu62_1_extract()
    }

    /// inserts the given value `val` into the field `hwu62_1`
    pub fn hwu62_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 28..28
        self.0.set_bits(28..=28, val);
        self
    }

    /// reads the register, updates the `hwu62_1` field, and writes the updated value
    pub fn hwu62_1_write(val: u64) {
        Self::with_reg_val().hwu62_1_insert(val).write();
    }

    /*
     * Field: hwu61_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn hwu61_1_extract(&self) -> u64 {
        // bits 27..27
        self.0.get_bits(27..=27)
    }

    /// reads the current register value and extract field `hwu61_1` from it
    pub fn hwu61_1_read() -> u64 {
        Self::with_reg_val().hwu61_1_extract()
    }

    /// inserts the given value `val` into the field `hwu61_1`
    pub fn hwu61_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 27..27
        self.0.set_bits(27..=27, val);
        self
    }

    /// reads the register, updates the `hwu61_1` field, and writes the updated value
    pub fn hwu61_1_write(val: u64) {
        Self::with_reg_val().hwu61_1_insert(val).write();
    }

    /*
     * Field: hwu60_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn hwu60_1_extract(&self) -> u64 {
        // bits 26..26
        self.0.get_bits(26..=26)
    }

    /// reads the current register value and extract field `hwu60_1` from it
    pub fn hwu60_1_read() -> u64 {
        Self::with_reg_val().hwu60_1_extract()
    }

    /// inserts the given value `val` into the field `hwu60_1`
    pub fn hwu60_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 26..26
        self.0.set_bits(26..=26, val);
        self
    }

    /// reads the register, updates the `hwu60_1` field, and writes the updated value
    pub fn hwu60_1_write(val: u64) {
        Self::with_reg_val().hwu60_1_insert(val).write();
    }

    /*
     * Field: hwu59_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn hwu59_1_extract(&self) -> u64 {
        // bits 25..25
        self.0.get_bits(25..=25)
    }

    /// reads the current register value and extract field `hwu59_1` from it
    pub fn hwu59_1_read() -> u64 {
        Self::with_reg_val().hwu59_1_extract()
    }

    /// inserts the given value `val` into the field `hwu59_1`
    pub fn hwu59_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 25..25
        self.0.set_bits(25..=25, val);
        self
    }

    /// reads the register, updates the `hwu59_1` field, and writes the updated value
    pub fn hwu59_1_write(val: u64) {
        Self::with_reg_val().hwu59_1_insert(val).write();
    }

    /*
     * Field: hd_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn hd_1_extract(&self) -> u64 {
        // bits 22..22
        self.0.get_bits(22..=22)
    }

    /// reads the current register value and extract field `hd_1` from it
    pub fn hd_1_read() -> u64 {
        Self::with_reg_val().hd_1_extract()
    }

    /// inserts the given value `val` into the field `hd_1`
    pub fn hd_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 22..22
        self.0.set_bits(22..=22, val);
        self
    }

    /// reads the register, updates the `hd_1` field, and writes the updated value
    pub fn hd_1_write(val: u64) {
        Self::with_reg_val().hd_1_insert(val).write();
    }

    /*
     * Field: ha_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn ha_1_extract(&self) -> u64 {
        // bits 21..21
        self.0.get_bits(21..=21)
    }

    /// reads the current register value and extract field `ha_1` from it
    pub fn ha_1_read() -> u64 {
        Self::with_reg_val().ha_1_extract()
    }

    /// inserts the given value `val` into the field `ha_1`
    pub fn ha_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 21..21
        self.0.set_bits(21..=21, val);
        self
    }

    /// reads the register, updates the `ha_1` field, and writes the updated value
    pub fn ha_1_write(val: u64) {
        Self::with_reg_val().ha_1_insert(val).write();
    }

    /*
     * Field: vs_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn vs_1_extract(&self) -> u64 {
        // bits 19..19
        self.0.get_bits(19..=19)
    }

    /// reads the current register value and extract field `vs_1` from it
    pub fn vs_1_read() -> u64 {
        Self::with_reg_val().vs_1_extract()
    }

    /// inserts the given value `val` into the field `vs_1`
    pub fn vs_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 19..19
        self.0.set_bits(19..=19, val);
        self
    }

    /// reads the register, updates the `vs_1` field, and writes the updated value
    pub fn vs_1_write(val: u64) {
        Self::with_reg_val().vs_1_insert(val).write();
    }

    /*
     * Field: ps
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn ps_extract(&self) -> u64 {
        // bits 16..18
        self.0.get_bits(16..=18)
    }

    /// reads the current register value and extract field `ps` from it
    pub fn ps_read() -> u64 {
        Self::with_reg_val().ps_extract()
    }

    /// inserts the given value `val` into the field `ps`
    pub fn ps_insert(&mut self, val: u64) -> &mut Self {
        // bits 16..18
        self.0.set_bits(16..=18, val);
        self
    }

    /// reads the register, updates the `ps` field, and writes the updated value
    pub fn ps_write(val: u64) {
        Self::with_reg_val().ps_insert(val).write();
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
    pub fn tg0_read() -> u64 {
        Self::with_reg_val().tg0_extract()
    }

    /// inserts the given value `val` into the field `tg0`
    pub fn tg0_insert(&mut self, val: u64) -> &mut Self {
        // bits 14..15
        self.0.set_bits(14..=15, val);
        self
    }

    /// reads the register, updates the `tg0` field, and writes the updated value
    pub fn tg0_write(val: u64) {
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
    pub fn sh0_read() -> u64 {
        Self::with_reg_val().sh0_extract()
    }

    /// inserts the given value `val` into the field `sh0`
    pub fn sh0_insert(&mut self, val: u64) -> &mut Self {
        // bits 12..13
        self.0.set_bits(12..=13, val);
        self
    }

    /// reads the register, updates the `sh0` field, and writes the updated value
    pub fn sh0_write(val: u64) {
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
    pub fn orgn0_read() -> u64 {
        Self::with_reg_val().orgn0_extract()
    }

    /// inserts the given value `val` into the field `orgn0`
    pub fn orgn0_insert(&mut self, val: u64) -> &mut Self {
        // bits 10..11
        self.0.set_bits(10..=11, val);
        self
    }

    /// reads the register, updates the `orgn0` field, and writes the updated value
    pub fn orgn0_write(val: u64) {
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
    pub fn irgn0_read() -> u64 {
        Self::with_reg_val().irgn0_extract()
    }

    /// inserts the given value `val` into the field `irgn0`
    pub fn irgn0_insert(&mut self, val: u64) -> &mut Self {
        // bits 8..9
        self.0.set_bits(8..=9, val);
        self
    }

    /// reads the register, updates the `irgn0` field, and writes the updated value
    pub fn irgn0_write(val: u64) {
        Self::with_reg_val().irgn0_insert(val).write();
    }

    /*
     * Field: sl0_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn sl0_1_extract(&self) -> u64 {
        // bits 6..7
        self.0.get_bits(6..=7)
    }

    /// reads the current register value and extract field `sl0_1` from it
    pub fn sl0_1_read() -> u64 {
        Self::with_reg_val().sl0_1_extract()
    }

    /// inserts the given value `val` into the field `sl0_1`
    pub fn sl0_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 6..7
        self.0.set_bits(6..=7, val);
        self
    }

    /// reads the register, updates the `sl0_1` field, and writes the updated value
    pub fn sl0_1_write(val: u64) {
        Self::with_reg_val().sl0_1_insert(val).write();
    }

    /*
     * Field: sl0_2
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn sl0_2_extract(&self) -> u64 {
        // bits 6..7
        self.0.get_bits(6..=7)
    }

    /// reads the current register value and extract field `sl0_2` from it
    pub fn sl0_2_read() -> u64 {
        Self::with_reg_val().sl0_2_extract()
    }

    /// inserts the given value `val` into the field `sl0_2`
    pub fn sl0_2_insert(&mut self, val: u64) -> &mut Self {
        // bits 6..7
        self.0.set_bits(6..=7, val);
        self
    }

    /// reads the register, updates the `sl0_2` field, and writes the updated value
    pub fn sl0_2_write(val: u64) {
        Self::with_reg_val().sl0_2_insert(val).write();
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
    pub fn t0sz_read() -> u64 {
        Self::with_reg_val().t0sz_extract()
    }

    /// inserts the given value `val` into the field `t0sz`
    pub fn t0sz_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..5
        self.0.set_bits(0..=5, val);
        self
    }

    /// reads the register, updates the `t0sz` field, and writes the updated value
    pub fn t0sz_write(val: u64) {
        Self::with_reg_val().t0sz_insert(val).write();
    }

}

impl Default for VtcrEl2 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> VtcrEl2 {
        VtcrEl2(0)
    }
}
