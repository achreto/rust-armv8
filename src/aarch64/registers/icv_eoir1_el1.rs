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
 * Generated on: 2022-08-22T15:51:28.520979
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
 * Register:    Interrupt Controller Virtual End Of Interrupt Register 1 (icv_eoir1_el1)
 * Group:       Virtualization registers
 * Type:        64-bit Register
 * Description: A PE writes to this register to inform the CPU interface that it has completed the processing of the specified virtual Group 1 interrupt.
 * File:        AArch64-icv_eoir1_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Interrupt Controller Virtual End Of Interrupt Register 1 value in memory
pub struct IcvEoir1El1(u64);

/// struct implementation for accessing the fields of register icv_eoir1_el1
impl IcvEoir1El1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> IcvEoir1El1 {
        Self::default()
    }

    /// collects the modifications of IcvEoir1El1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> IcvEoir1El1 {
        IcvEoir1El1(self.0)
    }

    // no current() method as it is write only

    /// writing the Interrupt Controller Virtual End Of Interrupt Register 1 (icv_eoir1_el1) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR ICC_EOIR1_EL1, <Xt>
            llvm_asm!("msr S3_0_C12_C12_1, $0" : : "r"(val));
        }
    }

    // register is not readable. not emitting read accessor

    // no read() method as it is write only

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.val)
    }

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

    // no extract() method for field intid
    /// inserts the given value `val` into the field `intid`
    pub fn intid_insert(&mut self, val: u64) -> &mut self {
        // bits 0..23
        self.val.set_bits(0..=23, val);
        self
    }

    /// sets the field `intid` to the given value `val`
    pub fn intid_write(&mut self, val: u64) {
        Self::default().intid_insert(val).write();
    }
}

impl Default for IcvEoir1El1 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> IcvEoir1El1 {
        IcvEoir1El1(0)
    }
}
