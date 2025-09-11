cd build &&
 rm client-ui &&
 rm *.deb &&
 cd .. &&
 npm run tauri build &&
 cp ../target/release/bundle/deb/client-ui_0.1.0_amd64/data/usr/bin/client-ui client-ui &&
 cp ../target/release/bundle/deb/client-ui_0.1.0_amd64/data/usr/bin/client-ui builds/client-ui &&
 cp ../target/release/bundle/deb/client-ui_0.1.0_amd64.deb builds/client-ui_0.1.0_amd64.deb