SHELL = /bin/bash
current_dir = $(shell pwd)

test:
	@cargo +nightly test

bench:
	@cargo +nightly bench


wheels:
	@ docker run --rm -v $(current_dir):/io quay.io/pypa/manylinux1_x86_64 /io/build-wheels.sh

