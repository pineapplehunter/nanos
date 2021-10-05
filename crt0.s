.section .text
.global _start
_start:
    .cfi_startproc
    .cfi_undefined ra
    .option push
    .option norelax
    la gp, __global_pointer$
    .option pop
    la sp, _stack_end
    add s0,sp,zero
    jal zero,main
    .cfi_endproc
