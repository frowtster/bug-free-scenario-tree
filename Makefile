.PHONY: \
	init \
	deps \
	clean \
	build \
	patch \
	test

init:
	rm -f go.mod
	go mod init github.com/frowtster/bug-free-scenario-tree
	go mod tidy
	go mod verify

all:
	go build -C src -o ../target/tree.x64
	GOOS=linux GOARCH=amd64 go build -C src -o ../target/tree.arm
