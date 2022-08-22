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

use bit_field::BitField;

/**************************************************************************************************
 *
 * !!!! WARNING: THIS FILE IS AUTO GENERATED. ANY CHANGES MAY BE OVERWRITTEN !!!!
 *
 * Generated on: 2022-08-22T15:51:28.522713
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
 * Register:    AArch64 Memory Model Feature Register 2 (id_aa64mmfr2_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Provides information about the implemented memory model and memory management support in AArch64 state.
 * File:        AArch64-id_aa64mmfr2_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the AArch64 Memory Model Feature Register 2 value in memory
pub struct IdAa64mmfr2El1(u64);

/// struct implementation for accessing the fields of register id_aa64mmfr2_el1
impl IdAa64mmfr2El1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> IdAa64mmfr2El1 {
        Self::default()
    }

    /// collects the modifications of IdAa64mmfr2El1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> IdAa64mmfr2El1 {
        IdAa64mmfr2El1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> IdAa64mmfr2El1 {
        let curval = Self::reg_rawrd() & 0xffff0fffffffffff;
        IdAa64mmfr2El1(curval)
    }

    /// reading the AArch64 Memory Model Feature Register 2 (id_aa64mmfr2_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ID_AA64MMFR2_EL1
            llvm_asm!("mrs $0, id_aa64mmfr2_el1" : "=r"(regval));
        }
        return regval;
    }

    // register is not writable. not emitting write accessor

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0xffff0fffffffffff;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 18446480190918885375;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: e0pd
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn e0pd_extract(&self) -> u64 {
        // bits 60..63
        self.val.get_bits(60..=63)
    }

    /// reads the current register value and extract field `e0pd` from it
    pub fn e0pd_read(&mut self) -> u64 {
        Self::with_reg_val().e0pd_extract()
    }
    // no insert() method for field e0pd
    /*
     * Field: evt
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn evt_extract(&self) -> u64 {
        // bits 56..59
        self.val.get_bits(56..=59)
    }

    /// reads the current register value and extract field `evt` from it
    pub fn evt_read(&mut self) -> u64 {
        Self::with_reg_val().evt_extract()
    }
    // no insert() method for field evt
    /*
     * Field: bbm
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn bbm_extract(&self) -> u64 {
        // bits 52..55
        self.val.get_bits(52..=55)
    }

    /// reads the current register value and extract field `bbm` from it
    pub fn bbm_read(&mut self) -> u64 {
        Self::with_reg_val().bbm_extract()
    }
    // no insert() method for field bbm
    /*
     * Field: ttl
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ttl_extract(&self) -> u64 {
        // bits 48..51
        self.val.get_bits(48..=51)
    }

    /// reads the current register value and extract field `ttl` from it
    pub fn ttl_read(&mut self) -> u64 {
        Self::with_reg_val().ttl_extract()
    }
    // no insert() method for field ttl
    /*
     * Field: fwb
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn fwb_extract(&self) -> u64 {
        // bits 40..43
        self.val.get_bits(40..=43)
    }

    /// reads the current register value and extract field `fwb` from it
    pub fn fwb_read(&mut self) -> u64 {
        Self::with_reg_val().fwb_extract()
    }
    // no insert() method for field fwb
    /*
     * Field: ids
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ids_extract(&self) -> u64 {
        // bits 36..39
        self.val.get_bits(36..=39)
    }

    /// reads the current register value and extract field `ids` from it
    pub fn ids_read(&mut self) -> u64 {
        Self::with_reg_val().ids_extract()
    }
    // no insert() method for field ids
    /*
     * Field: at
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn at_extract(&self) -> u64 {
        // bits 32..35
        self.val.get_bits(32..=35)
    }

    /// reads the current register value and extract field `at` from it
    pub fn at_read(&mut self) -> u64 {
        Self::with_reg_val().at_extract()
    }
    // no insert() method for field at
    /*
     * Field: st
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn st_extract(&self) -> u64 {
        // bits 28..31
        self.val.get_bits(28..=31)
    }

    /// reads the current register value and extract field `st` from it
    pub fn st_read(&mut self) -> u64 {
        Self::with_reg_val().st_extract()
    }
    // no insert() method for field st
    /*
     * Field: nv
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn nv_extract(&self) -> u64 {
        // bits 24..27
        self.val.get_bits(24..=27)
    }

    /// reads the current register value and extract field `nv` from it
    pub fn nv_read(&mut self) -> u64 {
        Self::with_reg_val().nv_extract()
    }
    // no insert() method for field nv
    /*
     * Field: ccidx
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ccidx_extract(&self) -> u64 {
        // bits 20..23
        self.val.get_bits(20..=23)
    }

    /// reads the current register value and extract field `ccidx` from it
    pub fn ccidx_read(&mut self) -> u64 {
        Self::with_reg_val().ccidx_extract()
    }
    // no insert() method for field ccidx
    /*
     * Field: varange
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn varange_extract(&self) -> u64 {
        // bits 16..19
        self.val.get_bits(16..=19)
    }

    /// reads the current register value and extract field `varange` from it
    pub fn varange_read(&mut self) -> u64 {
        Self::with_reg_val().varange_extract()
    }
    // no insert() method for field varange
    /*
     * Field: iesb
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn iesb_extract(&self) -> u64 {
        // bits 12..15
        self.val.get_bits(12..=15)
    }

    /// reads the current register value and extract field `iesb` from it
    pub fn iesb_read(&mut self) -> u64 {
        Self::with_reg_val().iesb_extract()
    }
    // no insert() method for field iesb
    /*
     * Field: lsm
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn lsm_extract(&self) -> u64 {
        // bits 8..11
        self.val.get_bits(8..=11)
    }

    /// reads the current register value and extract field `lsm` from it
    pub fn lsm_read(&mut self) -> u64 {
        Self::with_reg_val().lsm_extract()
    }
    // no insert() method for field lsm
    /*
     * Field: uao
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn uao_extract(&self) -> u64 {
        // bits 4..7
        self.val.get_bits(4..=7)
    }

    /// reads the current register value and extract field `uao` from it
    pub fn uao_read(&mut self) -> u64 {
        Self::with_reg_val().uao_extract()
    }
    // no insert() method for field uao
    /*
     * Field: cnp
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn cnp_extract(&self) -> u64 {
        // bits 0..3
        self.val.get_bits(0..=3)
    }

    /// reads the current register value and extract field `cnp` from it
    pub fn cnp_read(&mut self) -> u64 {
        Self::with_reg_val().cnp_extract()
    }
    // no insert() method for field cnp
}

impl Default for IdAa64mmfr2El1 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> IdAa64mmfr2El1 {
        IdAa64mmfr2El1(0)
    }
}
