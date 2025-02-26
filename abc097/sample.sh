
sed -i 's/version = 4/version = 3/' Cargo.lock
cargo compete test $1

