
all: build-docker

build-backend:
	cd ../backend && cargo build --release

install-backend: build-backend
	cp -v ../target/release/backend bin/

build-frontend:
	cd ../frontend && trunk build

install-frontend: build-frontend
	rm -rvf html
	cp -rv	../frontend/dist html

build-docker: install-backend install-frontend
	docker build -t pengzu:latest -f Dockerfile .
