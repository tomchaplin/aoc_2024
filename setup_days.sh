#!/bin/bash
digits=00

for day in $(seq 2 25);
do
  padded_day=${digits:${#day}:${#digits}}${day}
  cp "src/days/day01.rs" "src/days/day${padded_day}.rs"
done

