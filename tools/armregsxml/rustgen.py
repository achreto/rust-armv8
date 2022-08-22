#
# MIT License
#
# Copyright (c) 2020 Reto Achermann
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.
#
# SPDX-License-Identifier: MIT
#

import datetime

import shutil  # to copy files

from plumbum import local
from plumbum.cmd import cargo
from plumbum.commands import ProcessExecutionError

from logger import logverbose, logok, logwarn, logerr, logsetverbose, logtitle, log


# =======================================================================================
# Rust Section and File Header Templates
# =======================================================================================


RS_FILE_HEADER="""/*
 * MIT License
 *
 * Copyright (c) {year} Reto Achermann
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

"""

RS_AUTOGEN_WARNING="""
/**************************************************************************************************
 *
 * !!!! WARNING: THIS FILE IS AUTO GENERATED. ANY CHANGES MAY BE OVERWRITTEN !!!!
 *
 * Generated on: {date}
 * Version: {version}
 * Source: {source}
 *
 * !!!! WARNING: THIS FILE IS AUTO GENERATED. ANY CHANGES MAY BE OVERWRITTEN !!!!
 *
 *************************************************************************************************/
"""

RS_REGISTER_INFO="""
/*
 * ================================================================================================
 * Register Information
 * ================================================================================================
 *
 * Register:    {name} ({id})
 * Group:       {group}
 * Type:        {length}-bit Register
 * Description: {desc}
 * File:        {file}
 */
"""


RS_SECTION="""

/*
 * ================================================================================================
 * {header}
 * ================================================================================================
 */


"""

RS_SUBSECTION="""
    /*
     * {header}
     * --------------------------------------------------------------------------------------------
     */

"""

# =======================================================================================
# Rust Raw Register Access and System Instruction Templates
# =======================================================================================


RS_REG_READ="""
    /// reading the {name} ({id}) register
    #[inline(always)]
    fn reg_rawrd() -> u{length} {{
        let mut regval: u{length};
        unsafe {{
            // {access}
            llvm_asm!(\"{enc}\" : \"=r\"(regval));
        }}
        return regval;
    }}

"""

RS_REG_WRITE="""
    /// writing the {name} ({id}) register
    #[inline(always)]
    fn reg_rawwr(val: u{length}) {{
        unsafe {{
            // {access}
            llvm_asm!("{enc}" : : "r"(val));
        }}
    }}

"""

RS_SYSINSTR_ARG="""
/// {desc}
#[inline(always)]
pub fn {fn}(arg: u{length}) {{
    unsafe {{
        llvm_asm!("{instr}" : : "r"(arg));
    }}
}}
"""

RS_SYSINSTR="""
/// {desc}
#[inline(always)]
pub fn {fn}() {{
    unsafe {{
        llvm_asm!("{instr}");
    }}
}}
"""


# =======================================================================================
# Read / Write Fileds Templates
# =======================================================================================


RS_READ_FIELD="""
/// reads field val from register
pub fn {field}_read() -> u{length} {{
    // bits {lsb}..{msb}
    let val = reg_rawrd();
    (val >> {shift}) & 0x{mask:x}
}}
"""

RS_WRITE_FIELD="""
/// inserts field val into register
pub fn {field}_write(newval : u{length}) {{
    // bits {lsb}..{msb}
    let val = reg_rawrd();
    reg_rawwr(val & !(0x{mask:x}  << {shift}) | ((newval & 0x{mask:x} ) << {shift}));
}}
"""

RS_WRITE_FIELD_NO_READ="""
/// inserts field val into register
pub fn {field}_write(newval : u{length}) {{
    // bits {lsb}..{msb}
    reg_rawwr((newval & 0x{mask:x} ) << {shift});
}}
"""


# =======================================================================================
# Struct Definition Templates
# =======================================================================================

RS_STRUCT="""
/// struct holding a copy of the {name} value in memory
pub struct {typename}(u{length});
"""


RS_STRUCT_IMPL="""
/// struct implementation for accessing the fields of register {regid}
impl {typename} {{

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> {typename} {{
        Self::default()
    }}

    /// collects the modifications of {typename} and creates new object
    #[inline(always)]
    pub fn collect(&self) -> {typename} {{
        {typename}(self.0)
    }}

    {current}

    {regaccess}

    {read}
    {write}

    // sets the value of the struct
    //pub fn set(&mut self, newval: u{length}) {{
    //    self.val = newval & {allmask};
    //}}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u{length} {{
        self.0
    }}


    {fields}
}}

impl Default for {typename} {{
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> {typename} {{
        {typename}({default})
    }}
}}
"""

