.PHONY: build dev demo

build:
	wasm-pack build --target web

dev:
	wasm-pack build --target web --dev

demo:
	wasm-pack build --target web && cd demo && pnpm install && pnpm dev --open