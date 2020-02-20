for i in *.rs
do
rustc $i --out-dir ../target
done
