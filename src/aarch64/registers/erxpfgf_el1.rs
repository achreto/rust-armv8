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
 * Generated on: 2022-08-22T16:25:59.075156
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
 * Register:    Selected Pseudo-fault Generation Feature register (erxpfgf_el1)
 * Group:       RAS registers
 * Type:        64-bit Register
 * Description: Accesses 
 * File:        AArch64-erxpfgf_el1.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Selected Pseudo-fault Generation Feature register value in memory
pub struct ErxpfgfEl1(u64);

/// struct implementation for accessing the fields of register erxpfgf_el1
impl ErxpfgfEl1 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> ErxpfgfEl1 {
        Self::default()
    }

    /// collects the modifications of ErxpfgfEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> ErxpfgfEl1 {
        ErxpfgfEl1(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  ErxpfgfEl1 {
        let curval = Self::reg_rawrd() & 0xffffffffffffffff;
        ErxpfgfEl1(curval)
    }


    
    /// reading the Selected Pseudo-fault Generation Feature register (erxpfgf_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ERXPFGF_EL1
            asm!("mrs {}, S3_0_C5_C4_4", out(reg) regval);
        }
        return regval;
    }

// register is not writable. not emitting write accessor


    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xffffffffffffffff;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 18446744073709551615;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: val
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn val_extract(&self) -> u64 {
        // bits 0..63
        self.0.get_bits(0..=63)
    }

    /// reads the current register value and extract field `val` from it
    pub fn val_read(&mut self) -> u64 {
        Self::with_reg_val().val_extract()
    }
// no insert() method for field val
}

impl Default for ErxpfgfEl1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> ErxpfgfEl1 {
        ErxpfgfEl1(0)
    }
}
