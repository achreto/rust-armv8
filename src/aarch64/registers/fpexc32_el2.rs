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
 * Generated on: 2022-08-22T16:25:59.075962
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
 * Register:    Floating-Point Exception Control register (fpexc32_el2)
 * Group:       Floating-point registers
 * Type:        64-bit Register
 * Description: Allows access to the AArch32 register 
 * File:        AArch64-fpexc32_el2.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Floating-Point Exception Control register value in memory
pub struct Fpexc32El2(u64);

/// struct implementation for accessing the fields of register fpexc32_el2
impl Fpexc32El2 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> Fpexc32El2 {
        Self::default()
    }

    /// collects the modifications of Fpexc32El2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> Fpexc32El2 {
        Fpexc32El2(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  Fpexc32El2 {
        let curval = Self::reg_rawrd() & 0xfc00079f;
        Fpexc32El2(curval)
    }


    
    /// reading the Floating-Point Exception Control register (fpexc32_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, FPEXC32_EL2
            asm!("mrs {}, fpexc32_el2", out(reg) regval);
        }
        return regval;
    }


    /// writing the Floating-Point Exception Control register (fpexc32_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR FPEXC32_EL2, <Xt>
            asm!("msr fpexc32_el2, {}", in(reg) val);
        }
    }



    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xfc00079f;
        self
    }

    
    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }


    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 4227860383;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: ex
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn ex_extract(&self) -> u64 {
        // bits 31..31
        self.0.get_bits(31..=31)
    }

    /// reads the current register value and extract field `ex` from it
    pub fn ex_read(&mut self) -> u64 {
        Self::with_reg_val().ex_extract()
    }

    /// inserts the given value `val` into the field `ex`
    pub fn ex_insert(&mut self, val: u64) -> &mut Self {
        // bits 31..31
        self.0.set_bits(31..=31, val);
        self
    }

    /// reads the register, updates the `ex` field, and writes the updated value
    pub fn ex_write(&mut self, val: u64) {
        Self::with_reg_val().ex_insert(val).write();
    }

    /*
     * Field: en
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn en_extract(&self) -> u64 {
        // bits 30..30
        self.0.get_bits(30..=30)
    }

    /// reads the current register value and extract field `en` from it
    pub fn en_read(&mut self) -> u64 {
        Self::with_reg_val().en_extract()
    }

    /// inserts the given value `val` into the field `en`
    pub fn en_insert(&mut self, val: u64) -> &mut Self {
        // bits 30..30
        self.0.set_bits(30..=30, val);
        self
    }

    /// reads the register, updates the `en` field, and writes the updated value
    pub fn en_write(&mut self, val: u64) {
        Self::with_reg_val().en_insert(val).write();
    }

    /*
     * Field: dex
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn dex_extract(&self) -> u64 {
        // bits 29..29
        self.0.get_bits(29..=29)
    }

    /// reads the current register value and extract field `dex` from it
    pub fn dex_read(&mut self) -> u64 {
        Self::with_reg_val().dex_extract()
    }

    /// inserts the given value `val` into the field `dex`
    pub fn dex_insert(&mut self, val: u64) -> &mut Self {
        // bits 29..29
        self.0.set_bits(29..=29, val);
        self
    }

    /// reads the register, updates the `dex` field, and writes the updated value
    pub fn dex_write(&mut self, val: u64) {
        Self::with_reg_val().dex_insert(val).write();
    }

    /*
     * Field: fp2v
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn fp2v_extract(&self) -> u64 {
        // bits 28..28
        self.0.get_bits(28..=28)
    }

    /// reads the current register value and extract field `fp2v` from it
    pub fn fp2v_read(&mut self) -> u64 {
        Self::with_reg_val().fp2v_extract()
    }

    /// inserts the given value `val` into the field `fp2v`
    pub fn fp2v_insert(&mut self, val: u64) -> &mut Self {
        // bits 28..28
        self.0.set_bits(28..=28, val);
        self
    }

    /// reads the register, updates the `fp2v` field, and writes the updated value
    pub fn fp2v_write(&mut self, val: u64) {
        Self::with_reg_val().fp2v_insert(val).write();
    }

    /*
     * Field: vv
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn vv_extract(&self) -> u64 {
        // bits 27..27
        self.0.get_bits(27..=27)
    }

    /// reads the current register value and extract field `vv` from it
    pub fn vv_read(&mut self) -> u64 {
        Self::with_reg_val().vv_extract()
    }

    /// inserts the given value `val` into the field `vv`
    pub fn vv_insert(&mut self, val: u64) -> &mut Self {
        // bits 27..27
        self.0.set_bits(27..=27, val);
        self
    }

    /// reads the register, updates the `vv` field, and writes the updated value
    pub fn vv_write(&mut self, val: u64) {
        Self::with_reg_val().vv_insert(val).write();
    }

    /*
     * Field: tfv
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn tfv_extract(&self) -> u64 {
        // bits 26..26
        self.0.get_bits(26..=26)
    }

    /// reads the current register value and extract field `tfv` from it
    pub fn tfv_read(&mut self) -> u64 {
        Self::with_reg_val().tfv_extract()
    }

    /// inserts the given value `val` into the field `tfv`
    pub fn tfv_insert(&mut self, val: u64) -> &mut Self {
        // bits 26..26
        self.0.set_bits(26..=26, val);
        self
    }

    /// reads the register, updates the `tfv` field, and writes the updated value
    pub fn tfv_write(&mut self, val: u64) {
        Self::with_reg_val().tfv_insert(val).write();
    }

    /*
     * Field: vecitr
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn vecitr_extract(&self) -> u64 {
        // bits 8..10
        self.0.get_bits(8..=10)
    }

    /// reads the current register value and extract field `vecitr` from it
    pub fn vecitr_read(&mut self) -> u64 {
        Self::with_reg_val().vecitr_extract()
    }

    /// inserts the given value `val` into the field `vecitr`
    pub fn vecitr_insert(&mut self, val: u64) -> &mut Self {
        // bits 8..10
        self.0.set_bits(8..=10, val);
        self
    }

    /// reads the register, updates the `vecitr` field, and writes the updated value
    pub fn vecitr_write(&mut self, val: u64) {
        Self::with_reg_val().vecitr_insert(val).write();
    }

    /*
     * Field: idf
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn idf_extract(&self) -> u64 {
        // bits 7..7
        self.0.get_bits(7..=7)
    }

    /// reads the current register value and extract field `idf` from it
    pub fn idf_read(&mut self) -> u64 {
        Self::with_reg_val().idf_extract()
    }

    /// inserts the given value `val` into the field `idf`
    pub fn idf_insert(&mut self, val: u64) -> &mut Self {
        // bits 7..7
        self.0.set_bits(7..=7, val);
        self
    }

    /// reads the register, updates the `idf` field, and writes the updated value
    pub fn idf_write(&mut self, val: u64) {
        Self::with_reg_val().idf_insert(val).write();
    }

    /*
     * Field: ixf
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn ixf_extract(&self) -> u64 {
        // bits 4..4
        self.0.get_bits(4..=4)
    }

    /// reads the current register value and extract field `ixf` from it
    pub fn ixf_read(&mut self) -> u64 {
        Self::with_reg_val().ixf_extract()
    }

    /// inserts the given value `val` into the field `ixf`
    pub fn ixf_insert(&mut self, val: u64) -> &mut Self {
        // bits 4..4
        self.0.set_bits(4..=4, val);
        self
    }

    /// reads the register, updates the `ixf` field, and writes the updated value
    pub fn ixf_write(&mut self, val: u64) {
        Self::with_reg_val().ixf_insert(val).write();
    }

    /*
     * Field: uff
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn uff_extract(&self) -> u64 {
        // bits 3..3
        self.0.get_bits(3..=3)
    }

    /// reads the current register value and extract field `uff` from it
    pub fn uff_read(&mut self) -> u64 {
        Self::with_reg_val().uff_extract()
    }

    /// inserts the given value `val` into the field `uff`
    pub fn uff_insert(&mut self, val: u64) -> &mut Self {
        // bits 3..3
        self.0.set_bits(3..=3, val);
        self
    }

    /// reads the register, updates the `uff` field, and writes the updated value
    pub fn uff_write(&mut self, val: u64) {
        Self::with_reg_val().uff_insert(val).write();
    }

    /*
     * Field: off
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn off_extract(&self) -> u64 {
        // bits 2..2
        self.0.get_bits(2..=2)
    }

    /// reads the current register value and extract field `off` from it
    pub fn off_read(&mut self) -> u64 {
        Self::with_reg_val().off_extract()
    }

    /// inserts the given value `val` into the field `off`
    pub fn off_insert(&mut self, val: u64) -> &mut Self {
        // bits 2..2
        self.0.set_bits(2..=2, val);
        self
    }

    /// reads the register, updates the `off` field, and writes the updated value
    pub fn off_write(&mut self, val: u64) {
        Self::with_reg_val().off_insert(val).write();
    }

    /*
     * Field: dzf
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn dzf_extract(&self) -> u64 {
        // bits 1..1
        self.0.get_bits(1..=1)
    }

    /// reads the current register value and extract field `dzf` from it
    pub fn dzf_read(&mut self) -> u64 {
        Self::with_reg_val().dzf_extract()
    }

    /// inserts the given value `val` into the field `dzf`
    pub fn dzf_insert(&mut self, val: u64) -> &mut Self {
        // bits 1..1
        self.0.set_bits(1..=1, val);
        self
    }

    /// reads the register, updates the `dzf` field, and writes the updated value
    pub fn dzf_write(&mut self, val: u64) {
        Self::with_reg_val().dzf_insert(val).write();
    }

    /*
     * Field: iof
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn iof_extract(&self) -> u64 {
        // bits 0..0
        self.0.get_bits(0..=0)
    }

    /// reads the current register value and extract field `iof` from it
    pub fn iof_read(&mut self) -> u64 {
        Self::with_reg_val().iof_extract()
    }

    /// inserts the given value `val` into the field `iof`
    pub fn iof_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..0
        self.0.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `iof` field, and writes the updated value
    pub fn iof_write(&mut self, val: u64) {
        Self::with_reg_val().iof_insert(val).write();
    }

}

impl Default for Fpexc32El2 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> Fpexc32El2 {
        Fpexc32El2(0)
    }
}
