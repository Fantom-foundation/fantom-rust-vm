client-dev:
	cargo build
	mv target/debug/client ~/.fantom/bin/fvm-client
	chmod ugo+x ~/.fantom/bin/fvm-client
