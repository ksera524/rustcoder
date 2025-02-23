# make contest directory and files
cargo compete new $1
cd $1

# make settings.json
mkdir .vscode
touch .vscode/settings.json 
echo '{
    "editor.formatOnSave": true,
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.checkOnSave.extraArgs": ["--", "-A", "clippy::needless_return"]
}
' >> .vscode/settings.json

# make test.sh
touch test.sh
echo '
sed -i 's/version = 4/version = 3/' Cargo.lock
cargo compete test $1
' >> test.sh

#make submit.sh
touch submit.sh
echo '
sed -i 's/version = 4/version = 3/' Cargo.lock
cargo compete submit $1
' >> submit.sh

# add execute permission to test.sh and submit.sh
chmod +x test.sh submit.sh

# add opt-level to Cargo.toml
echo '
[profile.dev]
opt-level = 3' >> Cargo.toml

# open vscode
code .
