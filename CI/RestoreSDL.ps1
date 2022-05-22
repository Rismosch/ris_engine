$sdl_include = [IO.Path]::Combine($PSScriptRoot, "..","SDL2","2.0.22","include")
$headers = Get-ChildItem -Path $sdl_include

write-host $headers

#write-host $PSScriptRoot