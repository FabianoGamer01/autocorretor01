fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "windows" {
        let mut res = winresource::WindowsResource::new();

        // Ícone do app embutido no .exe
        res.set_icon("../../data/icon.ico");

        // NÃO embutir manifesto admin aqui — causa erro SxS.
        // A elevação admin é feita via atalho com "Executar como administrador".

        res.compile().expect("Failed to compile Windows resources");
    }
}
