$name = (git describe --tags --exact-match 2>$null)
if ($LASTEXITCODE -ne 0) {
  $name = (git symbolic-ref -q --short HEAD)
}
Write-Output $name