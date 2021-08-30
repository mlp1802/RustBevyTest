#cargo check 2>errors.txt

cargo check 2> >(grep --color=always -A 10 -B 10 error)
#cat errors.txt | grep -A 10 -B 10 error 
