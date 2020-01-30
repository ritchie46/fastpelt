SHELL = /bin/bash

test:
	@cargo +nightly test

bench:
	@cargo +nightly bench