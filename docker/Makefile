
all: build-docker

build-backend:
	cd ../backend && cargo build --release

install-backend: build-backend
	cp -v ../target/release/backend bin/
	cp -v ../target/release/import_categories bin/
	cp -v ../target/release/import_library bin/

build-frontend:
	cd ../frontend && trunk build

install-frontend: build-frontend
	rm -rvf frontend
	cp -rv	../frontend/dist frontend

build-backend-docker: install-backend
	docker build -t pengzu-backend:latest -f Dockerfile.backend .

build-frontend-docker: install-frontend
	docker build -t pengzu-frontend:latest -f Dockerfile.frontend .
