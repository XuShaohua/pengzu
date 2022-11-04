CURRENT_TAG = $(shell git describe --abbrev=0 --tags)

all: cleanup-target push-backend-docker push-frontend-docker

build-backend-docker:
	docker build -t xushaohua/pengzu-backend:latest -f Dockerfile.backend ..
	docker tag xushaohua/pengzu-backend:latest xushaohua/pengzu-backend:$(CURRENT_TAG)

push-backend-docker: build-backend-docker
	docker push xushaohua/pengzu-backend:latest
	docker push xushaohua/pengzu-backend:$(CURRENT_TAG)

build-frontend-docker:
	docker build -t xushaohua/pengzu-frontend:latest -f Dockerfile.frontend ..
	docker tag xushaohua/pengzu-frontend:latest xushaohua/pengzu-frontend:$(CURRENT_TAG)

push-frontend-docker: build-frontend-docker
	docker push xushaohua/pengzu-frontend:latest
	docker push xushaohua/pengzu-frontend:$(CURRENT_TAG)

# Cleanup target folder before building new version.
cleanup-target:
	rm -rf ../target || true
