counter=1

cd "./p${counter}"

while [ $counter -le 10 ]
do
    echo "========== PROBLEM ${counter} =========="
    cd "../p${counter}"
    cargo run
    ((counter++))
done
