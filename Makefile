r9cc: src/main.rs
	cargo build

test: r9cc
	./test.sh

clean:
	rm -f 9cc *.o *~ tmp*
