$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$exe = Join-Path $root 'target\release\mens-life.exe'
$crash = Join-Path $root 'target\release\last-crash.txt'
$stderrPath = Join-Path $env:RUNNER_TEMP 'mens-life-smoke.stderr.txt'
$stdoutPath = Join-Path $env:RUNNER_TEMP 'mens-life-smoke.stdout.txt'
$env:RUST_LOG = 'wgpu_core=trace,wgpu_hal=warn,bevy_render=debug,bevy_asset=warn'
$env:RUST_BACKTRACE = '1'

if (-not (Test-Path -LiteralPath $exe)) { throw "Falta el ejecutable para probar: $exe" }
Remove-Item -LiteralPath $crash, $stderrPath, $stdoutPath -Force -ErrorAction SilentlyContinue

Write-Host '[smoke] Abriendo mens-life.exe con la escena completa (sin captura de pantalla)...'
$process = Start-Process -FilePath $exe -WorkingDirectory $root -ArgumentList @('--smoke') -RedirectStandardError $stderrPath -RedirectStandardOutput $stdoutPath -PassThru
try {
    if ($process.WaitForExit(10000)) {
        throw "El juego se cerró prematuramente con código $($process.ExitCode)."
    }
    if (Test-Path -LiteralPath $crash) {
        throw 'El juego produjo una excepción en segundo plano al cargar el renderizador o los assets.'
    }
    Write-Host '[smoke] El ejecutable abrió la ventana y continuó en ejecución durante la prueba.'
}
catch {
    Write-Host '[smoke] Falló la ejecución real de mens-life.' -ForegroundColor Red
    if (Test-Path -LiteralPath $stderrPath) {
        Write-Host '--- renderer stderr (last 150 lines) ---'
        Get-Content -LiteralPath $stderrPath -Tail 150
    }
    if (Test-Path -LiteralPath $crash) {
        Write-Host '--- last-crash.txt ---'
        Get-Content -LiteralPath $crash
    }
    throw
}
finally {
    if (-not $process.HasExited) {
        Stop-Process -Id $process.Id -Force -ErrorAction SilentlyContinue
    }
}
