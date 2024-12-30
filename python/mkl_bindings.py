import ctypes
from ctypes.util import find_library
from ctypes import POINTER, c_void_p, c_uint, c_long

# Load Intel MKL shared library
mkl_lib = ctypes.cdll.LoadLibrary(find_library('mkl_rt'))


# Define the datatype for the handle and constants
class DFTI_DESCRIPTOR(ctypes.Structure):
    _fields_ = []

DFTI_DESCRIPTOR_HANDLE = POINTER(DFTI_DESCRIPTOR)

DFTI_CONFIG_VALUE = c_uint
DFTI_SINGLE = 35  # Single precision floating point
DFTI_COMPLEX = 32  # Complex number data type

# Define the bindings similarly to the Rust bindings
DftiCreateDescriptor = mkl_lib.DftiCreateDescriptor
DftiCreateDescriptor.restype = c_long
DftiCreateDescriptor.argtypes = [POINTER(DFTI_DESCRIPTOR_HANDLE), DFTI_CONFIG_VALUE, DFTI_CONFIG_VALUE, c_long]

DftiFreeDescriptor = mkl_lib.DftiFreeDescriptor
DftiFreeDescriptor.restype = c_long
DftiFreeDescriptor.argtypes = [POINTER(DFTI_DESCRIPTOR_HANDLE)]


DftiCommitDescriptor = mkl_lib.DftiCommitDescriptor
DftiCommitDescriptor.restype = c_long
DftiCommitDescriptor.argtypes = [DFTI_DESCRIPTOR_HANDLE]

DftiComputeForward = mkl_lib.DftiComputeForward
DftiComputeForward.restype = c_long
DftiComputeForward.argtypes = [DFTI_DESCRIPTOR_HANDLE, c_void_p]