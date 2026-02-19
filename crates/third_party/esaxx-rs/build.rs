// Patch local da esaxx-rs: removido static_crt(true) para compatibilidade
// com binários /MD do ONNX Runtime (ort crate).
// Flags de C++ ajustadas por compilador (MSVC vs GCC/Clang).

#[cfg(feature = "cpp")]
fn main() {
    let mut build = cc::Build::new();
    build.cpp(true);
    // static_crt(true) REMOVIDO: causava conflito /MT vs /MD com ort

    // A flag -std=c++11 é inválida no MSVC (cl.exe); usar apenas em GCC/Clang
    if !build.get_compiler().is_like_msvc() {
        build.flag("-std=c++11");
    }

    // No macOS, linkar com libc++
    #[cfg(target_os = "macos")]
    build.flag("-stdlib=libc++");

    build.file("src/esaxx.cpp").include("src").compile("esaxx");
}

#[cfg(not(feature = "cpp"))]
fn main() {}
