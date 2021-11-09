#!/usr/bin/env sh

cargo run > result.txt
result=$(diff result.txt correct_output.txt)
if ["$result" == ""]; then
    echo "passing"
    exit 0
fi

echo $result
exit 1
