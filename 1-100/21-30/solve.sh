counter=21

cd "./p${counter}"

while [ $counter -le 30 ]
do
    echo "========== PROBLEM ${counter} =========="
    cd "../p${counter}"
    cargo run
    ((counter++))
done
