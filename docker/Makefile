
all: build-docker

build-backend:
	cd ../backend && cargo build --release

install-backend: build-backend
	rm -rvf bin
	mkdir -v bin
	cp -v ../target/release/backend bin/

build-frontend:
	cd ../frontend && trunk build

install-frontend: build-frontend
	rm -rvf frontend
	cp -rv	../frontend/dist frontend

build-docker: install-backend
	docker -t pengzu:latest .
