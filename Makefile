.PHONY: ckeck install upload

BUILD_VERSION ?= $(shell cat VERSION)
DOCKER_IMAGE ?= ghcr.io/teadove/fun-datascience:$(BUILD_VERSION)
PYTHON_PRE ?= ../.venv/bin/python

install:
	python3.11 -m venv .venv
	cd ds && $(PYTHON_PRE) -m pip install poetry
	cd ds && $(PYTHON_PRE) -m poetry update

run:
	cd ds && $(PYTHON_PRE) main.py

run-dev:
	cd ds && _UVICORN__WORKERS=1 $(PYTHON_PRE) main.py

docker-login:
	docker login ghcr.io

docker-buildx: docker-login
	docker buildx build --platform linux/arm64,linux/amd64 -f=Dockerfile . --tag $(DOCKER_IMAGE) --push

update:
	git pull
	docker-compose down
	docker-compose up -d
	docker-compose logs -f ds
