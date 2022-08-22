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
 * Generated on: 2022-08-22T15:51:28.540753
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
 * Register:    Virtualization Secure Translation Table Base Register (vsttbr_el2)
 * Group:       Generic System Control
 * Type:        64-bit Register
 * Description: The base register for stage 2 of the Secure EL1&0 translation regime. Holds the base address of the translation table for the initial lookup for stage 2 of an address translation in the Secure EL1&0 translation regime, and other information for this translation stage.
 * File:        AArch64-vsttbr_el2.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Virtualization Secure Translation Table Base Register value in memory
pub struct VsttbrEl2(u64);

/// struct implementation for accessing the fields of register vsttbr_el2
impl VsttbrEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> VsttbrEl2 {
        Self::default()
    }

    /// collects the modifications of VsttbrEl2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> VsttbrEl2 {
        VsttbrEl2(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> VsttbrEl2 {
        let curval = Self::reg_rawrd() & 0xffffffffffff;
        VsttbrEl2(curval)
    }

    /// reading the Virtualization Secure Translation Table Base Register (vsttbr_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, VSTTBR_EL2
            llvm_asm!("mrs $0, S3_4_C2_C6_0" : "=r"(regval));
        }
        return regval;
    }

    /// writing the Virtualization Secure Translation Table Base Register (vsttbr_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR VSTTBR_EL2, <Xt>
            llvm_asm!("msr S3_4_C2_C6_0, $0" : : "r"(val));
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0xffffffffffff;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.val)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 281474976710655;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: baddr
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn baddr_extract(&self) -> u64 {
        // bits 1..47
        self.val.get_bits(1..=47)
    }

    /// reads the current register value and extract field `baddr` from it
    pub fn baddr_read(&mut self) -> u64 {
        Self::with_reg_val().baddr_extract()
    }

    /// inserts the given value `val` into the field `baddr`
    pub fn baddr_insert(&mut self, val: u64) -> &mut self {
        // bits 1..47
        self.val.set_bits(1..=47, val);
        self
    }

    /// reads the register, updates the `baddr` field, and writes the updated value
    pub fn baddr_write(&mut self, val: u64) {
        Self::with_reg_val().baddr_insert(val).write();
    }

    /*
     * Field: cnp
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn cnp_extract(&self) -> u64 {
        // bits 0..0
        self.val.get_bits(0..=0)
    }

    /// reads the current register value and extract field `cnp` from it
    pub fn cnp_read(&mut self) -> u64 {
        Self::with_reg_val().cnp_extract()
    }

    /// inserts the given value `val` into the field `cnp`
    pub fn cnp_insert(&mut self, val: u64) -> &mut self {
        // bits 0..0
        self.val.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `cnp` field, and writes the updated value
    pub fn cnp_write(&mut self, val: u64) {
        Self::with_reg_val().cnp_insert(val).write();
    }
}

impl Default for VsttbrEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> VsttbrEl2 {
        VsttbrEl2(0)
    }
}
