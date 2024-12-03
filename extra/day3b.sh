read -d '' script << 'EOF'
BEGIN {
  enabled=1
  sum=0
} 
{
  if ($0 == "do()") enabled = 1;
  if ($0 == "don't()") enabled = 0;
  if (enabled) sum+= $1*$2;
}
END {print sum}
EOF

cat data/inputs/03.txt |\
   grep -o -e "mul([0-9]*,[0-9]*)" -e "do()" -e "don't()" |\
   sed 's/mul(\([0-9]*\),\([0-9]*\))/\1,\2/g' |\
   awk -F"," "$script"

