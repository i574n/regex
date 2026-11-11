param(
    $fast,
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. deps/spiral/scripts/core.ps1
. deps/spiral/lib/spiral/lib.ps1

$libModules = @(Get-ChildItem src, bench, staged -Filter *.spi | Select-String -Pattern '^:< (regex_\w+)$' | ForEach-Object { $_.Matches[0].Groups[1].Value } | Sort-Object -Unique)
$libNotebooks = foreach ($m in $libModules) {
    $p = Join-Path $ScriptDir 'deps' 'spiral' 'lib' 'spiral' "$m.spi"
    if (-not (Test-Path -LiteralPath $p)) { throw "build.ps1 / regex opens lib module $m but $p does not exist" }
    '--path'; (Resolve-Path -LiteralPath $p).Path
}
Write-Output "build.ps1 / lib regex modules gated here: $($libModules -join ' ')"
$notebooks = @(Get-ChildItem src -Filter *.spi | ForEach-Object { '--path'; $_.FullName }) + @('--path', (Join-Path $ScriptDir 'staged' 'staged_check.spi')) + @($libNotebooks)
$env:SPIRAL_CELLS_MEMO = '0'
$cellsStarted = Get-Date
Push-Location ($env:SPIRAL_CELLS_ROOT ? $env:SPIRAL_CELLS_ROOT : (Join-Path 'deps' 'spiral' 'apps' 'cells'))
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

if (!(BuildSpiral units/rx_crate/rx_crate.spi units/rx_crate/rx_crate.rs "regex/rx_crate")) {
    throw "RUST-FAILED regex/rx_crate / compile"
}
if (!(BuildSpiral units/rx_external/rx_external.spi units/rx_external/rx_external.rs "regex/rx_external")) {
    throw "RUST-FAILED regex/rx_external / compile"
}
if (!(BuildSpiral bench/bench.spi bench/bench.rs "regex/bench")) {
    throw "RUST-FAILED regex/bench / compile"
}
{ cargo build --release } | Invoke-Block
$cargoTarget = (cargo metadata --format-version 1 --no-deps | ConvertFrom-Json).target_directory
if (!$cargoTarget) { throw "RUST-FAILED regex / cargo metadata found no target directory" }

$report = @(& (Join-Path $cargoTarget 'release' "regex-bench$(_exe)"))
$benchExit = $LASTEXITCODE
[IO.File]::WriteAllText((Join-Path $ScriptDir 'bench' 'report.html'), (($report -join "`n") + "`n"))
if ($benchExit -ne 0) { throw "build.ps1 / regex-bench exited ${benchExit}: a deterministic row or an arm count disagrees (bench/report.html)" }
Write-Output "build.ps1 / bench / bench/report.html"

$env:REGEX_EMIT_STAGED = '1'
$staged = @(& (Join-Path $cargoTarget 'release' "regex-bench$(_exe)"))
$stagedExit = $LASTEXITCODE
Remove-Item Env:REGEX_EMIT_STAGED
if ($stagedExit -ne 0) { throw "build.ps1 / regex-bench staged emit exited $stagedExit" }
$stagedPath = Join-Path $ScriptDir 'staged' 'staged_tables.spi'
$stagedText = ($staged -join "`n") + "`n"
$stagedNow = if (Test-Path $stagedPath) { [IO.File]::ReadAllText($stagedPath).Replace("`r`n", "`n") } else { '' }
if ($stagedNow -ne $stagedText) {
    [IO.File]::WriteAllText($stagedPath, $stagedText)
    throw "build.ps1 / staged/staged_tables.spi was stale and is now regenerated from the bench emitter; commit the regenerated file"
}
Write-Output "build.ps1 / staged / staged_tables.spi matches the emitter"

$generated = @(Get-ChildItem bench, units/rx_external, units/rx_crate -Filter *.rs | ForEach-Object { [pscustomobject]@{ Path = $_.FullName; Lines = @(Get-Content $_.FullName).Count } } | Sort-Object Lines -Descending)
Write-Output "build.ps1 / largest generated file / $($generated[0].Lines) lines / $($generated[0].Path)"
if ($generated[0].Lines -gt 1000) { Write-Output "::warning::build.ps1 / 1000-line gate OPEN: $($generated[0].Path) has $($generated[0].Lines) lines; bench.rs stays uncommitted until REGEX-BENCH-PAGE splits bench.spi into crates by hand" }
Write-Output "REGEX-OK"
