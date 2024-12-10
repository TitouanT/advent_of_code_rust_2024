#!/bin/bash
year=2024
day=$1
url="https://adventofcode.com/${year}/day/${day}"


curl200 () {
    . .aoc_cookie
    http_code=$(curl --silent --output "${1}" --cookie "${cookie}" --write-out "%{http_code}" "${2}" 2> /dev/null)
    if [ ! "${http_code}" = "200" ]
    then
        rm "${1}"
        return 1
    else
        return 0
    fi
}

mkdir -p input/${year}
mkdir -p html
dayinput="input/${year}/day${day}.txt"
daypart1="input/${year}/day${day}.output.1.txt"
daypart2="input/${year}/day${day}.output.2.txt"
page="html/day${day}.html"
if [ ! -f "${dayinput}" ]
then
    curl200 "${dayinput}" "${url}/input" || exit 1
fi

if [ ! -f "${daypart1}" ] || [ ! -f "${daypart2}" ]
then
    curl200 "${page}" "${url}" || exit 1
    pup -p 'article:nth-of-type(1) + p > code text{}' < "${page}" > "${daypart1}"
    pup -p 'article:nth-of-type(2) + p > code text{}' < "${page}" > "${daypart2}"

    # remove uncompleted parts
    [ ! -s "${daypart1}" ] && rm "${daypart1}"
    [ ! -s "${daypart2}" ] && rm "${daypart2}"
fi


