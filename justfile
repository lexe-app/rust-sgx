check:
    cargo check --workspace --exclude=async-usercalls
    RUSTC_BOOTSTRAP=1 cargo check --target=x86_64-fortanix-unknown-sgx \
      -p aesm-client \
      -p async-usercalls \
      -p fortanix-sgx-abi \
      -p insecure-time \
      -p sgx-isa \
      -p ipc-queue \
      -p rs-libc@0.2.5

# SGX (dlmalloc)
# NUM_THREADS,LATENCY_SMALL_THREADS,LATENCY_MEDIUM_THREADS,LATENCY_LARGE_THREADS,GLOBAL_AVERAGE
# 4,32932.25,32570.25,0,32751.25
mem-alloc-test-sgx:
    cd examples/mem-alloc-test && \
      RUSTC_BOOTSTRAP=1 cargo run --target=x86_64-fortanix-unknown-sgx --release

# Linux (glibc):
# NUM_THREADS,LATENCY_SMALL_THREADS,LATENCY_MEDIUM_THREADS,LATENCY_LARGE_THREADS,GLOBAL_AVERAGE
# 4,511.75,577.25,0,544.5
mem-alloc-test:
    cd examples/mem-alloc-test && \
      RUSTC_BOOTSTRAP=1 cargo run --release
