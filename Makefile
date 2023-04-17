#export RUSTFLAGS="-Awarnings"

KERNEL=cg
CLASS=S
NUM_THREADS=8
EXEC_COMMAND=cargo +nightly run --bin $(KERNEL) --release -- $(CLASS) $(NUM_THREADS)

all:
	$(EXEC_COMMAND)