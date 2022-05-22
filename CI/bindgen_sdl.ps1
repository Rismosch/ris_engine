$ignore = @(
    "begin_code.h",
    "close_code.h",
    "SDL_config.h.cmake",
    "SDL_revision.h.cmake",
    "SDL_config.h.in"
)

$root = [IO.Path]::Combine($PSScriptRoot, "..")
$ris_engine = [IO.Path]::Combine($root, "ris_engine")
$sdl_include = [IO.Path]::Combine($root,"SDL2","2.0.22","include")
$sdl_sys = [IO.Path]::Combine($ris_engine,"sdl-sys","src")

$librs = "lib.rs"
$librs_path = [IO.Path]::Combine($sdl_sys, $librs)

New-Item -Path $sdl_sys -Name $librs -ItemType "file" -Value "#![allow(warnings)]`n" -Force

$sdl_headers = Get-ChildItem -Path $sdl_include

[System.Collections.ArrayList]$excluded_files = @()

$length_error = 0
$length_ignored = 0

foreach ($sdl_header in $sdl_headers)
{
    $source = $sdl_header.FullName
    $filename = [System.IO.Path]::GetFileName($source)
    $filenamewithoutextension = [System.IO.Path]::GetFileNameWithoutExtension($source)
    
    if ($ignore.Contains($filename))
    {
        Write-Host "ignoring $filenamewithoutextension"
        $excluded_files.Add($source)
        ++$length_ignored
        continue
    }
    else
    {
        Write-Host "building $filenamewithoutextension..."
    }

    $target = [IO.Path]::Combine($sdl_sys, "$filenamewithoutextension.rs")
    bindgen $source -o $target

    if ($LASTEXITCODE -ne 0)
    {
        $excluded_files.Add($source)
        ++$length_error
        Remove-Item $target
    }
    else {
        Add-Content -Path $librs_path -Value "pub mod $filenamewithoutextension;"
    }
}

$length_excluded = $excluded_files.Count
$length_source = $sdl_headers.Length
$length_target = $length_source - $length_excluded

Write-Host "`nsuccess: $length_target"
Write-Host "erroneous: $length_error"
Write-Host "ignored: $length_ignored"
Write-Host "total: $length_source"

if ($length_excluded -gt 0)
{
    Write-Host "`nexcluded $length_excluded due to error or ignore:"
    foreach ($excluded_file in $excluded_files)
    {
        Write-Host "    -> $excluded_file"
    }
}
