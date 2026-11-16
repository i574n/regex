param(
    $fast,
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. deps/spiral/scripts/core.ps1
. deps/spiral/lib/spiral/lib.ps1

$notebooks = @(Get-ChildItem lib, src -Filter *.spi | ForEach-Object { '--path'; $_.FullName }) + @('--path', (Join-Path $ScriptDir 'precomputed' 'precomputed_check.spi'))
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

if (!(BuildSpiral rx_crate/rx_crate.spi rx_crate/rx_crate.rs "regex/rx_crate")) {
    throw "RUST-FAILED regex/rx_crate / compile"
}
if (!(BuildSpiral rx_lazy/rx_lazy.spi rx_lazy/rx_lazy.rs "regex/rx_lazy")) {
    throw "RUST-FAILED regex/rx_lazy / compile"
}
if (!(BuildSpiral rx_emit/rx_emit.spi rx_emit/rx_emit.rs "regex/rx_emit")) {
    throw "RUST-FAILED regex/rx_emit / compile"
}
if (!(BuildSpiral rx_facts/rx_facts.spi rx_facts/rx_facts.rs "regex/rx_facts")) {
    throw "RUST-FAILED regex/rx_facts / compile"
}
if (!(BuildSpiral rx_conformance/rx_conformance.spi rx_conformance/rx_conformance.rs "regex/rx_conformance")) {
    throw "RUST-FAILED regex/rx_conformance / compile"
}
if (!(BuildSpiral rx_external/rx_external.spi rx_external/rx_external.rs "regex/rx_external")) {
    throw "RUST-FAILED regex/rx_external / compile"
}
if (!(BuildSpiral ssv_video/ssv_video.spi ssv_video/ssv_video.rs "regex/ssv_video")) {
    throw "RUST-FAILED regex/ssv_video / compile"
}
if (!(BuildSpiral bench/bench.spi bench/bench.rs "regex/bench")) {
    throw "RUST-FAILED regex/bench / compile"
}
$spiral = Ensure-SpiralCompiler | Select-Object -Last 1
foreach ($crate in 'rx_crate', 'rx_lazy', 'rx_emit', 'rx_facts', 'rx_conformance', 'rx_external', 'ssv_video', 'bench') {
    & $spiral.Dotnet $spiral.Compiler partition-rust (Join-Path $ScriptDir $crate "$crate.rs") --limit 1000
    if ($LASTEXITCODE -ne 0) { throw "RUST-FAILED regex/$crate / partition-rust exited $LASTEXITCODE" }
}
$denseBodies = @(Get-ChildItem bench -Filter *.rs -Recurse | Where-Object { $_.FullName -notmatch '[\\/]target[\\/]' } | ForEach-Object {
        $lines = @(Get-Content $_.FullName)
        for ($i = 0; $i -lt $lines.Count; $i++) {
            if ($lines[$i] -match '^pub fn (dense_full|dense_step)_\d+') {
                $name = $Matches[0] -replace '^pub fn ', ''
                $end = $i + 1
                while ($end -lt $lines.Count -and $lines[$end] -ne '}') { $end++ }
                [pscustomobject]@{ Name = $name; Body = ($lines[($i + 1)..$end] -join "`n") }
            }
        } })
if ($denseBodies.Count -lt 2) { throw "build.ps1 / dense census: found $($denseBodies.Count) of the dense_full and dense_step bodies in the generated bench crate" }
$denseFaults = @($denseBodies | Where-Object { $_.Body -match '\.clone\(\)|Rc::new|wrapping_mul' } | ForEach-Object Name)
if ($denseFaults) { throw "build.ps1 / dense census: the dense matcher loop clones, allocates an Rc or multiplies per step in $($denseFaults -join ', ')" }
Write-Output "build.ps1 / dense census / $($denseBodies.Count) generated bodies, no clone, no Rc::new, no per-step multiply"
{ cargo build --release } | Invoke-Block
$cargoTarget = (cargo metadata --format-version 1 --no-deps | ConvertFrom-Json).target_directory
if (!$cargoTarget) { throw "RUST-FAILED regex / cargo metadata found no target directory" }

$report = @(& (Join-Path $cargoTarget 'release' "regex-bench$(_exe)"))
$benchExit = $LASTEXITCODE
[IO.File]::WriteAllText((Join-Path $ScriptDir 'bench' 'report.html'), (($report -join "`n") + "`n"))
if ($benchExit -ne 0) { throw "build.ps1 / regex-bench exited ${benchExit}: a deterministic row or an arm count disagrees (bench/report.html)" }
Write-Output "build.ps1 / bench / bench/report.html"

$env:REGEX_EMIT_PRECOMPUTED = '1'
$precomputed = @(& (Join-Path $cargoTarget 'release' "regex-bench$(_exe)"))
$precomputedExit = $LASTEXITCODE
Remove-Item Env:REGEX_EMIT_PRECOMPUTED
if ($precomputedExit -ne 0) { throw "build.ps1 / regex-bench precomputed emit exited $precomputedExit" }
$precomputedPath = Join-Path $ScriptDir 'precomputed' 'precomputed_tables.spi'
$precomputedText = ($precomputed -join "`n") + "`n"
$precomputedNow = if (Test-Path $precomputedPath) { [IO.File]::ReadAllText($precomputedPath).Replace("`r`n", "`n") } else { '' }
if ($precomputedNow -ne $precomputedText) {
    [IO.File]::WriteAllText($precomputedPath, $precomputedText)
    throw "build.ps1 / precomputed/precomputed_tables.spi was stale and is now regenerated from the bench emitter; commit the regenerated file"
}
Write-Output "build.ps1 / precomputed / precomputed_tables.spi matches the emitter"

$generated = @(Get-ChildItem bench, rx_external, rx_crate, ssv_video, rx_lazy, rx_emit, rx_facts, rx_conformance -Filter *.rs -Recurse | Where-Object { $_.FullName -notmatch '[\\/]target[\\/]' } | ForEach-Object { [pscustomobject]@{ Path = $_.FullName; Lines = @(Get-Content $_.FullName).Count } } | Sort-Object Lines -Descending)
Write-Output "build.ps1 / largest generated file / $($generated[0].Lines) lines / $($generated[0].Path)"
if ($generated[0].Lines -gt 1000) { throw "build.ps1 / 1000-line gate: $($generated[0].Path) has $($generated[0].Lines) lines after partition-rust (a single generated function over the limit; split it at the Spiral source)" }
Write-Output "REGEX-OK"
