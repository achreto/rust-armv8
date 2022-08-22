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
 * Generated on: 2022-08-22T15:51:28.524953
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
 * Register:    Instruction Fault Status Register (EL2) (ifsr32_el2)
 * Group:       Exception and fault handling registers
 * Type:        64-bit Register
 * Description: Allows access to the AArch32
 * File:        AArch64-ifsr32_el2.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Instruction Fault Status Register (EL2) value in memory
pub struct Ifsr32El2(u64);

/// struct implementation for accessing the fields of register ifsr32_el2
impl Ifsr32El2 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> Ifsr32El2 {
        Self::default()
    }

    /// collects the modifications of Ifsr32El2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> Ifsr32El2 {
        Ifsr32El2(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> Ifsr32El2 {
        let curval = Self::reg_rawrd() & 0x1123f;
        Ifsr32El2(curval)
    }

    /// reading the Instruction Fault Status Register (EL2) (ifsr32_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, IFSR32_EL2
            llvm_asm!("mrs $0, ifsr32_el2" : "=r"(regval));
        }
        return regval;
    }

    /// writing the Instruction Fault Status Register (EL2) (ifsr32_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR IFSR32_EL2, <Xt>
            llvm_asm!("msr ifsr32_el2, $0" : : "r"(val));
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0x1123f;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.val)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 70207;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: fnv
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn fnv_extract(&self) -> u64 {
        // bits 16..16
        self.val.get_bits(16..=16)
    }

    /// reads the current register value and extract field `fnv` from it
    pub fn fnv_read(&mut self) -> u64 {
        Self::with_reg_val().fnv_extract()
    }

    /// inserts the given value `val` into the field `fnv`
    pub fn fnv_insert(&mut self, val: u64) -> &mut self {
        // bits 16..16
        self.val.set_bits(16..=16, val);
        self
    }

    /// reads the register, updates the `fnv` field, and writes the updated value
    pub fn fnv_write(&mut self, val: u64) {
        Self::with_reg_val().fnv_insert(val).write();
    }

    /*
     * Field: ext
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ext_extract(&self) -> u64 {
        // bits 12..12
        self.val.get_bits(12..=12)
    }

    /// reads the current register value and extract field `ext` from it
    pub fn ext_read(&mut self) -> u64 {
        Self::with_reg_val().ext_extract()
    }

    /// inserts the given value `val` into the field `ext`
    pub fn ext_insert(&mut self, val: u64) -> &mut self {
        // bits 12..12
        self.val.set_bits(12..=12, val);
        self
    }

    /// reads the register, updates the `ext` field, and writes the updated value
    pub fn ext_write(&mut self, val: u64) {
        Self::with_reg_val().ext_insert(val).write();
    }

    /*
     * Field: lpae
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn lpae_extract(&self) -> u64 {
        // bits 9..9
        self.val.get_bits(9..=9)
    }

    /// reads the current register value and extract field `lpae` from it
    pub fn lpae_read(&mut self) -> u64 {
        Self::with_reg_val().lpae_extract()
    }

    /// inserts the given value `val` into the field `lpae`
    pub fn lpae_insert(&mut self, val: u64) -> &mut self {
        // bits 9..9
        self.val.set_bits(9..=9, val);
        self
    }

    /// reads the register, updates the `lpae` field, and writes the updated value
    pub fn lpae_write(&mut self, val: u64) {
        Self::with_reg_val().lpae_insert(val).write();
    }

    /*
     * Field: status
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn status_extract(&self) -> u64 {
        // bits 0..5
        self.val.get_bits(0..=5)
    }

    /// reads the current register value and extract field `status` from it
    pub fn status_read(&mut self) -> u64 {
        Self::with_reg_val().status_extract()
    }

    /// inserts the given value `val` into the field `status`
    pub fn status_insert(&mut self, val: u64) -> &mut self {
        // bits 0..5
        self.val.set_bits(0..=5, val);
        self
    }

    /// reads the register, updates the `status` field, and writes the updated value
    pub fn status_write(&mut self, val: u64) {
        Self::with_reg_val().status_insert(val).write();
    }
}

impl Default for Ifsr32El2 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> Ifsr32El2 {
        Ifsr32El2(0)
    }
}
