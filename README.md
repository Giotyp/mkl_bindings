# Intel MKL Bindings

Intel [oneMKL](https://www.intel.com/content/www/us/en/developer/tools/oneapi/onemkl.html) is a high performance math library for Intel CPUs and GPUs. 
Utilizing it in C/C++ is non-trivial, as after installing the [oneAPI Math Kernel Library](https://www.intel.com/content/www/us/en/developer/tools/oneapi/onemkl-download.html), the user has to only include the respective header `mkl_dfti.h`, call the required functions and lastly link with the library during compilation.
The process however to utilize oneMKL in languages different languages is quite different and can be summarized in the following steps:

1. Create bindings for desired classes, structs, functions in the target language.
2. Use the bindings to call required functions.
3. Link with the mkl library.

This repository focuses on bindings in Rust, which is used by increasingly more users, and Python, which already has a huge community. 

## Rust
To utilize Rust's [Foreign Function Interface](https://doc.rust-lang.org/nomicon/ffi.html) we need to create bindings for the desired Intel MKL functions. 
One way to do this is by manually coding the necessary structs and functions, but luckily [bindgen](https://rust-lang.github.io/rust-bindgen/) can automate this process and generate them by providing the path to `mkl_dfti.h`, as shown in [build](rust/build.rs).
My importing the generated [mkl_bindings](rust/src/mkl_bindings.rs) we can easily call all required functions, as shown in [main](rust/src/main.rs).


## Python
The **NumPy** library supports acceleration through Intel's MKL library, by installing the required package:

```
pip install -i https://software.repos.intel.com/python/pypi numpy
```

However, NumPy operations like FFT, do not support **in-place** computations.
For large computations, where this feauture is needed, custom bindings need to be created.
To do this, we load the *mkl_rt* library and implement the required structs using *ctypes*, as shown in [mkl_bindings](python/mkl_bindings.py).
An example use of performing in-place DFT computation is shown in [mkl_class](python/mkl_class.py).