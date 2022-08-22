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
 * Generated on: 2022-08-22T16:35:53.067503
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
 * Register:    AArch32 Memory Model Feature Register 3 (id_mmfr3_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Provides information about the implemented memory model and memory management support in AArch32 state.
 * File:        AArch64-id_mmfr3_el1.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the AArch32 Memory Model Feature Register 3 value in memory
pub struct IdMmfr3El1(u64);

/// struct implementation for accessing the fields of register id_mmfr3_el1
impl IdMmfr3El1 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> IdMmfr3El1 {
        Self::default()
    }

    /// collects the modifications of IdMmfr3El1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> IdMmfr3El1 {
        IdMmfr3El1(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  IdMmfr3El1 {
        let curval = Self::reg_rawrd() & 0xffffffff;
        IdMmfr3El1(curval)
    }


    
    /// reading the AArch32 Memory Model Feature Register 3 (id_mmfr3_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ID_MMFR3_EL1
            asm!("mrs {}, id_mmfr3_el1", out(reg) regval);
        }
        return regval;
    }

// register is not writable. not emitting write accessor


    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xffffffff;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 4294967295;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: supersec
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn supersec_extract(&self) -> u64 {
        // bits 28..31
        self.0.get_bits(28..=31)
    }

    /// reads the current register value and extract field `supersec` from it
    pub fn supersec_read() -> u64 {
        Self::with_reg_val().supersec_extract()
    }
// no insert() method for field supersec
    /*
     * Field: cmemsz
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn cmemsz_extract(&self) -> u64 {
        // bits 24..27
        self.0.get_bits(24..=27)
    }

    /// reads the current register value and extract field `cmemsz` from it
    pub fn cmemsz_read() -> u64 {
        Self::with_reg_val().cmemsz_extract()
    }
// no insert() method for field cmemsz
    /*
     * Field: cohwalk
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn cohwalk_extract(&self) -> u64 {
        // bits 20..23
        self.0.get_bits(20..=23)
    }

    /// reads the current register value and extract field `cohwalk` from it
    pub fn cohwalk_read() -> u64 {
        Self::with_reg_val().cohwalk_extract()
    }
// no insert() method for field cohwalk
    /*
     * Field: pan
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn pan_extract(&self) -> u64 {
        // bits 16..19
        self.0.get_bits(16..=19)
    }

    /// reads the current register value and extract field `pan` from it
    pub fn pan_read() -> u64 {
        Self::with_reg_val().pan_extract()
    }
// no insert() method for field pan
    /*
     * Field: maintbcst
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn maintbcst_extract(&self) -> u64 {
        // bits 12..15
        self.0.get_bits(12..=15)
    }

    /// reads the current register value and extract field `maintbcst` from it
    pub fn maintbcst_read() -> u64 {
        Self::with_reg_val().maintbcst_extract()
    }
// no insert() method for field maintbcst
    /*
     * Field: bpmaint
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn bpmaint_extract(&self) -> u64 {
        // bits 8..11
        self.0.get_bits(8..=11)
    }

    /// reads the current register value and extract field `bpmaint` from it
    pub fn bpmaint_read() -> u64 {
        Self::with_reg_val().bpmaint_extract()
    }
// no insert() method for field bpmaint
    /*
     * Field: cmaintsw
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn cmaintsw_extract(&self) -> u64 {
        // bits 4..7
        self.0.get_bits(4..=7)
    }

    /// reads the current register value and extract field `cmaintsw` from it
    pub fn cmaintsw_read() -> u64 {
        Self::with_reg_val().cmaintsw_extract()
    }
// no insert() method for field cmaintsw
    /*
     * Field: cmaintva
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn cmaintva_extract(&self) -> u64 {
        // bits 0..3
        self.0.get_bits(0..=3)
    }

    /// reads the current register value and extract field `cmaintva` from it
    pub fn cmaintva_read() -> u64 {
        Self::with_reg_val().cmaintva_extract()
    }
// no insert() method for field cmaintva
}

impl Default for IdMmfr3El1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> IdMmfr3El1 {
        IdMmfr3El1(0)
    }
}
