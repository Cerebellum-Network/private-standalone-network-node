docker build . --file Dockerfile.Bin -t cere/substrate20
docker image prune -f --filter label=description="This is the temp builder."
