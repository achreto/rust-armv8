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
 * Generated on: 2022-08-22T16:35:53.068412
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
 * Register:    LORegionID (EL1) (lorid_el1)
 * Group:       Virtual memory control registers
 * Type:        64-bit Register
 * Description: Indicates the number of LORegions and LORegion descriptors supported by the PE.
 * File:        AArch64-lorid_el1.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the LORegionID (EL1) value in memory
pub struct LoridEl1(u64);

/// struct implementation for accessing the fields of register lorid_el1
impl LoridEl1 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> LoridEl1 {
        Self::default()
    }

    /// collects the modifications of LoridEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> LoridEl1 {
        LoridEl1(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  LoridEl1 {
        let curval = Self::reg_rawrd() & 0xff00ff;
        LoridEl1(curval)
    }


    
    /// reading the LORegionID (EL1) (lorid_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, LORID_EL1
            asm!("mrs {}, S3_0_C10_C4_7", out(reg) regval);
        }
        return regval;
    }

// register is not writable. not emitting write accessor


    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xff00ff;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 16711935;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: ld
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn ld_extract(&self) -> u64 {
        // bits 16..23
        self.0.get_bits(16..=23)
    }

    /// reads the current register value and extract field `ld` from it
    pub fn ld_read() -> u64 {
        Self::with_reg_val().ld_extract()
    }
// no insert() method for field ld
    /*
     * Field: lr
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn lr_extract(&self) -> u64 {
        // bits 0..7
        self.0.get_bits(0..=7)
    }

    /// reads the current register value and extract field `lr` from it
    pub fn lr_read() -> u64 {
        Self::with_reg_val().lr_extract()
    }
// no insert() method for field lr
}

impl Default for LoridEl1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> LoridEl1 {
        LoridEl1(0)
    }
}
