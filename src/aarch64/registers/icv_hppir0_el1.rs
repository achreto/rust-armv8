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
 * Generated on: 2022-08-22T15:51:28.521056
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
 * Register:    Interrupt Controller Virtual Highest Priority Pending Interrupt Register 0 (icv_hppir0_el1)
 * Group:       Virtualization registers
 * Type:        64-bit Register
 * Description: Indicates the highest priority pending virtual Group 0 interrupt on the virtual CPU interface.
 * File:        AArch64-icv_hppir0_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Interrupt Controller Virtual Highest Priority Pending Interrupt Register 0 value in memory
pub struct IcvHppir0El1(u64);

/// struct implementation for accessing the fields of register icv_hppir0_el1
impl IcvHppir0El1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> IcvHppir0El1 {
        Self::default()
    }

    /// collects the modifications of IcvHppir0El1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> IcvHppir0El1 {
        IcvHppir0El1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> IcvHppir0El1 {
        let curval = Self::reg_rawrd() & 0xffffff;
        IcvHppir0El1(curval)
    }

    /// reading the Interrupt Controller Virtual Highest Priority Pending Interrupt Register 0 (icv_hppir0_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ICC_HPPIR0_EL1
            llvm_asm!("mrs $0, S3_0_C12_C8_2" : "=r"(regval));
        }
        return regval;
    }

    // register is not writable. not emitting write accessor

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0xffffff;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 16777215;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: intid
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn intid_extract(&self) -> u64 {
        // bits 0..23
        self.val.get_bits(0..=23)
    }

    /// reads the current register value and extract field `intid` from it
    pub fn intid_read(&mut self) -> u64 {
        Self::with_reg_val().intid_extract()
    }
    // no insert() method for field intid
}

impl Default for IcvHppir0El1 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> IcvHppir0El1 {
        IcvHppir0El1(0)
    }
}
