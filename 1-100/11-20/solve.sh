counter=11

cd "./p${counter}"

while [ $counter -le 20 ]
do
    echo "========== PROBLEM ${counter} =========="
    cd "../p${counter}"
    cargo run
    ((counter++))
done
