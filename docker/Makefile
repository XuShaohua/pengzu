CURRENT_TAG = $(shell git describe --abbrev=0 --tags | sed 's/v//')

all: build-backend-docker push-backend-docker build-frontend-docker push-frontend-docker

build-backend-docker:
	sudo docker build -t xushaohua/pengzu-backend:latest -f Dockerfile.backend ..
	sudo docker tag xushaohua/pengzu-backend:latest xushaohua/pengzu-backend:$(CURRENT_TAG)
	sudo docker tag xushaohua/pengzu-backend:latest docker.biofan.org/pengzu-backend:$(CURRENT_TAG)
	sudo docker tag xushaohua/pengzu-backend:latest docker.biofan.org/pengzu-backend:latest

push-backend-docker:
	sudo docker push xushaohua/pengzu-backend:latest
	sudo docker push xushaohua/pengzu-backend:$(CURRENT_TAG)

build-frontend-docker:
	sudo docker build -t xushaohua/pengzu-frontend:latest -f Dockerfile.frontend ..
	sudo docker tag xushaohua/pengzu-frontend:latest xushaohua/pengzu-frontend:$(CURRENT_TAG)
	sudo docker tag xushaohua/pengzu-frontend:latest docker.biofan.org/pengzu-frontend:$(CURRENT_TAG)
	sudo docker tag xushaohua/pengzu-frontend:latest docker.biofan.org/pengzu-frontend:latest

push-frontend-docker:
	sudo docker push xushaohua/pengzu-frontend:latest
	sudo docker push xushaohua/pengzu-frontend:$(CURRENT_TAG)

show-tag:
	echo $(CURRENT_TAG)
