@echo off
REM Build + demo the installer: stage the sample payload into demo-install, then verify.
cd /d "%~dp0"
echo == cargo build ==
cargo build || goto :eof
echo.
echo == install payload -^> demo-install ==
cargo run -- install .\payload .\demo-install
echo.
echo == verify demo-install ==
cargo run -- verify .\demo-install
echo.
echo (run "cargo run -- uninstall .\demo-install" to remove it, or "cargo test" for the suite)
