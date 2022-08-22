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
 * Generated on: 2022-08-22T16:25:59.080629
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
 * Register:    Interrupt Controller Interrupt Priority Mask Register (icc_pmr_el1)
 * Group:       Virtualization registers
 * Type:        64-bit Register
 * Description: Provides an interrupt priority filter. Only interrupts with a higher priority than the value in this register are signaled to the PE.
 * File:        AArch64-icc_pmr_el1.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Interrupt Controller Interrupt Priority Mask Register value in memory
pub struct IccPmrEl1(u64);

/// struct implementation for accessing the fields of register icc_pmr_el1
impl IccPmrEl1 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> IccPmrEl1 {
        Self::default()
    }

    /// collects the modifications of IccPmrEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> IccPmrEl1 {
        IccPmrEl1(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  IccPmrEl1 {
        let curval = Self::reg_rawrd() & 0xff;
        IccPmrEl1(curval)
    }


    
    /// reading the Interrupt Controller Interrupt Priority Mask Register (icc_pmr_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ICC_PMR_EL1
            asm!("mrs {}, icc_pmr_el1", out(reg) regval);
        }
        return regval;
    }


    /// writing the Interrupt Controller Interrupt Priority Mask Register (icc_pmr_el1) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR ICC_PMR_EL1, <Xt>
            asm!("msr icc_pmr_el1, {}", in(reg) val);
        }
    }



    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xff;
        self
    }

    
    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }


    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 255;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: priority
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn priority_extract(&self) -> u64 {
        // bits 0..7
        self.0.get_bits(0..=7)
    }

    /// reads the current register value and extract field `priority` from it
    pub fn priority_read(&mut self) -> u64 {
        Self::with_reg_val().priority_extract()
    }

    /// inserts the given value `val` into the field `priority`
    pub fn priority_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..7
        self.0.set_bits(0..=7, val);
        self
    }

    /// reads the register, updates the `priority` field, and writes the updated value
    pub fn priority_write(&mut self, val: u64) {
        Self::with_reg_val().priority_insert(val).write();
    }

}

impl Default for IccPmrEl1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> IccPmrEl1 {
        IccPmrEl1(0)
    }
}
