.section .text.init
.global _start
_start:
    .option push
    .option norelax
    la gp, __global_pointer$
    .option pop
    la sp, __stack_end
    add s0,sp,zero
    jal zero,main
    j abort

.section .text
.global abort
abort:
    wfi
    j abort
