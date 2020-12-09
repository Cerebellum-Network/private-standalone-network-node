docker build . -t cere/substrate20
docker image prune -f --filter label=description="This is the build stage to create the binary."
