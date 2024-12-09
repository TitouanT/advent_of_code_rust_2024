#!/bin/bash
printf "Decrypting "
for f in $(find ./input -name "*.gpg")
do
    name="$(dirname "$f")/$(basename "$f" .gpg)"

    if [ ! -f "${name}" ]
    then
        printf "."
        gpg --batch --passphrase-file .passphrase --output "${name}" --decrypt "${f}" 2>/dev/null >/dev/null &
    fi
done

wait
printf " done\n"
ls ./input/2024/
