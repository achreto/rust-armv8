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
 * Generated on: 2022-08-22T16:35:53.046589
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
 * Register:    Accelerator Data (accdata_el1)
 * Group:       A group mapping that does not have a known primary
 * Type:        64-bit Register
 * Description: Holds the lower 32 bits of the data that is stored by an ST64BV0, Single-copy atomic 64-byte EL0 store instruction.
 * File:        AArch64-accdata_el1.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Accelerator Data value in memory
pub struct AccdataEl1(u64);

/// struct implementation for accessing the fields of register accdata_el1
impl AccdataEl1 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> AccdataEl1 {
        Self::default()
    }

    /// collects the modifications of AccdataEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> AccdataEl1 {
        AccdataEl1(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  AccdataEl1 {
        let curval = Self::reg_rawrd() & 0xffffffff;
        AccdataEl1(curval)
    }


    
    /// reading the Accelerator Data (accdata_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ACCDATA_EL1
            asm!("mrs {}, S3_0_C13_C0_5", out(reg) regval);
        }
        return regval;
    }


    /// writing the Accelerator Data (accdata_el1) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR ACCDATA_EL1, <Xt>
            asm!("msr S3_0_C13_C0_5, {}", in(reg) val);
        }
    }



    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xffffffff;
        self
    }

    
    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }


    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 4294967295;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: accdata
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn accdata_extract(&self) -> u64 {
        // bits 0..31
        self.0.get_bits(0..=31)
    }

    /// reads the current register value and extract field `accdata` from it
    pub fn accdata_read() -> u64 {
        Self::with_reg_val().accdata_extract()
    }

    /// inserts the given value `val` into the field `accdata`
    pub fn accdata_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..31
        self.0.set_bits(0..=31, val);
        self
    }

    /// reads the register, updates the `accdata` field, and writes the updated value
    pub fn accdata_write(val: u64) {
        Self::with_reg_val().accdata_insert(val).write();
    }

}

impl Default for AccdataEl1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> AccdataEl1 {
        AccdataEl1(0)
    }
}
