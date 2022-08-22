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
 * Generated on: 2022-08-22T16:35:53.055057
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
 * Register:    Data Independent Timing (dit)
 * Group:       Process state registers
 * Type:        64-bit Register
 * Description: Allows access to the Data Independent Timing bit.
 * File:        AArch64-dit.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Data Independent Timing value in memory
pub struct Dit(u64);

/// struct implementation for accessing the fields of register dit
impl Dit {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> Dit {
        Self::default()
    }

    /// collects the modifications of Dit and creates new object
    #[inline(always)]
    pub fn collect(&self) -> Dit {
        Dit(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  Dit {
        let curval = Self::reg_rawrd() & 0x1000000;
        Dit(curval)
    }


    
    /// reading the Data Independent Timing (dit) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, DIT
            asm!("mrs {}, S3_3_C4_C2_5", out(reg) regval);
        }
        return regval;
    }


    /// writing the Data Independent Timing (dit) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR DIT, <Xt>
            asm!("msr S3_3_C4_C2_5, {}", in(reg) val);
        }
    }



    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0x1000000;
        self
    }

    
    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }


    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 16777216;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: dit
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn dit_extract(&self) -> u64 {
        // bits 24..24
        self.0.get_bits(24..=24)
    }

    /// reads the current register value and extract field `dit` from it
    pub fn dit_read() -> u64 {
        Self::with_reg_val().dit_extract()
    }

    /// inserts the given value `val` into the field `dit`
    pub fn dit_insert(&mut self, val: u64) -> &mut Self {
        // bits 24..24
        self.0.set_bits(24..=24, val);
        self
    }

    /// reads the register, updates the `dit` field, and writes the updated value
    pub fn dit_write(val: u64) {
        Self::with_reg_val().dit_insert(val).write();
    }

}

impl Default for Dit {
    /// creates a new default value
    #[inline(always)]
    fn default() -> Dit {
        Dit(0)
    }
}