RS_EXTRACT_FIELD="""
    /// extracts field val from current value
    pub fn {field}_extract(&self) -> u{length} {{
        // bits {lsb}..{msb}
        self.val.get_bits({lsb}..={msb})
    }}
"""

RS_READ_FIELD="""
    /// reads the current register value and extract field `{field}` from it
    pub fn {field}_read(&mut self) -> u{length} {{
        Self::with_reg_val().{field}_extract()
    }}
"""

RS_INSERT_FIELD="""
    /// inserts the given value `val` into the field `{field}`
    pub fn {field}_insert(&mut self, val: u{length}) -> &mut self {{
        // bits {lsb}..{msb}
        self.val.set_bits({lsb}..={msb}, val);
        self
    }}
"""

RS_WRITE_FIELD="""
    /// reads the register, updates the `{field}` field, and writes the updated value
    pub fn {field}_write(&mut self, val: u{length}) {{
        Self::with_reg_val().{field}_insert(val).write();
    }}
"""

RS_SET_FIELD="""
    /// sets the field `{field}` to the given value `val`
    pub fn {field}_write(&mut self, val: u{length}) {{
        Self::default().{field}_insert(val).write();
    }}
"""

RS_STRUCT_READ="""
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {{
        self.val = Self::reg_rawrd() & 0x{mask:x};
        self
    }}
"""

RS_STRUCT_WRITE="""
    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {{
        Self::reg_rawwr(self.val)
    }}
"""

RS_STRUCT_CURRENT="""
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  {typename} {{
        let curval = Self::reg_rawrd() & 0x{mask:x};
        {typename}(curval)
    }}
"""


# =======================================================================================
# Code Generation Register Access
# =======================================================================================

def id_to_struct_name(id):
    parts = [ p[0:1].upper() + p[1:] for p in id.split('_') ]
    return ''.join(parts)


def generate_accessor(accessor, instr, reg) :
    accfmt = instr[accessor['access_type']]
    if accfmt['type'] == 'read' :
        templ =  RS_REG_READ
    elif accfmt['type'] == 'write' :
        templ = RS_REG_WRITE
    elif accfmt['type'] == 'modify' :
        logwarn(f"modify is not handled yet! {accessor['access_type']}")
        return ""
    else :
        logerr("unknown access format type '{}'".format(accfmt['type']))
    if reg['use_encoding'] :
        encstr = "S{}_{}_C{}_C{}_{}".format(
                int(accessor['encoding']['op0'], base=2),
                int(accessor['encoding']['op1'], base=2),
                int(accessor['encoding']['crn'], base=2),
                int(accessor['encoding']['crm'], base=2),
                int(accessor['encoding']['op2'], base=2),)
    else :
        encstr = reg['id']

    encoding = accfmt['fmt'][0] + " " + ", ".join(accfmt['fmt'][1:]).format(encstr)

    return templ.format(name = reg['name'], id = reg['id'], length = reg['length'],
                                access = accessor['access_instruction'], enc = encoding)


def field_accessor_shif_mask(field) :
    if field['reserved'] :
        return (0, 0)

    mask = (1 << field['size']) - 1
    shift = field['lsb']

    return (mask, shift)


def generate_struct_field_accessor(reg, field, allmask) :
    if field['reserved'] :
        return []

    mask = (1 << field['size']) - 1
    shift = field['lsb']
    if reg['is_readable'] :
        readfield = [RS_EXTRACT_FIELD.format(field = field['id'], length = reg['length'],
                                            lsb = field['lsb'], msb = field['msb'],
                                            shift = shift, mask = mask),
                    RS_READ_FIELD.format(field = field['id'], length = reg['length'])]

    else :
        readfield = ["// no extract() method for field {}".format(field["id"])]


    if reg['is_writable'] :
        writefield = [RS_INSERT_FIELD.format(field = field['id'], length = reg['length'],
                                            lsb = field['lsb'], msb = field['msb'],
                                            shift = shift, mask = mask)]
        if reg['is_readable'] :
            writefield = writefield + [RS_WRITE_FIELD.format(field = field['id'], length = reg['length'])]
        else :
            writefield = writefield + [RS_SET_FIELD.format(field = field['id'], length = reg['length'])]
    else :
        writefield = ["// no insert() method for field {}".format(field["id"])]


    return [RS_SUBSECTION.format(header = f"Field: {field['id']}")] + readfield + writefield


