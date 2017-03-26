param(
  [parameter(mandatory)]
  [string]$pkgname
)

Remove-Item -Recurse -Force ".\$($pkgname)" -ErrorAction SilentlyContinue
cmd /c "cargo install --root `".\$($pkgname)`" 2>&1"

7z a "$($pkgname).zip" ".\$($pkgname)\"
