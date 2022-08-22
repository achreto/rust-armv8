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
 * Generated on: 2022-08-22T16:25:59.070201
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
 * Register:    Counter-timer Virtual Timer TimerValue Register (EL2) (cnthv_tval_el2)
 * Group:       Generic Timer registers
 * Type:        64-bit Register
 * Description: Holds the timer value for the EL2 virtual timer.
 * File:        AArch64-cnthv_tval_el2.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Counter-timer Virtual Timer TimerValue Register (EL2) value in memory
pub struct CnthvTvalEl2(u64);

/// struct implementation for accessing the fields of register cnthv_tval_el2
impl CnthvTvalEl2 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> CnthvTvalEl2 {
        Self::default()
    }

    /// collects the modifications of CnthvTvalEl2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> CnthvTvalEl2 {
        CnthvTvalEl2(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  CnthvTvalEl2 {
        let curval = Self::reg_rawrd() & 0xffffffff;
        CnthvTvalEl2(curval)
    }


    
    /// reading the Counter-timer Virtual Timer TimerValue Register (EL2) (cnthv_tval_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, CNTHV_TVAL_EL2
            asm!("mrs {}, S3_4_C14_C3_0", out(reg) regval);
        }
        return regval;
    }


    /// writing the Counter-timer Virtual Timer TimerValue Register (EL2) (cnthv_tval_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR CNTHV_TVAL_EL2, <Xt>
            asm!("msr S3_4_C14_C3_0, {}", in(reg) val);
        }
    }



    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xffffffff;
        self
    }

    
    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }


    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 4294967295;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: timervalue
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn timervalue_extract(&self) -> u64 {
        // bits 0..31
        self.0.get_bits(0..=31)
    }

    /// reads the current register value and extract field `timervalue` from it
    pub fn timervalue_read(&mut self) -> u64 {
        Self::with_reg_val().timervalue_extract()
    }

    /// inserts the given value `val` into the field `timervalue`
    pub fn timervalue_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..31
        self.0.set_bits(0..=31, val);
        self
    }

    /// reads the register, updates the `timervalue` field, and writes the updated value
    pub fn timervalue_write(&mut self, val: u64) {
        Self::with_reg_val().timervalue_insert(val).write();
    }

}

impl Default for CnthvTvalEl2 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> CnthvTvalEl2 {
        CnthvTvalEl2(0)
    }
}
