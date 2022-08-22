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
 * Generated on: 2022-08-22T16:25:59.073109
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
 * Register:    Debug Data Transfer Register, Receive (dbgdtrrx_el0)
 * Group:       Debug registers
 * Type:        64-bit Register
 * Description: Transfers data from an external debugger to the PE. For example, it is used by a debugger transferring commands and data to a debug target. See 
 * File:        AArch64-dbgdtrrx_el0.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Debug Data Transfer Register, Receive value in memory
pub struct DbgdtrrxEl0(u64);

/// struct implementation for accessing the fields of register dbgdtrrx_el0
impl DbgdtrrxEl0 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> DbgdtrrxEl0 {
        Self::default()
    }

    /// collects the modifications of DbgdtrrxEl0 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> DbgdtrrxEl0 {
        DbgdtrrxEl0(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  DbgdtrrxEl0 {
        let curval = Self::reg_rawrd() & 0xffffffff;
        DbgdtrrxEl0(curval)
    }


    
    /// reading the Debug Data Transfer Register, Receive (dbgdtrrx_el0) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, DBGDTRRX_EL0
            asm!("mrs {}, dbgdtrrx_el0", out(reg) regval);
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
     * Field: none_31_0
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn none_31_0_extract(&self) -> u64 {
        // bits 0..31
        self.0.get_bits(0..=31)
    }

    /// reads the current register value and extract field `none_31_0` from it
    pub fn none_31_0_read(&mut self) -> u64 {
        Self::with_reg_val().none_31_0_extract()
    }
// no insert() method for field none_31_0
}

impl Default for DbgdtrrxEl0 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> DbgdtrrxEl0 {
        DbgdtrrxEl0(0)
    }
}
