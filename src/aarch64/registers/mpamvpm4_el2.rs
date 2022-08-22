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
 * Generated on: 2022-08-22T16:35:53.070837
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
 * Register:    MPAM Virtual PARTID Mapping Register 4 (mpamvpm4_el2)
 * Group:       Memory Partitioning and Monitoring registers
 * Type:        64-bit Register
 * Description: MPAMVPM4_EL2 provides mappings from virtual PARTIDs 16 - 19 to physical PARTIDs.
 * File:        AArch64-mpamvpm4_el2.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the MPAM Virtual PARTID Mapping Register 4 value in memory
pub struct Mpamvpm4El2(u64);

/// struct implementation for accessing the fields of register mpamvpm4_el2
impl Mpamvpm4El2 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> Mpamvpm4El2 {
        Self::default()
    }

    /// collects the modifications of Mpamvpm4El2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> Mpamvpm4El2 {
        Mpamvpm4El2(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  Mpamvpm4El2 {
        let curval = Self::reg_rawrd() & 0xffffffffffffffff;
        Mpamvpm4El2(curval)
    }


    
    /// reading the MPAM Virtual PARTID Mapping Register 4 (mpamvpm4_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, MPAMVPM4_EL2
            asm!("mrs {}, S3_4_C10_C6_4", out(reg) regval);
        }
        return regval;
    }


    /// writing the MPAM Virtual PARTID Mapping Register 4 (mpamvpm4_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR MPAMVPM4_EL2, <Xt>
            asm!("msr S3_4_C10_C6_4, {}", in(reg) val);
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
     * Field: phypartid19
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn phypartid19_extract(&self) -> u64 {
        // bits 48..63
        self.0.get_bits(48..=63)
    }

    /// reads the current register value and extract field `phypartid19` from it
    pub fn phypartid19_read() -> u64 {
        Self::with_reg_val().phypartid19_extract()
    }

    /// inserts the given value `val` into the field `phypartid19`
    pub fn phypartid19_insert(&mut self, val: u64) -> &mut Self {
        // bits 48..63
        self.0.set_bits(48..=63, val);
        self
    }

    /// reads the register, updates the `phypartid19` field, and writes the updated value
    pub fn phypartid19_write(val: u64) {
        Self::with_reg_val().phypartid19_insert(val).write();
    }

    /*
     * Field: phypartid18
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn phypartid18_extract(&self) -> u64 {
        // bits 32..47
        self.0.get_bits(32..=47)
    }

    /// reads the current register value and extract field `phypartid18` from it
    pub fn phypartid18_read() -> u64 {
        Self::with_reg_val().phypartid18_extract()
    }

    /// inserts the given value `val` into the field `phypartid18`
    pub fn phypartid18_insert(&mut self, val: u64) -> &mut Self {
        // bits 32..47
        self.0.set_bits(32..=47, val);
        self
    }

    /// reads the register, updates the `phypartid18` field, and writes the updated value
    pub fn phypartid18_write(val: u64) {
        Self::with_reg_val().phypartid18_insert(val).write();
    }

    /*
     * Field: phypartid17
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn phypartid17_extract(&self) -> u64 {
        // bits 16..31
        self.0.get_bits(16..=31)
    }

    /// reads the current register value and extract field `phypartid17` from it
    pub fn phypartid17_read() -> u64 {
        Self::with_reg_val().phypartid17_extract()
    }

    /// inserts the given value `val` into the field `phypartid17`
    pub fn phypartid17_insert(&mut self, val: u64) -> &mut Self {
        // bits 16..31
        self.0.set_bits(16..=31, val);
        self
    }

    /// reads the register, updates the `phypartid17` field, and writes the updated value
    pub fn phypartid17_write(val: u64) {
        Self::with_reg_val().phypartid17_insert(val).write();
    }

    /*
     * Field: phypartid16
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn phypartid16_extract(&self) -> u64 {
        // bits 0..15
        self.0.get_bits(0..=15)
    }

    /// reads the current register value and extract field `phypartid16` from it
    pub fn phypartid16_read() -> u64 {
        Self::with_reg_val().phypartid16_extract()
    }

    /// inserts the given value `val` into the field `phypartid16`
    pub fn phypartid16_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..15
        self.0.set_bits(0..=15, val);
        self
    }

    /// reads the register, updates the `phypartid16` field, and writes the updated value
    pub fn phypartid16_write(val: u64) {
        Self::with_reg_val().phypartid16_insert(val).write();
    }

}

impl Default for Mpamvpm4El2 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> Mpamvpm4El2 {
        Mpamvpm4El2(0)
    }
}
