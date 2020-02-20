FILE=../target
if test -f "$FILE"; then
    echo "$FILE exist"
fi


for i in *.rs
do
rustc $i --out-dir ../target
done
