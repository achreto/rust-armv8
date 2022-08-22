#! /usr/bin/env python3

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
import sys
import argparse
import json
import pathlib

import shutil  # to copy files


from plumbum import local
from plumbum.cmd import wget, mkdir, tar
from plumbum.commands import ProcessExecutionError


from logger import logverbose, logok, logwarn, logerr, logsetverbose, logtitle, log, logcmd
import rustgen
from armsysregxml import parse_xml_file, parse_xml_instructions


# ---------------------------------------------------------------------------------------
# STATIC CONFIGURATION
# ---------------------------------------------------------------------------------------

DEFAULT_ARCH = "aarch64"

# this is the script path, i.e. the repository root
SCRIPT_ROOT = pathlib.Path(os.path.dirname(os.path.realpath(__file__)))

# get the root of the repository
REPO_ROOT = SCRIPT_ROOT.parent.parent

# the configuration file
FILE_CONFIG = SCRIPT_ROOT / "config.json"

# the data directory holding the XML specifications
DIR_DATA = SCRIPT_ROOT / ".data"

# the output directory to store the generated files
DIR_OUTPUT = REPO_ROOT / "src" / DEFAULT_ARCH


def get_reg_path(args):
    return  pathlib.Path(args.dst) / "registers "

def get_instr_path(args):
    return  pathlib.Path(args.dst) / "instructions "


# =======================================================================================
# Generate Command
# =======================================================================================


CODEGENS = ["rust"]

def generate(cfg, args) :
    logtitle("Generating AArch64 register bindings for {}".format(args.target))

    jsonfile = DIR_DATA / args.version / "sysreg.json"
    if not jsonfile.is_file() :
        logwarn("json file is not present, trying to generated")
        do_parse(cfg, args)

    # load the json file
    sysregdata = None
    try :
        with open(jsonfile) as sysregjson:
            sysregdata = json.load(sysregjson)
    except:
        logerr("Failed to load configuraton file.")

    if args.target == 'rust' :
        rustgen.generate(sysregdata, DIR_OUTPUT)
    else :
        logerr("unknown target {}".format(args.target))

    logok("Generated access code for {}".format(args.target))

# =======================================================================================
# Parsing the Arm System Register Files
# =======================================================================================

def do_parse(cfg, args) :
    version = cfg["versions"][args.version]

    log("Version: {}".format(args.version))

    # check if the data directory exists, if not download files
    xmldir = DIR_DATA / args.version
    if not xmldir.is_dir() :
        logwarn("xml files are not present, trying to download...")
        do_download(cfg, args)

    # get the instructions
    instrxmlfile = DIR_DATA / args.version / version["instructions"]
    instr = parse_xml_instructions(cfg, args, instrxmlfile)

    registers = []
    sysinstr = []

    xmldir = xmlfilepath = DIR_DATA / args.version
    for f in os.listdir(xmldir) :
        # skip non xml files
        if not f.endswith(".xml") :
            continue
        # skip the instructions file
        if f == version["instructions"] :
            continue
        # dskip ignored files
        if f in version['filemap']['ignored'] :
            continue
        # skip
        use_encoding = False
        if f in version["filemap"]['useencoding'] :
            use_encoding = True


        xmlfilepath = xmldir / f
        (parsed_registers, parsed_others) = parse_xml_file(cfg, args, instr, xmlfilepath)
        for pr in parsed_registers :
            pr['use_encoding'] = use_encoding
            registers.append(pr)
        for po in parsed_others :
            po['use_encoding'] = use_encoding
            sysinstr.append(po)


    registers.sort(key = lambda x: x['id'])
    sysinstr.sort(key = lambda x: x['id'])

    # the json file for the parsed output
    jsonfile = DIR_DATA / args.version / "sysreg.json"
    with open(jsonfile, "w") as write_file:
        jsondata = {
            'instrfmt' : instr,
            'version' : args.version,
            'url' : version['url'],
            'registers' : registers,
            'instructions' : sysinstr
        }
        try :
            json.dump(jsondata, write_file, indent=2)
        except :
            logerr("could not store the sysregs json file")


