alias b := build

default: lint

# build main
build:
    cargo lambda build --release

# build main
deploy: build
    cargo publish

# run the dev server
lint:
    cargo watch -x clippy

# run the dev server
test:
    cargo watch -x test

# check rust files format
check:
    cargo fmt -v --check

# format rust files
format:
    cargo fmt -v
