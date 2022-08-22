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
 * Generated on: 2022-08-22T16:35:53.073103
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
 * Register:    Profiling Buffer Write Pointer Register (pmbptr_el1)
 * Group:       Statistical Profiling Extension registers
 * Type:        64-bit Register
 * Description: Defines the current write pointer for the profiling buffer.
 * File:        AArch64-pmbptr_el1.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Profiling Buffer Write Pointer Register value in memory
pub struct PmbptrEl1(u64);

/// struct implementation for accessing the fields of register pmbptr_el1
impl PmbptrEl1 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> PmbptrEl1 {
        Self::default()
    }

    /// collects the modifications of PmbptrEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> PmbptrEl1 {
        PmbptrEl1(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  PmbptrEl1 {
        let curval = Self::reg_rawrd() & 0xffffffffffffffff;
        PmbptrEl1(curval)
    }


    
    /// reading the Profiling Buffer Write Pointer Register (pmbptr_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, PMBPTR_EL1
            asm!("mrs {}, S3_0_C9_C10_1", out(reg) regval);
        }
        return regval;
    }


    /// writing the Profiling Buffer Write Pointer Register (pmbptr_el1) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR PMBPTR_EL1, <Xt>
            asm!("msr S3_0_C9_C10_1, {}", in(reg) val);
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
     * Field: ptr
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn ptr_extract(&self) -> u64 {
        // bits 0..63
        self.0.get_bits(0..=63)
    }

    /// reads the current register value and extract field `ptr` from it
    pub fn ptr_read() -> u64 {
        Self::with_reg_val().ptr_extract()
    }

    /// inserts the given value `val` into the field `ptr`
    pub fn ptr_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..63
        self.0.set_bits(0..=63, val);
        self
    }

    /// reads the register, updates the `ptr` field, and writes the updated value
    pub fn ptr_write(val: u64) {
        Self::with_reg_val().ptr_insert(val).write();
    }

}

impl Default for PmbptrEl1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> PmbptrEl1 {
        PmbptrEl1(0)
    }
}
