npm run tauri build || exit 1;

rm -rf testing

mkdir testing
mkdir testing/u1
mkdir testing/u2

cp ../target/release/bundle/deb/client-ui_0.1.0_amd64/data/usr/bin/client-ui testing/u1/client-ui
cp ../target/release/bundle/deb/client-ui_0.1.0_amd64/data/usr/bin/client-ui testing/u2/client-ui

( cd testing/u1 && ./client-ui ) &
( cd testing/u2 && ./client-ui ) &