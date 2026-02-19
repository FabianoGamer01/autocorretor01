param(
    [string]$InstallPath = "$env:USERPROFILE\Downloads\AutocorretorPTBR",
    [string]$RepoOwner = "FabianoGamer01",
    [string]$RepoName = "autocorretor01",
    [switch]$SkipAdmin,
    [switch]$SkipRegsvr,
    [switch]$SkipStartup
)

# Verifica se o script está rodando como Administrador
# Como este script é projetado para ser rodado via 'iex' (Invoke-Expression),
# não podemos reiniciar o processo automaticamente facilmente.
if (!$SkipAdmin -and !([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
    Write-Host "ERRO: Este instalador precisa de privilégios de Administrador." -ForegroundColor Red
    Write-Host "Por favor, abra o PowerShell como Administrador e tente novamente." -ForegroundColor Yellow
    exit 1
}

$ErrorActionPreference = "Stop"

try {
    Write-Host "=== Instalador do Autocorretor PT-BR ===" -ForegroundColor Cyan
    
    # 1. Preparar Diretório
    if (!(Test-Path $InstallPath)) {
        New-Item -ItemType Directory -Force -Path $InstallPath | Out-Null
        Write-Host "Criado diretório: $InstallPath"
    }

    # 2. Parar processos existentes
    $processName = "autocorretor-tray"
    if (Get-Process $processName -ErrorAction SilentlyContinue) {
        Write-Host "Parando o $processName..."
        Stop-Process -Name $processName -Force
        Start-Sleep -Seconds 1
    }

    # 3. Determinar URLs de Download (Lógica para pegar a última release)
    # NOTA: Como ainda pode não haver releases, vou usar URLs placeholder que devem ser ajustadas
    # Se houver releases, a lógica seria:
    # $latestRelease = Invoke-RestMethod -Uri "https://api.github.com/repos/$RepoOwner/$RepoName/releases/latest"
    # $trayUrl = $latestRelease.assets | Where-Object name -like "*tray*.exe" | Select-Object -ExpandProperty browser_download_url
    # $dllUrl = $latestRelease.assets | Where-Object name -like "*ime*.dll" | Select-Object -ExpandProperty browser_download_url
    
    # POR ENQUANTO: Usando placeholder ou paths diretos se estiver rodando local para teste
    # Vou deixar preparado para baixar de um release genérico "vLatest" ou similar.
    # Ajuste manual necessário aqui se os nomes dos assets mudarem.
    
    $baseUrl = "https://github.com/FabianoGamer01/autocorretor01/releases/latest/download"
    $trayExeName = "draco_head.exe"
    $dllName = "draco_body.dll"
    $directmlName = "DirectML.dll"

    $trayUrl = "$baseUrl/$trayExeName"
    $dllUrl = "$baseUrl/$dllName"
    $directmlUrl = "$baseUrl/$directmlName"

    Write-Host "Baixando arquivos..."
    
    # Função auxiliar de download
    function Download-File {
        param($Url, $Dest)
        Write-Host "Baixando: $Url -> $Dest"
        try {
            Invoke-WebRequest -Uri $Url -OutFile $Dest -UseBasicParsing
        }
        catch {
            Write-Warning "Falha ao baixar $Url. Verifique se a Release existe no GitHub."
            throw $_
        }
    }

    # Baixa os arquivos (Comentado para teste local se os arquivos não existirem online ainda)
    # Download-File -Url $trayUrl -Dest "$InstallPath\$trayExeName"
    # Download-File -Url $dllUrl -Dest "$InstallPath\$dllName"

    # PARA TESTE LOCAL (Se o arquivo existir localmente, copia. Se não, tenta baixar)
    # Isso ajuda a testar o script antes de publicar a release
    
    # Nome do binário gerado pelo cargo normalmente é tray-app.exe
    $localTrayName = "tray-app.exe"
    
    if (Test-Path "target\release\$localTrayName") {
        Write-Host "Encontrado build local (Release): $localTrayName"
        Copy-Item "target\release\$localTrayName" "$InstallPath\$trayExeName" -Force
    }
    elseif (Test-Path "target\debug\$localTrayName") {
        Write-Host "Encontrado build local (Debug): $localTrayName"
        Copy-Item "target\debug\$localTrayName" "$InstallPath\$trayExeName" -Force
    }
    else {
        # Tenta baixar se não achar local
        Download-File -Url $trayUrl -Dest "$InstallPath\$trayExeName"
    }

    if (Test-Path "target\release\$dllName") {
        Write-Host "Encontrado build local (Release): $dllName"
        Copy-Item "target\release\$dllName" "$InstallPath\$dllName" -Force
    }
    elseif (Test-Path "target\debug\$dllName") {
        Write-Host "Encontrado build local (Debug): $dllName"
        Copy-Item "target\debug\$dllName" "$InstallPath\$dllName" -Force
    }
    else {
        Download-File -Url $dllUrl -Dest "$InstallPath\$dllName"
    }

    if (Test-Path "target\release\$directmlName") {
        Write-Host "Encontrado build local (Release): $directmlName"
        Copy-Item "target\release\$directmlName" "$InstallPath\$directmlName" -Force
    }
    else {
        Download-File -Url $directmlUrl -Dest "$InstallPath\$directmlName"
    }

    # 4. Registrar a DLL
    if ($SkipRegsvr) {
        Write-Host "PULANDO registro da DLL (Modo de Teste)." -ForegroundColor Yellow
    }
    else {
        Write-Host "Registrando a DLL do IME..."
        $regsvr = "regsvr32.exe"
        $dllPath = "$InstallPath\$dllName"
        
        $proc = Start-Process -FilePath $regsvr -ArgumentList "/s `"$dllPath`"" -PassThru -Wait
        
        if ($proc.ExitCode -eq 0) {
            Write-Host "IME registrado com sucesso!" -ForegroundColor Green
        }
        else {
            Write-Error "Falha ao registrar a DLL. Código de saída: $($proc.ExitCode)"
        }
    }

    # 5. Adicionar aos programas de inicialização (Startup)
    if ($SkipStartup) {
        Write-Host "PULANDO criação de atalho de inicialização (Modo de Teste)." -ForegroundColor Yellow
    }
    else {
        # Cria um atalho no menu iniciar ou startup do usuário
        $WshShell = New-Object -comObject WScript.Shell
        $ShortcutPath = "$env:APPDATA\Microsoft\Windows\Start Menu\Programs\Startup\Autocorretor.lnk"
        $Shortcut = $WshShell.CreateShortcut($ShortcutPath)
        $Shortcut.TargetPath = "$InstallPath\$trayExeName"
        $Shortcut.Description = "Inicializador do Autocorretor PT-BR"
        $Shortcut.Save()
        Write-Host "Adicionado à inicialização do Windows."
    }

    # 6. Iniciar o Tray App agora
    Write-Host "Iniciando a aplicação..."
    Start-Process "$InstallPath\$trayExeName"

    Write-Host "`nInstalação Concluída!" -ForegroundColor Green
    Write-Host "Vá em Configurações > Hora e Idioma > Idioma e ative o teclado 'Autocorretor PT-BR'."

}
catch {
    Write-Error "Erro durante a instalação: $_"
    Read-Host "Pressione Enter para sair..."
}
