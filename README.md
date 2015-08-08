# safe_ffi
This library provides FFI bindings for safe_nfs and safe_dns. It also includes a working C-test code to verify functionality.

## Build Instructions:

All paths start from the repository root. Instructions are tested for Ubuntu-Linux. Modify for other platforms accordingly.

Build static library `libsafe_ffi.a` out of `safe_ffi` crate:
```
cd repository_root/rust/ -> cargo build --release -> target/release/libsafe_ffi.a
```
Change location:
```
cd repository_root/c/test
mkdir build
cd build
```
Build shared library `libc_wrapper.so` out of `c_wrapper.c` file:
```
gcc -c -std=c99 -Wall -Werror -fPIC ../../c_wrapper.c
gcc -shared -o libc_wrapper.so c_wrapper.o -L./../../../rust/target/release -lsafe_ffi
```

Build native test executable `a.out` out of `main.c` file, set library load path and run test executable:
```
gcc -std=c99 -Wall -Werror -O2 ../main.c -L. -lc_wrapper -lsodium -lm -ldl -lpthread
export LD_LIBRARY_PATH=./
./a.out
```
