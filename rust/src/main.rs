extern crate intel_mkl_src;
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;
use num_complex::Complex;
use crate::mkl_bindings::*;

mod mkl_bindings;

fn generate_complex_float_array(size: usize, min_val: f32, max_val: f32) -> Vec<Complex<f32>> {
    let mut rng = thread_rng();
    let dist = Uniform::new(min_val, max_val);

    let mut arr = Vec::with_capacity(size);
    for _ in 0..size {
        arr.push(Complex {
            re: dist.sample(&mut rng),
            im: dist.sample(&mut rng),
        });
    }

    arr
}

fn computefft(fft_buf: &mut Vec<Complex<f32>>, array_size: usize) {

    let dfti_single = DFTI_CONFIG_VALUE_DFTI_SINGLE;
    let dfti_complex = DFTI_CONFIG_VALUE_DFTI_COMPLEX;
    let dim = 1;

    let mut mkl_handle: DFTI_DESCRIPTOR_HANDLE = std::ptr::null_mut();
    unsafe {
    DftiCreateDescriptor(&mut mkl_handle, dfti_single, dfti_complex, 
                            dim, array_size);

    // Commit the descriptor
    DftiCommitDescriptor(mkl_handle);
    }

    let input_data = fft_buf.as_mut_ptr();

    unsafe {
        DftiComputeForward(mkl_handle, input_data as *mut libc::c_void);
    }
} 


fn main() {
    let size = 100;
    let mut fft_buffer = generate_complex_float_array(size, -1.5, 1.5);

    println!("Before FFT: {:?}\n", &fft_buffer[0..10]);
    computefft(&mut fft_buffer, size);
    println!("After FFT: {:?}", &fft_buffer[0..10]);
}
