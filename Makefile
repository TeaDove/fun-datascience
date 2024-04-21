.PHONY: ckeck install upload

BUILD_VERSION ?= $(shell cat VERSION)
DOCKER_IMAGE ?= ghcr.io/teadove/fun-datascience:$(BUILD_VERSION)

run:
	cargo run

docker-login:
	docker login ghcr.io

docker-buildx: docker-login
	docker buildx build --platform linux/arm64,linux/amd64 -f=Dockerfile . --tag $(DOCKER_IMAGE) --push

update:
	git pull
	docker-compose down
	docker-compose up -d
	docker-compose logs -f ds
