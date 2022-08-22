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
 * Generated on: 2022-08-22T15:51:28.530455
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
 * Register:    Performance Monitors Control Register (pmcr_el0)
 * Group:       Performance Monitors registers
 * Type:        64-bit Register
 * Description: Provides details of the Performance Monitors implementation, including the number of counters implemented, and configures and controls the counters.
 * File:        AArch64-pmcr_el0.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Performance Monitors Control Register value in memory
pub struct PmcrEl0(u64);

/// struct implementation for accessing the fields of register pmcr_el0
impl PmcrEl0 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> PmcrEl0 {
        Self::default()
    }

    /// collects the modifications of PmcrEl0 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> PmcrEl0 {
        PmcrEl0(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> PmcrEl0 {
        let curval = Self::reg_rawrd() & 0x1fffffaff;
        PmcrEl0(curval)
    }

    /// reading the Performance Monitors Control Register (pmcr_el0) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, PMCR_EL0
            llvm_asm!("mrs $0, pmcr_el0" : "=r"(regval));
        }
        return regval;
    }

    /// writing the Performance Monitors Control Register (pmcr_el0) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR PMCR_EL0, <Xt>
            llvm_asm!("msr pmcr_el0, $0" : : "r"(val));
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0x1fffffaff;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.val)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 8589933311;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: fzs_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn fzs_1_extract(&self) -> u64 {
        // bits 32..32
        self.val.get_bits(32..=32)
    }

    /// reads the current register value and extract field `fzs_1` from it
    pub fn fzs_1_read(&mut self) -> u64 {
        Self::with_reg_val().fzs_1_extract()
    }

    /// inserts the given value `val` into the field `fzs_1`
    pub fn fzs_1_insert(&mut self, val: u64) -> &mut self {
        // bits 32..32
        self.val.set_bits(32..=32, val);
        self
    }

    /// reads the register, updates the `fzs_1` field, and writes the updated value
    pub fn fzs_1_write(&mut self, val: u64) {
        Self::with_reg_val().fzs_1_insert(val).write();
    }

    /*
     * Field: imp_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn imp_1_extract(&self) -> u64 {
        // bits 24..31
        self.val.get_bits(24..=31)
    }

    /// reads the current register value and extract field `imp_1` from it
    pub fn imp_1_read(&mut self) -> u64 {
        Self::with_reg_val().imp_1_extract()
    }

    /// inserts the given value `val` into the field `imp_1`
    pub fn imp_1_insert(&mut self, val: u64) -> &mut self {
        // bits 24..31
        self.val.set_bits(24..=31, val);
        self
    }

    /// reads the register, updates the `imp_1` field, and writes the updated value
    pub fn imp_1_write(&mut self, val: u64) {
        Self::with_reg_val().imp_1_insert(val).write();
    }

    /*
     * Field: idcode_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn idcode_1_extract(&self) -> u64 {
        // bits 16..23
        self.val.get_bits(16..=23)
    }

    /// reads the current register value and extract field `idcode_1` from it
    pub fn idcode_1_read(&mut self) -> u64 {
        Self::with_reg_val().idcode_1_extract()
    }

    /// inserts the given value `val` into the field `idcode_1`
    pub fn idcode_1_insert(&mut self, val: u64) -> &mut self {
        // bits 16..23
        self.val.set_bits(16..=23, val);
        self
    }

    /// reads the register, updates the `idcode_1` field, and writes the updated value
    pub fn idcode_1_write(&mut self, val: u64) {
        Self::with_reg_val().idcode_1_insert(val).write();
    }

    /*
     * Field: n
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn n_extract(&self) -> u64 {
        // bits 11..15
        self.val.get_bits(11..=15)
    }

    /// reads the current register value and extract field `n` from it
    pub fn n_read(&mut self) -> u64 {
        Self::with_reg_val().n_extract()
    }

    /// inserts the given value `val` into the field `n`
    pub fn n_insert(&mut self, val: u64) -> &mut self {
        // bits 11..15
        self.val.set_bits(11..=15, val);
        self
    }

    /// reads the register, updates the `n` field, and writes the updated value
    pub fn n_write(&mut self, val: u64) {
        Self::with_reg_val().n_insert(val).write();
    }

    /*
     * Field: fzo_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn fzo_1_extract(&self) -> u64 {
        // bits 9..9
        self.val.get_bits(9..=9)
    }

    /// reads the current register value and extract field `fzo_1` from it
    pub fn fzo_1_read(&mut self) -> u64 {
        Self::with_reg_val().fzo_1_extract()
    }

    /// inserts the given value `val` into the field `fzo_1`
    pub fn fzo_1_insert(&mut self, val: u64) -> &mut self {
        // bits 9..9
        self.val.set_bits(9..=9, val);
        self
    }

    /// reads the register, updates the `fzo_1` field, and writes the updated value
    pub fn fzo_1_write(&mut self, val: u64) {
        Self::with_reg_val().fzo_1_insert(val).write();
    }

    /*
     * Field: lp_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn lp_1_extract(&self) -> u64 {
        // bits 7..7
        self.val.get_bits(7..=7)
    }

    /// reads the current register value and extract field `lp_1` from it
    pub fn lp_1_read(&mut self) -> u64 {
        Self::with_reg_val().lp_1_extract()
    }

    /// inserts the given value `val` into the field `lp_1`
    pub fn lp_1_insert(&mut self, val: u64) -> &mut self {
        // bits 7..7
        self.val.set_bits(7..=7, val);
        self
    }

    /// reads the register, updates the `lp_1` field, and writes the updated value
    pub fn lp_1_write(&mut self, val: u64) {
        Self::with_reg_val().lp_1_insert(val).write();
    }

    /*
     * Field: lc_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn lc_1_extract(&self) -> u64 {
        // bits 6..6
        self.val.get_bits(6..=6)
    }

    /// reads the current register value and extract field `lc_1` from it
    pub fn lc_1_read(&mut self) -> u64 {
        Self::with_reg_val().lc_1_extract()
    }

    /// inserts the given value `val` into the field `lc_1`
    pub fn lc_1_insert(&mut self, val: u64) -> &mut self {
        // bits 6..6
        self.val.set_bits(6..=6, val);
        self
    }

    /// reads the register, updates the `lc_1` field, and writes the updated value
    pub fn lc_1_write(&mut self, val: u64) {
        Self::with_reg_val().lc_1_insert(val).write();
    }

    /*
     * Field: dp_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dp_1_extract(&self) -> u64 {
        // bits 5..5
        self.val.get_bits(5..=5)
    }

    /// reads the current register value and extract field `dp_1` from it
    pub fn dp_1_read(&mut self) -> u64 {
        Self::with_reg_val().dp_1_extract()
    }

    /// inserts the given value `val` into the field `dp_1`
    pub fn dp_1_insert(&mut self, val: u64) -> &mut self {
        // bits 5..5
        self.val.set_bits(5..=5, val);
        self
    }

    /// reads the register, updates the `dp_1` field, and writes the updated value
    pub fn dp_1_write(&mut self, val: u64) {
        Self::with_reg_val().dp_1_insert(val).write();
    }

    /*
     * Field: x_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn x_1_extract(&self) -> u64 {
        // bits 4..4
        self.val.get_bits(4..=4)
    }

    /// reads the current register value and extract field `x_1` from it
    pub fn x_1_read(&mut self) -> u64 {
        Self::with_reg_val().x_1_extract()
    }

    /// inserts the given value `val` into the field `x_1`
    pub fn x_1_insert(&mut self, val: u64) -> &mut self {
        // bits 4..4
        self.val.set_bits(4..=4, val);
        self
    }

    /// reads the register, updates the `x_1` field, and writes the updated value
    pub fn x_1_write(&mut self, val: u64) {
        Self::with_reg_val().x_1_insert(val).write();
    }

    /*
     * Field: d_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn d_1_extract(&self) -> u64 {
        // bits 3..3
        self.val.get_bits(3..=3)
    }

    /// reads the current register value and extract field `d_1` from it
    pub fn d_1_read(&mut self) -> u64 {
        Self::with_reg_val().d_1_extract()
    }

    /// inserts the given value `val` into the field `d_1`
    pub fn d_1_insert(&mut self, val: u64) -> &mut self {
        // bits 3..3
        self.val.set_bits(3..=3, val);
        self
    }

    /// reads the register, updates the `d_1` field, and writes the updated value
    pub fn d_1_write(&mut self, val: u64) {
        Self::with_reg_val().d_1_insert(val).write();
    }

    /*
     * Field: c
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn c_extract(&self) -> u64 {
        // bits 2..2
        self.val.get_bits(2..=2)
    }

    /// reads the current register value and extract field `c` from it
    pub fn c_read(&mut self) -> u64 {
        Self::with_reg_val().c_extract()
    }

    /// inserts the given value `val` into the field `c`
    pub fn c_insert(&mut self, val: u64) -> &mut self {
        // bits 2..2
        self.val.set_bits(2..=2, val);
        self
    }

    /// reads the register, updates the `c` field, and writes the updated value
    pub fn c_write(&mut self, val: u64) {
        Self::with_reg_val().c_insert(val).write();
    }

    /*
     * Field: p
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn p_extract(&self) -> u64 {
        // bits 1..1
        self.val.get_bits(1..=1)
    }

    /// reads the current register value and extract field `p` from it
    pub fn p_read(&mut self) -> u64 {
        Self::with_reg_val().p_extract()
    }

    /// inserts the given value `val` into the field `p`
    pub fn p_insert(&mut self, val: u64) -> &mut self {
        // bits 1..1
        self.val.set_bits(1..=1, val);
        self
    }

    /// reads the register, updates the `p` field, and writes the updated value
    pub fn p_write(&mut self, val: u64) {
        Self::with_reg_val().p_insert(val).write();
    }

    /*
     * Field: e
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn e_extract(&self) -> u64 {
        // bits 0..0
        self.val.get_bits(0..=0)
    }

    /// reads the current register value and extract field `e` from it
    pub fn e_read(&mut self) -> u64 {
        Self::with_reg_val().e_extract()
    }

    /// inserts the given value `val` into the field `e`
    pub fn e_insert(&mut self, val: u64) -> &mut self {
        // bits 0..0
        self.val.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `e` field, and writes the updated value
    pub fn e_write(&mut self, val: u64) {
        Self::with_reg_val().e_insert(val).write();
    }
}

impl Default for PmcrEl0 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> PmcrEl0 {
        PmcrEl0(0)
    }
}
