
sed -i "s/version = 4/version = 3/" Cargo.lock
p=$(pwd)
dir=${p##*/} 
cargo run --bin "${dir,,}-$1"

