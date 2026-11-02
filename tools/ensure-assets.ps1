$ErrorActionPreference = 'Stop'

$root = Split-Path -Parent $PSScriptRoot
$target = Join-Path $root 'assets\house\house.glb'
$temp = "$target.download"

# Pinned immutable source used to assemble source checkouts and CI packages.
# The produced Windows package always contains assets\house\house.glb.
$url = 'https://raw.githubusercontent.com/crazy0codes/hollow-ward/bfed478dd712872cdfe764c4c669a2a8616dea67/public/assets/psx_-_backroom.glb'
$expectedBytes = 1110460

function Test-Backroom([string]$path) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) { return $false }
    $file = Get-Item -LiteralPath $path
    if ($file.Length -ne $expectedBytes) { return $false }
    $stream = [System.IO.File]::OpenRead($path)
    try {
        $magic = New-Object byte[] 4
        if ($stream.Read($magic, 0, 4) -ne 4) { return $false }
        return ([System.Text.Encoding]::ASCII.GetString($magic) -eq 'glTF')
    }
    finally {
        $stream.Dispose()
    }
}

if (Test-Backroom $target) {
    Write-Host '[assets] PSX Backroom listo dentro del proyecto.'
    exit 0
}

New-Item -ItemType Directory -Force (Split-Path -Parent $target) | Out-Null
Remove-Item -LiteralPath $temp -Force -ErrorAction SilentlyContinue

Write-Host '[assets] Instalando PSX Backroom del paquete fijado del proyecto...'
Invoke-WebRequest -UseBasicParsing -Uri $url -OutFile $temp

if (-not (Test-Backroom $temp)) {
    Remove-Item -LiteralPath $temp -Force -ErrorAction SilentlyContinue
    throw "El asset PSX Backroom descargado no coincide con el paquete esperado ($expectedBytes bytes, GLB)."
}

Move-Item -LiteralPath $temp -Destination $target -Force
Write-Host '[assets] PSX Backroom instalado en assets\house\house.glb.'
