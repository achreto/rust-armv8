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
 * Generated on: 2022-08-22T15:51:28.529054
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
 * Register:    OS Lock Access Register (oslar_el1)
 * Group:       Debug registers
 * Type:        64-bit Register
 * Description: Used to lock or unlock the OS Lock.
 * File:        AArch64-oslar_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the OS Lock Access Register value in memory
pub struct OslarEl1(u64);

/// struct implementation for accessing the fields of register oslar_el1
impl OslarEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> OslarEl1 {
        Self::default()
    }

    /// collects the modifications of OslarEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> OslarEl1 {
        OslarEl1(self.0)
    }

    // no current() method as it is write only

    /// writing the OS Lock Access Register (oslar_el1) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR OSLAR_EL1, <Xt>
            llvm_asm!("msr oslar_el1, $0" : : "r"(val));
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
    //    self.val = newval & 1;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: oslk
     * --------------------------------------------------------------------------------------------
     */

    // no extract() method for field oslk
    /// inserts the given value `val` into the field `oslk`
    pub fn oslk_insert(&mut self, val: u64) -> &mut self {
        // bits 0..0
        self.val.set_bits(0..=0, val);
        self
    }

    /// sets the field `oslk` to the given value `val`
    pub fn oslk_write(&mut self, val: u64) {
        Self::default().oslk_insert(val).write();
    }
}

impl Default for OslarEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> OslarEl1 {
        OslarEl1(0)
    }
}
