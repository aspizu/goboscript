@echo off
REM this uses C:\System32\tar.exe
REM despite the fact that it's named tar, it doesn't actually handle tar files,
REM instead, it only handles zip.

if /i "%~1"=="compile" (
    cargo run build -i playground
    tar -C playground -xkf playground/playground.sb3 project.json
    python -m json.tool --indent 4 playground/project.json playground/project.json
    node tools/sb3.js playground/project.json
    goto :end
)

if /i "%~1"=="check" (
    tar -C playground -xf playground/playground.sb3 project.json
    python -m json.tool --indent 4 playground/project.json playground/project.json
    node tools/sb3.js playground/project.json
    goto :end
)

if /i "%~1"=="uncompile" (
    tar -C playground -xf playground/playground.sb3 project.json
    python -m json.tool --indent 4 playground/project.json playground/project.json
    node tools/sb3.js playground/project.json
    goto :end
)

if /i "%~1"=="patch" (
    node tools/sb3.js playground/project.json
    jq -c . playground/project.json > project.json
    tar -C playground -uf playground/playground.sb3 project.json
    del project.json
    goto :end
)

if /i "%~1"=="test" (
    echo test not yet implemented in run.cmd.
    echo if possible, run the Linux shell file.
    goto :end
)

echo "Usage: tools/run {compile|uncompile|patch|check}"
:end exit /B