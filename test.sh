#! /bin/bash

export RUSTFLAGS="-Awarnings"

SAVE_PATH=./results
KERNEL=ep-pp
CLASS=B
NUM_THREADS=8
EXEC_COMMAND="cargo +nightly run --bin $KERNEL --release -- $CLASS $NUM_THREADS"

$EXEC_COMMAND

for i in {1..30}
do
	$EXEC_COMMAND > $SAVE_PATH/$KERNEL.$CLASS.$NUM_THREADS.$i.txt
done
