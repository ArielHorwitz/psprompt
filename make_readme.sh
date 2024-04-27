#! /bin/bash

set -e
cd $(dirname $0)

rm "README.md"

exec &> "README.md"
echo '```'
cargo run --quiet -- --help
echo '```'
exec &> /dev/tty

echo success
