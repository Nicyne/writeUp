.PHONY: build-image
build-image:
	docker buildx build --push --platform linux/arm/v7,linux/arm64/v8,linux/amd64 --tag ghcr.io/nicyne/writeup-dev:latest .

.PHONY: docs
docs:
	cargo doc --open