# nanos

## 勉強

https://twilco.github.io/riscv-from-scratch/2019/04/27/riscv-from-scratch-2.html

### 最初
`riscv-gcc`で`add.c`をコンパイルするとqemuで正しく認識されない。

```sh
# コンパイル
$ riscv64-elf-gcc add.c -O0 -g
# 実行とデバッグ環境のセットアップ
$ qemu-system-riscv64 machine virt -m 128M -gdb tcp::1234 -kernel a.out
# デバッグ
riscv64-unknown-elf-gdb --tui a.out
```

## `-v`
コンパイラはいろんなことをしている

```
$ riscv64-elf-gcc add.c -O0 -g -v
Using built-in specs.
COLLECT_GCC=riscv64-elf-gcc
COLLECT_LTO_WRAPPER=/usr/lib/gcc/riscv64-elf/11.1.0/lto-wrapper
Target: riscv64-elf
Configured with: /build/riscv64-elf-gcc/src/gcc-11.1.0/configure --target=riscv64-elf --prefix=/usr --with-sysroot=/usr/riscv64-elf --with-native-system-header-dir=/include --libexecdir=/usr/lib --enable-languages=c,c++ --enable-threads=single --enable-plugins --enable-multilib --enable-libgcc --disable-libgomp --disable-libquadmath --disable-libffi --disable-libssp --disable-libmudflap --disable-decimal-float --disable-libstdcxx-pch --disable-nls --disable-shared --disable-tls --with-newlib --with-gnu-as --with-gnu-ld --with-system-zlib --with-headers=/usr/riscv64-elf/include --with-python-dir=share/gcc-riscv64-elf --with-gmp --with-mpfr --with-mpc --with-isl --with-libelf --enable-gnu-indirect-function --with-pkgversion='Arch Linux Repositories' --with-bugurl=https://bugs.archlinux.org/
Thread model: single
Supported LTO compression algorithms: zlib zstd
gcc version 11.1.0 (Arch Linux Repositories) 
COLLECT_GCC_OPTIONS='-O0' '-g' '-v' '-march=rv64gc' '-mabi=lp64d' '-march=rv64imafdc' '-dumpdir' 'a-'
 /usr/lib/gcc/riscv64-elf/11.1.0/cc1 -quiet -v -imultilib rv64imafdc/lp64d add.c -quiet -dumpdir a- -dumpbase add.c -dumpbase-ext .c -march=rv64gc -mabi=lp64d -march=rv64imafdc -g -O0 -version -o /tmp/ccgX4vJD.s
GNU C17 (Arch Linux Repositories) version 11.1.0 (riscv64-elf)
        compiled by GNU C version 11.1.0, GMP version 6.2.1, MPFR version 4.1.0, MPC version 1.2.1, isl version isl-0.22-GMP

warning: MPFR header version 4.1.0 differs from library version 4.1.0-p13.
GGC heuristics: --param ggc-min-expand=100 --param ggc-min-heapsize=131072
ignoring nonexistent directory "/usr/riscv64-elf/usr/local/include"
ignoring duplicate directory "/usr/riscv64-elf/include"
#include "..." search starts here:
#include <...> search starts here:
 /usr/lib/gcc/riscv64-elf/11.1.0/include
 /usr/lib/gcc/riscv64-elf/11.1.0/include-fixed
 /usr/lib/gcc/riscv64-elf/11.1.0/../../../../riscv64-elf/include
End of search list.
GNU C17 (Arch Linux Repositories) version 11.1.0 (riscv64-elf)
        compiled by GNU C version 11.1.0, GMP version 6.2.1, MPFR version 4.1.0, MPC version 1.2.1, isl version isl-0.22-GMP

warning: MPFR header version 4.1.0 differs from library version 4.1.0-p13.
GGC heuristics: --param ggc-min-expand=100 --param ggc-min-heapsize=131072
Compiler executable checksum: b44360d7a66a55c15ebbf8b028a31cb0
COLLECT_GCC_OPTIONS='-O0' '-g' '-v' '-march=rv64gc' '-mabi=lp64d' '-march=rv64imafdc' '-dumpdir' 'a-'
 /usr/lib/gcc/riscv64-elf/11.1.0/../../../../riscv64-elf/bin/as -v --gdwarf-5 --traditional-format -march=rv64gc -march=rv64imafdc -mabi=lp64d -o /tmp/ccCReJVp.o /tmp/ccgX4vJD.s
GNU assembler version 2.36.1 (riscv64-elf) using BFD version (GNU Binutils) 2.36.1
COMPILER_PATH=/usr/lib/gcc/riscv64-elf/11.1.0/:/usr/lib/gcc/riscv64-elf/11.1.0/:/usr/lib/gcc/riscv64-elf/:/usr/lib/gcc/riscv64-elf/11.1.0/:/usr/lib/gcc/riscv64-elf/:/usr/lib/gcc/riscv64-elf/11.1.0/../../../../riscv64-elf/bin/
LIBRARY_PATH=/usr/lib/gcc/riscv64-elf/11.1.0/rv64imafdc/lp64d/:/usr/lib/gcc/riscv64-elf/11.1.0/../../../../riscv64-elf/lib/rv64imafdc/lp64d/:/usr/riscv64-elf/lib/rv64imafdc/lp64d/:/usr/lib/gcc/riscv64-elf/11.1.0/:/usr/lib/gcc/riscv64-elf/11.1.0/../../../../riscv64-elf/lib/:/usr/riscv64-elf/lib/
COLLECT_GCC_OPTIONS='-O0' '-g' '-v' '-march=rv64gc' '-mabi=lp64d' '-march=rv64imafdc' '-dumpdir' 'a.'
 /usr/lib/gcc/riscv64-elf/11.1.0/collect2 -plugin /usr/lib/gcc/riscv64-elf/11.1.0/liblto_plugin.so -plugin-opt=/usr/lib/gcc/riscv64-elf/11.1.0/lto-wrapper -plugin-opt=-fresolution=/tmp/ccc4EVtP.res -plugin-opt=-pass-through=-lgcc -plugin-opt=-pass-through=-lc -plugin-opt=-pass-through=-lgloss -plugin-opt=-pass-through=-lgcc --sysroot=/usr/riscv64-elf -melf64lriscv /usr/lib/gcc/riscv64-elf/11.1.0/../../../../riscv64-elf/lib/rv64imafdc/lp64d/crt0.o /usr/lib/gcc/riscv64-elf/11.1.0/rv64imafdc/lp64d/crtbegin.o -L/usr/lib/gcc/riscv64-elf/11.1.0/rv64imafdc/lp64d -L/usr/lib/gcc/riscv64-elf/11.1.0/../../../../riscv64-elf/lib/rv64imafdc/lp64d -L/usr/riscv64-elf/lib/rv64imafdc/lp64d -L/usr/lib/gcc/riscv64-elf/11.1.0 -L/usr/lib/gcc/riscv64-elf/11.1.0/../../../../riscv64-elf/lib -L/usr/riscv64-elf/lib /tmp/ccCReJVp.o -lgcc --start-group -lc -lgloss --end-group -lgcc /usr/lib/gcc/riscv64-elf/11.1.0/rv64imafdc/lp64d/crtend.o
COLLECT_GCC_OPTIONS='-O0' '-g' '-v' '-march=rv64gc' '-mabi=lp64d' '-march=rv64imafdc' '-dumpdir' 'a.'
```

## qemuからデータを出す

```sh
$ qemu-system-riscv64 -machine virt -machine dumpdtb=riscv64-virt.dtb
$ dtc -I dtb -O dts -o riscv64-virt.dts riscv64-virt.dtb
```

## リンカスクリプトを読み出す
```sh
$ riscv64-elf-ld --verbose > riscv64-virt.lds
```