#!/bin/bash

if [ ! -f "psp.json.in" ]; then
    echo "Make sure 'psp.json.in' exists." >&2
    exit 2
fi

if [ ! command -v psp-config >/dev/null 2>&1 ]; then
    echo "'psp-config' must be in your path." >&2
    exit 3
fi

PSPSDK_LIBPATH="`psp-config --pspsdk-path`"
PSPSDK_LIBPATH+="/lib/"

sed "s#\$PSPSDK_LIBPATH#$PSPSDK_LIBPATH#" psp.json.in > psp.json
