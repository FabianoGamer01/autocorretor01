# 📦 Como Criar uma Release no GitHub

Este guia explica como publicar uma nova versão do **Autocorretor PT-BR** para que os usuários possam instalar facilmente usando o comando no README ou baixando o instalador.

## Pré-requisitos

1.  Você deve ter o arquivo `InstaladorAutocorretor.exe` gerado (ele está na raiz do projeto após rodar a build).
2.  (Opcional) Os arquivos `ime_core.dll` e `autocorretor-tray.exe` localizados em `target/release/` também são úteis para quem usa o script do PowerShell.

## Passo a Passo

1.  **Acesse a página de Releases**:
    Vá para a página do seu repositório no GitHub: [https://github.com/FabianoGamer01/autocorretor01](https://github.com/FabianoGamer01/autocorretor01)
    Clique em **Releases** na barra lateral direita, ou acesse: [https://github.com/FabianoGamer01/autocorretor01/releases](https://github.com/FabianoGamer01/autocorretor01/releases)

2.  **Crie uma nova Release**:
    Clique no botão **"Draft a new release"** (Criar um rascunho de nova release).

3.  **Escolha a Tag**:
    Clique em **"Choose a tag"**.
    Digite a versão que você está lançando, por exemplo: `v0.1.0`.
    Clique em **"Create new tag: v0.1.0"**.

4.  **Título e Descrição**:
    *   **Release title**: Coloque um título, ex: "Versão Inicial - v0.1.0".
    *   **Description**: Descreva as novidades. Exemplo:
        ```markdown
        Lançamento inicial do Autocorretor PT-BR com IA.
        
        ### Novidades
        * Correção inteligente em tempo real.
        * Ícone na bandeja do sistema.
        * Instalador automático.
        ```

5.  **Anexar Arquivos (IMPORTANTE)**:
    Arraste e solte os seguintes arquivos para a área **"Attach binaries by dropping them here..."**:
    
    *   `InstaladorAutocorretor.exe` (O instalador único que criamos).
    *   `target/release/autocorretor-tray.exe` (Para o script PowerShell).
    *   `target/release/ime_core.dll` (Para o script PowerShell).

    > **Nota**: O script `install.ps1` que colocamos no README procura por arquivos com nomes específicos. Se você anexar `autocorretor-tray.exe` e `ime_core.dll`, o script funcionará perfeitamente para quem não quiser baixar o `.exe`. Mas o `InstaladorAutocorretor.exe` é a forma mais fácil para usuários finais.

6.  **Publicar**:
    Se for uma versão de teste, marque a caixa **"Set as a pre-release"**.
    Clique no botão verde **"Publish release"**.

## Testando

Após publicar:
1.  Copie o link do `InstaladorAutocorretor.exe` na página da release e envie para seus amigos.
2.  Ou peça para eles rodarem o comando do PowerShell que está no README. Ele vai baixar automaticamente os arquivos `autocorretor-tray.exe` e `ime_core.dll` dessa release.
