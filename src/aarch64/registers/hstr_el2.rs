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
 * Generated on: 2022-08-22T15:51:28.517701
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
 * Register:    Hypervisor System Trap Register (hstr_el2)
 * Group:       Virtualization registers
 * Type:        64-bit Register
 * Description: Controls trapping to EL2 of EL1 or lower AArch32 accesses to the System register in the coproc ==
 * File:        AArch64-hstr_el2.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Hypervisor System Trap Register value in memory
pub struct HstrEl2(u64);

/// struct implementation for accessing the fields of register hstr_el2
impl HstrEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> HstrEl2 {
        Self::default()
    }

    /// collects the modifications of HstrEl2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> HstrEl2 {
        HstrEl2(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> HstrEl2 {
        let curval = Self::reg_rawrd() & 0xbfef;
        HstrEl2(curval)
    }

    /// reading the Hypervisor System Trap Register (hstr_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, HSTR_EL2
            llvm_asm!("mrs $0, hstr_el2" : "=r"(regval));
        }
        return regval;
    }

    /// writing the Hypervisor System Trap Register (hstr_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR HSTR_EL2, <Xt>
            llvm_asm!("msr hstr_el2, $0" : : "r"(val));
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0xbfef;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.val)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 49135;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: tn
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tn_extract(&self) -> u64 {
        // bits 15..15
        self.val.get_bits(15..=15)
    }

    /// reads the current register value and extract field `tn` from it
    pub fn tn_read(&mut self) -> u64 {
        Self::with_reg_val().tn_extract()
    }

    /// inserts the given value `val` into the field `tn`
    pub fn tn_insert(&mut self, val: u64) -> &mut self {
        // bits 15..15
        self.val.set_bits(15..=15, val);
        self
    }

    /// reads the register, updates the `tn` field, and writes the updated value
    pub fn tn_write(&mut self, val: u64) {
        Self::with_reg_val().tn_insert(val).write();
    }

    /*
     * Field: t15_15_15
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn t15_15_15_extract(&self) -> u64 {
        // bits 15..15
        self.val.get_bits(15..=15)
    }

    /// reads the current register value and extract field `t15_15_15` from it
    pub fn t15_15_15_read(&mut self) -> u64 {
        Self::with_reg_val().t15_15_15_extract()
    }

    /// inserts the given value `val` into the field `t15_15_15`
    pub fn t15_15_15_insert(&mut self, val: u64) -> &mut self {
        // bits 15..15
        self.val.set_bits(15..=15, val);
        self
    }

    /// reads the register, updates the `t15_15_15` field, and writes the updated value
    pub fn t15_15_15_write(&mut self, val: u64) {
        Self::with_reg_val().t15_15_15_insert(val).write();
    }

    /*
     * Field: t13_13_13
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn t13_13_13_extract(&self) -> u64 {
        // bits 13..13
        self.val.get_bits(13..=13)
    }

    /// reads the current register value and extract field `t13_13_13` from it
    pub fn t13_13_13_read(&mut self) -> u64 {
        Self::with_reg_val().t13_13_13_extract()
    }

    /// inserts the given value `val` into the field `t13_13_13`
    pub fn t13_13_13_insert(&mut self, val: u64) -> &mut self {
        // bits 13..13
        self.val.set_bits(13..=13, val);
        self
    }

    /// reads the register, updates the `t13_13_13` field, and writes the updated value
    pub fn t13_13_13_write(&mut self, val: u64) {
        Self::with_reg_val().t13_13_13_insert(val).write();
    }

    /*
     * Field: t12_12_12
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn t12_12_12_extract(&self) -> u64 {
        // bits 12..12
        self.val.get_bits(12..=12)
    }

    /// reads the current register value and extract field `t12_12_12` from it
    pub fn t12_12_12_read(&mut self) -> u64 {
        Self::with_reg_val().t12_12_12_extract()
    }

    /// inserts the given value `val` into the field `t12_12_12`
    pub fn t12_12_12_insert(&mut self, val: u64) -> &mut self {
        // bits 12..12
        self.val.set_bits(12..=12, val);
        self
    }

    /// reads the register, updates the `t12_12_12` field, and writes the updated value
    pub fn t12_12_12_write(&mut self, val: u64) {
        Self::with_reg_val().t12_12_12_insert(val).write();
    }

    /*
     * Field: t11_11_11
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn t11_11_11_extract(&self) -> u64 {
        // bits 11..11
        self.val.get_bits(11..=11)
    }

    /// reads the current register value and extract field `t11_11_11` from it
    pub fn t11_11_11_read(&mut self) -> u64 {
        Self::with_reg_val().t11_11_11_extract()
    }

    /// inserts the given value `val` into the field `t11_11_11`
    pub fn t11_11_11_insert(&mut self, val: u64) -> &mut self {
        // bits 11..11
        self.val.set_bits(11..=11, val);
        self
    }

    /// reads the register, updates the `t11_11_11` field, and writes the updated value
    pub fn t11_11_11_write(&mut self, val: u64) {
        Self::with_reg_val().t11_11_11_insert(val).write();
    }

    /*
     * Field: t10_10_10
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn t10_10_10_extract(&self) -> u64 {
        // bits 10..10
        self.val.get_bits(10..=10)
    }

    /// reads the current register value and extract field `t10_10_10` from it
    pub fn t10_10_10_read(&mut self) -> u64 {
        Self::with_reg_val().t10_10_10_extract()
    }

    /// inserts the given value `val` into the field `t10_10_10`
    pub fn t10_10_10_insert(&mut self, val: u64) -> &mut self {
        // bits 10..10
        self.val.set_bits(10..=10, val);
        self
    }

    /// reads the register, updates the `t10_10_10` field, and writes the updated value
    pub fn t10_10_10_write(&mut self, val: u64) {
        Self::with_reg_val().t10_10_10_insert(val).write();
    }

    /*
     * Field: t9_9_9
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn t9_9_9_extract(&self) -> u64 {
        // bits 9..9
        self.val.get_bits(9..=9)
    }

    /// reads the current register value and extract field `t9_9_9` from it
    pub fn t9_9_9_read(&mut self) -> u64 {
        Self::with_reg_val().t9_9_9_extract()
    }

    /// inserts the given value `val` into the field `t9_9_9`
    pub fn t9_9_9_insert(&mut self, val: u64) -> &mut self {
        // bits 9..9
        self.val.set_bits(9..=9, val);
        self
    }

    /// reads the register, updates the `t9_9_9` field, and writes the updated value
    pub fn t9_9_9_write(&mut self, val: u64) {
        Self::with_reg_val().t9_9_9_insert(val).write();
    }

    /*
     * Field: t8_8_8
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn t8_8_8_extract(&self) -> u64 {
        // bits 8..8
        self.val.get_bits(8..=8)
    }

    /// reads the current register value and extract field `t8_8_8` from it
    pub fn t8_8_8_read(&mut self) -> u64 {
        Self::with_reg_val().t8_8_8_extract()
    }

    /// inserts the given value `val` into the field `t8_8_8`
    pub fn t8_8_8_insert(&mut self, val: u64) -> &mut self {
        // bits 8..8
        self.val.set_bits(8..=8, val);
        self
    }

    /// reads the register, updates the `t8_8_8` field, and writes the updated value
    pub fn t8_8_8_write(&mut self, val: u64) {
        Self::with_reg_val().t8_8_8_insert(val).write();
    }

    /*
     * Field: t7_7_7
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn t7_7_7_extract(&self) -> u64 {
        // bits 7..7
        self.val.get_bits(7..=7)
    }

    /// reads the current register value and extract field `t7_7_7` from it
    pub fn t7_7_7_read(&mut self) -> u64 {
        Self::with_reg_val().t7_7_7_extract()
    }

    /// inserts the given value `val` into the field `t7_7_7`
    pub fn t7_7_7_insert(&mut self, val: u64) -> &mut self {
        // bits 7..7
        self.val.set_bits(7..=7, val);
        self
    }

    /// reads the register, updates the `t7_7_7` field, and writes the updated value
    pub fn t7_7_7_write(&mut self, val: u64) {
        Self::with_reg_val().t7_7_7_insert(val).write();
    }

    /*
     * Field: t6_6_6
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn t6_6_6_extract(&self) -> u64 {
        // bits 6..6
        self.val.get_bits(6..=6)
    }

    /// reads the current register value and extract field `t6_6_6` from it
    pub fn t6_6_6_read(&mut self) -> u64 {
        Self::with_reg_val().t6_6_6_extract()
    }

    /// inserts the given value `val` into the field `t6_6_6`
    pub fn t6_6_6_insert(&mut self, val: u64) -> &mut self {
        // bits 6..6
        self.val.set_bits(6..=6, val);
        self
    }

    /// reads the register, updates the `t6_6_6` field, and writes the updated value
    pub fn t6_6_6_write(&mut self, val: u64) {
        Self::with_reg_val().t6_6_6_insert(val).write();
    }

    /*
     * Field: t5_5_5
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn t5_5_5_extract(&self) -> u64 {
        // bits 5..5
        self.val.get_bits(5..=5)
    }

    /// reads the current register value and extract field `t5_5_5` from it
    pub fn t5_5_5_read(&mut self) -> u64 {
        Self::with_reg_val().t5_5_5_extract()
    }

    /// inserts the given value `val` into the field `t5_5_5`
    pub fn t5_5_5_insert(&mut self, val: u64) -> &mut self {
        // bits 5..5
        self.val.set_bits(5..=5, val);
        self
    }

    /// reads the register, updates the `t5_5_5` field, and writes the updated value
    pub fn t5_5_5_write(&mut self, val: u64) {
        Self::with_reg_val().t5_5_5_insert(val).write();
    }

    /*
     * Field: t3_3_3
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn t3_3_3_extract(&self) -> u64 {
        // bits 3..3
        self.val.get_bits(3..=3)
    }

    /// reads the current register value and extract field `t3_3_3` from it
    pub fn t3_3_3_read(&mut self) -> u64 {
        Self::with_reg_val().t3_3_3_extract()
    }

    /// inserts the given value `val` into the field `t3_3_3`
    pub fn t3_3_3_insert(&mut self, val: u64) -> &mut self {
        // bits 3..3
        self.val.set_bits(3..=3, val);
        self
    }

    /// reads the register, updates the `t3_3_3` field, and writes the updated value
    pub fn t3_3_3_write(&mut self, val: u64) {
        Self::with_reg_val().t3_3_3_insert(val).write();
    }

    /*
     * Field: t2_2_2
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn t2_2_2_extract(&self) -> u64 {
        // bits 2..2
        self.val.get_bits(2..=2)
    }

    /// reads the current register value and extract field `t2_2_2` from it
    pub fn t2_2_2_read(&mut self) -> u64 {
        Self::with_reg_val().t2_2_2_extract()
    }

    /// inserts the given value `val` into the field `t2_2_2`
    pub fn t2_2_2_insert(&mut self, val: u64) -> &mut self {
        // bits 2..2
        self.val.set_bits(2..=2, val);
        self
    }

    /// reads the register, updates the `t2_2_2` field, and writes the updated value
    pub fn t2_2_2_write(&mut self, val: u64) {
        Self::with_reg_val().t2_2_2_insert(val).write();
    }

    /*
     * Field: t1_1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn t1_1_1_extract(&self) -> u64 {
        // bits 1..1
        self.val.get_bits(1..=1)
    }

    /// reads the current register value and extract field `t1_1_1` from it
    pub fn t1_1_1_read(&mut self) -> u64 {
        Self::with_reg_val().t1_1_1_extract()
    }

    /// inserts the given value `val` into the field `t1_1_1`
    pub fn t1_1_1_insert(&mut self, val: u64) -> &mut self {
        // bits 1..1
        self.val.set_bits(1..=1, val);
        self
    }

    /// reads the register, updates the `t1_1_1` field, and writes the updated value
    pub fn t1_1_1_write(&mut self, val: u64) {
        Self::with_reg_val().t1_1_1_insert(val).write();
    }

    /*
     * Field: t0_0_0
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn t0_0_0_extract(&self) -> u64 {
        // bits 0..0
        self.val.get_bits(0..=0)
    }

    /// reads the current register value and extract field `t0_0_0` from it
    pub fn t0_0_0_read(&mut self) -> u64 {
        Self::with_reg_val().t0_0_0_extract()
    }

    /// inserts the given value `val` into the field `t0_0_0`
    pub fn t0_0_0_insert(&mut self, val: u64) -> &mut self {
        // bits 0..0
        self.val.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `t0_0_0` field, and writes the updated value
    pub fn t0_0_0_write(&mut self, val: u64) {
        Self::with_reg_val().t0_0_0_insert(val).write();
    }
}

impl Default for HstrEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> HstrEl2 {
        HstrEl2(0)
    }
}
