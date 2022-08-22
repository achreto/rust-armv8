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
 * Generated on: 2022-08-22T15:51:28.520134
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
 * Register:    Interrupt Controller Maintenance Interrupt State Register (ich_misr_el2)
 * Group:       Virtualization registers
 * Type:        64-bit Register
 * Description: Indicates which maintenance interrupts are asserted.
 * File:        AArch64-ich_misr_el2.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Interrupt Controller Maintenance Interrupt State Register value in memory
pub struct IchMisrEl2(u64);

/// struct implementation for accessing the fields of register ich_misr_el2
impl IchMisrEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> IchMisrEl2 {
        Self::default()
    }

    /// collects the modifications of IchMisrEl2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> IchMisrEl2 {
        IchMisrEl2(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> IchMisrEl2 {
        let curval = Self::reg_rawrd() & 0xff;
        IchMisrEl2(curval)
    }

    /// reading the Interrupt Controller Maintenance Interrupt State Register (ich_misr_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ICH_MISR_EL2
            llvm_asm!("mrs $0, ich_misr_el2" : "=r"(regval));
        }
        return regval;
    }

    // register is not writable. not emitting write accessor

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0xff;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 255;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: vgrp1d
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn vgrp1d_extract(&self) -> u64 {
        // bits 7..7
        self.val.get_bits(7..=7)
    }

    /// reads the current register value and extract field `vgrp1d` from it
    pub fn vgrp1d_read(&mut self) -> u64 {
        Self::with_reg_val().vgrp1d_extract()
    }
    // no insert() method for field vgrp1d
    /*
     * Field: vgrp1e
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn vgrp1e_extract(&self) -> u64 {
        // bits 6..6
        self.val.get_bits(6..=6)
    }

    /// reads the current register value and extract field `vgrp1e` from it
    pub fn vgrp1e_read(&mut self) -> u64 {
        Self::with_reg_val().vgrp1e_extract()
    }
    // no insert() method for field vgrp1e
    /*
     * Field: vgrp0d
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn vgrp0d_extract(&self) -> u64 {
        // bits 5..5
        self.val.get_bits(5..=5)
    }

    /// reads the current register value and extract field `vgrp0d` from it
    pub fn vgrp0d_read(&mut self) -> u64 {
        Self::with_reg_val().vgrp0d_extract()
    }
    // no insert() method for field vgrp0d
    /*
     * Field: vgrp0e
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn vgrp0e_extract(&self) -> u64 {
        // bits 4..4
        self.val.get_bits(4..=4)
    }

    /// reads the current register value and extract field `vgrp0e` from it
    pub fn vgrp0e_read(&mut self) -> u64 {
        Self::with_reg_val().vgrp0e_extract()
    }
    // no insert() method for field vgrp0e
    /*
     * Field: np
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn np_extract(&self) -> u64 {
        // bits 3..3
        self.val.get_bits(3..=3)
    }

    /// reads the current register value and extract field `np` from it
    pub fn np_read(&mut self) -> u64 {
        Self::with_reg_val().np_extract()
    }
    // no insert() method for field np
    /*
     * Field: lrenp
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn lrenp_extract(&self) -> u64 {
        // bits 2..2
        self.val.get_bits(2..=2)
    }

    /// reads the current register value and extract field `lrenp` from it
    pub fn lrenp_read(&mut self) -> u64 {
        Self::with_reg_val().lrenp_extract()
    }
    // no insert() method for field lrenp
    /*
     * Field: u
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn u_extract(&self) -> u64 {
        // bits 1..1
        self.val.get_bits(1..=1)
    }

    /// reads the current register value and extract field `u` from it
    pub fn u_read(&mut self) -> u64 {
        Self::with_reg_val().u_extract()
    }
    // no insert() method for field u
    /*
     * Field: eoi
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn eoi_extract(&self) -> u64 {
        // bits 0..0
        self.val.get_bits(0..=0)
    }

    /// reads the current register value and extract field `eoi` from it
    pub fn eoi_read(&mut self) -> u64 {
        Self::with_reg_val().eoi_extract()
    }
    // no insert() method for field eoi
}

impl Default for IchMisrEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> IchMisrEl2 {
        IchMisrEl2(0)
    }
}
