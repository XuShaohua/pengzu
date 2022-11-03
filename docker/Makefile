CURRENT_TAG = $(shell git describe --abbrev=0 --tags)

all: build-backend-docker

build-backend:
	cd ../backend && cargo build --release

install-backend: build-backend
	cp -v ../target/release/backend bin/

build-backend-docker: install-backend
	docker build -t xushaohua/pengzu-backend:latest -f Dockerfile.backend .
	docker tag xushaohua/pengzu-backend:latest xushaohua/pengzu-backend:$(CURRENT_TAG)

build-frontend:
	cd ../frontend && trunk build --release

install-frontend: build-frontend
	rm -rvf html
	cp -rv	../frontend/dist html

build-frontend-docker: install-frontend
	docker build -t xushaohua/pengzu-frontend:latest -f Dockerfile.frontend .
	docker tag xushaohua/pengzu-frontend:latest xushaohua/pengzu-frontend:$(CURRENT_TAG)
