LIB_PATHS=boolean_algebra
EXERCICES_DIR_LIST = $(wildcard ex*/)

lib:
	echo "Building lib"
	cd boolean_algebra && cargo build

lib-release:
	echo "Building lib in release mode"
	cd boolean_algebra && cargo build --release

build-all: lib-release
	@for dir in $(EXERCICES_DIR_LIST); do \
		echo "Building $$dir"; \
		cd $$dir && cargo build; \
		cd ..; \
	done

launch-all: lib-release build-all
	@for dir in $(EXERCICES_DIR_LIST); do \
		echo "Launching $$dir"; \
		cd $$dir && cargo run; \
		cd ..; \
	done