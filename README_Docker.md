# -------------------- I. Build seperatly docker image for cere-substrate --------------------
### Note: All scripts command can run directly in terminal
### Phase 1: create builder image with clean temp image
```bash
./scripts/docker_create_builder.sh
```
### Phase 2: build main node and create main node image with clean temp image
```bash
./scripts/docker_build_bin.sh
```
### Run the main node with dev mode by command line:
```bash
docker run -it --name cerenode -d -p 9944:9944 -p 9933:9933 --entrypoint '/bin/sh' cerebellumnetwork/turnkey-private-blockchain-network -c '/usr/local/bin/cere --dev --ws-external'
```

# -------------------- II. Build docker image for cere-substrate --------------------
### Create builder image with clean temp image
```bash
./scripts/docker_build.sh
```
### Run the main node with dev mode by command line:
```bash
Refer I.
```

# -------------------- III. Build and run docker image for cere-substrate --------------------
### A. Build and run main node by docker-compose script
```bash
./scripts/docker_run.sh
```
### B. For blockchain development: Run substrate node without re-compiling, dev mode, expose websocket port
```bash
./scripts/docker_run.sh ./target/release/node-template --dev --ws-external
```
### C. For blockchain development: Check whether the code is compilable
```bash
./scripts/docker_run.sh cargo check
```
### D. For blockchain development: SSH to container:
```bash
docker exec -it "containerName" /bin/bash
```
