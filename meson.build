project('monolithium', 'cpp')

custom_target('monolithium',
    build_by_default: true,
    input:  'monolithium/main.cu',
    output: 'monolithium',
    command: [
        find_program('nvcc', required: true),
        '@INPUT@', '-o', '@OUTPUT@',
    ]
)
