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
 * Generated on: 2022-08-22T15:51:28.523228
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
 * Register:    AArch32 Debug Feature Register 0 (id_dfr0_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Provides top level information about the debug system in AArch32 state.
 * File:        AArch64-id_dfr0_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the AArch32 Debug Feature Register 0 value in memory
pub struct IdDfr0El1(u64);

/// struct implementation for accessing the fields of register id_dfr0_el1
impl IdDfr0El1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> IdDfr0El1 {
        Self::default()
    }

    /// collects the modifications of IdDfr0El1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> IdDfr0El1 {
        IdDfr0El1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> IdDfr0El1 {
        let curval = Self::reg_rawrd() & 0xffffffff;
        IdDfr0El1(curval)
    }

    /// reading the AArch32 Debug Feature Register 0 (id_dfr0_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ID_DFR0_EL1
            llvm_asm!("mrs $0, id_dfr0_el1" : "=r"(regval));
        }
        return regval;
    }

    // register is not writable. not emitting write accessor

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0xffffffff;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 4294967295;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: tracefilt
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tracefilt_extract(&self) -> u64 {
        // bits 28..31
        self.val.get_bits(28..=31)
    }

    /// reads the current register value and extract field `tracefilt` from it
    pub fn tracefilt_read(&mut self) -> u64 {
        Self::with_reg_val().tracefilt_extract()
    }
    // no insert() method for field tracefilt
    /*
     * Field: perfmon
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn perfmon_extract(&self) -> u64 {
        // bits 24..27
        self.val.get_bits(24..=27)
    }

    /// reads the current register value and extract field `perfmon` from it
    pub fn perfmon_read(&mut self) -> u64 {
        Self::with_reg_val().perfmon_extract()
    }
    // no insert() method for field perfmon
    /*
     * Field: mprofdbg
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn mprofdbg_extract(&self) -> u64 {
        // bits 20..23
        self.val.get_bits(20..=23)
    }

    /// reads the current register value and extract field `mprofdbg` from it
    pub fn mprofdbg_read(&mut self) -> u64 {
        Self::with_reg_val().mprofdbg_extract()
    }
    // no insert() method for field mprofdbg
    /*
     * Field: mmaptrc
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn mmaptrc_extract(&self) -> u64 {
        // bits 16..19
        self.val.get_bits(16..=19)
    }

    /// reads the current register value and extract field `mmaptrc` from it
    pub fn mmaptrc_read(&mut self) -> u64 {
        Self::with_reg_val().mmaptrc_extract()
    }
    // no insert() method for field mmaptrc
    /*
     * Field: coptrc
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn coptrc_extract(&self) -> u64 {
        // bits 12..15
        self.val.get_bits(12..=15)
    }

    /// reads the current register value and extract field `coptrc` from it
    pub fn coptrc_read(&mut self) -> u64 {
        Self::with_reg_val().coptrc_extract()
    }
    // no insert() method for field coptrc
    /*
     * Field: mmapdbg
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn mmapdbg_extract(&self) -> u64 {
        // bits 8..11
        self.val.get_bits(8..=11)
    }

    /// reads the current register value and extract field `mmapdbg` from it
    pub fn mmapdbg_read(&mut self) -> u64 {
        Self::with_reg_val().mmapdbg_extract()
    }
    // no insert() method for field mmapdbg
    /*
     * Field: copsdbg
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn copsdbg_extract(&self) -> u64 {
        // bits 4..7
        self.val.get_bits(4..=7)
    }

    /// reads the current register value and extract field `copsdbg` from it
    pub fn copsdbg_read(&mut self) -> u64 {
        Self::with_reg_val().copsdbg_extract()
    }
    // no insert() method for field copsdbg
    /*
     * Field: copdbg
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn copdbg_extract(&self) -> u64 {
        // bits 0..3
        self.val.get_bits(0..=3)
    }

    /// reads the current register value and extract field `copdbg` from it
    pub fn copdbg_read(&mut self) -> u64 {
        Self::with_reg_val().copdbg_extract()
    }
    // no insert() method for field copdbg
}

impl Default for IdDfr0El1 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> IdDfr0El1 {
        IdDfr0El1(0)
    }
}
