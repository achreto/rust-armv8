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
 * Generated on: 2022-08-22T16:25:59.091262
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
 * Register:    Physical Address Register (par_el1)
 * Group:       Address translation instructions
 * Type:        64-bit Register
 * Description: Returns the output address (OA) from an Address translation instruction that executed successfully, or fault information if the instruction did not execute successfully.
 * File:        AArch64-par_el1.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Physical Address Register value in memory
pub struct ParEl1(u64);

/// struct implementation for accessing the fields of register par_el1
impl ParEl1 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> ParEl1 {
        Self::default()
    }

    /// collects the modifications of ParEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> ParEl1 {
        ParEl1(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  ParEl1 {
        let curval = Self::reg_rawrd() & 0xffff00000000037f;
        ParEl1(curval)
    }


    
    /// reading the Physical Address Register (par_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, PAR_EL1
            asm!("mrs {}, par_el1", out(reg) regval);
        }
        return regval;
    }


    /// writing the Physical Address Register (par_el1) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR PAR_EL1, <Xt>
            asm!("msr par_el1, {}", in(reg) val);
        }
    }



    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xffff00000000037f;
        self
    }

    
    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }


    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 18446462598732841855;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: implementation_defined_63_56
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn implementation_defined_63_56_extract(&self) -> u64 {
        // bits 56..63
        self.0.get_bits(56..=63)
    }

    /// reads the current register value and extract field `implementation_defined_63_56` from it
    pub fn implementation_defined_63_56_read(&mut self) -> u64 {
        Self::with_reg_val().implementation_defined_63_56_extract()
    }

    /// inserts the given value `val` into the field `implementation_defined_63_56`
    pub fn implementation_defined_63_56_insert(&mut self, val: u64) -> &mut Self {
        // bits 56..63
        self.0.set_bits(56..=63, val);
        self
    }

    /// reads the register, updates the `implementation_defined_63_56` field, and writes the updated value
    pub fn implementation_defined_63_56_write(&mut self, val: u64) {
        Self::with_reg_val().implementation_defined_63_56_insert(val).write();
    }

    /*
     * Field: implementation_defined_55_52
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn implementation_defined_55_52_extract(&self) -> u64 {
        // bits 52..55
        self.0.get_bits(52..=55)
    }

    /// reads the current register value and extract field `implementation_defined_55_52` from it
    pub fn implementation_defined_55_52_read(&mut self) -> u64 {
        Self::with_reg_val().implementation_defined_55_52_extract()
    }

    /// inserts the given value `val` into the field `implementation_defined_55_52`
    pub fn implementation_defined_55_52_insert(&mut self, val: u64) -> &mut Self {
        // bits 52..55
        self.0.set_bits(52..=55, val);
        self
    }

    /// reads the register, updates the `implementation_defined_55_52` field, and writes the updated value
    pub fn implementation_defined_55_52_write(&mut self, val: u64) {
        Self::with_reg_val().implementation_defined_55_52_insert(val).write();
    }

    /*
     * Field: implementation_defined_51_48
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn implementation_defined_51_48_extract(&self) -> u64 {
        // bits 48..51
        self.0.get_bits(48..=51)
    }

    /// reads the current register value and extract field `implementation_defined_51_48` from it
    pub fn implementation_defined_51_48_read(&mut self) -> u64 {
        Self::with_reg_val().implementation_defined_51_48_extract()
    }

    /// inserts the given value `val` into the field `implementation_defined_51_48`
    pub fn implementation_defined_51_48_insert(&mut self, val: u64) -> &mut Self {
        // bits 48..51
        self.0.set_bits(48..=51, val);
        self
    }

    /// reads the register, updates the `implementation_defined_51_48` field, and writes the updated value
    pub fn implementation_defined_51_48_write(&mut self, val: u64) {
        Self::with_reg_val().implementation_defined_51_48_insert(val).write();
    }

    /*
     * Field: s
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn s_extract(&self) -> u64 {
        // bits 9..9
        self.0.get_bits(9..=9)
    }

    /// reads the current register value and extract field `s` from it
    pub fn s_read(&mut self) -> u64 {
        Self::with_reg_val().s_extract()
    }

    /// inserts the given value `val` into the field `s`
    pub fn s_insert(&mut self, val: u64) -> &mut Self {
        // bits 9..9
        self.0.set_bits(9..=9, val);
        self
    }

    /// reads the register, updates the `s` field, and writes the updated value
    pub fn s_write(&mut self, val: u64) {
        Self::with_reg_val().s_insert(val).write();
    }

    /*
     * Field: ptw
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn ptw_extract(&self) -> u64 {
        // bits 8..8
        self.0.get_bits(8..=8)
    }

    /// reads the current register value and extract field `ptw` from it
    pub fn ptw_read(&mut self) -> u64 {
        Self::with_reg_val().ptw_extract()
    }

    /// inserts the given value `val` into the field `ptw`
    pub fn ptw_insert(&mut self, val: u64) -> &mut Self {
        // bits 8..8
        self.0.set_bits(8..=8, val);
        self
    }

    /// reads the register, updates the `ptw` field, and writes the updated value
    pub fn ptw_write(&mut self, val: u64) {
        Self::with_reg_val().ptw_insert(val).write();
    }

    /*
     * Field: fst
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn fst_extract(&self) -> u64 {
        // bits 1..6
        self.0.get_bits(1..=6)
    }

    /// reads the current register value and extract field `fst` from it
    pub fn fst_read(&mut self) -> u64 {
        Self::with_reg_val().fst_extract()
    }

    /// inserts the given value `val` into the field `fst`
    pub fn fst_insert(&mut self, val: u64) -> &mut Self {
        // bits 1..6
        self.0.set_bits(1..=6, val);
        self
    }

    /// reads the register, updates the `fst` field, and writes the updated value
    pub fn fst_write(&mut self, val: u64) {
        Self::with_reg_val().fst_insert(val).write();
    }

    /*
     * Field: f
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn f_extract(&self) -> u64 {
        // bits 0..0
        self.0.get_bits(0..=0)
    }

    /// reads the current register value and extract field `f` from it
    pub fn f_read(&mut self) -> u64 {
        Self::with_reg_val().f_extract()
    }

    /// inserts the given value `val` into the field `f`
    pub fn f_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..0
        self.0.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `f` field, and writes the updated value
    pub fn f_write(&mut self, val: u64) {
        Self::with_reg_val().f_insert(val).write();
    }

}

impl Default for ParEl1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> ParEl1 {
        ParEl1(0)
    }
}
