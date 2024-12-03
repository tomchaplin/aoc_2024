cat data/inputs/03.txt | grep -o 'mul([0-9]*,[0-9]*)' | cut -c 5- | rev | cut -c 2- | rev | awk -F "," '{ sum += $1*$2 }; END { print sum }'
