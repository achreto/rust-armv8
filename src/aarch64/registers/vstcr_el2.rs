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
 * Generated on: 2022-08-22T16:25:59.102105
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
 * Register:    Virtualization Secure Translation Control Register (vstcr_el2)
 * Group:       Generic System Control
 * Type:        64-bit Register
 * Description: The control register for stage 2 of the Secure EL1&0 translation regime.
 * File:        AArch64-vstcr_el2.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Virtualization Secure Translation Control Register value in memory
pub struct VstcrEl2(u64);

/// struct implementation for accessing the fields of register vstcr_el2
impl VstcrEl2 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> VstcrEl2 {
        Self::default()
    }

    /// collects the modifications of VstcrEl2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> VstcrEl2 {
        VstcrEl2(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  VstcrEl2 {
        let curval = Self::reg_rawrd() & 0x26000c0ff;
        VstcrEl2(curval)
    }


    
    /// reading the Virtualization Secure Translation Control Register (vstcr_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, VSTCR_EL2
            asm!("mrs {}, S3_4_C2_C6_2", out(reg) regval);
        }
        return regval;
    }


    /// writing the Virtualization Secure Translation Control Register (vstcr_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR VSTCR_EL2, <Xt>
            asm!("msr S3_4_C2_C6_2, {}", in(reg) val);
        }
    }



    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0x26000c0ff;
        self
    }

    
    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }


    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 10200596735;
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
    pub fn sl2_1_read(&mut self) -> u64 {
        Self::with_reg_val().sl2_1_extract()
    }

    /// inserts the given value `val` into the field `sl2_1`
    pub fn sl2_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 33..33
        self.0.set_bits(33..=33, val);
        self
    }

    /// reads the register, updates the `sl2_1` field, and writes the updated value
    pub fn sl2_1_write(&mut self, val: u64) {
        Self::with_reg_val().sl2_1_insert(val).write();
    }

    /*
     * Field: sa
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn sa_extract(&self) -> u64 {
        // bits 30..30
        self.0.get_bits(30..=30)
    }

    /// reads the current register value and extract field `sa` from it
    pub fn sa_read(&mut self) -> u64 {
        Self::with_reg_val().sa_extract()
    }

    /// inserts the given value `val` into the field `sa`
    pub fn sa_insert(&mut self, val: u64) -> &mut Self {
        // bits 30..30
        self.0.set_bits(30..=30, val);
        self
    }

    /// reads the register, updates the `sa` field, and writes the updated value
    pub fn sa_write(&mut self, val: u64) {
        Self::with_reg_val().sa_insert(val).write();
    }

    /*
     * Field: sw
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn sw_extract(&self) -> u64 {
        // bits 29..29
        self.0.get_bits(29..=29)
    }

    /// reads the current register value and extract field `sw` from it
    pub fn sw_read(&mut self) -> u64 {
        Self::with_reg_val().sw_extract()
    }

    /// inserts the given value `val` into the field `sw`
    pub fn sw_insert(&mut self, val: u64) -> &mut Self {
        // bits 29..29
        self.0.set_bits(29..=29, val);
        self
    }

    /// reads the register, updates the `sw` field, and writes the updated value
    pub fn sw_write(&mut self, val: u64) {
        Self::with_reg_val().sw_insert(val).write();
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
     * Field: sl0_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn sl0_1_extract(&self) -> u64 {
        // bits 6..7
        self.0.get_bits(6..=7)
    }

    /// reads the current register value and extract field `sl0_1` from it
    pub fn sl0_1_read(&mut self) -> u64 {
        Self::with_reg_val().sl0_1_extract()
    }

    /// inserts the given value `val` into the field `sl0_1`
    pub fn sl0_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 6..7
        self.0.set_bits(6..=7, val);
        self
    }

    /// reads the register, updates the `sl0_1` field, and writes the updated value
    pub fn sl0_1_write(&mut self, val: u64) {
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
    pub fn sl0_2_read(&mut self) -> u64 {
        Self::with_reg_val().sl0_2_extract()
    }

    /// inserts the given value `val` into the field `sl0_2`
    pub fn sl0_2_insert(&mut self, val: u64) -> &mut Self {
        // bits 6..7
        self.0.set_bits(6..=7, val);
        self
    }

    /// reads the register, updates the `sl0_2` field, and writes the updated value
    pub fn sl0_2_write(&mut self, val: u64) {
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

impl Default for VstcrEl2 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> VstcrEl2 {
        VstcrEl2(0)
    }
}
