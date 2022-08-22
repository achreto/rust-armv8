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
 * Generated on: 2022-08-22T16:35:53.075316
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
 * Register:    Sampling Profiling ID Register (pmsidr_el1)
 * Group:       Statistical Profiling Extension registers
 * Type:        64-bit Register
 * Description: Describes the Statistical Profiling implementation to software
 * File:        AArch64-pmsidr_el1.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Sampling Profiling ID Register value in memory
pub struct PmsidrEl1(u64);

/// struct implementation for accessing the fields of register pmsidr_el1
impl PmsidrEl1 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> PmsidrEl1 {
        Self::default()
    }

    /// collects the modifications of PmsidrEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> PmsidrEl1 {
        PmsidrEl1(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  PmsidrEl1 {
        let curval = Self::reg_rawrd() & 0xffff7f;
        PmsidrEl1(curval)
    }


    
    /// reading the Sampling Profiling ID Register (pmsidr_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, PMSIDR_EL1
            asm!("mrs {}, S3_0_C9_C9_7", out(reg) regval);
        }
        return regval;
    }

// register is not writable. not emitting write accessor


    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xffff7f;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 16777087;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: format_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn format_1_extract(&self) -> u64 {
        // bits 20..23
        self.0.get_bits(20..=23)
    }

    /// reads the current register value and extract field `format_1` from it
    pub fn format_1_read() -> u64 {
        Self::with_reg_val().format_1_extract()
    }
// no insert() method for field format_1
    /*
     * Field: countsize
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn countsize_extract(&self) -> u64 {
        // bits 16..19
        self.0.get_bits(16..=19)
    }

    /// reads the current register value and extract field `countsize` from it
    pub fn countsize_read() -> u64 {
        Self::with_reg_val().countsize_extract()
    }
// no insert() method for field countsize
    /*
     * Field: maxsize
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn maxsize_extract(&self) -> u64 {
        // bits 12..15
        self.0.get_bits(12..=15)
    }

    /// reads the current register value and extract field `maxsize` from it
    pub fn maxsize_read() -> u64 {
        Self::with_reg_val().maxsize_extract()
    }
// no insert() method for field maxsize
    /*
     * Field: interval
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn interval_extract(&self) -> u64 {
        // bits 8..11
        self.0.get_bits(8..=11)
    }

    /// reads the current register value and extract field `interval` from it
    pub fn interval_read() -> u64 {
        Self::with_reg_val().interval_extract()
    }
// no insert() method for field interval
    /*
     * Field: fne
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn fne_extract(&self) -> u64 {
        // bits 6..6
        self.0.get_bits(6..=6)
    }

    /// reads the current register value and extract field `fne` from it
    pub fn fne_read() -> u64 {
        Self::with_reg_val().fne_extract()
    }
// no insert() method for field fne
    /*
     * Field: ernd
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn ernd_extract(&self) -> u64 {
        // bits 5..5
        self.0.get_bits(5..=5)
    }

    /// reads the current register value and extract field `ernd` from it
    pub fn ernd_read() -> u64 {
        Self::with_reg_val().ernd_extract()
    }
// no insert() method for field ernd
    /*
     * Field: lds
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn lds_extract(&self) -> u64 {
        // bits 4..4
        self.0.get_bits(4..=4)
    }

    /// reads the current register value and extract field `lds` from it
    pub fn lds_read() -> u64 {
        Self::with_reg_val().lds_extract()
    }
// no insert() method for field lds
    /*
     * Field: archinst
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn archinst_extract(&self) -> u64 {
        // bits 3..3
        self.0.get_bits(3..=3)
    }

    /// reads the current register value and extract field `archinst` from it
    pub fn archinst_read() -> u64 {
        Self::with_reg_val().archinst_extract()
    }
// no insert() method for field archinst
    /*
     * Field: fl
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn fl_extract(&self) -> u64 {
        // bits 2..2
        self.0.get_bits(2..=2)
    }

    /// reads the current register value and extract field `fl` from it
    pub fn fl_read() -> u64 {
        Self::with_reg_val().fl_extract()
    }
// no insert() method for field fl
    /*
     * Field: ft
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn ft_extract(&self) -> u64 {
        // bits 1..1
        self.0.get_bits(1..=1)
    }

    /// reads the current register value and extract field `ft` from it
    pub fn ft_read() -> u64 {
        Self::with_reg_val().ft_extract()
    }
// no insert() method for field ft
    /*
     * Field: fe
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn fe_extract(&self) -> u64 {
        // bits 0..0
        self.0.get_bits(0..=0)
    }

    /// reads the current register value and extract field `fe` from it
    pub fn fe_read() -> u64 {
        Self::with_reg_val().fe_extract()
    }
// no insert() method for field fe
}

impl Default for PmsidrEl1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> PmsidrEl1 {
        PmsidrEl1(0)
    }
}
