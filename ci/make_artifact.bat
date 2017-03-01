@echo on

if not defined TARGET (
  set TARGET=x86_64-pc-windows-msvc
)

if ""%APPVEYOR_REPO_TAG_NAME%""=="""" (
  set PKGNAME=%PROJECT%-%APPVEYOR_REPO_BRANCH%-%TARGET%
) else (
  set PKGNAME=%PROJECT%-%APPVEYOR_REPO_TAG_NAME%-%TARGET%
)

if exist "%PKGNAME%\" del %PKGNAME%

if not exist %PKGNAME% mkdir %PKGNAME%
cargo build --release --target=%TARGET%
copy target\%TARGET%\release\rhq.exe %PKGNAME%\
