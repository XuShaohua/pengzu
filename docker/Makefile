
all: build-backend-docker

build-backend:
	cd ../backend && cargo build --release

install-backend: build-backend
	cp -v ../target/release/backend bin/

build-backend-docker: install-backend
	docker build -t pengzu-backend:latest -f Dockerfile.backend .

build-frontend:
	cd ../frontend && trunk build --release

install-frontend: build-frontend
	rm -rvf html
	cp -rv	../frontend/dist html

build-frontend-docker: install-frontend
	docker build -t pengzu-frontend:latest -f Dockerfile.frontend .
