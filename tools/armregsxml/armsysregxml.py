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

import os
import re
import xml.etree.ElementTree as ET

from logger import logverbose, logok, logwarn, logerr, log

# ---------------------------------------------------------------------------------------
# Patterns for Extracting Values
# ---------------------------------------------------------------------------------------

identpattern = re.compile('[^a-z0-9_]+')

def filterident(ident) :
    return identpattern.sub('', ident.replace(" ", "_").lower())


def parse_field_node(field_node, length) :
    # <field ..>
    #     <field_msb>63</field_msb>
    #     <field_lsb>32</field_lsb>
    #     <field_description order="before">
    #     ...
    #     </field_description>
    # </field>
    if field_node is None :
        logerr("    - expected a field node, but was None!")
        return None

    field_msb = int(field_node.find('field_msb').text)
    field_lsb =  int(field_node.find('field_lsb').text)
    field_size = field_msb - field_lsb + 1

    field_name_node = field_node.find('field_name')
    field_id = filterident(field_node.attrib['id'])
    if field_id == "none_{}_0".format(length - 1) :
        field_id = 'val'

    if field_name_node is None :
        field_name = field_id
    else :
        field_name = field_name_node.text



    is_reserved = False
    desc = field_node.find('field_description/para')
    if desc is not None:
        if desc.text is not None:
            is_reserved = desc.text.startswith("Reserved")

    logverbose("     - {} [{}..{}] ({} bits)".format(field_id, field_lsb, field_msb, field_size))

    return {
        "id" : field_id,
        "name" : field_name,
        "msb" : field_msb,
        "lsb" : field_lsb,
        "size" : field_size,
        "reserved" : is_reserved
    }


def parse_reg_fieldsets_node(reg_fieldsets_node, is_register) :
    # <reg_fieldsets>
    #     ...
    #     <fields length="64">
    #         <field ..>
    #             <field_msb>63</field_msb>
    #             <field_lsb>32</field_lsb>
    #             <field_description order="before">
    #             ...
    #             </field_description>
    #         </field>
    #     </fields>
    #     ...
    #     <reg_fieldset length="64">
    #        <fieldat id="0_63_32" lsb="32" msb="63"/>
    #        ...
    #     </reg_fieldset>
    #     ...
    # </reg_fieldsets>

    if reg_fieldsets_node == None:
        logerr("   - expected a reg_fieldsets  node, but was None!")
        return None

    # here there can be multiple fields/fieldset definitions. we take the one for aarc64
    fields_nodes = []
    for fields_node in reg_fieldsets_node.findall("fields") :
        fields_condition = fields_node.find("fields_condition")
        if fields_condition is None:
            fields_nodes.append(fields_node)
            continue
        if fields_condition.text is None:
            fields_nodes.append(fields_node)
            continue
        # do not take anything from AAarch32
        if "from AArch32" in fields_condition.text or \
            "to AArch32" in fields_condition.text or \
            "using AArch32" in fields_condition.text:
            continue
        if fields_condition.text.endswith("== 0"):
            continue

        if "AArch32" in fields_condition.text :
            logwarn('    - had aarch32')

        fields_nodes.append(fields_node)


    # we have no fields, probably this is a cache
    if len(fields_nodes) == 0:
        if not is_register:
            return (0, None)
        else :
            logerr("    - expected fields_nodes to have exactly one element, had 0")

    if len(fields_nodes) != 1 :
        logwarn("    - expected fields_nodes to have exactly one element, had {}".format(len(fields_nodes) ))

    fields_node = fields_nodes[0]
    length = int(fields_node.attrib["length"])

    fields = []
    for field_node in fields_node.findall("field") :
        field = parse_field_node(field_node, length)
        fields.append(field)

    return (length, fields)

def accessor_type(accessor) :
    fst = accessor.split(" ")[0].lower()
    # need to rewrite some of the access types to match known instructions
    if fst == "msrregister":
        return 'msr_reg'
    if fst == "msrimmediate" :
        return "msr_imm"
    return fst


