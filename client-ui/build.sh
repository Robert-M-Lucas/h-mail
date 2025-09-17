cd builds &&
rm -f client-ui &&
rm -f *.deb &&
cd .. &&
npm run tauri build &&

VERSION=$(ls ../target/release/bundle/deb/ | grep -oP 'client-ui_\K[0-9]+\.[0-9]+\.[0-9]+' | sort -V | tail -n 1) &&
echo "Found build version $VERSION" &&

{
  cp ../target/release/bundle/deb/client-ui_${VERSION}_amd64/data/usr/bin/client-ui client-ui &&
  cp ../target/release/bundle/deb/client-ui_${VERSION}_amd64/data/usr/bin/client-ui builds/client-ui &&
  cp ../target/release/bundle/deb/client-ui_${VERSION}_amd64.deb builds/client-ui_${VERSION}_amd64.deb
}
