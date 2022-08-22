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
 * Generated on: 2022-08-22T16:35:53.070636
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
 * Register:    MPAM Virtual PARTID Mapping Register 2 (mpamvpm2_el2)
 * Group:       Memory Partitioning and Monitoring registers
 * Type:        64-bit Register
 * Description: MPAMVPM2_EL2 provides mappings from virtual PARTIDs 8 - 11 to physical PARTIDs.
 * File:        AArch64-mpamvpm2_el2.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the MPAM Virtual PARTID Mapping Register 2 value in memory
pub struct Mpamvpm2El2(u64);

/// struct implementation for accessing the fields of register mpamvpm2_el2
impl Mpamvpm2El2 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> Mpamvpm2El2 {
        Self::default()
    }

    /// collects the modifications of Mpamvpm2El2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> Mpamvpm2El2 {
        Mpamvpm2El2(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  Mpamvpm2El2 {
        let curval = Self::reg_rawrd() & 0xffffffffffffffff;
        Mpamvpm2El2(curval)
    }


    
    /// reading the MPAM Virtual PARTID Mapping Register 2 (mpamvpm2_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, MPAMVPM2_EL2
            asm!("mrs {}, S3_4_C10_C6_2", out(reg) regval);
        }
        return regval;
    }


    /// writing the MPAM Virtual PARTID Mapping Register 2 (mpamvpm2_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR MPAMVPM2_EL2, <Xt>
            asm!("msr S3_4_C10_C6_2, {}", in(reg) val);
        }
    }



    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xffffffffffffffff;
        self
    }

    
    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }


    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 18446744073709551615;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: phypartid11
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn phypartid11_extract(&self) -> u64 {
        // bits 48..63
        self.0.get_bits(48..=63)
    }

    /// reads the current register value and extract field `phypartid11` from it
    pub fn phypartid11_read() -> u64 {
        Self::with_reg_val().phypartid11_extract()
    }

    /// inserts the given value `val` into the field `phypartid11`
    pub fn phypartid11_insert(&mut self, val: u64) -> &mut Self {
        // bits 48..63
        self.0.set_bits(48..=63, val);
        self
    }

    /// reads the register, updates the `phypartid11` field, and writes the updated value
    pub fn phypartid11_write(val: u64) {
        Self::with_reg_val().phypartid11_insert(val).write();
    }

    /*
     * Field: phypartid10
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn phypartid10_extract(&self) -> u64 {
        // bits 32..47
        self.0.get_bits(32..=47)
    }

    /// reads the current register value and extract field `phypartid10` from it
    pub fn phypartid10_read() -> u64 {
        Self::with_reg_val().phypartid10_extract()
    }

    /// inserts the given value `val` into the field `phypartid10`
    pub fn phypartid10_insert(&mut self, val: u64) -> &mut Self {
        // bits 32..47
        self.0.set_bits(32..=47, val);
        self
    }

    /// reads the register, updates the `phypartid10` field, and writes the updated value
    pub fn phypartid10_write(val: u64) {
        Self::with_reg_val().phypartid10_insert(val).write();
    }

    /*
     * Field: phypartid9
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn phypartid9_extract(&self) -> u64 {
        // bits 16..31
        self.0.get_bits(16..=31)
    }

    /// reads the current register value and extract field `phypartid9` from it
    pub fn phypartid9_read() -> u64 {
        Self::with_reg_val().phypartid9_extract()
    }

    /// inserts the given value `val` into the field `phypartid9`
    pub fn phypartid9_insert(&mut self, val: u64) -> &mut Self {
        // bits 16..31
        self.0.set_bits(16..=31, val);
        self
    }

    /// reads the register, updates the `phypartid9` field, and writes the updated value
    pub fn phypartid9_write(val: u64) {
        Self::with_reg_val().phypartid9_insert(val).write();
    }

    /*
     * Field: phypartid8
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn phypartid8_extract(&self) -> u64 {
        // bits 0..15
        self.0.get_bits(0..=15)
    }

    /// reads the current register value and extract field `phypartid8` from it
    pub fn phypartid8_read() -> u64 {
        Self::with_reg_val().phypartid8_extract()
    }

    /// inserts the given value `val` into the field `phypartid8`
    pub fn phypartid8_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..15
        self.0.set_bits(0..=15, val);
        self
    }

    /// reads the register, updates the `phypartid8` field, and writes the updated value
    pub fn phypartid8_write(val: u64) {
        Self::with_reg_val().phypartid8_insert(val).write();
    }

}

impl Default for Mpamvpm2El2 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> Mpamvpm2El2 {
        Mpamvpm2El2(0)
    }
}
