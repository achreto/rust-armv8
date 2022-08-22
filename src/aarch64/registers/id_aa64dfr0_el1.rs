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
 * Generated on: 2022-08-22T16:35:53.065010
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
 * Register:    AArch64 Debug Feature Register 0 (id_aa64dfr0_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Provides top level information about the debug system in AArch64 state.
 * File:        AArch64-id_aa64dfr0_el1.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the AArch64 Debug Feature Register 0 value in memory
pub struct IdAa64dfr0El1(u64);

/// struct implementation for accessing the fields of register id_aa64dfr0_el1
impl IdAa64dfr0El1 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> IdAa64dfr0El1 {
        Self::default()
    }

    /// collects the modifications of IdAa64dfr0El1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> IdAa64dfr0El1 {
        IdAa64dfr0El1(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  IdAa64dfr0El1 {
        let curval = Self::reg_rawrd() & 0xf0ffff0f0ffff;
        IdAa64dfr0El1(curval)
    }


    
    /// reading the AArch64 Debug Feature Register 0 (id_aa64dfr0_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ID_AA64DFR0_EL1
            asm!("mrs {}, id_aa64dfr0_el1", out(reg) regval);
        }
        return regval;
    }

// register is not writable. not emitting write accessor


    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xf0ffff0f0ffff;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 4239716584062975;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: mtpmu
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn mtpmu_extract(&self) -> u64 {
        // bits 48..51
        self.0.get_bits(48..=51)
    }

    /// reads the current register value and extract field `mtpmu` from it
    pub fn mtpmu_read() -> u64 {
        Self::with_reg_val().mtpmu_extract()
    }
// no insert() method for field mtpmu
    /*
     * Field: tracefilt
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn tracefilt_extract(&self) -> u64 {
        // bits 40..43
        self.0.get_bits(40..=43)
    }

    /// reads the current register value and extract field `tracefilt` from it
    pub fn tracefilt_read() -> u64 {
        Self::with_reg_val().tracefilt_extract()
    }
// no insert() method for field tracefilt
    /*
     * Field: doublelock
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn doublelock_extract(&self) -> u64 {
        // bits 36..39
        self.0.get_bits(36..=39)
    }

    /// reads the current register value and extract field `doublelock` from it
    pub fn doublelock_read() -> u64 {
        Self::with_reg_val().doublelock_extract()
    }
// no insert() method for field doublelock
    /*
     * Field: pmsver
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn pmsver_extract(&self) -> u64 {
        // bits 32..35
        self.0.get_bits(32..=35)
    }

    /// reads the current register value and extract field `pmsver` from it
    pub fn pmsver_read() -> u64 {
        Self::with_reg_val().pmsver_extract()
    }
// no insert() method for field pmsver
    /*
     * Field: ctx_cmps
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn ctx_cmps_extract(&self) -> u64 {
        // bits 28..31
        self.0.get_bits(28..=31)
    }

    /// reads the current register value and extract field `ctx_cmps` from it
    pub fn ctx_cmps_read() -> u64 {
        Self::with_reg_val().ctx_cmps_extract()
    }
// no insert() method for field ctx_cmps
    /*
     * Field: wrps
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn wrps_extract(&self) -> u64 {
        // bits 20..23
        self.0.get_bits(20..=23)
    }

    /// reads the current register value and extract field `wrps` from it
    pub fn wrps_read() -> u64 {
        Self::with_reg_val().wrps_extract()
    }
// no insert() method for field wrps
    /*
     * Field: brps
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn brps_extract(&self) -> u64 {
        // bits 12..15
        self.0.get_bits(12..=15)
    }

    /// reads the current register value and extract field `brps` from it
    pub fn brps_read() -> u64 {
        Self::with_reg_val().brps_extract()
    }
// no insert() method for field brps
    /*
     * Field: pmuver
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn pmuver_extract(&self) -> u64 {
        // bits 8..11
        self.0.get_bits(8..=11)
    }

    /// reads the current register value and extract field `pmuver` from it
    pub fn pmuver_read() -> u64 {
        Self::with_reg_val().pmuver_extract()
    }
// no insert() method for field pmuver
    /*
     * Field: tracever
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn tracever_extract(&self) -> u64 {
        // bits 4..7
        self.0.get_bits(4..=7)
    }

    /// reads the current register value and extract field `tracever` from it
    pub fn tracever_read() -> u64 {
        Self::with_reg_val().tracever_extract()
    }
// no insert() method for field tracever
    /*
     * Field: debugver
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn debugver_extract(&self) -> u64 {
        // bits 0..3
        self.0.get_bits(0..=3)
    }

    /// reads the current register value and extract field `debugver` from it
    pub fn debugver_read() -> u64 {
        Self::with_reg_val().debugver_extract()
    }
// no insert() method for field debugver
}

impl Default for IdAa64dfr0El1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> IdAa64dfr0El1 {
        IdAa64dfr0El1(0)
    }
}
