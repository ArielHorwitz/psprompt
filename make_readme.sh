#! /bin/bash

set -e
cd $(dirname $0)

TEMPFILE="target/README.md"

exec &> $TEMPFILE
echo '```'
cargo run --quiet -- --help
echo '```'
exec &> /dev/tty

mv $TEMPFILE .
echo success
