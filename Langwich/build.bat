@echo off
REM Build script for Langwich (Rust Edition)
REM Requires Rust installed from https://rustup.rs/

echo 🦀 Building Langwich v1.2.0...
echo.

REM Check if Rust is installed
where rustc >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo ❌ Rust toolchain is not installed!
    echo Please install Rust from https://rustup.rs/
    echo Then run this script again.
    exit /b 1
)

REM Check Cargo version
rustc --version
cargo --version
echo.

REM Clean previous builds
echo 🧹 Cleaning old builds...
cargo clean
echo.

REM Run tests first
echo 🧪 Running tests...
cargo test --quiet
if %ERRORLEVEL% NEQ 0 (
    echo ❌ Tests failed! Fix errors before building.
    exit /b 1
)
echo ✅ All tests passed!
echo.

REM Build debug version
echo 🔨 Building debug version...
cargo build --quiet
if %ERRORLEVEL% NEQ 0 (
    echo ❌ Debug build failed!
    exit /b 1
)
echo ✅ Debug build complete: target\debug\langwich.exe
echo.

REM Build release version (optimized)
echo ⚡ Building release version...
cargo build --release --quiet
if %ERRORLEVEL% NEQ 0 (
    echo ❌ Release build failed!
    exit /b 1
)
echo ✅ Release build complete: target\release\langwich.exe
echo.

REM Show file sizes
dir /B target\release\langwich.exe
echo.

echo 🎉 Build successful!
echo.
echo Binary location: ^%cd^%\target\release\langwich.exe
echo Debug binary: ^%cd^%\target\debug\langwich.exe
echo.
pause
