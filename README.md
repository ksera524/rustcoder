環境構築

1. rust-toolchainに使うRustのversionを記載して固定

2. cargo-competeのinstall
    ```bash
    sudo apt install pkg-config libssl-dev
    cargo install cargo-compete
    cargo compete init atcoder
    cargo compete login atcoder
    ```

3. 実践
    ```bash
    ./atcoder.sh abcXXX
    #test
    ./test.sh a
    #submit
    ./submit.sh a
    #debug
    ./debug.sh a