def parse_access_mechanisms_node(regname, access_mechanisms_node, instr) :
    # <access_mechanisms>
    #     <access_mechanism accessor="...">
    #         <encoding>
    #             <access_instruction>...</access_instruction>
    #             <enc n=".." v=".."/>
    #             <enc n=".." v=".."/>
    #             <enc n=".." v=".."/>
    #             <enc n=".." v=".."/>
    #             <enc n=".." v=".."/>
    #         </encoding>
    #         ...
    #     </access_mechanism>
    #     ...
    # </access_mechanisms>

    if access_mechanisms_node == None:
        logerr("expected a access_mechanisms_node node, but was None!")
        return None

    is_writable = False
    is_readable = False
    access_mechanisms = []
    for access_machism in access_mechanisms_node.findall("access_mechanism") :

        # getting the encoding node
        encoding_node = access_machism.find("encoding")
        if encoding_node is None:
            logerr("expected finding an encoding node, but was none")

        # get the accessor node and extract the accessor type
        accessor = access_machism.attrib["accessor"]
        accesstype = accessor_type(accessor)

        # the accessor type should be in the known instructions
        if (accesstype not in instr) :
            logerr("accessor type '{}' not found in {}".format(accesstype, instr.keys()))

        # check if the register is writable or readable
        access_instr = instr[accesstype]
        if access_instr["type"] == "write" :
            is_writable = True

        if access_instr["type"] == "read" :
            is_readable = True

        # get the access_instruction node within
        access_instruction = encoding_node.find("access_instruction")
        if access_instruction is None:
            logerr("expected finding an access_instruction node, but was none")

        # skip those accessors with two execution levels
        access_ident = filterident(access_instruction.text)
        if "_el12" in access_ident :
            continue
        if "_el02" in access_ident :
            continue
        # skipping the nxs variants of those accesses
        if "tlbi" in access_ident and "nxs" in access_ident :
            continue
        regname = regname.replace("icv", 'icc')
        if "_el" in regname and regname not in access_ident :
            #logwarn("XXX {}  {}".format(regname, access_ident))
            continue


        encodings = {}
        for enc in encoding_node.findall("enc") :
            n = enc.attrib["n"]
            v = enc.attrib["v"]
            encodings[n.lower()] = v

        access_mechanisms.append({
            "access_instruction" : access_instruction.text,
            "access_type" : accesstype,
            "encoding" : encodings
        })

    if len(access_mechanisms) == 0:
        logerr("no access mechanisms present")

    return (is_writable, is_readable, access_mechanisms)


def parse_reg_groups_node(reg_groups_node) :
    # layout of the reg_groups node
    # <reg_groups>
    #     <reg_group>...</reg_group>
    #     ...
    # </reg_groups>

    if reg_groups_node == None:
        logerr("expected a reg_groups node, but was None!")
        return None

    reg_groups = []
    for reg_group in reg_groups_node.findall("reg_group") :
        reg_groups.append(reg_group.text)

    if len(reg_groups) == 1 :
        return reg_groups[0]
    if len(reg_groups) == 0 :
        logerr("   - expected at least one reg group!")

    reg_groups_filtered = []
    reg_groups_discarded = []
    for reg_group in reg_groups :
        if reg_group in ['IMPLEMENTATION DEFINED', 'Virtualization registers'] :
            reg_groups_discarded.append(reg_group)
        else :
            reg_groups_filtered.append(reg_group)

    if len(reg_groups_filtered) == 1:
        return reg_groups_filtered[0]
    if len(reg_groups_filtered) > 2 :
        print("filtered", reg_groups_filtered)
        print("discarded", reg_groups_discarded)
        return reg_groups_filtered[0]

    return 'Virtualization registers'



def parse_reg_purpose_node(reg_purpose_node) :
    # layout of the reg_purpose node
    #
    # <reg_purpose>
    #     <purpose_text>
    #         <para>...</para>
    #     </purpose_text>
    # </reg_purpose>

    if reg_purpose_node == None:
        logerr("   - expected a reg_purpose node, but was None!")
        return None

    para_node = reg_purpose_node.find("purpose_text/para")
    if para_node is None:
        logerr('  - expected a `reg_purpose` node in the register description. found none.')

    return para_node.text


