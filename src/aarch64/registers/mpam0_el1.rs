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
 * Generated on: 2022-08-22T16:35:53.069775
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
 * Register:    MPAM0 Register (EL1) (mpam0_el1)
 * Group:       Memory Partitioning and Monitoring registers
 * Type:        64-bit Register
 * Description: Holds information to generate MPAM labels for memory requests when executing at EL0. When EL2 is present and enabled, the MPAM virtualization option is present, 
 * File:        AArch64-mpam0_el1.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the MPAM0 Register (EL1) value in memory
pub struct Mpam0El1(u64);

/// struct implementation for accessing the fields of register mpam0_el1
impl Mpam0El1 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> Mpam0El1 {
        Self::default()
    }

    /// collects the modifications of Mpam0El1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> Mpam0El1 {
        Mpam0El1(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  Mpam0El1 {
        let curval = Self::reg_rawrd() & 0xffffffffffff;
        Mpam0El1(curval)
    }


    
    /// reading the MPAM0 Register (EL1) (mpam0_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, MPAM0_EL1
            asm!("mrs {}, S3_0_C10_C5_1", out(reg) regval);
        }
        return regval;
    }


    /// writing the MPAM0 Register (EL1) (mpam0_el1) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR MPAM0_EL1, <Xt>
            asm!("msr S3_0_C10_C5_1, {}", in(reg) val);
        }
    }



    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xffffffffffff;
        self
    }

    
    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }


    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 281474976710655;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: pmg_d
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn pmg_d_extract(&self) -> u64 {
        // bits 40..47
        self.0.get_bits(40..=47)
    }

    /// reads the current register value and extract field `pmg_d` from it
    pub fn pmg_d_read() -> u64 {
        Self::with_reg_val().pmg_d_extract()
    }

    /// inserts the given value `val` into the field `pmg_d`
    pub fn pmg_d_insert(&mut self, val: u64) -> &mut Self {
        // bits 40..47
        self.0.set_bits(40..=47, val);
        self
    }

    /// reads the register, updates the `pmg_d` field, and writes the updated value
    pub fn pmg_d_write(val: u64) {
        Self::with_reg_val().pmg_d_insert(val).write();
    }

    /*
     * Field: pmg_i
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn pmg_i_extract(&self) -> u64 {
        // bits 32..39
        self.0.get_bits(32..=39)
    }

    /// reads the current register value and extract field `pmg_i` from it
    pub fn pmg_i_read() -> u64 {
        Self::with_reg_val().pmg_i_extract()
    }

    /// inserts the given value `val` into the field `pmg_i`
    pub fn pmg_i_insert(&mut self, val: u64) -> &mut Self {
        // bits 32..39
        self.0.set_bits(32..=39, val);
        self
    }

    /// reads the register, updates the `pmg_i` field, and writes the updated value
    pub fn pmg_i_write(val: u64) {
        Self::with_reg_val().pmg_i_insert(val).write();
    }

    /*
     * Field: partid_d
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn partid_d_extract(&self) -> u64 {
        // bits 16..31
        self.0.get_bits(16..=31)
    }

    /// reads the current register value and extract field `partid_d` from it
    pub fn partid_d_read() -> u64 {
        Self::with_reg_val().partid_d_extract()
    }

    /// inserts the given value `val` into the field `partid_d`
    pub fn partid_d_insert(&mut self, val: u64) -> &mut Self {
        // bits 16..31
        self.0.set_bits(16..=31, val);
        self
    }

    /// reads the register, updates the `partid_d` field, and writes the updated value
    pub fn partid_d_write(val: u64) {
        Self::with_reg_val().partid_d_insert(val).write();
    }

    /*
     * Field: partid_i
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn partid_i_extract(&self) -> u64 {
        // bits 0..15
        self.0.get_bits(0..=15)
    }

    /// reads the current register value and extract field `partid_i` from it
    pub fn partid_i_read() -> u64 {
        Self::with_reg_val().partid_i_extract()
    }

    /// inserts the given value `val` into the field `partid_i`
    pub fn partid_i_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..15
        self.0.set_bits(0..=15, val);
        self
    }

    /// reads the register, updates the `partid_i` field, and writes the updated value
    pub fn partid_i_write(val: u64) {
        Self::with_reg_val().partid_i_insert(val).write();
    }

}

impl Default for Mpam0El1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> Mpam0El1 {
        Mpam0El1(0)
    }
}
