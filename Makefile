LIB_PATHS=boolean_algebra

lib:
	cd boolean_algebra && cargo build

lib-release:
	cd boolean_algebra && cargo build --release