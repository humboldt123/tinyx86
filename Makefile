all:
	cargo xbuild --release --target target.json

clean:
	rm -rf target/
