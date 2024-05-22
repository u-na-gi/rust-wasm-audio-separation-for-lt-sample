
all: build

build:
	wasm-pack build --target web
	rm -rf ../static/pkg
	cp -r pkg ../static

test:
	wasm-pack test --firefox