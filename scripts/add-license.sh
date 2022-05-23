#!/bin/bash

SOURCES="$(find sbc* -name '*.rs')"
SOURCES=($SOURCES)

for(( i = 0; i < ${#SOURCES[@]} ; i++));do
	if ! grep -q "This file is part of the serde-bindgen-core libraries" "${SOURCES[$i]}"
	then
		echo "${SOURCES[$i]}" want copyright
		cat ./scripts/copyright.txt "${SOURCES[$i]}" > "${SOURCES[$i]}.new"
		mv "${SOURCES[$i]}.new" "${SOURCES[$i]}"
	fi
done
