#!/bin/bash
printf "Decrypting "
for f in ./input/*/*.gpg
do
    name="$(dirname "$f")/$(basename "$f" .gpg)"

    if [ ! -f "${name}" ]
    then
        printf "."
        gpg --batch --passphrase-file .passphrase --output "${name}" --decrypt "${f}" 2>/dev/null >/dev/null &
    fi
done

wait
sleep 1
printf " done\n"
