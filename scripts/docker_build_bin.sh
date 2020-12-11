docker build . --file Dockerfile.Bin -t cerebellumnetwork/turnkey-private-blockchain-network
docker image prune -f --filter label=description="This is the temp builder."
