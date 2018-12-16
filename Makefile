client-dev:
	cargo build
	mv target/debug/client /usr/local/bin/fvm-client
	chmod ugo+x /usr/local/bin/fvm-client