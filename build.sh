cargo build --release --target asmjs-unknown-emscripten;
emcc -O3 -s EXPORTED_FUNCTIONS="['_run']" --js-library ./js/lib.js target/asmjs-unknown-emscripten/release/libvdom_compare.a -o target/asmjs-unknown-emscripten/release/libvdom_compare.js;