#!/bin/bash
# SPDX-License-Identifier: AGPL-3.0-or-later
# Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

set -euo pipefail

MISSING=0

for f in $(find . -name '*.rs' -o -name '*.scm' | grep -v target/ | grep -v '/\.'); do
    if ! grep -q 'SPDX-License-Identifier: AGPL-3.0-or-later' "$f"; then
        echo "Missing SPDX header: $f"
        MISSING=$((MISSING + 1))
    fi
done

if [ "$MISSING" -gt 0 ]; then
    echo "FAIL: $MISSING file(s) missing SPDX header"
    exit 1
fi

echo "OK: all source files have SPDX header"
