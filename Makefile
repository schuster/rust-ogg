all: libogg

libogg: src/lib.rs
	mkdir -p target
	rustc --crate-type lib --out-dir target src/lib.rs

examples: libogg examples/*.rs
	rustc -L target --out-dir examples examples/*.rs

test:
	mkdir -p bin
	rustc --test -o bin/test src/lib.rs
	./bin/test
