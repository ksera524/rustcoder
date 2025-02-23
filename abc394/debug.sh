sed -i 's/version = 4/version = 3/' Cargo.lock
p=$(pwd)
dir=${p##*/}  # カレントディレクトリ名を取得（フルパス不要）

cargo run --bin "${dir,,}-$1"
