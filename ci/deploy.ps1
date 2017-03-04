param(
  [parameter(mandatory)]
  [string]$pkgname
)

Remove-Item -Recurse -Force ".\$($pkgname)"
cargo install --root ".\$($pkgname)"

7z a "$($pkgname).zip" ".\$($pkgname)\"
