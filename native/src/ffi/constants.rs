use crate::ffi::types::*;

// Error Codes:
pub const CL_SUCCESS: cl_int = 0;
pub const CL_DEVICE_NOT_FOUND: cl_int = -1;
pub const CL_DEVICE_NOT_AVAILABLE: cl_int = -2;
pub const CL_COMPILER_NOT_AVAILABLE: cl_int = -3;
pub const CL_MEM_OBJECT_ALLOCATION_FAILURE: cl_int = -4;
pub const CL_OUT_OF_RESOURCES: cl_int = -5;
pub const CL_OUT_OF_HOST_MEMORY: cl_int = -6;
pub const CL_PROFILING_INFO_NOT_AVAILABLE: cl_int = -7;
pub const CL_MEM_COPY_OVERLAP: cl_int = -8;
pub const CL_IMAGE_FORMAT_MISMATCH: cl_int = -9;
pub const CL_IMAGE_FORMAT_NOT_SUPPORTED: cl_int = -10;
pub const CL_BUILD_PROGRAM_FAILURE: cl_int = -11;
pub const CL_MAP_FAILURE: cl_int = -12;
pub const CL_MISALIGNED_SUB_BUFFER_OFFSET: cl_int = -13;
pub const CL_EXEC_STATUS_ERROR_FOR_EVENTS_IN_WAIT_LIST: cl_int = -14;
pub const CL_COMPILE_PROGRAM_FAILURE: cl_int = -15;
pub const CL_LINKER_NOT_AVAILABLE: cl_int = -16;
pub const CL_LINK_PROGRAM_FAILURE: cl_int = -17;
pub const CL_DEVICE_PARTITION_FAILED: cl_int = -18;
pub const CL_KERNEL_ARG_INFO_NOT_AVAILABLE: cl_int = -19;

pub const CL_INVALID_VALUE: cl_int = -30;
pub const CL_INVALID_DEVICE_TYPE: cl_int = -31;
pub const CL_INVALID_PLATFORM: cl_int = -32;
pub const CL_INVALID_DEVICE: cl_int = -33;
pub const CL_INVALID_CONTEXT: cl_int = -34;
pub const CL_INVALID_QUEUE_PROPERTIES: cl_int = -35;
pub const CL_INVALID_COMMAND_QUEUE: cl_int = -36;
pub const CL_INVALID_HOST_PTR: cl_int = -37;
pub const CL_INVALID_MEM_OBJECT: cl_int = -38;
pub const CL_INVALID_IMAGE_FORMAT_DESCRIPTOR: cl_int = -39;
pub const CL_INVALID_IMAGE_SIZE: cl_int = -40;
pub const CL_INVALID_SAMPLER: cl_int = -41;
pub const CL_INVALID_BINARY: cl_int = -42;
pub const CL_INVALID_BUILD_OPTIONS: cl_int = -43;
pub const CL_INVALID_PROGRAM: cl_int = -44;
pub const CL_INVALID_PROGRAM_EXECUTABLE: cl_int = -45;
pub const CL_INVALID_KERNEL_NAME: cl_int = -46;
pub const CL_INVALID_KERNEL_DEFINITION: cl_int = -47;
pub const CL_INVALID_KERNEL: cl_int = -48;
pub const CL_INVALID_ARG_INDEX: cl_int = -49;
pub const CL_INVALID_ARG_VALUE: cl_int = -50;
pub const CL_INVALID_ARG_SIZE: cl_int = -51;
pub const CL_INVALID_KERNEL_ARGS: cl_int = -52;
pub const CL_INVALID_WORK_DIMENSION: cl_int = -53;
pub const CL_INVALID_WORK_GROUP_SIZE: cl_int = -54;
pub const CL_INVALID_WORK_ITEM_SIZE: cl_int = -55;
pub const CL_INVALID_GLOBAL_OFFSET: cl_int = -56;
pub const CL_INVALID_EVENT_WAIT_LIST: cl_int = -57;
pub const CL_INVALID_EVENT: cl_int = -58;
pub const CL_INVALID_OPERATION: cl_int = -59;
pub const CL_INVALID_GL_OBJECT: cl_int = -60;
pub const CL_INVALID_BUFFER_SIZE: cl_int = -61;
pub const CL_INVALID_MIP_LEVEL: cl_int = -62;
pub const CL_INVALID_GLOBAL_WORK_SIZE: cl_int = -63;
pub const CL_INVALID_PROPERTY: cl_int = -64;
pub const CL_INVALID_IMAGE_DESCRIPTOR: cl_int = -65;
pub const CL_INVALID_COMPILER_OPTIONS: cl_int = -66;
pub const CL_INVALID_LINKER_OPTIONS: cl_int = -67;
pub const CL_INVALID_DEVICE_PARTITION_COUNT: cl_int = -68;
pub const CL_INVALID_PIPE_SIZE: cl_int = -69;
pub const CL_INVALID_DEVICE_QUEUE: cl_int = -70;
pub const CL_PLATFORM_NOT_FOUND_KHR: cl_int = -1001;

