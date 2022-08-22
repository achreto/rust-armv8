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
 * Generated on: 2022-08-22T16:35:53.063156
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
 * Register:    Interrupt Controller Hyp Control Register (ich_hcr_el2)
 * Group:       Virtualization registers
 * Type:        64-bit Register
 * Description: Controls the environment for VMs.
 * File:        AArch64-ich_hcr_el2.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Interrupt Controller Hyp Control Register value in memory
pub struct IchHcrEl2(u64);

/// struct implementation for accessing the fields of register ich_hcr_el2
impl IchHcrEl2 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> IchHcrEl2 {
        Self::default()
    }

    /// collects the modifications of IchHcrEl2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> IchHcrEl2 {
        IchHcrEl2(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  IchHcrEl2 {
        let curval = Self::reg_rawrd() & 0xf800fdff;
        IchHcrEl2(curval)
    }


    
    /// reading the Interrupt Controller Hyp Control Register (ich_hcr_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ICH_HCR_EL2
            asm!("mrs {}, ich_hcr_el2", out(reg) regval);
        }
        return regval;
    }


    /// writing the Interrupt Controller Hyp Control Register (ich_hcr_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR ICH_HCR_EL2, <Xt>
            asm!("msr ich_hcr_el2, {}", in(reg) val);
        }
    }



    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xf800fdff;
        self
    }

    
    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }


    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 4160814591;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: eoicount
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn eoicount_extract(&self) -> u64 {
        // bits 27..31
        self.0.get_bits(27..=31)
    }

    /// reads the current register value and extract field `eoicount` from it
    pub fn eoicount_read() -> u64 {
        Self::with_reg_val().eoicount_extract()
    }

    /// inserts the given value `val` into the field `eoicount`
    pub fn eoicount_insert(&mut self, val: u64) -> &mut Self {
        // bits 27..31
        self.0.set_bits(27..=31, val);
        self
    }

    /// reads the register, updates the `eoicount` field, and writes the updated value
    pub fn eoicount_write(val: u64) {
        Self::with_reg_val().eoicount_insert(val).write();
    }

    /*
     * Field: dvim_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn dvim_1_extract(&self) -> u64 {
        // bits 15..15
        self.0.get_bits(15..=15)
    }

    /// reads the current register value and extract field `dvim_1` from it
    pub fn dvim_1_read() -> u64 {
        Self::with_reg_val().dvim_1_extract()
    }

    /// inserts the given value `val` into the field `dvim_1`
    pub fn dvim_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 15..15
        self.0.set_bits(15..=15, val);
        self
    }

    /// reads the register, updates the `dvim_1` field, and writes the updated value
    pub fn dvim_1_write(val: u64) {
        Self::with_reg_val().dvim_1_insert(val).write();
    }

    /*
     * Field: tdir
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn tdir_extract(&self) -> u64 {
        // bits 14..14
        self.0.get_bits(14..=14)
    }

    /// reads the current register value and extract field `tdir` from it
    pub fn tdir_read() -> u64 {
        Self::with_reg_val().tdir_extract()
    }

    /// inserts the given value `val` into the field `tdir`
    pub fn tdir_insert(&mut self, val: u64) -> &mut Self {
        // bits 14..14
        self.0.set_bits(14..=14, val);
        self
    }

    /// reads the register, updates the `tdir` field, and writes the updated value
    pub fn tdir_write(val: u64) {
        Self::with_reg_val().tdir_insert(val).write();
    }

    /*
     * Field: tsei
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn tsei_extract(&self) -> u64 {
        // bits 13..13
        self.0.get_bits(13..=13)
    }

    /// reads the current register value and extract field `tsei` from it
    pub fn tsei_read() -> u64 {
        Self::with_reg_val().tsei_extract()
    }

    /// inserts the given value `val` into the field `tsei`
    pub fn tsei_insert(&mut self, val: u64) -> &mut Self {
        // bits 13..13
        self.0.set_bits(13..=13, val);
        self
    }

    /// reads the register, updates the `tsei` field, and writes the updated value
    pub fn tsei_write(val: u64) {
        Self::with_reg_val().tsei_insert(val).write();
    }

    /*
     * Field: tall1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn tall1_extract(&self) -> u64 {
        // bits 12..12
        self.0.get_bits(12..=12)
    }

    /// reads the current register value and extract field `tall1` from it
    pub fn tall1_read() -> u64 {
        Self::with_reg_val().tall1_extract()
    }

    /// inserts the given value `val` into the field `tall1`
    pub fn tall1_insert(&mut self, val: u64) -> &mut Self {
        // bits 12..12
        self.0.set_bits(12..=12, val);
        self
    }

    /// reads the register, updates the `tall1` field, and writes the updated value
    pub fn tall1_write(val: u64) {
        Self::with_reg_val().tall1_insert(val).write();
    }

    /*
     * Field: tall0
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn tall0_extract(&self) -> u64 {
        // bits 11..11
        self.0.get_bits(11..=11)
    }

    /// reads the current register value and extract field `tall0` from it
    pub fn tall0_read() -> u64 {
        Self::with_reg_val().tall0_extract()
    }

    /// inserts the given value `val` into the field `tall0`
    pub fn tall0_insert(&mut self, val: u64) -> &mut Self {
        // bits 11..11
        self.0.set_bits(11..=11, val);
        self
    }

    /// reads the register, updates the `tall0` field, and writes the updated value
    pub fn tall0_write(val: u64) {
        Self::with_reg_val().tall0_insert(val).write();
    }

    /*
     * Field: tc
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn tc_extract(&self) -> u64 {
        // bits 10..10
        self.0.get_bits(10..=10)
    }

    /// reads the current register value and extract field `tc` from it
    pub fn tc_read() -> u64 {
        Self::with_reg_val().tc_extract()
    }

    /// inserts the given value `val` into the field `tc`
    pub fn tc_insert(&mut self, val: u64) -> &mut Self {
        // bits 10..10
        self.0.set_bits(10..=10, val);
        self
    }

    /// reads the register, updates the `tc` field, and writes the updated value
    pub fn tc_write(val: u64) {
        Self::with_reg_val().tc_insert(val).write();
    }

    /*
     * Field: vsgieoicount_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn vsgieoicount_1_extract(&self) -> u64 {
        // bits 8..8
        self.0.get_bits(8..=8)
    }

    /// reads the current register value and extract field `vsgieoicount_1` from it
    pub fn vsgieoicount_1_read() -> u64 {
        Self::with_reg_val().vsgieoicount_1_extract()
    }

    /// inserts the given value `val` into the field `vsgieoicount_1`
    pub fn vsgieoicount_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 8..8
        self.0.set_bits(8..=8, val);
        self
    }

    /// reads the register, updates the `vsgieoicount_1` field, and writes the updated value
    pub fn vsgieoicount_1_write(val: u64) {
        Self::with_reg_val().vsgieoicount_1_insert(val).write();
    }

    /*
     * Field: vgrp1die
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn vgrp1die_extract(&self) -> u64 {
        // bits 7..7
        self.0.get_bits(7..=7)
    }

    /// reads the current register value and extract field `vgrp1die` from it
    pub fn vgrp1die_read() -> u64 {
        Self::with_reg_val().vgrp1die_extract()
    }

    /// inserts the given value `val` into the field `vgrp1die`
    pub fn vgrp1die_insert(&mut self, val: u64) -> &mut Self {
        // bits 7..7
        self.0.set_bits(7..=7, val);
        self
    }

    /// reads the register, updates the `vgrp1die` field, and writes the updated value
    pub fn vgrp1die_write(val: u64) {
        Self::with_reg_val().vgrp1die_insert(val).write();
    }

    /*
     * Field: vgrp1eie
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn vgrp1eie_extract(&self) -> u64 {
        // bits 6..6
        self.0.get_bits(6..=6)
    }

    /// reads the current register value and extract field `vgrp1eie` from it
    pub fn vgrp1eie_read() -> u64 {
        Self::with_reg_val().vgrp1eie_extract()
    }

    /// inserts the given value `val` into the field `vgrp1eie`
    pub fn vgrp1eie_insert(&mut self, val: u64) -> &mut Self {
        // bits 6..6
        self.0.set_bits(6..=6, val);
        self
    }

    /// reads the register, updates the `vgrp1eie` field, and writes the updated value
    pub fn vgrp1eie_write(val: u64) {
        Self::with_reg_val().vgrp1eie_insert(val).write();
    }

    /*
     * Field: vgrp0die
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn vgrp0die_extract(&self) -> u64 {
        // bits 5..5
        self.0.get_bits(5..=5)
    }

    /// reads the current register value and extract field `vgrp0die` from it
    pub fn vgrp0die_read() -> u64 {
        Self::with_reg_val().vgrp0die_extract()
    }

    /// inserts the given value `val` into the field `vgrp0die`
    pub fn vgrp0die_insert(&mut self, val: u64) -> &mut Self {
        // bits 5..5
        self.0.set_bits(5..=5, val);
        self
    }

    /// reads the register, updates the `vgrp0die` field, and writes the updated value
    pub fn vgrp0die_write(val: u64) {
        Self::with_reg_val().vgrp0die_insert(val).write();
    }

    /*
     * Field: vgrp0eie
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn vgrp0eie_extract(&self) -> u64 {
        // bits 4..4
        self.0.get_bits(4..=4)
    }

    /// reads the current register value and extract field `vgrp0eie` from it
    pub fn vgrp0eie_read() -> u64 {
        Self::with_reg_val().vgrp0eie_extract()
    }

    /// inserts the given value `val` into the field `vgrp0eie`
    pub fn vgrp0eie_insert(&mut self, val: u64) -> &mut Self {
        // bits 4..4
        self.0.set_bits(4..=4, val);
        self
    }

    /// reads the register, updates the `vgrp0eie` field, and writes the updated value
    pub fn vgrp0eie_write(val: u64) {
        Self::with_reg_val().vgrp0eie_insert(val).write();
    }

    /*
     * Field: npie
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn npie_extract(&self) -> u64 {
        // bits 3..3
        self.0.get_bits(3..=3)
    }

    /// reads the current register value and extract field `npie` from it
    pub fn npie_read() -> u64 {
        Self::with_reg_val().npie_extract()
    }

    /// inserts the given value `val` into the field `npie`
    pub fn npie_insert(&mut self, val: u64) -> &mut Self {
        // bits 3..3
        self.0.set_bits(3..=3, val);
        self
    }

    /// reads the register, updates the `npie` field, and writes the updated value
    pub fn npie_write(val: u64) {
        Self::with_reg_val().npie_insert(val).write();
    }

    /*
     * Field: lrenpie
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn lrenpie_extract(&self) -> u64 {
        // bits 2..2
        self.0.get_bits(2..=2)
    }

    /// reads the current register value and extract field `lrenpie` from it
    pub fn lrenpie_read() -> u64 {
        Self::with_reg_val().lrenpie_extract()
    }

    /// inserts the given value `val` into the field `lrenpie`
    pub fn lrenpie_insert(&mut self, val: u64) -> &mut Self {
        // bits 2..2
        self.0.set_bits(2..=2, val);
        self
    }

    /// reads the register, updates the `lrenpie` field, and writes the updated value
    pub fn lrenpie_write(val: u64) {
        Self::with_reg_val().lrenpie_insert(val).write();
    }

    /*
     * Field: uie
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn uie_extract(&self) -> u64 {
        // bits 1..1
        self.0.get_bits(1..=1)
    }

    /// reads the current register value and extract field `uie` from it
    pub fn uie_read() -> u64 {
        Self::with_reg_val().uie_extract()
    }

    /// inserts the given value `val` into the field `uie`
    pub fn uie_insert(&mut self, val: u64) -> &mut Self {
        // bits 1..1
        self.0.set_bits(1..=1, val);
        self
    }

    /// reads the register, updates the `uie` field, and writes the updated value
    pub fn uie_write(val: u64) {
        Self::with_reg_val().uie_insert(val).write();
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
    pub fn en_read() -> u64 {
        Self::with_reg_val().en_extract()
    }

    /// inserts the given value `val` into the field `en`
    pub fn en_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..0
        self.0.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `en` field, and writes the updated value
    pub fn en_write(val: u64) {
        Self::with_reg_val().en_insert(val).write();
    }

}

impl Default for IchHcrEl2 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> IchHcrEl2 {
        IchHcrEl2(0)
    }
}
