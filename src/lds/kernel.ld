OUTPUT_ARCH( "riscv" )
ENTRY( _start )

MEMORY {
  RAM : ORIGIN = 0x80000000, LENGTH = 128M
}

SECTIONS
{
  PROVIDE(__uart_start = 0x10000000);
  PROVIDE(__ram_start = ORIGIN(RAM));
  PROVIDE(__ram_end = ORIGIN(RAM) + LENGTH(RAM));
  /*
   * ensure that entry.S / _entry is at 0x80000000,
   * where qemu's -kernel jumps.
   */
  . = ORIGIN(RAM);
  
  .text : {
    *(.text.init)
    *(.text .text.*)
  } > RAM

  . = ALIGN(8);
  PROVIDE(__global_pointer$ = .);

  .rodata : {
    . = ALIGN(16);
    *(.srodata .srodata.*) /* do not need to distinguish this from .rodata */
    . = ALIGN(16);
    *(.rodata .rodata.*)
  } > RAM

  .data : {
    . = ALIGN(16);
    *(.sdata .sdata.*) /* do not need to distinguish this from .data */
    . = ALIGN(16);
    *(.data .data.*)
  } > RAM

  .bss : {
    . = ALIGN(16);
    *(.sbss .sbss.*) /* do not need to distinguish this from .bss */
    . = ALIGN(16);
    *(.bss .bss.*)
  } > RAM

  .eh_frame (INFO) : { KEEP(*(.eh_frame)) }
  .eh_frame_hdr (INFO) : { *(.eh_frame_hdr) }

  . = ALIGN(1M);
  PROVIDE(__stack_start = .);
  . += 8M;
  PROVIDE(__stack_end = .);

  . = ALIGN(1M);
  PROVIDE(__heap_start = .);
  PROVIDE(__heap_end = ORIGIN(RAM) + LENGTH(RAM));
}
