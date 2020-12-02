docker build . --file Dockerfile.Builder -t cereio/substrate20-builder
docker image prune -f --filter label=description="To create the builder image."