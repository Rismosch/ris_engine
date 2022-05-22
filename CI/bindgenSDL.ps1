$root = [IO.Path]::Combine($PSScriptRoot, "..")
$ris_engine = [IO.Path]::Combine($root, "ris_engine")
$sdl_include = [IO.Path]::Combine($root,"SDL2","2.0.22","include")
$sdl_sys = [IO.Path]::Combine($ris_engine,"sdl-sys","src")

$sdl_headers = Get-ChildItem -Path $sdl_include

[System.Collections.ArrayList]$erroneous_files = @()

foreach ($sdl_header in $sdl_headers)
{
    $source = $sdl_header.FullName
    $filename = [System.IO.Path]::GetFileNameWithoutExtension($source);
    $target = [IO.Path]::Combine($sdl_sys, "$filename.rs")

    bindgen $source -o $target

    if ($LASTEXITCODE -ne 0)
    {
        $erroneous_files.Add($source);
        Remove-Item $target
    }
}

$length_source = $sdl_headers.Length
$length_target = $length_source - $erroneous_files.Count

Write-Host "`nsuccessfully generated $length_target/$length_source files"

Write-Host "`nfollowing files did not generate properly:"
foreach ($erroneous_file in $erroneous_files)
{
    Write-Host "`t-> $erroneous_file"
}