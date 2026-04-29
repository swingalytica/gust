.PHONY: build dev demo

build:
	wasm-pack build --target web --scope swingalytica

dev:
	wasm-pack build --target web --dev

demo:
	make build && cd demo && pnpm install && pnpm dev --open