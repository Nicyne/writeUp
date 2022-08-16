.PHONY: build-image
build:
	docker buildx build --push --platform linux/arm/v7,linux/arm64/v8,linux/amd64 --tag ghcr.io/nicyne/writeup-dev:latest .