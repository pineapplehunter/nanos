.section .text.init
.global _start
_start:
    .option push
    .option norelax
    la gp, __global_pointer$
    .option pop
    la sp, _stack_end
    add s0,sp,zero
    jal zero,main
    j _abort

.section .text
.global _abort
_abort:
    wfi
    j _abort
