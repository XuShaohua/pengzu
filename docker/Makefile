CURRENT_TAG = $(shell git describe --abbrev=0 --tags)

all: push-backend-docker

build-backend:
	docker build -t xushaohua/pengzu-backend:latest -f Dockerfile.backend-multi ..
	docker tag xushaohua/pengzu-backend:latest xushaohua/pengzu-backend:$(CURRENT_TAG)

push-backend-docker: build-backend
	docker push xushaohua/pengzu-backend:latest
	docker push xushaohua/pengzu-backend:$(CURRENT_TAG)

# Build with separated stages
build-backend-bin:
	docker run --rm --user "$(shell id -u)":"$(shell id -g)" -v "${PWD}/..":/build \
		-w /build rust:bullseye cargo build --release --bin backend

install-backend-bin:
	install -m755 -v ../target/release/backend bin/

build-backend-docker: install-backend-bin
	docker build -t xushaohua/pengzu-backend:latest -f Dockerfile.backend .
	docker tag xushaohua/pengzu-backend:latest xushaohua/pengzu-backend:$(CURRENT_TAG)

# Cleanup target folder before building new version.
cleanup-target:
	rm -rf ../target || true

build-frontend:
	cd ../frontend && trunk build --release

install-frontend: build-frontend
	rm -rvf html
	cp -rv	../frontend/dist html

build-frontend-docker: install-frontend
	docker build -t xushaohua/pengzu-frontend:latest -f Dockerfile.frontend .
	docker tag xushaohua/pengzu-frontend:latest xushaohua/pengzu-frontend:$(CURRENT_TAG)

push-frontend-docker: build-frontend-docker
	docker push xushaohua/pengzu-frontend:latest
	docker push xushaohua/pengzu-frontend:$(CURRENT_TAG)
