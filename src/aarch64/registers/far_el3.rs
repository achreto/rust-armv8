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
use core::arch::asm;

/**************************************************************************************************
 *
 * !!!! WARNING: THIS FILE IS AUTO GENERATED. ANY CHANGES MAY BE OVERWRITTEN !!!!
 *
 * Generated on: 2022-08-22T16:35:53.057265
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
 * Register:    Fault Address Register (EL3) (far_el3)
 * Group:       Exception and fault handling registers
 * Type:        64-bit Register
 * Description: Holds the faulting Virtual Address for all synchronous Instruction or Data Abort and PC alignment fault exceptions that are taken to EL3.
 * File:        AArch64-far_el3.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Fault Address Register (EL3) value in memory
pub struct FarEl3(u64);

/// struct implementation for accessing the fields of register far_el3
impl FarEl3 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> FarEl3 {
        Self::default()
    }

    /// collects the modifications of FarEl3 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> FarEl3 {
        FarEl3(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> FarEl3 {
        let curval = Self::reg_rawrd() & 0xffffffffffffffff;
        FarEl3(curval)
    }

    /// reading the Fault Address Register (EL3) (far_el3) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, FAR_EL3
            asm!("mrs {}, far_el3", out(reg) regval);
        }
        return regval;
    }

    /// writing the Fault Address Register (EL3) (far_el3) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR FAR_EL3, <Xt>
            asm!("msr far_el3, {}", in(reg) val);
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xffffffffffffffff;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 18446744073709551615;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: val
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn val_extract(&self) -> u64 {
        // bits 0..63
        self.0.get_bits(0..=63)
    }

    /// reads the current register value and extract field `val` from it
    pub fn val_read() -> u64 {
        Self::with_reg_val().val_extract()
    }

    /// inserts the given value `val` into the field `val`
    pub fn val_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..63
        self.0.set_bits(0..=63, val);
        self
    }

    /// reads the register, updates the `val` field, and writes the updated value
    pub fn val_write(val: u64) {
        Self::with_reg_val().val_insert(val).write();
    }
}

impl Default for FarEl3 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> FarEl3 {
        FarEl3(0)
    }
}
