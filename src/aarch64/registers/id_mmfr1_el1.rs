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
 * Generated on: 2022-08-22T16:25:59.086063
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
 * Register:    AArch32 Memory Model Feature Register 1 (id_mmfr1_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Provides information about the implemented memory model and memory management support in AArch32 state.
 * File:        AArch64-id_mmfr1_el1.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the AArch32 Memory Model Feature Register 1 value in memory
pub struct IdMmfr1El1(u64);

/// struct implementation for accessing the fields of register id_mmfr1_el1
impl IdMmfr1El1 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> IdMmfr1El1 {
        Self::default()
    }

    /// collects the modifications of IdMmfr1El1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> IdMmfr1El1 {
        IdMmfr1El1(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  IdMmfr1El1 {
        let curval = Self::reg_rawrd() & 0xffffffff;
        IdMmfr1El1(curval)
    }


    
    /// reading the AArch32 Memory Model Feature Register 1 (id_mmfr1_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ID_MMFR1_EL1
            asm!("mrs {}, id_mmfr1_el1", out(reg) regval);
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
     * Field: bpred
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn bpred_extract(&self) -> u64 {
        // bits 28..31
        self.0.get_bits(28..=31)
    }

    /// reads the current register value and extract field `bpred` from it
    pub fn bpred_read(&mut self) -> u64 {
        Self::with_reg_val().bpred_extract()
    }
// no insert() method for field bpred
    /*
     * Field: l1tstcln
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn l1tstcln_extract(&self) -> u64 {
        // bits 24..27
        self.0.get_bits(24..=27)
    }

    /// reads the current register value and extract field `l1tstcln` from it
    pub fn l1tstcln_read(&mut self) -> u64 {
        Self::with_reg_val().l1tstcln_extract()
    }
// no insert() method for field l1tstcln
    /*
     * Field: l1uni
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn l1uni_extract(&self) -> u64 {
        // bits 20..23
        self.0.get_bits(20..=23)
    }

    /// reads the current register value and extract field `l1uni` from it
    pub fn l1uni_read(&mut self) -> u64 {
        Self::with_reg_val().l1uni_extract()
    }
// no insert() method for field l1uni
    /*
     * Field: l1hvd
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn l1hvd_extract(&self) -> u64 {
        // bits 16..19
        self.0.get_bits(16..=19)
    }

    /// reads the current register value and extract field `l1hvd` from it
    pub fn l1hvd_read(&mut self) -> u64 {
        Self::with_reg_val().l1hvd_extract()
    }
// no insert() method for field l1hvd
    /*
     * Field: l1unisw
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn l1unisw_extract(&self) -> u64 {
        // bits 12..15
        self.0.get_bits(12..=15)
    }

    /// reads the current register value and extract field `l1unisw` from it
    pub fn l1unisw_read(&mut self) -> u64 {
        Self::with_reg_val().l1unisw_extract()
    }
// no insert() method for field l1unisw
    /*
     * Field: l1hvdsw
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn l1hvdsw_extract(&self) -> u64 {
        // bits 8..11
        self.0.get_bits(8..=11)
    }

    /// reads the current register value and extract field `l1hvdsw` from it
    pub fn l1hvdsw_read(&mut self) -> u64 {
        Self::with_reg_val().l1hvdsw_extract()
    }
// no insert() method for field l1hvdsw
    /*
     * Field: l1univa
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn l1univa_extract(&self) -> u64 {
        // bits 4..7
        self.0.get_bits(4..=7)
    }

    /// reads the current register value and extract field `l1univa` from it
    pub fn l1univa_read(&mut self) -> u64 {
        Self::with_reg_val().l1univa_extract()
    }
// no insert() method for field l1univa
    /*
     * Field: l1hvdva
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn l1hvdva_extract(&self) -> u64 {
        // bits 0..3
        self.0.get_bits(0..=3)
    }

    /// reads the current register value and extract field `l1hvdva` from it
    pub fn l1hvdva_read(&mut self) -> u64 {
        Self::with_reg_val().l1hvdva_extract()
    }
// no insert() method for field l1hvdva
}

impl Default for IdMmfr1El1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> IdMmfr1El1 {
        IdMmfr1El1(0)
    }
}