def parse(cfg, args) :
    logtitle("Generating AArch64 Registers")
    do_parse(cfg, args)
    logok("OK. AArch64 registers parsed.")


# =======================================================================================
# Download Command
# =======================================================================================


# extracting it in the output directory using `tar`
def extract_tar(tarfile, outdir) :
    try :
        with local.cwd(outdir):
            log("Extracting archive...")
            tarargs = ["-xf", tarfile]
            logcmd("tar", tarargs)
            tar(*tarargs)
    except ProcessExecutionError as e:
        logerr("Could not extract file: {}".format(e.retcode))

# downloading it with `wget` tools
def download_file(url, outfile) :
    if outfile.is_file() :
        log("Skipping download, as file already exists")
        return

    try :
        log("Downloading file...")
        wgetargs = ["-O", outfile, url]
        logcmd("wget", wgetargs)
        wget(*wgetargs)
    except ProcessExecutionError as e:
        logerr("Could not download file: {}".format(e.errno))

# copy all xml files into the data directory
def copy_xml_files(srcdir, dstdir, files) :
    for f in files :
        logverbose(" - {} -> {}".format(srcdir / f, dstdir / f))
        shutil.copy2(str(srcdir / f), str(dstdir))

# handle the downloads
def do_download(cfg, args) :
    version = cfg["versions"][args.version]
    log("Downloading AArch64 Registers Version '{}'".format(args.version))
    log("URL: {}\n".format(version["url"]))

    # create the output directory
    outdir = DIR_DATA / args.version / "download"
    outdir.mkdir(0o755, parents = True, exist_ok = True)

    # this is the file where the downlaoded register specs are stored
    outfile = outdir / "sysregs.tar.gz"

    # download the Arm  Architecture System Registers
    download_file(version["url"], outfile)

    # extract the downloaded tar archive
    extract_tar(outfile, outdir)

    # copy the XML files of interest into the data directory
    xmldir = outdir / version["subfolder"]
    dstdir = DIR_DATA / args.version

    files = [version["instructions"]]
    for f in os.listdir(xmldir) :
        if f.startswith("AArch64") :
            files.append(f)

    log("Copying XML files")
    copy_xml_files(xmldir, dstdir, files)

# handler for the download command
def download(cfg, args) :
    logtitle("Downloading AArch64 Register Descriptions")
    do_download(cfg, args)
    logok("OK. AArch64 Register Descriptions downloaded")


# =======================================================================================
# Main Function
# =======================================================================================


# the commands map
commands = {
    'download' : download,
    'parse' : parse,
    'generate' : generate,
}

def main() :

    # load the configuration file
    cfg = None
    try :
        with open(FILE_CONFIG) as cfgfile:
            cfg = json.load(cfgfile)
    except:
        logerr("Failed to load configuraton file.")

    # setting up argsparse
    parser = argparse.ArgumentParser(description='Building AArch64 Registers bindings')

    parser.add_argument('command',  metavar='cmd', choices=commands.keys(),
                        help='The command to be executed {}'.format([s for s in commands.keys()]))

    versions = [str(v) for v in cfg["versions"].keys()]
    parser.add_argument('--version',  metavar='version', choices=versions, default=cfg["default"],
                        help='The version to build for {}'.format(versions))

    parser.add_argument("--arch", metavar="arch", choices=["aarch64", "aarch32"], default=DEFAULT_ARCH,
                        help=f"The architecture to build for (aarch64 or aarch32), default: {DEFAULT_ARCH}")

    parser.add_argument("--dstdir", metavar="dstdir", default=DIR_OUTPUT,
                        help=f"the output directory, defaults to {DIR_OUTPUT}")

    parser.add_argument('--target',  metavar='target', choices=CODEGENS, default=CODEGENS[0],
                        help=f"The version to build for {CODEGENS}")

    parser.add_argument('--verbose', default=False, action='store_true',
                        help='print commands before executing them')

    args = parser.parse_args()

    # set the global VERBOSE argument
    logsetverbose(args.verbose)

    # run the command
    commands[args.command](cfg, args)


if __name__ == '__main__':
    main()