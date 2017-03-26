$name = try {
  git describe --tags --exact-match 2>$null
} catch {
  $null
}
if ($name -eq $null) {
  $name = try {
    git symbolic-ref -q --short HEAD 2>$null
  } catch {
    $null
  }
}
if ($name -eq $null) {
  $name = try {
    git rev-parse --short HEAD 2>$null
  } catch {
    $null
  }
}
if ($name -eq $null) {
  $name = "UNKNOWN"
}
Write-Output $name