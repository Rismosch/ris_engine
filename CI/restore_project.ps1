$continue = Read-Host "checkout and clean git? (y/n)"

if ($continue -eq "y")
{
    git checkout -- .
    git clean -dxf
}


$bindgen_sdl = [IO.Path]::Combine($PSScriptRoot, "bindgen_sdl.ps1");
. $bindgen_sdl