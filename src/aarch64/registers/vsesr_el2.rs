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
 * Generated on: 2022-08-22T15:51:28.540510
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
 * Register:    Virtual SError Exception Syndrome Register (vsesr_el2)
 * Group:       RAS registers
 * Type:        64-bit Register
 * Description: Provides the syndrome value reported to software on taking a virtual SError interrupt exception to EL1, or on executing an
 * File:        AArch64-vsesr_el2.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Virtual SError Exception Syndrome Register value in memory
pub struct VsesrEl2(u64);

/// struct implementation for accessing the fields of register vsesr_el2
impl VsesrEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> VsesrEl2 {
        Self::default()
    }

    /// collects the modifications of VsesrEl2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> VsesrEl2 {
        VsesrEl2(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> VsesrEl2 {
        let curval = Self::reg_rawrd() & 0x1ffffff;
        VsesrEl2(curval)
    }

    /// reading the Virtual SError Exception Syndrome Register (vsesr_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, VSESR_EL2
            llvm_asm!("mrs $0, S3_4_C5_C2_3" : "=r"(regval));
        }
        return regval;
    }

    /// writing the Virtual SError Exception Syndrome Register (vsesr_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR VSESR_EL2, <Xt>
            llvm_asm!("msr S3_4_C5_C2_3, $0" : : "r"(val));
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0x1ffffff;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.val)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 33554431;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: ids
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ids_extract(&self) -> u64 {
        // bits 24..24
        self.val.get_bits(24..=24)
    }

    /// reads the current register value and extract field `ids` from it
    pub fn ids_read(&mut self) -> u64 {
        Self::with_reg_val().ids_extract()
    }

    /// inserts the given value `val` into the field `ids`
    pub fn ids_insert(&mut self, val: u64) -> &mut self {
        // bits 24..24
        self.val.set_bits(24..=24, val);
        self
    }

    /// reads the register, updates the `ids` field, and writes the updated value
    pub fn ids_write(&mut self, val: u64) {
        Self::with_reg_val().ids_insert(val).write();
    }

    /*
     * Field: iss
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn iss_extract(&self) -> u64 {
        // bits 0..23
        self.val.get_bits(0..=23)
    }

    /// reads the current register value and extract field `iss` from it
    pub fn iss_read(&mut self) -> u64 {
        Self::with_reg_val().iss_extract()
    }

    /// inserts the given value `val` into the field `iss`
    pub fn iss_insert(&mut self, val: u64) -> &mut self {
        // bits 0..23
        self.val.set_bits(0..=23, val);
        self
    }

    /// reads the register, updates the `iss` field, and writes the updated value
    pub fn iss_write(&mut self, val: u64) {
        Self::with_reg_val().iss_insert(val).write();
    }
}

impl Default for VsesrEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> VsesrEl2 {
        VsesrEl2(0)
    }
}