// Version:
pub const CL_VERSION_1_0: cl_bool = 1;
pub const CL_VERSION_1_1: cl_bool = 1;
pub const CL_VERSION_1_2: cl_bool = 1;
pub const CL_VERSION_2_0: cl_bool = 1;
pub const CL_VERSION_2_1: cl_bool = 1;

// cl_bool:
pub const CL_FALSE: cl_bool = 0;
pub const CL_TRUE: cl_bool = 1;
pub const CL_BLOCKING: cl_bool = CL_TRUE;
pub const CL_NON_BLOCKING: cl_bool = CL_FALSE;

// cl_platform_info:
pub const CL_PLATFORM_PROFILE: cl_uint = 0x0900;
pub const CL_PLATFORM_VERSION: cl_uint = 0x0901;
pub const CL_PLATFORM_NAME: cl_uint = 0x0902;
pub const CL_PLATFORM_VENDOR: cl_uint = 0x0903;
pub const CL_PLATFORM_EXTENSIONS: cl_uint = 0x0904;
// ###### NEW ########
pub const CL_PLATFORM_HOST_TIMER_RESOLUTION: cl_uint = 0x0905;

// cl_device_type - bitfield:
pub const CL_DEVICE_TYPE_DEFAULT: cl_bitfield = 1 << 0;
pub const CL_DEVICE_TYPE_CPU: cl_bitfield = 1 << 1;
pub const CL_DEVICE_TYPE_GPU: cl_bitfield = 1 << 2;
pub const CL_DEVICE_TYPE_ACCELERATOR: cl_bitfield = 1 << 3;
pub const CL_DEVICE_TYPE_CUSTOM: cl_bitfield = 1 << 4;
pub const CL_DEVICE_TYPE_ALL: cl_bitfield = 0xFFFFFFFF;