def generate_struct_implementation(reg, allmask, instr) :

    typename = id_to_struct_name(reg['id'])

    regaccess = ""
    for accessor in reg['access'] :
        regaccess = regaccess + generate_accessor(accessor, instr, reg)

    if not reg['is_readable'] :
        regaccess = regaccess + "// register is not readable. not emitting read accessor\n"
    if not reg['is_writable'] :
        regaccess = regaccess + "// register is not writable. not emitting write accessor\n"


    if reg['is_readable'] :
        mcurrent = RS_STRUCT_CURRENT.format(mask = allmask, typename = typename)
        mread = RS_STRUCT_READ.format(mask = allmask)
    else :
        mcurrent = "// no current() method as it is write only"
        mread = "// no read() method as it is write only"

    if reg['is_writable'] :
        mwrite = RS_STRUCT_WRITE.format(mask = allmask)
    else :
        mwrite = "// no write() method as it is read only"

    fields = []
    for field in reg['fields'] :
        fields.extend(generate_struct_field_accessor(reg, field, allmask))

    return RS_STRUCT_IMPL.format(regid = reg['id'], default = "0", current = mcurrent, read = mread,
                                 write = mwrite, length = reg['length'], allmask = allmask,
                                 regaccess = regaccess,
                                 typename = typename, fields = "".join(fields))


def generate_register_access_file(reg, version, url, instr, outdir) :
    outfile = outdir / "{}.rs".format(reg['id'])
    log(" - generating register access file: {}".format(outfile))

    typename = id_to_struct_name(reg['id'])

    # open the rust file
    rsfile = open(outfile, "w")

    # get the current time of generation
    now = datetime.datetime.now()

    # write the module header
    rsfile.write(RS_FILE_HEADER.format(year = now.year))
    rsfile.write("use bit_field::BitField;\n\n")

    rsfile.write(RS_AUTOGEN_WARNING.format(date = now.isoformat(),
                                           version = version, source = url))
    rsfile.write(RS_REGISTER_INFO.format(name = reg['name'], id = reg['id'], length = reg['length'],
                                         group = reg['group'], desc = reg['purpose'],
                                         file = reg['file']))

    # calculate the mask of all non-reserved fields
    allmask = 0

    # emitting field accessors
    for field in reg['fields'] :
        mask, shift = field_accessor_shif_mask(field)
        # calculate the mask
        allmask = allmask | (mask << shift)

    # emitting the struct and implementation
    rsfile.write(RS_SECTION.format(header = "Data Structure Definitions"))
    rsfile.write(RS_STRUCT.format(name = reg['name'], length = reg['length'], typename = typename))
    rsfile.write(generate_struct_implementation(reg, allmask, instr))
    rsfile.flush()


def generate_register_access_mod_file(version, url, registers, outdir) :

    outfile = outdir / "mod.rs"
    modfile = open(outfile, "w")

    log(" - generating module file: {}".format(outfile))

    # get the current time of generation
    now = datetime.datetime.now()

    # write the module header
    modfile.write(RS_FILE_HEADER.format(year = now.year))
    modfile.write(RS_AUTOGEN_WARNING.format(date = now.isoformat(),
                                            version = version, source = url))

    # gather the modules
    modules = {}
    for r in registers :
        reg_group = r['group']
        if reg_group in modules:
            modules[reg_group].append((r['id'], r['name']))
        else :
            modules[reg_group] = [(r['id'], r['name'])]

    modgroups = [m for m in modules.keys()]
    modgroups.sort()
    for modgroup in modgroups:
        modfile.write(RS_SECTION.format(header = modgroup))
        mods = modules[modgroup]
        mods.sort()
        for m in mods :
            reg_id, reg_name = m
            modfile.write(f"// {reg_name}\n")
            modfile.write(f"mod {reg_id};\n")
            modfile.write(f"pub use {reg_id}::{id_to_struct_name(reg_id)};\n\n")
            modfile.flush()

    modfile.write("\n\n// end of file")
    modfile.flush()
    modfile.close()


def generate_register_access_module(sysreg, outdir) :
    log("rust: generating system register access bindings ")

    # clear the output directory for the registers
    logverbose("clearing output directory {}".format(outdir))
    shutil.rmtree(outdir, ignore_errors=True)
    outdir.mkdir(parents = True, exist_ok = True)

    # generate the module file
    generate_register_access_mod_file(sysreg['version'], sysreg['url'],
                                      sysreg["registers"], outdir)

    # generate the register files

    for reg in sysreg['registers'] :
        generate_register_access_file(reg, sysreg['version'], sysreg['url'],
                                      sysreg['instrfmt'], outdir)

