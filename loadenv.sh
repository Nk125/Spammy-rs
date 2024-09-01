#!/bin/bash

if [ -z "$1" ]; then
    fpath=".env"
else
    fpath="$1"
fi

if [ ! -f "$fpath" ]; then
    echo ".env file must exist"
    exit 1
fi

echo "Loaded env variables:"

while read line; do
	if [ ${line:0:1} == "#" ]; then
		continue
	fi
    export "$line"
    key=$(echo "$line" | sed 's/=.*//')
    echo "#    $key"
done < "$fpath"

exit 0
