# REMOVE RUST WARNINGS
#export RUSTFLAGS="-Awarnings"

# CALLING THIS MAKE FILE
# make <target> KERNEL=<kernel> CLASS=<class> NUM_THREADS=<num_threads>

# DEFAULT KERNEL
KERNEL=cg-pp
# DEFAULT CLASS
CLASS=S
# DEFAULT NUM_THREADS
NUM_THREADS=8
# DONT OVERWRITE THIS
PARAMS_COMMAND=cargo +nightly run --bin setparams --release -- $(KERNEL) $(CLASS) $(NUM_THREADS)

COMP_COMMAND=cargo +nightly rustc --bin $(KERNEL)-$(CLASS)

EXEC_COMMAND=cargo +nightly run --bin $(KERNEL)$(CLASS)

compile:
	$(PARAMS_COMMAND)
	$(COMP_COMMAND)

run:
	$(EXEC_COMMAND)