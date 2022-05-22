$root = [IO.Path]::Combine($PSScriptRoot, "..")
$ris_engine = [IO.Path]::Combine($root, "ris_engine")
$sdl_include = [IO.Path]::Combine($root,"SDL2","2.0.22","include")
$sdl_sys = [IO.Path]::Combine($ris_engine,"sdl-sys","src")

$sdl_headers = Get-ChildItem -Path $sdl_include

foreach ($sdl_header in $sdl_headers)
{
    $source = $sdl_header.FullName
    $target = [IO.Path]::Combine($sdl_sys, $sdl_header.Name)

    # bindgen $source -o $target
    Write-Host "$source > $target"
}