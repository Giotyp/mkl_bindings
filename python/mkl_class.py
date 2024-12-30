from mkl_bindings import *
import numpy as np

class MKLFFT:
    def __init__(self, size):
        self.size = size
        self.descriptor = DFTI_DESCRIPTOR_HANDLE()

        # Create DFTI descriptor for single precision complex 1D FFT
        status = DftiCreateDescriptor(ctypes.byref(self.descriptor), DFTI_SINGLE, DFTI_COMPLEX, 1, self.size)
        if status != 0:
            raise ValueError("MKL DftiCreateDescriptor failed with status: " + str(status))

        # Commit the descriptor
        status = DftiCommitDescriptor(self.descriptor)
        if status != 0:
            raise ValueError("MKL DftiCommitDescriptor failed with status: " + str(status))

    def compute_forward(self, data):
        # Perform in-place FFT
        data_ptr = data.ctypes.data_as(ctypes.c_void_p)
        status = DftiComputeForward(self.descriptor, data_ptr)
        if status != 0:
            raise ValueError("MKL DftiComputeForward failed with status: " + str(status))

    def __del__(self):
        # Free the descriptor
        status = DftiFreeDescriptor(ctypes.byref(self.descriptor))
        if status != 0:
            print("Warning: MKL DftiFreeDescriptor failed with status:", status)


def generate_complex_float_array(n, dtype=np.complex64):
    array = np.random.rand(n) + 1j*np.random.rand(n)
    return array.astype(dtype)

def using_mkl():
    try:
        mkl_lib = ctypes.cdll.LoadLibrary('libmkl_rt.so')
        return mkl_lib
    except OSError:
        return None

if __name__ == "__main__":
    
    if using_mkl() is not None:
        print("Intel MKL is being used.")
    else:
        print("Intel MKL is not being used.")

    size = 100
    fft_buffer = generate_complex_float_array(size)

    mkl_fft = MKLFFT(size)
    print("Before FFT:", fft_buffer[:10], end="\n\n")
    mkl_fft.compute_forward(fft_buffer)
    print("After FFT:", fft_buffer[:10])