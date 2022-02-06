all:
	EMCC_CFLAGS="-s USE_SDL=2 -s USE_SDL_TTF=2 -s INITIAL_MEMORY=67108864 -s ALLOW_MEMORY_GROWTH=1 -s ERROR_ON_UNDEFINED_SYMBOLS=0" cargo build --target wasm32-unknown-emscripten --release
	mkdir -p web
	cp src/index.html web/
	cp target/wasm32-unknown-emscripten/release/deps/weird_crash_demo.wasm web/
	cp target/wasm32-unknown-emscripten/release/deps/weird_crash_demo.js web/
	rm -f web/weird_crash_demo.data
	cd web && python3 -m http.server
