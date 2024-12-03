#!/bin/bash
for f in ./input/*/*.gpg
do
    name="$(dirname "$f")/$(basename "$f" .gpg)"

    if [ ! -f "${name}" ]
    then
        gpg --batch --passphrase-file .passphrase --output "${name}" --decrypt "${f}"
    fi
done