def parse_register_node(cfg, args, instr, reg_node) :

    # the structure of the register node is as follows:
    #
    # <register execution_state="AArch64" is_register="True" is_internal="True" is_banked="False" is_stub_entry="False">
    #   <reg_short_name>SPSR_EL1</reg_short_name>
    #   <reg_long_name>Saved Program Status Register (EL1)</reg_long_name>
    #   <reg_reset_value></reg_reset_value>
    #   <reg_mappings> .. </reg_mappings>
    #   <reg_purpose> .. </reg_purpose>
    #   <reg_groups> .. </reg_groups>
    #   <reg_configuration></reg_configuration>
    #   <reg_attributes> .. </reg_attributes>
    #   <reg_fieldsets> .. </reg_fieldsets>
    #   <access_mechanisms> .. </access_mechanisms>
    #   <arch_variants> .. </arch_variants>
    # </register>

    # extract the `reg_short_name` node text, remove all non-identifier-compatible args
    reg_id = filterident(reg_node.find("reg_short_name").text)

    # extract the long name from the register node
    reg_name = reg_node.find("reg_long_name").text

    if "execution_state" not in reg_node.attrib:
        logerr(" @ Register: {} ({}) has no execution state, skipping.".format(reg_name, reg_id))
        return None

    if reg_node.attrib["execution_state"] != "AArch64" :
        logerr(" @ Register: {} ({}) is not AArch64, skipping.".format(reg_name, reg_id))
        return None

    # is this a register description or an instruction?
    is_register = (reg_node.attrib["is_register"] == "True")

    if is_register:
        log(" @ Parsing register description: {} ({})".format(reg_name, reg_id))
    else :
        log(" @ Parsing instruction description: {} ({})".format(reg_name, reg_id))

    # reg_reset_value seems to be never set, skipping it
    #if len (reg_node.find("reg_reset_value").getchildren()) > 0 :
    #    logerr("reg_reset_value was not empty")

    # reg_mappings won't be used later on as they describe mapping onto AArch32
    # if len (reg_node.find("reg_mappings").getchildren()) > 0 :
    #     logerr("reg_mappings was not empty")

    # extract the purpose of this register
    reg_purpose = parse_reg_purpose_node(reg_node.find("reg_purpose"))

    # get the register groups
    reg_group = parse_reg_groups_node(reg_node.find("reg_groups"))

    # reg_configuration won't be used
    # if len (reg_node.find("reg_configuration").getchildren()) > 0 :
    #     logerr("reg_configuration was not empty")

    # reg_attributes won't be used
    # if len (reg_node.find("reg_attributes").getchildren()) > 0 :
    #     logwarn("reg_attributes was not empty")

    # extract the reg_fieldsets
    length, reg_fieldsets = parse_reg_fieldsets_node(reg_node.find("reg_fieldsets"), is_register)

    # extract the access_mechanisms
    parsed_access_mechanisms = parse_access_mechanisms_node(reg_id, reg_node.find("access_mechanisms"), instr)
    reg_writable = parsed_access_mechanisms[0]
    reg_readable = parsed_access_mechanisms[1]
    access_mechanisms = parsed_access_mechanisms[2]

    # arch_variants won't be used they describe which architecture version e.g. 8.7a supports it
    # if len (reg_node.find("arch_variants").getchildren()) > 0 :
    #     logerr("arch_variants was not empty")

    log("   + Length: {}".format(length))
    log("   + Access: Writable: {}, Readable: {}".format(reg_writable, reg_readable))
    log("   + Register: {}".format("yes" if is_register else "no"))
    log("   + Group: {}".format(reg_group))
    # the parsed register data.
    regdata = {
        "id" : reg_id,
        "name" : reg_name,
        "length" : length,
        "is_register" : is_register,
        "is_writable" : reg_writable,
        "is_readable" : reg_readable,
        "purpose" : reg_purpose,
        "group" : reg_group,
        "access" : access_mechanisms,
        "fields" : reg_fieldsets,
    }

    return regdata


