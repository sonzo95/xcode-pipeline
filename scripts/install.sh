cd "${0%/*}";
cd ..;
echo "Installing from project directory $PWD"
cargo build --release &&
rm /usr/local/bin/xccd
mv target/release/cli /usr/local/bin/xccd &&
chmod +x /usr/local/bin/xccd &&
echo "Successfully installed cli as 'xccd'"