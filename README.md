# System-wide AI-assisted Input Method for Portuguese (PT-BR IME)

Este projeto implementa um Input Method Editor (IME) avançado para Windows, focado na correção e previsão de texto em Português do Brasil utilizando inteligência artificial.

## 📋 Visão Geral

O objetivo deste software é fornecer correções ortográficas e previsões contextuais em tempo real, integrando-se diretamente ao sistema operacional. Isso permite que a assistência de escrita funcione em qualquer aplicativo (Bloco de Notas, Navegadores, Discord, etc.).

## 📥 Instalação (Recomendado)

Para instalar a versão mais recente diretamente do GitHub, abra o **PowerShell como Administrador** e execute o seguinte comando:

```powershell
iwr -useb https://raw.githubusercontent.com/FabianoGamer01/autocorretor01/main/install.ps1 | iex
```

Isso irá baixar, registrar e iniciar o Autocorretor automaticamente.

## 🚀 Como Compilar (Para Desenvolvedores)

Se você deseja contribuir ou compilar do zero:

### Pré-requisitos
*   [Rust](https://www.rust-lang.org/tools/install) (Linguagem de programação e gerenciador de pacotes).
*   Windows 10 ou 11.

### Instalação e Execução

1.  **Clone o repositório**:
    ```powershell
    git clone https://github.com/FabianoGamer01/autocorretor01.git
    cd autocorretor01
    ```

2.  **Compile o projeto**:
    ```powershell
    cargo build --release
    ```

3.  **Execute o avaliador (se disponível)** ou a DLL do IME conforme a documentação técnica na pasta `docs/` (se houver).

## 🛠️ Para Desenvolvedores

Se você deseja contribuir com o código:

1.  Faça um **Fork** deste repositório.
2.  Crie uma branch para sua funcionalidade (`git checkout -b feature/nova-funcionalidade`).
3.  Faça commit de suas alterações.
4.  Faça o push para a branch.
5.  Abra um **Pull Request**.

## 🔒 Privacidade e Segurança

Este software processa o texto localmente para fornecer correções. Nenhuma informação de digitação é enviada para servidores externos sem o consentimento explícito do usuário.

---
*Desenvolvido com foco em performance e privacidade.*
