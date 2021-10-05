.section .text, "ax"
.global write
write:
    la t0, _uart_start
    sb a0, 0(t0)
    ret

.end
