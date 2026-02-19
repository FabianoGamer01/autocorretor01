# 🐉 Autocorretor PT-BR (O Dragão)

```text
                \||/
                |  @___oo
      /\  /\   / (__,,,,|
     ) /^\) ^\/ _)
     )   /^\/   _)
     )   _ /  / _)
 /\  )/\/ ||  | )_)
<  >      |(,,) )__)
 ||      /    \)___)\
 | \____(      )___) )___
 \______(_______;;; __;;;
```

Este projeto implementa um **Input Method Editor (IME)** avançado para Windows, focado na correção e previsão de texto em Português do Brasil utilizando inteligência artificial.

## 🧬 Anatomia do Dragão (Estrutura do Projeto)

Para entender como este sistema funciona, imagine-o como um organismo vivo:

*   **🐲 A Cabeça (Interface):** `crates/tray-app`
    *   É o que você vê. Fica na bandeja do sistema (perto do relógio), observando e permitindo interações.
*   **🧠 O Cérebro (IA):** `crates/correction-engine`
    *   Onde a mágica acontece. Processa o texto, prevê o que você quer dizer e corrige seus erros usando modelos de Inteligência Artificial.
*   **🦴 O Corpo (Sistema):** `crates/ime-core`
    *   A estrutura que se conecta profundamente ao Windows. É a DLL que o sistema operacional carrega para interceptar e enviar o texto.
*   **🦅 As Garras (Instalador):** `crates/installer`
    *   Agarra o sistema para garantir que tudo fique no lugar certo. Instala, registra e fixa o dragão no seu computador.

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
    
3.  **Gere o instalador (Opcional)**:
    Se você quiser criar o arquivo `setup.exe` que instala tudo automaticamente e cria ícones:
    ```powershell
    cargo build --release -p installer
    # O arquivo estará em target/release/installer.exe
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
