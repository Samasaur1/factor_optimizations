NUM_WARMUPS=5
BENCHMARK_TARGETS=rust/target/release/rust rust2/target/release/rust2 rust2memoized/target/release/rust2memoized rust2precalc/target/release/rust2precalc ocaml/_build/default/bin/main.exe "python3 python/__pycache__/main.cpython-311.pyc" "python3 pythonMemoized/__pycache__/main.cpython-311.pyc" swift/.build/release/swift

.PHONY: all benchmark rust ocaml rust2 rust2memoized rust2precalc python pythonMemoized swift
all: rust ocaml rust2 rust2memoized rust2precalc python pythonMemoized swift

rust:
	cd rust && cargo build --release
rust2:
	cd rust2 && cargo build --release
rust2memoized:
	cd rust2memoized && cargo build --release
rust2precalc:
	cd rust2precalc && cargo build --release
ocaml:
	cd ocaml && dune build --release
python:
	cd python && python3 -m py_compile main.py
pythonMemoized:
	cd pythonMemoized && python3 -m py_compile main.py
swift:
	cd swift && swift build -c release

benchmark: all
	hyperfine --warmup $(NUM_WARMUPS) $(BENCHMARK_TARGETS)

clean:
	rm -rf rust/target
	rm -rf rust2/target
	rm -rf rust2memoized/target
	rm -rf rust2precalc/target
	rm -rf ocaml/_build
	rm -rf python/__pycache__
	rm -rf pythonMemoized/__pycache__
	rm -rf swift/.build
