

# todo: at some point this should get just get deleted
# trunk is cool n' all but it seems to only target wasm?
# maybe it could be the bundler not sure.

bundle: ./target/debug/io_server react-build
	if [ ! -d "bundle" ]; then mkdir bundle; fi
	cp ./target/debug/io_server bundle
	rm -rf bundle/assets
	cp -r my-app/build/. bundle/assets

RUST_SRC := $(shell find src -name '*.rs')

./target/debug/io_server: $(RUST_SRC)
	cargo build

WEB_SRC := $(shell find my-app/public) $(shell find my-app/src)

react-build: $(WEB_SRC)
	cd my-app && npm run build 



run: bundle
	cd bundle && ./io_server
