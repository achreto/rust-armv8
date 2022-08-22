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

use core::arch::asm;
use bit_field::BitField;


/**************************************************************************************************
 *
 * !!!! WARNING: THIS FILE IS AUTO GENERATED. ANY CHANGES MAY BE OVERWRITTEN !!!!
 *
 * Generated on: 2022-08-22T16:25:59.084869
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
 * Register:    SVE Feature ID register 0 (id_aa64zfr0_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Provides additional information about the implemented features of the AArch64 Scalable Vector Extension, when the 
 * File:        AArch64-id_aa64zfr0_el1.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the SVE Feature ID register 0 value in memory
pub struct IdAa64zfr0El1(u64);

/// struct implementation for accessing the fields of register id_aa64zfr0_el1
impl IdAa64zfr0El1 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> IdAa64zfr0El1 {
        Self::default()
    }

    /// collects the modifications of IdAa64zfr0El1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> IdAa64zfr0El1 {
        IdAa64zfr0El1(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  IdAa64zfr0El1 {
        let curval = Self::reg_rawrd() & 0xff0f00000f0000f;
        IdAa64zfr0El1(curval)
    }


    
    /// reading the SVE Feature ID register 0 (id_aa64zfr0_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ID_AA64ZFR0_EL1
            asm!("mrs {}, S3_0_C0_C4_4", out(reg) regval);
        }
        return regval;
    }

// register is not writable. not emitting write accessor


    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xff0f00000f0000f;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 1148681787785871375;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: f64mm
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn f64mm_extract(&self) -> u64 {
        // bits 56..59
        self.0.get_bits(56..=59)
    }

    /// reads the current register value and extract field `f64mm` from it
    pub fn f64mm_read(&mut self) -> u64 {
        Self::with_reg_val().f64mm_extract()
    }
// no insert() method for field f64mm
    /*
     * Field: f32mm
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn f32mm_extract(&self) -> u64 {
        // bits 52..55
        self.0.get_bits(52..=55)
    }

    /// reads the current register value and extract field `f32mm` from it
    pub fn f32mm_read(&mut self) -> u64 {
        Self::with_reg_val().f32mm_extract()
    }
// no insert() method for field f32mm
    /*
     * Field: i8mm
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn i8mm_extract(&self) -> u64 {
        // bits 44..47
        self.0.get_bits(44..=47)
    }

    /// reads the current register value and extract field `i8mm` from it
    pub fn i8mm_read(&mut self) -> u64 {
        Self::with_reg_val().i8mm_extract()
    }
// no insert() method for field i8mm
    /*
     * Field: bf16
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn bf16_extract(&self) -> u64 {
        // bits 20..23
        self.0.get_bits(20..=23)
    }

    /// reads the current register value and extract field `bf16` from it
    pub fn bf16_read(&mut self) -> u64 {
        Self::with_reg_val().bf16_extract()
    }
// no insert() method for field bf16
    /*
     * Field: svever
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn svever_extract(&self) -> u64 {
        // bits 0..3
        self.0.get_bits(0..=3)
    }

    /// reads the current register value and extract field `svever` from it
    pub fn svever_read(&mut self) -> u64 {
        Self::with_reg_val().svever_extract()
    }
// no insert() method for field svever
}

impl Default for IdAa64zfr0El1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> IdAa64zfr0El1 {
        IdAa64zfr0El1(0)
    }
}