# parse an Arm Architecture System Registers description XML file
def parse_xml_file(cfg, args, instr, xmlfile) :
    log("\nRegister File: {}".format(xmlfile))


    # the parsed register list of the file
    parsed_registers = []
    parsed_sysinstr = []

    # parse the xml file
    root = ET.parse(xmlfile).getroot()

    # the structure of the file is as follows:
    #
    # <register_page>
    #   <registers>
    #     <register> ... </register>
    #     <register> ... </register>
    #   </registers>
    # </register_page>

    # loop over all XML nodes `register`, and parse them
    for reg_node in root.findall(".//register"):
        res = parse_register_node(cfg, args, instr, reg_node)
        res["file"] = os.path.basename(xmlfile)
        if res["is_register"] :
            parsed_registers.append(res)
        else :
            parsed_sysinstr.append(res)

    # return the list of parsed registers
    return (parsed_registers, parsed_sysinstr)


# this function parses the instructions file to obtain the way the instructions are formatted
def parse_xml_instructions(cfg, args, xmlfile) :

    # start off with the default access instructins from the config
    accessinstr = cfg["accessinstructions"]

    # if there is no instructions file, take the ones from the configurations
    if not xmlfile.is_file() :
        log("Using default access instructions from config file")
        return accessinstr

    log("Parsing instructions file: {}".format(xmlfile))

    # the structure of the file is as follows
    #
    # <access_instruction_defs>
    #   <access_instruction_def execution_state="AArch64" id="MRS">
    #     <name>MRS</name>
    #     <access_type type="read"/>
    #     <access_syntax name="MRS">
    #       <var n="Xt"/>
    #       <var n="systemreg"/>
    #     </access_syntax>
    #   </access_instruction_def>
    # </access_instruction_defs>

    # get the root of the XML tree
    root = ET.parse(xmlfile).getroot()

    for ai_node in root.findall(".//access_instruction_def"):
        # we only care about aarch64
        if ai_node.attrib["execution_state"].lower() != "aarch64" :
            continue

        # access name should always be present
        access_name = ai_node.find("name").text.lower()

        # the id should always be present
        access_id = ai_node.attrib["id"].lower()

        # the acces type might not be there (e.g. for TLBI instructions)
        acc_type_node = ai_node.find("access_type")
        if acc_type_node is not None:
            access_type = acc_type_node.attrib["type"].lower()
        else :
            access_type = None

        # getting the access syntax, there might be multiple variants
        as_fmt = None
        as_syntax = None
        for as_node in ai_node.findall("access_syntax") :
            # skip the no input variants
            if "variant" in as_node.attrib:
                continue

            # get the instruction name
            as_fmt = [as_node.attrib["name"].lower()]
            as_syntax = [as_node.attrib["name"].lower()]

            # get the variables / parameters
            for var_node in as_node.findall("var") :
                n = var_node.attrib["n"]
                as_syntax.append(n)

                # if n = 'Xt' this is a generic register, this gets replaced by asm '$0'
                if n == 'Xt' :
                    n = "$0"
                elif n == 'imm' :
                    n = "#{imm}"
                # if n = 'systemreg' this means we need to replace it with the register name
                elif n in ["systemreg", "dc_op", "pstatefield", "ic_op",
                           "at_op", "tlbi_op", "cfp_op", "cpp_op", "dvp_op", "op1", "op2"] :
                    n = "{}"
                elif n in ["Cn", "Cm"]  :
                    prefix = var_node.attrib["prefix"]
                    n = prefix + "{}"
                else :
                    logwarn("Unhandled access method: {}".format(n))
                as_fmt.append(n)

        accessinstr[access_id] = {
            "name" : access_name,
            "type" : access_type,
            "fmt"  : as_fmt,
            "syntax" : as_syntax
        }

    return accessinstr