# =======================================================================================
# Code Generation System Instructions
# =======================================================================================

def group_to_ident(group) :
    return group.lower().replace(" instructions", "").replace(" ", "_")

def generate_system_instr(reg, instr) :
    systeminstr = RS_SECTION.format(header = reg['name'])
    for accessor in reg['access'] :
        accfmt = instr[accessor['access_type']]
        opinstr = accessor['access_instruction']
        if accfmt['type'] != None:
            logerr("expected none, was {}".format(accfmt['type'] ))

        encstr = reg['id'].split('_')[1]

        # this is an optional argument
        if '{, <Xt>}' in opinstr :
            # template = RS_SYSINSTR_ARG + RS_SYSINSTR
            template = RS_SYSINSTR
            instrfmt = accfmt['fmt'][0] + " " + ", ".join(accfmt['fmt'][1:-1])
        else :
            if '$0' in accfmt['fmt'] :
                template = RS_SYSINSTR_ARG
                instrfmt = accfmt['fmt'][0] + " " + ", ".join(accfmt['fmt'][1:])
            else :
                template = RS_SYSINSTR
                instrfmt = accfmt['fmt'][0] + " " + ", ".join(accfmt['fmt'][1:-1])

        systeminstr = systeminstr + template.format(fn = reg['id'], desc = reg['name'],
                                                    length = reg['length'],
                                                    instr = instrfmt.format(encstr))

    return systeminstr

def generate_system_instr_file(mid, regs, version, url, instr, outdir) :
    outfile = outdir / "{}.rs".format(group_to_ident(mid))
    log(" - generating system register file: {}".format(outfile))

    # open the rust file
    rsfile = open(outfile, "w")

    # get the current time of generation
    now = datetime.datetime.now()

    # write the module header
    rsfile.write(RS_FILE_HEADER.format(year = now.year))
    rsfile.write(RS_AUTOGEN_WARNING.format(date = now.isoformat(),
                                           version = version, source = url))

    for reg in regs :
        rsfile.write(generate_system_instr(reg, instr))

    # rsfile.write(RS_REGISTER_INFO.format(name = mid, id = mid, length = reg['length'],
    #                                      group = reg['group'], desc = reg['purpose'],
    #                                      file = reg['file']))


def generate_sysinstr_mod_file(version, url, registers, outdir) :

    outfile = outdir / "mod.rs"
    modfile = open(outfile, "w")

    log(" - generating module file: {}".format(outfile))

    # get the current time of generation
    now = datetime.datetime.now()

    # write the module header
    modfile.write(RS_FILE_HEADER.format(year = now.year))
    modfile.write(RS_AUTOGEN_WARNING.format(date = now.isoformat(),
                                            version = version, source = url))

    # gather the modules
    modules = {}
    for r in registers :
        reg_group = r['group']
        modules[reg_group] = True

    modgroups = [m for m in modules.keys()]
    modgroups.sort()
    for modgroup in modgroups:
        modfile.write("\n// {}\n".format(modgroup))
        modfile.write("pub mod {};\n".format(group_to_ident(modgroup)))
        modfile.flush()

    modfile.write("\n\n// end of file")
    modfile.flush()
    modfile.close()


def generate_sysinstr(sysreg, outdir) :
    log("rust: generating system instructions bindings ")

    logverbose("clearing output directory {}".format(outdir))
    shutil.rmtree(outdir, ignore_errors=True)
    outdir.mkdir(parents = True, exist_ok = True)

    # generate the module file
    generate_sysinstr_mod_file(sysreg['version'], sysreg['url'],
                               sysreg["instructions"], outdir)

    # gather the modules
    modgroups = {}
    for r in  sysreg['instructions'] :
        if r['group'] in modgroups :
            modgroups[r['group']].append(r)
        else :
            modgroups[r['group']] = [r]

    for modgroup in modgroups:
        generate_system_instr_file(modgroup, modgroups[modgroup],
                               sysreg['version'], sysreg['url'],
                               sysreg["instrfmt"], outdir)


def generate(sysreg, outdir) :
    generate_register_access_module(sysreg, outdir / "registers")
    generate_sysinstr(sysreg, outdir / "instructions")

    try :
        cargo("fmt")
    except ProcessExecutionError as e:
        logerr("Could not format. {}".format(e.retcode))