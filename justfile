#!/usr/bin/env just --justfile
# ^ A shebang isn't required, but allows a justfile to be executed
#   like a script, with `./justfile test`, for example.

# https://github.com/casey/just

# VARIABLES
application := "fit2csv"

# ALIASES
alias b := build

# SHORTCUTS AND COMMANDS

# Builds and documents the project - Default; runs if nothing else is specified
@default: check

# Check if it builds at all
@check:
    cargo lcheck  --color 'always'

# Only compiles the project
@build:
   cargo lbuild --color 'always'

# Cleans and builds again
@rebuild:
    cargo clean
    cargo lbuild --color 'always'

# Cleans up the project directory
@clean:
    cargo clean
    -rm tree.txt
    -rm graph.png

# Documents the project, builds and installs the release version, and cleans up
@release:
    cargo lbuild --release  --color 'always'
    cargo strip
    cp {{invocation_directory()}}/target/release/{{application}} /usr/local/bin/
    cargo clean

# Documents the project
@doc:
    cargo fmt -- --emit=files
    cargo doc --no-deps
    cargo depgraph | dot -Tpng > graph.png
    cargo tree > tree.txt
    tokei
    cargo outdated

# Documents the project and all dependencies
@docs:
    cargo fmt -- --emit=files
    cargo doc
    cargo depgraph | dot -Tpng > graph.png
    cargo tree > tree.txt
    tokei
# Tests the project
@test:
    cargo test

# Checks the project for inefficiencies and bloat
@inspect: doc lint
    cargo geiger
    cargo audit
    cargo bloat

# Checks for potential code improvements
@lint:
    cargo lclippy

# Builds (if necessary) and runs the project
@run:
    cargo lrun  --color 'always'

# Build and run with a --help parameter
@runh:
    cargo lrun  --color 'always' -- --help

# Build and run with a --debug parameter
@rund:
    export RUST_BACKTRACE=full
    cargo lrun  --color 'always' -- --debug
    export RUST_BACKTRACE=0

# Build and run with double --debug parameters
@rundd:
    export RUST_BACKTRACE=full
    cargo lrun  --color 'always' -- --debug --debug
    export RUST_BACKTRACE=0

# Copy this justfile to the templates directory
@just:
    cp {{invocation_directory()}}/justfile ~/CloudStation/Source/_Templates/justfile.template

# Check, but verbose
@checkv:
    cargo lcheck --color 'always' --verbose

# Install the relevant cargo add-ons used in this file
@prepare:
    -cargo install cargo-limit
    -cargo install cargo-geiger
    -cargo install cargo-depgraph
    -cargo install cargo-audit
    -cargo install cargo-bloat
    -cargo install --locked cargo-outdated
    -cargo install tokei
    echo "Make sure to also install Graphviz."
