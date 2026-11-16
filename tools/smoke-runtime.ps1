$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$exe = Join-Path $root 'target\release\mens-life.exe'
$picture = Join-Path $env:RUNNER_TEMP 'mens-life-startup-smoke.png'
$crash = Join-Path $root 'target\release\last-crash.txt'

if (-not (Test-Path -LiteralPath $exe)) { throw "Falta el ejecutable para probar: $exe" }
Remove-Item -LiteralPath $picture, $crash -Force -ErrorAction SilentlyContinue

Write-Host '[smoke] Abriendo mens-life.exe con el renderer y los assets reales...'
$process = Start-Process -FilePath $exe -WorkingDirectory $root -ArgumentList @('--capture', "`"$picture`"", '--start') -PassThru
try {
    if (-not $process.WaitForExit(120000)) {
        Stop-Process -Id $process.Id -Force -ErrorAction SilentlyContinue
        throw 'El ejecutable no completó la captura de arranque dentro del límite.'
    }
    if ($process.ExitCode -ne 0) {
        throw "El proceso finalizó con código $($process.ExitCode)."
    }
    if (-not (Test-Path -LiteralPath $picture -PathType Leaf)) {
        throw 'La aplicación salió sin renderizar una captura. Un build que solo compila no basta.'
    }
    $bytes = (Get-Item -LiteralPath $picture).Length
    if ($bytes -lt 1024) { throw "Captura de arranque inválida: $bytes bytes." }
    Write-Host "[smoke] Arranque, render y cierre controlado OK ($bytes bytes)."
}
catch {
    Write-Host '[smoke] Falló la ejecución real de mens-life.' -ForegroundColor Red
    if (Test-Path -LiteralPath $crash) {
        Write-Host '--- last-crash.txt ---'
        Get-Content -LiteralPath $crash
    }
    throw
}
