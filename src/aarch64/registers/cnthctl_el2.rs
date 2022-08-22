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
 * Generated on: 2022-08-22T16:25:59.069382
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
 * Register:    Counter-timer Hypervisor Control register (cnthctl_el2)
 * Group:       Virtualization registers
 * Type:        64-bit Register
 * Description: Controls the generation of an event stream from the physical counter, and access from EL1 to the physical counter and the EL1 physical timer.
 * File:        AArch64-cnthctl_el2.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Counter-timer Hypervisor Control register value in memory
pub struct CnthctlEl2(u64);

/// struct implementation for accessing the fields of register cnthctl_el2
impl CnthctlEl2 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> CnthctlEl2 {
        Self::default()
    }

    /// collects the modifications of CnthctlEl2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> CnthctlEl2 {
        CnthctlEl2(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  CnthctlEl2 {
        let curval = Self::reg_rawrd() & 0x3ffff;
        CnthctlEl2(curval)
    }


    
    /// reading the Counter-timer Hypervisor Control register (cnthctl_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, CNTHCTL_EL2
            asm!("mrs {}, cnthctl_el2", out(reg) regval);
        }
        return regval;
    }


    /// writing the Counter-timer Hypervisor Control register (cnthctl_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR CNTHCTL_EL2, <Xt>
            asm!("msr cnthctl_el2, {}", in(reg) val);
        }
    }



    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0x3ffff;
        self
    }

    
    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }


    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 262143;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: evntis_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn evntis_1_extract(&self) -> u64 {
        // bits 17..17
        self.0.get_bits(17..=17)
    }

    /// reads the current register value and extract field `evntis_1` from it
    pub fn evntis_1_read(&mut self) -> u64 {
        Self::with_reg_val().evntis_1_extract()
    }

    /// inserts the given value `val` into the field `evntis_1`
    pub fn evntis_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 17..17
        self.0.set_bits(17..=17, val);
        self
    }

    /// reads the register, updates the `evntis_1` field, and writes the updated value
    pub fn evntis_1_write(&mut self, val: u64) {
        Self::with_reg_val().evntis_1_insert(val).write();
    }

    /*
     * Field: el1nvvct_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn el1nvvct_1_extract(&self) -> u64 {
        // bits 16..16
        self.0.get_bits(16..=16)
    }

    /// reads the current register value and extract field `el1nvvct_1` from it
    pub fn el1nvvct_1_read(&mut self) -> u64 {
        Self::with_reg_val().el1nvvct_1_extract()
    }

    /// inserts the given value `val` into the field `el1nvvct_1`
    pub fn el1nvvct_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 16..16
        self.0.set_bits(16..=16, val);
        self
    }

    /// reads the register, updates the `el1nvvct_1` field, and writes the updated value
    pub fn el1nvvct_1_write(&mut self, val: u64) {
        Self::with_reg_val().el1nvvct_1_insert(val).write();
    }

    /*
     * Field: el1nvpct_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn el1nvpct_1_extract(&self) -> u64 {
        // bits 15..15
        self.0.get_bits(15..=15)
    }

    /// reads the current register value and extract field `el1nvpct_1` from it
    pub fn el1nvpct_1_read(&mut self) -> u64 {
        Self::with_reg_val().el1nvpct_1_extract()
    }

    /// inserts the given value `val` into the field `el1nvpct_1`
    pub fn el1nvpct_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 15..15
        self.0.set_bits(15..=15, val);
        self
    }

    /// reads the register, updates the `el1nvpct_1` field, and writes the updated value
    pub fn el1nvpct_1_write(&mut self, val: u64) {
        Self::with_reg_val().el1nvpct_1_insert(val).write();
    }

    /*
     * Field: el1tvct_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn el1tvct_1_extract(&self) -> u64 {
        // bits 14..14
        self.0.get_bits(14..=14)
    }

    /// reads the current register value and extract field `el1tvct_1` from it
    pub fn el1tvct_1_read(&mut self) -> u64 {
        Self::with_reg_val().el1tvct_1_extract()
    }

    /// inserts the given value `val` into the field `el1tvct_1`
    pub fn el1tvct_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 14..14
        self.0.set_bits(14..=14, val);
        self
    }

    /// reads the register, updates the `el1tvct_1` field, and writes the updated value
    pub fn el1tvct_1_write(&mut self, val: u64) {
        Self::with_reg_val().el1tvct_1_insert(val).write();
    }

    /*
     * Field: el1tvt_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn el1tvt_1_extract(&self) -> u64 {
        // bits 13..13
        self.0.get_bits(13..=13)
    }

    /// reads the current register value and extract field `el1tvt_1` from it
    pub fn el1tvt_1_read(&mut self) -> u64 {
        Self::with_reg_val().el1tvt_1_extract()
    }

    /// inserts the given value `val` into the field `el1tvt_1`
    pub fn el1tvt_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 13..13
        self.0.set_bits(13..=13, val);
        self
    }

    /// reads the register, updates the `el1tvt_1` field, and writes the updated value
    pub fn el1tvt_1_write(&mut self, val: u64) {
        Self::with_reg_val().el1tvt_1_insert(val).write();
    }

    /*
     * Field: ecv_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn ecv_1_extract(&self) -> u64 {
        // bits 12..12
        self.0.get_bits(12..=12)
    }

    /// reads the current register value and extract field `ecv_1` from it
    pub fn ecv_1_read(&mut self) -> u64 {
        Self::with_reg_val().ecv_1_extract()
    }

    /// inserts the given value `val` into the field `ecv_1`
    pub fn ecv_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 12..12
        self.0.set_bits(12..=12, val);
        self
    }

    /// reads the register, updates the `ecv_1` field, and writes the updated value
    pub fn ecv_1_write(&mut self, val: u64) {
        Self::with_reg_val().ecv_1_insert(val).write();
    }

    /*
     * Field: el1pten
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn el1pten_extract(&self) -> u64 {
        // bits 11..11
        self.0.get_bits(11..=11)
    }

    /// reads the current register value and extract field `el1pten` from it
    pub fn el1pten_read(&mut self) -> u64 {
        Self::with_reg_val().el1pten_extract()
    }

    /// inserts the given value `val` into the field `el1pten`
    pub fn el1pten_insert(&mut self, val: u64) -> &mut Self {
        // bits 11..11
        self.0.set_bits(11..=11, val);
        self
    }

    /// reads the register, updates the `el1pten` field, and writes the updated value
    pub fn el1pten_write(&mut self, val: u64) {
        Self::with_reg_val().el1pten_insert(val).write();
    }

    /*
     * Field: el1pcten
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn el1pcten_extract(&self) -> u64 {
        // bits 10..10
        self.0.get_bits(10..=10)
    }

    /// reads the current register value and extract field `el1pcten` from it
    pub fn el1pcten_read(&mut self) -> u64 {
        Self::with_reg_val().el1pcten_extract()
    }

    /// inserts the given value `val` into the field `el1pcten`
    pub fn el1pcten_insert(&mut self, val: u64) -> &mut Self {
        // bits 10..10
        self.0.set_bits(10..=10, val);
        self
    }

    /// reads the register, updates the `el1pcten` field, and writes the updated value
    pub fn el1pcten_write(&mut self, val: u64) {
        Self::with_reg_val().el1pcten_insert(val).write();
    }

    /*
     * Field: el0pten
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn el0pten_extract(&self) -> u64 {
        // bits 9..9
        self.0.get_bits(9..=9)
    }

    /// reads the current register value and extract field `el0pten` from it
    pub fn el0pten_read(&mut self) -> u64 {
        Self::with_reg_val().el0pten_extract()
    }

    /// inserts the given value `val` into the field `el0pten`
    pub fn el0pten_insert(&mut self, val: u64) -> &mut Self {
        // bits 9..9
        self.0.set_bits(9..=9, val);
        self
    }

    /// reads the register, updates the `el0pten` field, and writes the updated value
    pub fn el0pten_write(&mut self, val: u64) {
        Self::with_reg_val().el0pten_insert(val).write();
    }

    /*
     * Field: el0vten
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn el0vten_extract(&self) -> u64 {
        // bits 8..8
        self.0.get_bits(8..=8)
    }

    /// reads the current register value and extract field `el0vten` from it
    pub fn el0vten_read(&mut self) -> u64 {
        Self::with_reg_val().el0vten_extract()
    }

    /// inserts the given value `val` into the field `el0vten`
    pub fn el0vten_insert(&mut self, val: u64) -> &mut Self {
        // bits 8..8
        self.0.set_bits(8..=8, val);
        self
    }

    /// reads the register, updates the `el0vten` field, and writes the updated value
    pub fn el0vten_write(&mut self, val: u64) {
        Self::with_reg_val().el0vten_insert(val).write();
    }

    /*
     * Field: evnti
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn evnti_extract(&self) -> u64 {
        // bits 4..7
        self.0.get_bits(4..=7)
    }

    /// reads the current register value and extract field `evnti` from it
    pub fn evnti_read(&mut self) -> u64 {
        Self::with_reg_val().evnti_extract()
    }

    /// inserts the given value `val` into the field `evnti`
    pub fn evnti_insert(&mut self, val: u64) -> &mut Self {
        // bits 4..7
        self.0.set_bits(4..=7, val);
        self
    }

    /// reads the register, updates the `evnti` field, and writes the updated value
    pub fn evnti_write(&mut self, val: u64) {
        Self::with_reg_val().evnti_insert(val).write();
    }

    /*
     * Field: evntdir
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn evntdir_extract(&self) -> u64 {
        // bits 3..3
        self.0.get_bits(3..=3)
    }

    /// reads the current register value and extract field `evntdir` from it
    pub fn evntdir_read(&mut self) -> u64 {
        Self::with_reg_val().evntdir_extract()
    }

    /// inserts the given value `val` into the field `evntdir`
    pub fn evntdir_insert(&mut self, val: u64) -> &mut Self {
        // bits 3..3
        self.0.set_bits(3..=3, val);
        self
    }

    /// reads the register, updates the `evntdir` field, and writes the updated value
    pub fn evntdir_write(&mut self, val: u64) {
        Self::with_reg_val().evntdir_insert(val).write();
    }

    /*
     * Field: evnten
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn evnten_extract(&self) -> u64 {
        // bits 2..2
        self.0.get_bits(2..=2)
    }

    /// reads the current register value and extract field `evnten` from it
    pub fn evnten_read(&mut self) -> u64 {
        Self::with_reg_val().evnten_extract()
    }

    /// inserts the given value `val` into the field `evnten`
    pub fn evnten_insert(&mut self, val: u64) -> &mut Self {
        // bits 2..2
        self.0.set_bits(2..=2, val);
        self
    }

    /// reads the register, updates the `evnten` field, and writes the updated value
    pub fn evnten_write(&mut self, val: u64) {
        Self::with_reg_val().evnten_insert(val).write();
    }

    /*
     * Field: el0vcten
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn el0vcten_extract(&self) -> u64 {
        // bits 1..1
        self.0.get_bits(1..=1)
    }

    /// reads the current register value and extract field `el0vcten` from it
    pub fn el0vcten_read(&mut self) -> u64 {
        Self::with_reg_val().el0vcten_extract()
    }

    /// inserts the given value `val` into the field `el0vcten`
    pub fn el0vcten_insert(&mut self, val: u64) -> &mut Self {
        // bits 1..1
        self.0.set_bits(1..=1, val);
        self
    }

    /// reads the register, updates the `el0vcten` field, and writes the updated value
    pub fn el0vcten_write(&mut self, val: u64) {
        Self::with_reg_val().el0vcten_insert(val).write();
    }

    /*
     * Field: el0pcten
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn el0pcten_extract(&self) -> u64 {
        // bits 0..0
        self.0.get_bits(0..=0)
    }

    /// reads the current register value and extract field `el0pcten` from it
    pub fn el0pcten_read(&mut self) -> u64 {
        Self::with_reg_val().el0pcten_extract()
    }

    /// inserts the given value `val` into the field `el0pcten`
    pub fn el0pcten_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..0
        self.0.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `el0pcten` field, and writes the updated value
    pub fn el0pcten_write(&mut self, val: u64) {
        Self::with_reg_val().el0pcten_insert(val).write();
    }

}

impl Default for CnthctlEl2 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> CnthctlEl2 {
        CnthctlEl2(0)
    }
}
