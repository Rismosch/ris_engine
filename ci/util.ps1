$root_dir = "$PSScriptRoot/.."
$ci_out_dir = "$root_dir/ci_out"

function GetAndClearCiOutDir {
    $caller_path = $MyInvocation.PSCommandPath
    $target_name = (Get-Item $caller_path).BaseName
    $target_dir = "$ci_out_dir/$target_name"

    Write-Host "destination directory is: ``$target_dir``"

    $ci_out_dir_exists = Test-Path $ci_out_dir
    if (!$ci_out_dir_exists) {
        New-Item -Path $ci_out_dir -ItemType Directory | out-null
    }

    if (Test-Path $target_dir) {
        Write-Host
        Write-Host "WARNING: destination directory exists already"
        $user_input = Read-Host "are you sure you want to delete ``$target_dir``? (y/N)"
        if ($user_input.ToLower() -eq "y") {
            Write-Host "deleting..."
            Remove-Item -Recurse -Force $target_dir
            Write-Host "deleted ``$target_dir``"
        }

    }

    $target_dir_exists = Test-Path $target_dir
    if (!$target_dir_exists) {
        New-Item -Path $target_dir -ItemType Directory | out-null
    }

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
