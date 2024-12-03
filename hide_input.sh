#!/bin/bash

for f in ./input/*/*.txt
do

    if [ ! -f "${f}.gpg" ]
    then
        echo "hiding <$f>"
        gpg --batch --passphrase-file .passphrase --symmetric --cipher-algo AES256 "${f}"
    fi
done