// cl_device_info:
pub const CL_DEVICE_TYPE: cl_uint = 0x1000;
pub const CL_DEVICE_VENDOR_ID: cl_uint = 0x1001;
pub const CL_DEVICE_MAX_COMPUTE_UNITS: cl_uint = 0x1002;
pub const CL_DEVICE_MAX_WORK_ITEM_DIMENSIONS: cl_uint = 0x1003;
pub const CL_DEVICE_MAX_WORK_GROUP_SIZE: cl_uint = 0x1004;
pub const CL_DEVICE_MAX_WORK_ITEM_SIZES: cl_uint = 0x1005;
pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_CHAR: cl_uint = 0x1006;
pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_SHORT: cl_uint = 0x1007;
pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_INT: cl_uint = 0x1008;
pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_LONG: cl_uint = 0x1009;
pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_FLOAT: cl_uint = 0x100A;
pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_DOUBLE: cl_uint = 0x100B;
pub const CL_DEVICE_MAX_CLOCK_FREQUENCY: cl_uint = 0x100C;
pub const CL_DEVICE_ADDRESS_BITS: cl_uint = 0x100D;
pub const CL_DEVICE_MAX_READ_IMAGE_ARGS: cl_uint = 0x100E;
pub const CL_DEVICE_MAX_WRITE_IMAGE_ARGS: cl_uint = 0x100F;
pub const CL_DEVICE_MAX_MEM_ALLOC_SIZE: cl_uint = 0x1010;
pub const CL_DEVICE_IMAGE2D_MAX_WIDTH: cl_uint = 0x1011;
pub const CL_DEVICE_IMAGE2D_MAX_HEIGHT: cl_uint = 0x1012;
pub const CL_DEVICE_IMAGE3D_MAX_WIDTH: cl_uint = 0x1013;
pub const CL_DEVICE_IMAGE3D_MAX_HEIGHT: cl_uint = 0x1014;
pub const CL_DEVICE_IMAGE3D_MAX_DEPTH: cl_uint = 0x1015;
pub const CL_DEVICE_IMAGE_SUPPORT: cl_uint = 0x1016;
pub const CL_DEVICE_MAX_PARAMETER_SIZE: cl_uint = 0x1017;
pub const CL_DEVICE_MAX_SAMPLERS: cl_uint = 0x1018;
pub const CL_DEVICE_MEM_BASE_ADDR_ALIGN: cl_uint = 0x1019;
pub const CL_DEVICE_MIN_DATA_TYPE_ALIGN_SIZE: cl_uint = 0x101A;
pub const CL_DEVICE_SINGLE_FP_CONFIG: cl_uint = 0x101B;
pub const CL_DEVICE_GLOBAL_MEM_CACHE_TYPE: cl_uint = 0x101C;
pub const CL_DEVICE_GLOBAL_MEM_CACHELINE_SIZE: cl_uint = 0x101D;
pub const CL_DEVICE_GLOBAL_MEM_CACHE_SIZE: cl_uint = 0x101E;
pub const CL_DEVICE_GLOBAL_MEM_SIZE: cl_uint = 0x101F;
pub const CL_DEVICE_MAX_CONSTANT_BUFFER_SIZE: cl_uint = 0x1020;
pub const CL_DEVICE_MAX_CONSTANT_ARGS: cl_uint = 0x1021;
pub const CL_DEVICE_LOCAL_MEM_TYPE: cl_uint = 0x1022;
pub const CL_DEVICE_LOCAL_MEM_SIZE: cl_uint = 0x1023;
pub const CL_DEVICE_ERROR_CORRECTION_SUPPORT: cl_uint = 0x1024;
pub const CL_DEVICE_PROFILING_TIMER_RESOLUTION: cl_uint = 0x1025;
pub const CL_DEVICE_ENDIAN_LITTLE: cl_uint = 0x1026;
pub const CL_DEVICE_AVAILABLE: cl_uint = 0x1027;
pub const CL_DEVICE_COMPILER_AVAILABLE: cl_uint = 0x1028;
pub const CL_DEVICE_EXECUTION_CAPABILITIES: cl_uint = 0x1029;
// DEPRICATED 2.0:
pub const CL_DEVICE_QUEUE_PROPERTIES: cl_uint = 0x102A;
pub const CL_DEVICE_QUEUE_ON_HOST_PROPERTIES: cl_uint = 0x102A;
pub const CL_DEVICE_NAME: cl_uint = 0x102B;
pub const CL_DEVICE_VENDOR: cl_uint = 0x102C;
pub const CL_DRIVER_VERSION: cl_uint = 0x102D;
pub const CL_DEVICE_PROFILE: cl_uint = 0x102E;
pub const CL_DEVICE_VERSION: cl_uint = 0x102F;
pub const CL_DEVICE_EXTENSIONS: cl_uint = 0x1030;
pub const CL_DEVICE_PLATFORM: cl_uint = 0x1031;
pub const CL_DEVICE_DOUBLE_FP_CONFIG: cl_uint = 0x1032;
pub const CL_DEVICE_HALF_FP_CONFIG: cl_uint = 0x1033;
pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_HALF: cl_uint = 0x1034;
// DEPRICATED 2.0:
pub const CL_DEVICE_HOST_UNIFIED_MEMORY: cl_uint = 0x1035;
pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_CHAR: cl_uint = 0x1036;
pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_SHORT: cl_uint = 0x1037;
pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_INT: cl_uint = 0x1038;
pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_LONG: cl_uint = 0x1039;
pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_FLOAT: cl_uint = 0x103A;
pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_DOUBLE: cl_uint = 0x103B;
pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_HALF: cl_uint = 0x103C;
pub const CL_DEVICE_OPENCL_C_VERSION: cl_uint = 0x103D;
pub const CL_DEVICE_LINKER_AVAILABLE: cl_uint = 0x103E;
pub const CL_DEVICE_BUILT_IN_KERNELS: cl_uint = 0x103F;
pub const CL_DEVICE_IMAGE_MAX_BUFFER_SIZE: cl_uint = 0x1040;
pub const CL_DEVICE_IMAGE_MAX_ARRAY_SIZE: cl_uint = 0x1041;
pub const CL_DEVICE_PARENT_DEVICE: cl_uint = 0x1042;
pub const CL_DEVICE_PARTITION_MAX_SUB_DEVICES: cl_uint = 0x1043;
pub const CL_DEVICE_PARTITION_PROPERTIES: cl_uint = 0x1044;
pub const CL_DEVICE_PARTITION_AFFINITY_DOMAIN: cl_uint = 0x1045;
pub const CL_DEVICE_PARTITION_TYPE: cl_uint = 0x1046;
pub const CL_DEVICE_REFERENCE_COUNT: cl_uint = 0x1047;
pub const CL_DEVICE_PREFERRED_INTEROP_USER_SYNC: cl_uint = 0x1048;
pub const CL_DEVICE_PRINTF_BUFFER_SIZE: cl_uint = 0x1049;
pub const CL_DEVICE_IMAGE_PITCH_ALIGNMENT: cl_uint = 0x104A;
pub const CL_DEVICE_IMAGE_BASE_ADDRESS_ALIGNMENT: cl_uint = 0x104B;
//###### NEW ########
pub const CL_DEVICE_MAX_READ_WRITE_IMAGE_ARGS: cl_uint = 0x104C;
pub const CL_DEVICE_MAX_GLOBAL_VARIABLE_SIZE: cl_uint = 0x104D;
pub const CL_DEVICE_QUEUE_ON_DEVICE_PROPERTIES: cl_uint = 0x104E;
pub const CL_DEVICE_QUEUE_ON_DEVICE_PREFERRED_SIZE: cl_uint = 0x104F;
pub const CL_DEVICE_QUEUE_ON_DEVICE_MAX_SIZE: cl_uint = 0x1050;
pub const CL_DEVICE_MAX_ON_DEVICE_QUEUES: cl_uint = 0x1051;
pub const CL_DEVICE_MAX_ON_DEVICE_EVENTS: cl_uint = 0x1052;
pub const CL_DEVICE_SVM_CAPABILITIES: cl_uint = 0x1053;
pub const CL_DEVICE_GLOBAL_VARIABLE_PREFERRED_TOTAL_SIZE: cl_uint = 0x1054;
pub const CL_DEVICE_MAX_PIPE_ARGS: cl_uint = 0x1055;
pub const CL_DEVICE_PIPE_MAX_ACTIVE_RESERVATIONS: cl_uint = 0x1056;
pub const CL_DEVICE_PIPE_MAX_PACKET_SIZE: cl_uint = 0x1057;
pub const CL_DEVICE_PREFERRED_PLATFORM_ATOMIC_ALIGNMENT: cl_uint = 0x1058;
pub const CL_DEVICE_PREFERRED_GLOBAL_ATOMIC_ALIGNMENT: cl_uint = 0x1059;
pub const CL_DEVICE_PREFERRED_LOCAL_ATOMIC_ALIGNMENT: cl_uint = 0x105A;
pub const CL_DEVICE_IL_VERSION: cl_uint = 0x105B;
pub const CL_DEVICE_MAX_NUM_SUB_GROUPS: cl_uint = 0x105C;
pub const CL_DEVICE_SUB_GROUP_INDEPENDENT_FORWARD_PROGRESS: cl_uint = 0x105D;

