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
 * Generated on: 2022-08-22T16:25:59.094384
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
 * Register:    Performance Monitors User Enable Register (pmuserenr_el0)
 * Group:       Performance Monitors registers
 * Type:        64-bit Register
 * Description: Enables or disables EL0 access to the Performance Monitors.
 * File:        AArch64-pmuserenr_el0.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Performance Monitors User Enable Register value in memory
pub struct PmuserenrEl0(u64);

/// struct implementation for accessing the fields of register pmuserenr_el0
impl PmuserenrEl0 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> PmuserenrEl0 {
        Self::default()
    }

    /// collects the modifications of PmuserenrEl0 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> PmuserenrEl0 {
        PmuserenrEl0(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  PmuserenrEl0 {
        let curval = Self::reg_rawrd() & 0xf;
        PmuserenrEl0(curval)
    }


    
    /// reading the Performance Monitors User Enable Register (pmuserenr_el0) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, PMUSERENR_EL0
            asm!("mrs {}, pmuserenr_el0", out(reg) regval);
        }
        return regval;
    }


    /// writing the Performance Monitors User Enable Register (pmuserenr_el0) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR PMUSERENR_EL0, <Xt>
            asm!("msr pmuserenr_el0, {}", in(reg) val);
        }
    }



    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xf;
        self
    }

    
    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }


    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 15;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: er
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn er_extract(&self) -> u64 {
        // bits 3..3
        self.0.get_bits(3..=3)
    }

    /// reads the current register value and extract field `er` from it
    pub fn er_read(&mut self) -> u64 {
        Self::with_reg_val().er_extract()
    }

    /// inserts the given value `val` into the field `er`
    pub fn er_insert(&mut self, val: u64) -> &mut Self {
        // bits 3..3
        self.0.set_bits(3..=3, val);
        self
    }

    /// reads the register, updates the `er` field, and writes the updated value
    pub fn er_write(&mut self, val: u64) {
        Self::with_reg_val().er_insert(val).write();
    }

    /*
     * Field: cr
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn cr_extract(&self) -> u64 {
        // bits 2..2
        self.0.get_bits(2..=2)
    }

    /// reads the current register value and extract field `cr` from it
    pub fn cr_read(&mut self) -> u64 {
        Self::with_reg_val().cr_extract()
    }

    /// inserts the given value `val` into the field `cr`
    pub fn cr_insert(&mut self, val: u64) -> &mut Self {
        // bits 2..2
        self.0.set_bits(2..=2, val);
        self
    }

    /// reads the register, updates the `cr` field, and writes the updated value
    pub fn cr_write(&mut self, val: u64) {
        Self::with_reg_val().cr_insert(val).write();
    }

    /*
     * Field: sw
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn sw_extract(&self) -> u64 {
        // bits 1..1
        self.0.get_bits(1..=1)
    }

    /// reads the current register value and extract field `sw` from it
    pub fn sw_read(&mut self) -> u64 {
        Self::with_reg_val().sw_extract()
    }

    /// inserts the given value `val` into the field `sw`
    pub fn sw_insert(&mut self, val: u64) -> &mut Self {
        // bits 1..1
        self.0.set_bits(1..=1, val);
        self
    }

    /// reads the register, updates the `sw` field, and writes the updated value
    pub fn sw_write(&mut self, val: u64) {
        Self::with_reg_val().sw_insert(val).write();
    }

    /*
     * Field: en
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn en_extract(&self) -> u64 {
        // bits 0..0
        self.0.get_bits(0..=0)
    }

    /// reads the current register value and extract field `en` from it
    pub fn en_read(&mut self) -> u64 {
        Self::with_reg_val().en_extract()
    }

    /// inserts the given value `val` into the field `en`
    pub fn en_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..0
        self.0.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `en` field, and writes the updated value
    pub fn en_write(&mut self, val: u64) {
        Self::with_reg_val().en_insert(val).write();
    }

}

impl Default for PmuserenrEl0 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> PmuserenrEl0 {
        PmuserenrEl0(0)
    }
}
