$root_dir = "$PSScriptRoot/.."
$ci_out_dir = "$root_dir/ci_out"

function GetAndClearCiOutDir {
    $caller_path = $MyInvocation.PSCommandPath
    $target_name = (Get-Item $caller_path).BaseName
    $target_dir = "$ci_out_dir/$target_name"

    $ci_out_dir_exists = Test-Path $ci_out_dir;
    if (!$ci_out_dir_exists) {
        New-Item -Path $ci_out_dir -ItemType Directory | out-null
    }

    if (Test-Path $target_dir) {
        Remove-Item -Recurse -Force $target_dir
    }

    New-Item -Path $target_dir -ItemType Directory | out-null

    $result = Resolve-Path $target_dir
    return $result
}

function RunCommand {
    param (
        $command
    )

    try {
        Write-Host "running command: $command"
        return Invoke-Expression $command
    }
    catch {
        return "error while running ``$command``"
    }
}
