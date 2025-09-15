project('cudalith', 'cpp')

custom_target('cudalith',
    build_by_default: true,
    input:  'monolithium/main.cu',
    output: 'cudalith',
    command: [
        find_program('nvcc', required: true),
        '-arch=native', '-O3',
        '@INPUT@', '-o', '@OUTPUT@',
    ]
)
