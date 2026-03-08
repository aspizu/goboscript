#Requires -Version 5.1
Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$RepoUrl    = "https://github.com/aspizu/goboscript"
$InstallDir = Join-Path $env:LOCALAPPDATA "goboscript"
$BinDir     = Join-Path $env:USERPROFILE  ".local\bin"
$BinaryName = "goboscript.exe"

function Write-Info    { param($msg) Write-Host "   info  $msg" -ForegroundColor Cyan }
function Write-Ok      { param($msg) Write-Host "     ok  $msg" -ForegroundColor Green }
function Write-Warn    { param($msg) Write-Host "   warn  $msg" -ForegroundColor Yellow }
function Write-Banner  { param($msg) Write-Host "`n  $msg`n"    -ForegroundColor Cyan }
function Write-Fatal   { param($msg) Write-Host "  error  $msg" -ForegroundColor Red; exit 1 }

function Write-Ask {
    param($msg)
    Write-Host "     ?>  $msg " -ForegroundColor Blue -NoNewline
}

function Find-Cargo {
    $cargo = Get-Command cargo -ErrorAction SilentlyContinue
    if ($cargo) { return $cargo.Source }

    $fallback = Join-Path $env:USERPROFILE ".cargo\bin\cargo.exe"
    if (Test-Path $fallback) { return $fallback }

    if ($env:CARGO_HOME) {
        $fromEnv = Join-Path $env:CARGO_HOME "bin\cargo.exe"
        if (Test-Path $fromEnv) { return $fromEnv }
    }

    return $null
}

function Install-Rustup {
    Write-Info "Downloading rustup installer..."
    $rustupInit = Join-Path $env:TEMP "rustup-init.exe"
    Invoke-WebRequest "https://win.rustup.rs/x86_64" -OutFile $rustupInit -UseBasicParsing
    & $rustupInit -y
    if ($LASTEXITCODE -ne 0) { Write-Fatal "rustup installation failed." }
    Remove-Item $rustupInit -Force
    $env:PATH += ";$env:USERPROFILE\.cargo\bin"
    Write-Ok "Rust toolchain installed."
}

function Invoke-Uninstall {
    Write-Banner "Uninstalling goboscript"
    $removed = $false

    $bin = Join-Path $BinDir $BinaryName
    if (Test-Path $bin) {
        Remove-Item $bin -Force
        Write-Ok "Removed $bin"
        $removed = $true
    } else {
        Write-Warn "Binary not found at $bin"
    }

    if (Test-Path $InstallDir) {
        Remove-Item $InstallDir -Recurse -Force
        Write-Ok "Removed $InstallDir"
        $removed = $true
    } else {
        Write-Warn "Source directory not found at $InstallDir"
    }

    if ($removed) { Write-Ok "goboscript uninstalled." } else { Write-Warn "Nothing to uninstall." }
}

function Invoke-Install {
    Write-Banner "Installing goboscript"

    $cargo = Find-Cargo

    if (-not $cargo) {
        Write-Warn "cargo not found."
        Write-Ask "Install via rustup? [y/N]"
        $response = Read-Host
        if ($response -notmatch "^[yY]") { Write-Fatal "cargo is required. Aborting." }
        Install-Rustup
        $cargo = Find-Cargo
        if (-not $cargo) { Write-Fatal "cargo still not found. Please restart your shell and re-run." }
    }

    Write-Ok "Found cargo: $cargo"

    if (Test-Path (Join-Path $InstallDir ".git")) {
        Write-Info "Updating existing source..."
        git -C $InstallDir pull --ff-only
        if ($LASTEXITCODE -ne 0) { Write-Warn "git pull failed; using existing source." }
    } else {
        Write-Info "Cloning $RepoUrl..."
        New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
        Remove-Item $InstallDir -Recurse -Force
        git clone --depth 1 $RepoUrl $InstallDir
        if ($LASTEXITCODE -ne 0) { Write-Fatal "Failed to clone repository." }
    }

    Write-Ok "Repository ready."
    Write-Info "Building goboscript..."
    New-Item -ItemType Directory -Force -Path $BinDir | Out-Null

    & $cargo install --path $InstallDir --root (Join-Path $env:USERPROFILE ".local") --locked
    if ($LASTEXITCODE -ne 0) { Write-Fatal "cargo install failed." }

    Write-Ok "Installed to $(Join-Path $BinDir $BinaryName)"

    $userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
    if ($userPath -notlike "*$BinDir*") {
        Write-Warn "$BinDir is not in your PATH. Adding it now..."
        [Environment]::SetEnvironmentVariable("PATH", "$userPath;$BinDir", "User")
        $env:PATH += ";$BinDir"
        Write-Ok "Added $BinDir to your user PATH."
    }

    Write-Ok "Done. Run '$($BinaryName -replace '\.exe$','') --help' to get started."
}

switch ($args[0]) {
    "--uninstall" { Invoke-Uninstall }
    "-u"          { Invoke-Uninstall }
    $null         { Invoke-Install   }
    default       { Write-Host "  error  Unknown argument: $($args[0])" -ForegroundColor Red
                    Write-Host "  Usage: .\install.ps1 [-u | --uninstall]"
                    exit 1 }
}
