#! /bin/bash

echo "Generating the ARMv8 register bindings."

python3 armregsxml/generate.py --help

python3 armregsxml/generate.py download
python3 armregsxml/generate parse
python3 armregsxml/generate.py generate

