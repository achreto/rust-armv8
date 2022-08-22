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
 * Generated on: 2022-08-22T15:51:28.533169
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
 * Register:    Random Allocation Tag Seed Register. (rgsr_el1)
 * Group:       Generic System Control
 * Type:        64-bit Register
 * Description: Random Allocation Tag Seed Register.
 * File:        AArch64-rgsr_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Random Allocation Tag Seed Register. value in memory
pub struct RgsrEl1(u64);

/// struct implementation for accessing the fields of register rgsr_el1
impl RgsrEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> RgsrEl1 {
        Self::default()
    }

    /// collects the modifications of RgsrEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> RgsrEl1 {
        RgsrEl1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> RgsrEl1 {
        let curval = Self::reg_rawrd() & 0xffff0f;
        RgsrEl1(curval)
    }

    /// reading the Random Allocation Tag Seed Register. (rgsr_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, RGSR_EL1
            llvm_asm!("mrs $0, S3_0_C1_C0_5" : "=r"(regval));
        }
        return regval;
    }

    /// writing the Random Allocation Tag Seed Register. (rgsr_el1) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR RGSR_EL1, <Xt>
            llvm_asm!("msr S3_0_C1_C0_5, $0" : : "r"(val));
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0xffff0f;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.val)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 16776975;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: seed
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn seed_extract(&self) -> u64 {
        // bits 8..23
        self.val.get_bits(8..=23)
    }

    /// reads the current register value and extract field `seed` from it
    pub fn seed_read(&mut self) -> u64 {
        Self::with_reg_val().seed_extract()
    }

    /// inserts the given value `val` into the field `seed`
    pub fn seed_insert(&mut self, val: u64) -> &mut self {
        // bits 8..23
        self.val.set_bits(8..=23, val);
        self
    }

    /// reads the register, updates the `seed` field, and writes the updated value
    pub fn seed_write(&mut self, val: u64) {
        Self::with_reg_val().seed_insert(val).write();
    }

    /*
     * Field: tag
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tag_extract(&self) -> u64 {
        // bits 0..3
        self.val.get_bits(0..=3)
    }

    /// reads the current register value and extract field `tag` from it
    pub fn tag_read(&mut self) -> u64 {
        Self::with_reg_val().tag_extract()
    }

    /// inserts the given value `val` into the field `tag`
    pub fn tag_insert(&mut self, val: u64) -> &mut self {
        // bits 0..3
        self.val.set_bits(0..=3, val);
        self
    }

    /// reads the register, updates the `tag` field, and writes the updated value
    pub fn tag_write(&mut self, val: u64) {
        Self::with_reg_val().tag_insert(val).write();
    }
}

impl Default for RgsrEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> RgsrEl1 {
        RgsrEl1(0)
    }
}
