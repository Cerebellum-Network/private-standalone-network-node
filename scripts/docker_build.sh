docker build . -t cerebellumnetwork/turnkey-private-blockchain-network
docker image prune -f --filter label=description="This is the build stage to create the binary."
