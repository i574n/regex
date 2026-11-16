param(
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ../deps/spiral/scripts/core.ps1

{ pwsh ../deps/spiral/scripts/publish-tree.ps1 -Root .. -Include 'report.html' } | Invoke-Block

$width, $height, $rate, $frames = 320, 180, 24, 48
$video = New-Item -ItemType Directory -Force ../dist/ssv
$raw = Join-Path $video frames.rgb
$mp4 = Join-Path $video state-space.mp4
$cargoTarget = (cargo metadata --format-version 1 --no-deps --manifest-path ../Cargo.toml | ConvertFrom-Json).target_directory
$renderer = Join-Path $cargoTarget 'release' "regex-ssv-video$(_exe)"
if (!(Test-Path $renderer)) { throw "VIDEO-FAILED regex/ssv / $renderer is missing; build.ps1 builds it" }
$env:SSV_WIDTH, $env:SSV_HEIGHT, $env:SSV_FRAMES = $width, $height, $frames
$hashes = foreach ($run in 1, 2) {
    $env:SSV_FRAMES_OUT = $raw
    { & $renderer } | Invoke-Block
    $rawBytes = (Get-Item $raw).Length
    if ($rawBytes -ne $width * $height * 3 * $frames) { throw "VIDEO-FAILED regex/ssv / raw frames $rawBytes B, expected $($width * $height * 3 * $frames)" }
    (Get-FileHash $raw -Algorithm SHA256).Hash
}
if ($hashes[0] -ne $hashes[1]) { throw "VIDEO-FAILED regex/ssv / two renders differ: $($hashes[0]) vs $($hashes[1])" }
Write-Output "VIDEO-FRAMES regex/ssv / two renders byte-identical sha256 $($hashes[0].Substring(0, 12))"
if (!(Get-Command ffmpeg -ErrorAction Ignore)) {
    if (!$IsLinux) { Remove-Item $raw -Force; Write-Output "VIDEO-SKIPPED regex/ssv / no ffmpeg on this host"; return }
    { sudo apt-get install -y -qq ffmpeg } | Invoke-Block
}
{ ffmpeg -v error -y -f rawvideo -pix_fmt rgb24 -s "$($width)x$($height)" -r $rate -i $raw -c:v libx264 -pix_fmt yuv420p -movflags +faststart $mp4 } | Invoke-Block
Remove-Item $raw -Force
$media = ffprobe -v error -count_frames -select_streams v:0 -show_entries stream=nb_read_frames,width,height -show_entries format=duration -of json $mp4 | ConvertFrom-Json
$stream = $media.streams[0]
$seconds = [double]$media.format.duration
$size = (Get-Item $mp4).Length
if ([int]$stream.nb_read_frames -ne $frames -or $stream.width -ne $width -or $stream.height -ne $height -or [math]::Abs($seconds - $frames / $rate) -gt 0.05 -or $size -gt 5MB) {
    throw "VIDEO-FAILED regex/ssv / frames $($stream.nb_read_frames) size $($stream.width)x$($stream.height) duration $seconds s bytes $size"
}
Write-Output "VIDEO-OK regex/ssv / state-space.mp4 $($stream.nb_read_frames) frames $($stream.width)x$($stream.height) $seconds s $size B"
