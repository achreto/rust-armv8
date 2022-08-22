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
 * Generated on: 2022-08-22T15:51:28.541045
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
 * Register:    Virtualization Translation Table Base Register (vttbr_el2)
 * Group:       Virtual memory control registers
 * Type:        64-bit Register
 * Description: Holds the base address of the translation table for the initial lookup for stage 2 of an address translation in the EL1&0 translation regime, and other information for this translation regime.
 * File:        AArch64-vttbr_el2.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Virtualization Translation Table Base Register value in memory
pub struct VttbrEl2(u64);

/// struct implementation for accessing the fields of register vttbr_el2
impl VttbrEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> VttbrEl2 {
        Self::default()
    }

    /// collects the modifications of VttbrEl2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> VttbrEl2 {
        VttbrEl2(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> VttbrEl2 {
        let curval = Self::reg_rawrd() & 0xffffffffffffffff;
        VttbrEl2(curval)
    }

    /// reading the Virtualization Translation Table Base Register (vttbr_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, VTTBR_EL2
            llvm_asm!("mrs $0, vttbr_el2" : "=r"(regval));
        }
        return regval;
    }

    /// writing the Virtualization Translation Table Base Register (vttbr_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR VTTBR_EL2, <Xt>
            llvm_asm!("msr vttbr_el2, $0" : : "r"(val));
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0xffffffffffffffff;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.val)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 18446744073709551615;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: vmid
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn vmid_extract(&self) -> u64 {
        // bits 48..63
        self.val.get_bits(48..=63)
    }

    /// reads the current register value and extract field `vmid` from it
    pub fn vmid_read(&mut self) -> u64 {
        Self::with_reg_val().vmid_extract()
    }

    /// inserts the given value `val` into the field `vmid`
    pub fn vmid_insert(&mut self, val: u64) -> &mut self {
        // bits 48..63
        self.val.set_bits(48..=63, val);
        self
    }

    /// reads the register, updates the `vmid` field, and writes the updated value
    pub fn vmid_write(&mut self, val: u64) {
        Self::with_reg_val().vmid_insert(val).write();
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
     * Field: cnp_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn cnp_1_extract(&self) -> u64 {
        // bits 0..0
        self.val.get_bits(0..=0)
    }

    /// reads the current register value and extract field `cnp_1` from it
    pub fn cnp_1_read(&mut self) -> u64 {
        Self::with_reg_val().cnp_1_extract()
    }

    /// inserts the given value `val` into the field `cnp_1`
    pub fn cnp_1_insert(&mut self, val: u64) -> &mut self {
        // bits 0..0
        self.val.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `cnp_1` field, and writes the updated value
    pub fn cnp_1_write(&mut self, val: u64) {
        Self::with_reg_val().cnp_1_insert(val).write();
    }
}

impl Default for VttbrEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> VttbrEl2 {
        VttbrEl2(0)
    }
}
