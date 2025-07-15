#!/bin/bash

set -x

python "test.py"
rustc -o "test" "test.rs" && ("test"; rm "test")
gofmt -e ../escape_sequences/go/* > /dev/null
