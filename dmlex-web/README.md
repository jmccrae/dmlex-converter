DMLEX Converter (Web version)
============================

To update the release do the following

```bash
cd ..
cargo build --release
cd dmlex-web
trunk build --release --public-url=https://john.mccr.ae/dmlex-converter/
sed -i 's/\/http/http/g' dist/index.html
```
