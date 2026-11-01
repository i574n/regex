param(
    $fast,
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. deps/spiral/scripts/core.ps1
. deps/spiral/lib/spiral/lib.ps1

$notebooks = @(Get-ChildItem src -Filter *.spi | ForEach-Object { '--path'; $_.FullName })
$env:SPIRAL_CELLS_MEMO = '0'
$cellsStarted = Get-Date
Push-Location (Join-Path 'deps' 'spiral' 'apps' 'cells')
try { $cells = @(& pwsh -NoProfile -File (Join-Path 'spi' 'run_notebook.ps1') --gate @notebooks 2>&1 | ForEach-Object { "$_" }); $cellsExit = $LASTEXITCODE }
finally { Pop-Location }
$cells | ForEach-Object { Write-Output "build.ps1 / cells / $_" }
if ($IsWindows -and ($cells -match 'compiler socket failed')) {
    Get-WinEvent -FilterHashtable @{ LogName = 'Application'; ProviderName = 'Application Error'; StartTime = $cellsStarted } -ErrorAction Ignore |
        Where-Object { $_.Message -match 'Faulting application name: dotnet\.exe' } |
        ForEach-Object { $_.Message -split "`r?`n" | Where-Object { $_ -match '^(Faulting module name|Exception code|Fault offset):' } } |
        ForEach-Object { Write-Output "build.ps1 / cells / compiler crash / $($_.Trim())" }
}
if ($cellsExit -ne 0) { throw "build.ps1 / Spiral test cells exited $cellsExit" }
if (!($cells -match '^spiral-tests total')) { throw "build.ps1 / Spiral test cells printed no spiral-tests total line" }

if (!(BuildSpiral bench/bench.spi bench/bench.rs "regex/bench")) {
    throw "RUST-FAILED regex/bench / compile"
}
{ cargo +nightly-2026-10-18 build --release } | Invoke-Block
$cargoTarget = (cargo +nightly-2026-10-18 metadata --format-version 1 --no-deps | ConvertFrom-Json).target_directory
if (!$cargoTarget) { throw "RUST-FAILED regex / cargo metadata found no target directory" }

$report = @(& (Join-Path $cargoTarget 'release' "regex-bench$(_exe)"))
$benchExit = $LASTEXITCODE
[IO.File]::WriteAllText((Join-Path $ScriptDir 'bench' 'report.html'), (($report -join "`n") + "`n"))
if ($benchExit -ne 0) { throw "build.ps1 / regex-bench exited ${benchExit}: a deterministic row or an arm count disagrees (bench/report.html)" }
Write-Output "build.ps1 / bench / bench/report.html"

$generated = @(Get-ChildItem bench -Filter *.rs | ForEach-Object { [pscustomobject]@{ Path = $_.FullName; Lines = @(Get-Content $_.FullName).Count } } | Sort-Object Lines -Descending)
Write-Output "build.ps1 / largest generated file / $($generated[0].Lines) lines / $($generated[0].Path)"
if ($generated[0].Lines -gt 1000) { throw "build.ps1 / $($generated[0].Path) has $($generated[0].Lines) lines, over the 1000-line gate: split its Spiral entry by hand" }
Write-Output "REGEX-OK"
