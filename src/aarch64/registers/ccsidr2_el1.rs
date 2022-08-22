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
 * Generated on: 2022-08-22T16:25:59.069073
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
 * Register:    Current Cache Size ID Register 2 (ccsidr2_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: When 
 * File:        AArch64-ccsidr2_el1.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Current Cache Size ID Register 2 value in memory
pub struct Ccsidr2El1(u64);

/// struct implementation for accessing the fields of register ccsidr2_el1
impl Ccsidr2El1 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> Ccsidr2El1 {
        Self::default()
    }

    /// collects the modifications of Ccsidr2El1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> Ccsidr2El1 {
        Ccsidr2El1(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  Ccsidr2El1 {
        let curval = Self::reg_rawrd() & 0xffffff;
        Ccsidr2El1(curval)
    }


    
    /// reading the Current Cache Size ID Register 2 (ccsidr2_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, CCSIDR2_EL1
            asm!("mrs {}, S3_1_C0_C0_2", out(reg) regval);
        }
        return regval;
    }

// register is not writable. not emitting write accessor


    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xffffff;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 16777215;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: numsets
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn numsets_extract(&self) -> u64 {
        // bits 0..23
        self.0.get_bits(0..=23)
    }

    /// reads the current register value and extract field `numsets` from it
    pub fn numsets_read(&mut self) -> u64 {
        Self::with_reg_val().numsets_extract()
    }
// no insert() method for field numsets
}

impl Default for Ccsidr2El1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> Ccsidr2El1 {
        Ccsidr2El1(0)
    }
}
