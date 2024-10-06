#!/bin/bash
git ls-files | egrep "^.+\.rs$|^.+\.asm$" | xargs wc -l