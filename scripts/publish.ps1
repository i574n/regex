param(
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ../deps/spiral/scripts/core.ps1

{ pwsh ../deps/spiral/scripts/publish-tree.ps1 -Root .. -Include 'report.html' } | Invoke-Block
