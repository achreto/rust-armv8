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
 * Generated on: 2022-08-22T15:51:28.513922
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
 * Register:    Exception Syndrome Register (EL3) (esr_el3)
 * Group:       Exception and fault handling registers
 * Type:        64-bit Register
 * Description: Holds syndrome information for an exception taken to EL3.
 * File:        AArch64-esr_el3.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Exception Syndrome Register (EL3) value in memory
pub struct EsrEl3(u64);

/// struct implementation for accessing the fields of register esr_el3
impl EsrEl3 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> EsrEl3 {
        Self::default()
    }

    /// collects the modifications of EsrEl3 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> EsrEl3 {
        EsrEl3(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> EsrEl3 {
        let curval = Self::reg_rawrd() & 0x1fffffffff;
        EsrEl3(curval)
    }

    /// reading the Exception Syndrome Register (EL3) (esr_el3) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ESR_EL3
            llvm_asm!("mrs $0, esr_el3" : "=r"(regval));
        }
        return regval;
    }

    /// writing the Exception Syndrome Register (EL3) (esr_el3) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR ESR_EL3, <Xt>
            llvm_asm!("msr esr_el3, $0" : : "r"(val));
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0x1fffffffff;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.val)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 137438953471;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: iss2_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn iss2_1_extract(&self) -> u64 {
        // bits 32..36
        self.val.get_bits(32..=36)
    }

    /// reads the current register value and extract field `iss2_1` from it
    pub fn iss2_1_read(&mut self) -> u64 {
        Self::with_reg_val().iss2_1_extract()
    }

    /// inserts the given value `val` into the field `iss2_1`
    pub fn iss2_1_insert(&mut self, val: u64) -> &mut self {
        // bits 32..36
        self.val.set_bits(32..=36, val);
        self
    }

    /// reads the register, updates the `iss2_1` field, and writes the updated value
    pub fn iss2_1_write(&mut self, val: u64) {
        Self::with_reg_val().iss2_1_insert(val).write();
    }

    /*
     * Field: ec
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ec_extract(&self) -> u64 {
        // bits 26..31
        self.val.get_bits(26..=31)
    }

    /// reads the current register value and extract field `ec` from it
    pub fn ec_read(&mut self) -> u64 {
        Self::with_reg_val().ec_extract()
    }

    /// inserts the given value `val` into the field `ec`
    pub fn ec_insert(&mut self, val: u64) -> &mut self {
        // bits 26..31
        self.val.set_bits(26..=31, val);
        self
    }

    /// reads the register, updates the `ec` field, and writes the updated value
    pub fn ec_write(&mut self, val: u64) {
        Self::with_reg_val().ec_insert(val).write();
    }

    /*
     * Field: il
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn il_extract(&self) -> u64 {
        // bits 25..25
        self.val.get_bits(25..=25)
    }

    /// reads the current register value and extract field `il` from it
    pub fn il_read(&mut self) -> u64 {
        Self::with_reg_val().il_extract()
    }

    /// inserts the given value `val` into the field `il`
    pub fn il_insert(&mut self, val: u64) -> &mut self {
        // bits 25..25
        self.val.set_bits(25..=25, val);
        self
    }

    /// reads the register, updates the `il` field, and writes the updated value
    pub fn il_write(&mut self, val: u64) {
        Self::with_reg_val().il_insert(val).write();
    }

    /*
     * Field: iss
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn iss_extract(&self) -> u64 {
        // bits 0..24
        self.val.get_bits(0..=24)
    }

    /// reads the current register value and extract field `iss` from it
    pub fn iss_read(&mut self) -> u64 {
        Self::with_reg_val().iss_extract()
    }

    /// inserts the given value `val` into the field `iss`
    pub fn iss_insert(&mut self, val: u64) -> &mut self {
        // bits 0..24
        self.val.set_bits(0..=24, val);
        self
    }

    /// reads the register, updates the `iss` field, and writes the updated value
    pub fn iss_write(&mut self, val: u64) {
        Self::with_reg_val().iss_insert(val).write();
    }
}

impl Default for EsrEl3 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> EsrEl3 {
        EsrEl3(0)
    }
}
