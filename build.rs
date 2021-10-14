fn main() {
    let mut build = cc::Build::new();
    if let Ok(tool) = build.try_get_compiler() {
        if !tool.path().exists() {
            build.compiler("riscv64-elf-gcc");
        }
    }
    build
        .file("runtime.s")
        .no_default_flags(true)
        .flag("-nostdlib")
        .flag("-nodefaultlibs")
        .flag("-g")
        .flag("-O0")
        .flag("-mabi=lp64d")
        .compile("runtime");
}
