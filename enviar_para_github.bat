@echo off
setlocal

echo ===================================================
echo   Enviando Atualizacoes para o GitHub
echo ===================================================

REM Verifica se e um repositorio git
if not exist .git (
    echo [ERRO] Este diretorio nao e um repositorio Git.
    echo Execute 'configurar_github.bat' primeiro.
    pause
    exit /b 1
)

echo.
echo Adicionando alteracoes...
git add .
git status

echo.
set /p COMMIT_MSG="Digite a mensagem do commit (Pressione Enter para usar 'Atualizacao automatica'): "
if "%COMMIT_MSG%"=="" set COMMIT_MSG="Atualizacao automatica em %date% %time%"

echo.
echo Realizando commit: "%COMMIT_MSG%"
git commit -m "%COMMIT_MSG%"

echo.
echo Enviando para o GitHub...
git push origin main

if %errorlevel% neq 0 (
    echo.
    echo [ERRO] Falha ao enviar. Verifique sua conexao ou permissoes.
) else (
    echo.
    echo [SUCESSO] Atualizacao enviada com sucesso!
)

pause
