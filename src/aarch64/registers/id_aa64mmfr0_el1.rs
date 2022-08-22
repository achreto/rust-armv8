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
 * Generated on: 2022-08-22T16:25:59.084267
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
 * Register:    AArch64 Memory Model Feature Register 0 (id_aa64mmfr0_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Provides information about the implemented memory model and memory management support in AArch64 state.
 * File:        AArch64-id_aa64mmfr0_el1.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the AArch64 Memory Model Feature Register 0 value in memory
pub struct IdAa64mmfr0El1(u64);

/// struct implementation for accessing the fields of register id_aa64mmfr0_el1
impl IdAa64mmfr0El1 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> IdAa64mmfr0El1 {
        Self::default()
    }

    /// collects the modifications of IdAa64mmfr0El1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> IdAa64mmfr0El1 {
        IdAa64mmfr0El1(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  IdAa64mmfr0El1 {
        let curval = Self::reg_rawrd() & 0xff00ffffffffffff;
        IdAa64mmfr0El1(curval)
    }


    
    /// reading the AArch64 Memory Model Feature Register 0 (id_aa64mmfr0_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ID_AA64MMFR0_EL1
            asm!("mrs {}, id_aa64mmfr0_el1", out(reg) regval);
        }
        return regval;
    }

// register is not writable. not emitting write accessor


    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xff00ffffffffffff;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 18374967954648334335;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: ecv
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn ecv_extract(&self) -> u64 {
        // bits 60..63
        self.0.get_bits(60..=63)
    }

    /// reads the current register value and extract field `ecv` from it
    pub fn ecv_read(&mut self) -> u64 {
        Self::with_reg_val().ecv_extract()
    }
// no insert() method for field ecv
    /*
     * Field: fgt
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn fgt_extract(&self) -> u64 {
        // bits 56..59
        self.0.get_bits(56..=59)
    }

    /// reads the current register value and extract field `fgt` from it
    pub fn fgt_read(&mut self) -> u64 {
        Self::with_reg_val().fgt_extract()
    }
// no insert() method for field fgt
    /*
     * Field: exs
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn exs_extract(&self) -> u64 {
        // bits 44..47
        self.0.get_bits(44..=47)
    }

    /// reads the current register value and extract field `exs` from it
    pub fn exs_read(&mut self) -> u64 {
        Self::with_reg_val().exs_extract()
    }
// no insert() method for field exs
    /*
     * Field: tgran4_2
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn tgran4_2_extract(&self) -> u64 {
        // bits 40..43
        self.0.get_bits(40..=43)
    }

    /// reads the current register value and extract field `tgran4_2` from it
    pub fn tgran4_2_read(&mut self) -> u64 {
        Self::with_reg_val().tgran4_2_extract()
    }
// no insert() method for field tgran4_2
    /*
     * Field: tgran64_2
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn tgran64_2_extract(&self) -> u64 {
        // bits 36..39
        self.0.get_bits(36..=39)
    }

    /// reads the current register value and extract field `tgran64_2` from it
    pub fn tgran64_2_read(&mut self) -> u64 {
        Self::with_reg_val().tgran64_2_extract()
    }
// no insert() method for field tgran64_2
    /*
     * Field: tgran16_2
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn tgran16_2_extract(&self) -> u64 {
        // bits 32..35
        self.0.get_bits(32..=35)
    }

    /// reads the current register value and extract field `tgran16_2` from it
    pub fn tgran16_2_read(&mut self) -> u64 {
        Self::with_reg_val().tgran16_2_extract()
    }
// no insert() method for field tgran16_2
    /*
     * Field: tgran4
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn tgran4_extract(&self) -> u64 {
        // bits 28..31
        self.0.get_bits(28..=31)
    }

    /// reads the current register value and extract field `tgran4` from it
    pub fn tgran4_read(&mut self) -> u64 {
        Self::with_reg_val().tgran4_extract()
    }
// no insert() method for field tgran4
    /*
     * Field: tgran64
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn tgran64_extract(&self) -> u64 {
        // bits 24..27
        self.0.get_bits(24..=27)
    }

    /// reads the current register value and extract field `tgran64` from it
    pub fn tgran64_read(&mut self) -> u64 {
        Self::with_reg_val().tgran64_extract()
    }
// no insert() method for field tgran64
    /*
     * Field: tgran16
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn tgran16_extract(&self) -> u64 {
        // bits 20..23
        self.0.get_bits(20..=23)
    }

    /// reads the current register value and extract field `tgran16` from it
    pub fn tgran16_read(&mut self) -> u64 {
        Self::with_reg_val().tgran16_extract()
    }
// no insert() method for field tgran16
    /*
     * Field: bigendel0
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn bigendel0_extract(&self) -> u64 {
        // bits 16..19
        self.0.get_bits(16..=19)
    }

    /// reads the current register value and extract field `bigendel0` from it
    pub fn bigendel0_read(&mut self) -> u64 {
        Self::with_reg_val().bigendel0_extract()
    }
// no insert() method for field bigendel0
    /*
     * Field: snsmem
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn snsmem_extract(&self) -> u64 {
        // bits 12..15
        self.0.get_bits(12..=15)
    }

    /// reads the current register value and extract field `snsmem` from it
    pub fn snsmem_read(&mut self) -> u64 {
        Self::with_reg_val().snsmem_extract()
    }
// no insert() method for field snsmem
    /*
     * Field: bigend
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn bigend_extract(&self) -> u64 {
        // bits 8..11
        self.0.get_bits(8..=11)
    }

    /// reads the current register value and extract field `bigend` from it
    pub fn bigend_read(&mut self) -> u64 {
        Self::with_reg_val().bigend_extract()
    }
// no insert() method for field bigend
    /*
     * Field: asidbits
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn asidbits_extract(&self) -> u64 {
        // bits 4..7
        self.0.get_bits(4..=7)
    }

    /// reads the current register value and extract field `asidbits` from it
    pub fn asidbits_read(&mut self) -> u64 {
        Self::with_reg_val().asidbits_extract()
    }
// no insert() method for field asidbits
    /*
     * Field: parange
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn parange_extract(&self) -> u64 {
        // bits 0..3
        self.0.get_bits(0..=3)
    }

    /// reads the current register value and extract field `parange` from it
    pub fn parange_read(&mut self) -> u64 {
        Self::with_reg_val().parange_extract()
    }
// no insert() method for field parange
}

impl Default for IdAa64mmfr0El1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> IdAa64mmfr0El1 {
        IdAa64mmfr0El1(0)
    }
}
