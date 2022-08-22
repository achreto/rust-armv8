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
 * Generated on: 2022-08-22T16:35:53.081441
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
 * Register:    Tag Fault Status Register (EL2) (tfsr_el2)
 * Group:       Generic System Control
 * Type:        64-bit Register
 * Description: Holds accumulated Tag Check Faults occurring in EL2 that are not taken precisely.
 * File:        AArch64-tfsr_el2.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Tag Fault Status Register (EL2) value in memory
pub struct TfsrEl2(u64);

/// struct implementation for accessing the fields of register tfsr_el2
impl TfsrEl2 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> TfsrEl2 {
        Self::default()
    }

    /// collects the modifications of TfsrEl2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> TfsrEl2 {
        TfsrEl2(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  TfsrEl2 {
        let curval = Self::reg_rawrd() & 0x3;
        TfsrEl2(curval)
    }


    
    /// reading the Tag Fault Status Register (EL2) (tfsr_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, TFSR_EL2
            asm!("mrs {}, S3_4_C5_C6_0", out(reg) regval);
        }
        return regval;
    }


    /// writing the Tag Fault Status Register (EL2) (tfsr_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR TFSR_EL2, <Xt>
            asm!("msr S3_4_C5_C6_0, {}", in(reg) val);
        }
    }



    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0x3;
        self
    }

    
    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }


    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 3;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: tf1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn tf1_extract(&self) -> u64 {
        // bits 1..1
        self.0.get_bits(1..=1)
    }

    /// reads the current register value and extract field `tf1` from it
    pub fn tf1_read() -> u64 {
        Self::with_reg_val().tf1_extract()
    }

    /// inserts the given value `val` into the field `tf1`
    pub fn tf1_insert(&mut self, val: u64) -> &mut Self {
        // bits 1..1
        self.0.set_bits(1..=1, val);
        self
    }

    /// reads the register, updates the `tf1` field, and writes the updated value
    pub fn tf1_write(val: u64) {
        Self::with_reg_val().tf1_insert(val).write();
    }

    /*
     * Field: tf0
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn tf0_extract(&self) -> u64 {
        // bits 0..0
        self.0.get_bits(0..=0)
    }

    /// reads the current register value and extract field `tf0` from it
    pub fn tf0_read() -> u64 {
        Self::with_reg_val().tf0_extract()
    }

    /// inserts the given value `val` into the field `tf0`
    pub fn tf0_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..0
        self.0.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `tf0` field, and writes the updated value
    pub fn tf0_write(val: u64) {
        Self::with_reg_val().tf0_insert(val).write();
    }

}

impl Default for TfsrEl2 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> TfsrEl2 {
        TfsrEl2(0)
    }
}
