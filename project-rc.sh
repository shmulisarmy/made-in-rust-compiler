f() { 
    touch "src/$1.rs"
    echo "mod $1;" | cat - src/main.rs > temp && mv temp src/main.rs
}