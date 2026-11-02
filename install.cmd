@echo off
setlocal EnableExtensions EnableDelayedExpansion
cd /d "%~dp0"

echo [mens-life] Revisando actualizaciones...
for /f %%H in ('git rev-parse HEAD 2^>nul') do set "OLD_HEAD=%%H"
if not defined OLD_HEAD (
    echo [ERROR] Esta carpeta no es un repositorio Git valido.
    exit /b 1
)

git pull --ff-only
if errorlevel 1 (
    echo [ERROR] git pull fallo. No se modifico la instalacion.
    exit /b 1
)

for /f %%H in ('git rev-parse HEAD') do set "NEW_HEAD=%%H"
set "NEED_BUILD=0"

if /I not "%OLD_HEAD%"=="%NEW_HEAD%" set "NEED_BUILD=1"
if not exist "target\release\mens-life.exe" set "NEED_BUILD=1"
if not exist "mens-life.exe" set "NEED_BUILD=1"

rem Si falta el interior, fuerza build.rs para localizar/copiar el GLB suministrado.
if not exist "assets\house\house.glb" set "NEED_BUILD=1"

if "%NEED_BUILD%"=="0" (
    echo [mens-life] Ya esta actualizado. No hay nada que instalar.
    exit /b 0
)

echo [mens-life] Hay cambios o faltan archivos. Compilando Release...
cargo build --release
if errorlevel 1 (
    echo [ERROR] La compilacion fallo. Se conserva el ejecutable anterior.
    exit /b 1
)

if not exist "target\release\mens-life.exe" (
    echo [ERROR] La compilacion termino sin generar target\release\mens-life.exe.
    exit /b 1
)

if not exist "assets\house\house.glb" (
    echo [ERROR] No se encontro el interior PSX Backroom.
    echo         Deja psx_-_backroom.glb en esta carpeta, Descargas o Escritorio,
    echo         o define MENS_LIFE_HOUSE_GLB con la ruta del archivo.
    exit /b 1
)

for %%I in ("assets\house\house.glb") do set "HOUSE_SIZE=%%~zI"
if not "%HOUSE_SIZE%"=="1110456" (
    echo [ERROR] assets\house\house.glb no es el archivo esperado ^(tamano: %HOUSE_SIZE% bytes^).
    exit /b 1
)

copy /Y "target\release\mens-life.exe" "mens-life.exe" >nul
if errorlevel 1 (
    echo [ERROR] No se pudo instalar mens-life.exe en la raiz del proyecto.
    exit /b 1
)

echo [mens-life] Instalacion actualizada correctamente.
echo [mens-life] Ejecutable: %CD%\mens-life.exe
exit /b 0
