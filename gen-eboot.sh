#!/bin/bash

psp-fixup-imports target/psp/psp-hello || exit 2
psp-prxgen target/psp/psp-hello target/psp/psp-hello.prx || exit 3
mksfo "Hello World" target/psp/PARAM.SFO || exit 4
pack-pbp target/psp/EBOOT.PBP target/psp/PARAM.SFO NULL NULL NULL NULL NULL target/psp/psp-hello.prx NULL || exit 5
