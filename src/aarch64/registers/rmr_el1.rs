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
 * Generated on: 2022-08-22T15:51:28.533262
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
 * Register:    Reset Management Register (EL1) (rmr_el1)
 * Group:       Reset management registers
 * Type:        64-bit Register
 * Description: When this register is implemented:
 * File:        AArch64-rmr_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Reset Management Register (EL1) value in memory
pub struct RmrEl1(u64);

/// struct implementation for accessing the fields of register rmr_el1
impl RmrEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> RmrEl1 {
        Self::default()
    }

    /// collects the modifications of RmrEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> RmrEl1 {
        RmrEl1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> RmrEl1 {
        let curval = Self::reg_rawrd() & 0x3;
        RmrEl1(curval)
    }

    /// reading the Reset Management Register (EL1) (rmr_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, RMR_EL1
            llvm_asm!("mrs $0, rmr_el1" : "=r"(regval));
        }
        return regval;
    }

    /// writing the Reset Management Register (EL1) (rmr_el1) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR RMR_EL1, <Xt>
            llvm_asm!("msr rmr_el1, $0" : : "r"(val));
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0x3;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.val)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 3;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: rr
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn rr_extract(&self) -> u64 {
        // bits 1..1
        self.val.get_bits(1..=1)
    }

    /// reads the current register value and extract field `rr` from it
    pub fn rr_read(&mut self) -> u64 {
        Self::with_reg_val().rr_extract()
    }

    /// inserts the given value `val` into the field `rr`
    pub fn rr_insert(&mut self, val: u64) -> &mut self {
        // bits 1..1
        self.val.set_bits(1..=1, val);
        self
    }

    /// reads the register, updates the `rr` field, and writes the updated value
    pub fn rr_write(&mut self, val: u64) {
        Self::with_reg_val().rr_insert(val).write();
    }

    /*
     * Field: aa64_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn aa64_1_extract(&self) -> u64 {
        // bits 0..0
        self.val.get_bits(0..=0)
    }

    /// reads the current register value and extract field `aa64_1` from it
    pub fn aa64_1_read(&mut self) -> u64 {
        Self::with_reg_val().aa64_1_extract()
    }

    /// inserts the given value `val` into the field `aa64_1`
    pub fn aa64_1_insert(&mut self, val: u64) -> &mut self {
        // bits 0..0
        self.val.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `aa64_1` field, and writes the updated value
    pub fn aa64_1_write(&mut self, val: u64) {
        Self::with_reg_val().aa64_1_insert(val).write();
    }
}

impl Default for RmrEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> RmrEl1 {
        RmrEl1(0)
    }
}
