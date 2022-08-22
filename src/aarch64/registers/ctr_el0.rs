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
 * Generated on: 2022-08-22T16:25:59.072276
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
 * Register:    Cache Type Register (ctr_el0)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Provides information about the architecture of the caches.
 * File:        AArch64-ctr_el0.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Cache Type Register value in memory
pub struct CtrEl0(u64);

/// struct implementation for accessing the fields of register ctr_el0
impl CtrEl0 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> CtrEl0 {
        Self::default()
    }

    /// collects the modifications of CtrEl0 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> CtrEl0 {
        CtrEl0(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  CtrEl0 {
        let curval = Self::reg_rawrd() & 0x3f3fffc00f;
        CtrEl0(curval)
    }


    
    /// reading the Cache Type Register (ctr_el0) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, CTR_EL0
            asm!("mrs {}, ctr_el0", out(reg) regval);
        }
        return regval;
    }

// register is not writable. not emitting write accessor


    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0x3f3fffc00f;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 271656665103;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: tminline_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn tminline_1_extract(&self) -> u64 {
        // bits 32..37
        self.0.get_bits(32..=37)
    }

    /// reads the current register value and extract field `tminline_1` from it
    pub fn tminline_1_read(&mut self) -> u64 {
        Self::with_reg_val().tminline_1_extract()
    }
// no insert() method for field tminline_1
    /*
     * Field: dic
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn dic_extract(&self) -> u64 {
        // bits 29..29
        self.0.get_bits(29..=29)
    }

    /// reads the current register value and extract field `dic` from it
    pub fn dic_read(&mut self) -> u64 {
        Self::with_reg_val().dic_extract()
    }
// no insert() method for field dic
    /*
     * Field: idc
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn idc_extract(&self) -> u64 {
        // bits 28..28
        self.0.get_bits(28..=28)
    }

    /// reads the current register value and extract field `idc` from it
    pub fn idc_read(&mut self) -> u64 {
        Self::with_reg_val().idc_extract()
    }
// no insert() method for field idc
    /*
     * Field: cwg
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn cwg_extract(&self) -> u64 {
        // bits 24..27
        self.0.get_bits(24..=27)
    }

    /// reads the current register value and extract field `cwg` from it
    pub fn cwg_read(&mut self) -> u64 {
        Self::with_reg_val().cwg_extract()
    }
// no insert() method for field cwg
    /*
     * Field: erg
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn erg_extract(&self) -> u64 {
        // bits 20..23
        self.0.get_bits(20..=23)
    }

    /// reads the current register value and extract field `erg` from it
    pub fn erg_read(&mut self) -> u64 {
        Self::with_reg_val().erg_extract()
    }
// no insert() method for field erg
    /*
     * Field: dminline
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn dminline_extract(&self) -> u64 {
        // bits 16..19
        self.0.get_bits(16..=19)
    }

    /// reads the current register value and extract field `dminline` from it
    pub fn dminline_read(&mut self) -> u64 {
        Self::with_reg_val().dminline_extract()
    }
// no insert() method for field dminline
    /*
     * Field: l1ip
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn l1ip_extract(&self) -> u64 {
        // bits 14..15
        self.0.get_bits(14..=15)
    }

    /// reads the current register value and extract field `l1ip` from it
    pub fn l1ip_read(&mut self) -> u64 {
        Self::with_reg_val().l1ip_extract()
    }
// no insert() method for field l1ip
    /*
     * Field: iminline
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn iminline_extract(&self) -> u64 {
        // bits 0..3
        self.0.get_bits(0..=3)
    }

    /// reads the current register value and extract field `iminline` from it
    pub fn iminline_read(&mut self) -> u64 {
        Self::with_reg_val().iminline_extract()
    }
// no insert() method for field iminline
}

impl Default for CtrEl0 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> CtrEl0 {
        CtrEl0(0)
    }
}
