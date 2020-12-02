docker build . --file Dockerfile.Bin -t thanh/substrate20
docker image prune -f --filter label=description="This is the temp builder."