// cl_mem_flags and cl_svm_mem_flags - bitfield:
pub const CL_MEM_READ_WRITE: cl_bitfield = 1 << 0;
pub const CL_MEM_WRITE_ONLY: cl_bitfield = 1 << 1;
pub const CL_MEM_READ_ONLY: cl_bitfield = 1 << 2;
pub const CL_MEM_USE_HOST_PTR: cl_bitfield = 1 << 3;
pub const CL_MEM_ALLOC_HOST_PTR: cl_bitfield = 1 << 4;
pub const CL_MEM_COPY_HOST_PTR: cl_bitfield = 1 << 5;
// pub const ____RESERVED: cl_bitfield = 1 << 6;
pub const CL_MEM_HOST_WRITE_ONLY: cl_bitfield = 1 << 7;
pub const CL_MEM_HOST_READ_ONLY: cl_bitfield = 1 << 8;
pub const CL_MEM_HOST_NO_ACCESS: cl_bitfield = 1 << 9;
// ###### NEW ########
pub const CL_MEM_SVM_FINE_GRAIN_BUFFER: cl_bitfield = 1 << 10; // used by cl_svm_mem_flags only
pub const CL_MEM_SVM_ATOMICS: cl_bitfield = 1 << 11; // used by cl_svm_mem_flags only
pub const CL_MEM_KERNEL_READ_AND_WRITE: cl_bitfield = 1 << 12;

// cl_program_build_info:
pub const CL_PROGRAM_BUILD_STATUS: cl_uint = 0x1181;
pub const CL_PROGRAM_BUILD_OPTIONS: cl_uint = 0x1182;
pub const CL_PROGRAM_BUILD_LOG: cl_uint = 0x1183;
pub const CL_PROGRAM_BINARY_TYPE: cl_uint = 0x1184;
// ###### NEW ########
pub const CL_PROGRAM_BUILD_GLOBAL_VARIABLE_TOTAL_SIZE: cl_uint = 0x1185;
