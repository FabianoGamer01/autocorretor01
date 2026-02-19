@echo off
setlocal enableextensions disabledelayedexpansion

echo ===================================================
echo   Configuracao Inicial do GitHub - PT-BR IME
echo ===================================================

if not exist .git (
    echo [INFO] Inicializando repositorio git...
    git init
    git branch -M main
) else (
    echo [INFO] Repositorio git ja inicializado.
)

REM Check for git configuration
git config user.email >nul 2>&1
if %errorlevel% neq 0 (
    echo.
    echo [AVISO] Usuario Git nao configurado!
    echo Para que seus commits sejam identificados corretamente no GitHub,
    echo precisamos configurar seu Nome e Email.
    echo.
    
    set /p GIT_EMAIL="Digite seu email (o mesmo usado no GitHub): "
    git config user.email "%GIT_EMAIL%"
    
    set /p GIT_NAME="Digite seu nome (como vai aparecer nos commits): "
    git config user.name "%GIT_NAME%"
    
    echo.
    echo [SUCESSO] Usuario Git configurado!
) else (
    echo [INFO] Usuario Git ja configurado.
)

REM Check if remote origin exists
git remote get-url origin >nul 2>&1
if %errorlevel% neq 0 (
    echo.
    echo [AVISO] Nenhum repositorio remoto configurado.
    set /p REPO_URL="Digite a URL do seu repositorio GitHub (ex: https://github.com/usuario/repo.git): "
    
    if not defined REPO_URL (
        echo [ERRO] Nenhuma URL fornecida. Saindo.
        pause
        exit /b 1
    )
    
    REM Remove quotes if present
    set "REPO_URL=%REPO_URL:"=%"
    
    git remote add origin "%REPO_URL%"
    echo [SUCESSO] Remote 'origin' adicionado.
) else (
    echo [INFO] Remote 'origin' ja esta configurado.
)

echo.
echo Adicionando arquivos...
git add .

echo.
echo Realizando commit inicial...
git commit -m "Commit inicial via script de automacao"

echo.
echo Enviando para o GitHub...
git push -u origin main

if %errorlevel% neq 0 (
    echo.
    echo [ERRO] Falha ao enviar para o GitHub. Verifique suas credenciais ou a URL.
    echo Talvez voce precise configurar um token de acesso pessoal - PAT - se a autenticacao por senha falhar.
) else (
    echo.
    echo [SUCESSO] Codigo enviado para o GitHub com sucesso!
)

pause
