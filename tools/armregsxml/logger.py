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
import sys
from plumbum import colors

# ---------------------------------------------------------------------------------------
# Loggin
# ---------------------------------------------------------------------------------------

VERBOSE = False

def logsetverbose(v) :
    global VERBOSE
    VERBOSE = v

def log(msg):
    print(msg)

def logverbose(msg) :
    if VERBOSE :
        log(msg)

def logok(msg) :
    print(colors.bold & colors.success | "\n" + msg),
    print(colors.bold.reset & colors.success.reset)

def logwarn(msg) :
    print(colors.bold & colors.yellow | msg, end=''),
    print(colors.bold.reset & colors.yellow.reset)

def logerr(msg) :
    print(colors.bold & colors.fatal | msg),
    print(colors.bold.reset & colors.fatal.reset)
    sys.exit(1)

def logtitle(msg) :
    print(colors.bold & colors.info | "\n" + msg)
    print('=' * len(msg))
    print(colors.bold.reset)

def logcmd(cmd, args) :
    if VERBOSE :
        print("$ {} {}".format(cmd, " ".join([str(s) for s in args])))