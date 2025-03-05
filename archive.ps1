Write-Host
Write-Host "This script is used to archive the entire workspace. The script cleans the repo, vendors dependencies, and compresses the result."
Write-Host
Write-Host
Write-Host

$root_dir = "$PSScriptRoot"
$cli_out_dir = "$root_dir/cli_out"

$ErrorActionPreference = "Stop"
Push-Location $root_dir

function GetAndClearCiOutDir {
    $caller_path = $MyInvocation.PSCommandPath
    $target_name = (Get-Item $caller_path).BaseName
    $target_dir = "$cli_out_dir/$target_name"

    $cli_out_dir_exists = Test-Path $cli_out_dir
    if (!$cli_out_dir_exists) {
        New-Item -Path $cli_out_dir -ItemType Directory | out-null
    }

    $destinationDirectoryWasLogged = $false

    if (Test-Path $target_dir) {
        $target_dir = Resolve-Path $target_dir
        Write-Host "destination directory is: ``$target_dir``"
        $destinationDirectoryWasLogged = $true

        Write-Host
        Write-Warning "destination directory exists already"
        $target_dir = Resolve-Path $target_dir
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

    if ($destinationDirectoryWasLogged -eq $false) {
        $target_dir = Resolve-Path $target_dir
        Write-Host "destination directory is: ``$target_dir``"
        $destinationDirectoryWasLogged = $true
    }

    Write-Host

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

try {
    Write-Host "checking dependencies..."

    $missing_dependencies = 0
    if (Get-Command 7z.exe -ErrorAction SilentlyContinue) {
        Write-Output "7zip found"
    } else {
        $missing_dependencies += 1
        Write-Warning "7zip not found"
    }

    if (wsl which tars) {
        Write-Output "wsl tar found"
    } else {
        $missing_dependencies += 1
        Write-Warning "wsl tar not found"
    }

    if ($missing_dependencies -gt 0) {
        Write-Warning "some dependencies could not be found. compression may fail"
        $user_input = Read-Host "continue? (y/N)"
        if ($user_input.ToLower() -ne "y") {
            exit;
        }
    }

    Write-Host "asking for user input..."
    $enum_clean_none = 0
    $enum_clean_except_vendor = 1
    $enum_clean_all = 2
    $cli_clean_value = $enum_clean_none

    $cli_vendor_value = $false

    $cli_compress_value = $false

    $user_input = Read-Host "should the workspace be cleaned? (y/N)"
    if ($user_input.ToLower() -eq "y") {
        $user_input = Read-Host "exclude vendor from clean? (Y/n)"

        if ($user_input.ToLower() -eq "n") {
            $cli_clean_value = $enum_clean_all
        } else {
            $cli_clean_value = $enum_clean_except_vendor
        }
    }

    $user_input = Read-Host "should dependencies be downloaded? (y/N)"
    if ($user_input.ToLower() -eq "y") {
        $cli_vendor_value = $true
    }

    $user_input = Read-Host "should be compressed? (y/N)"
    if ($user_input.ToLower() -eq "y") {
        $cli_compress_value = $true
    }

    Write-Host
    Write-Host "confirm settings:"

    if ($cli_clean_value -eq $enum_clean_none) {
        $cli_clean_display = "no"
    } elseif ($cli_clean_value -eq $enum_clean_except_vendor) {
        $cli_clean_display = "yes, except vendor"
    } elseif ($cli_clean_value -eq $enum_clean_all) {
        $cli_clean_display = "yes, all"
    }

    if ($cli_vendor_value -eq $true) {
        $cli_vendor_display = "yes"
    } else {
        $cli_vendor_display = "no"
    }

    if ($cli_compress_value -eq $true) {
        $cli_compress_display = "yes"
    } else {
        $cli_compress_display = "no"
    }

    Write-Host "  clean:    $cli_clean_display"
    Write-Host "  vendor:   $cli_vendor_display"
    Write-Host "  compress: $cli_compress_display"

    
    $user_input = Read-Host "continue with these settings? (Y/n)"
    if ($user_input.ToLower() -eq "n") {
        exit
    }

    if ($cli_clean_value -ne $enum_clean_none) {
        Write-Host "cleaning workspace..."

        Write-Host "git reset ."
        git reset .

        Write-Host "git checkout -- ."
        git checkout -- .

        if ($cli_clean_value -eq $enum_clean_except_vendor) {
            Write-Host "git clean -dxf -e `"vendor/`" -e `".cargo/`""
            git clean -dxf -e "vendor/" -e ".cargo/"
        } else {
            Write-Host "git clean -dxf"
            git clean -dxf
        }
    }

    if ($cli_vendor_value -eq $true) {
        Write-Host "clearing cargo config directory..."
        $cargo_config_path = ".cargo/config.toml";
        $cargo_config_directory = Split-Path -parent $cargo_config_path

        if (Test-Path $cargo_config_directory) {
            Remove-Item -Recurse -Force $cargo_config_directory
        }

        New-Item -Path $cargo_config_directory -ItemType Directory | out-null

        Write-Host "downloading dependencies..."
        $vendor_output = cargo vendor | Out-String

        Write-Host $vendor_output

        New-Item -Path $cargo_config_path -ItemType File | out-null
        Set-Content -Path $cargo_config_path -Value $vendor_output
    }

    if ($cli_compress_value -eq $true) {
        Write-Host "creating destination directory..."
        $final_directory = GetAndClearCiOutDir

        Write-Host "prepare compression for zip..."
        $archive_date = Get-Date -Format "yyyy_MM_dd"
        $target_path = "$final_directory/ris_engine_$archive_date"

        Write-Host "compressing..."
        RunCommand ".`7z.exe a -t7z -m0=lzma -mx=9 -mfb=64 -md=32m -ms=on -x'!cli_out' -x'!.git' $target_path.7z *"
        RunCommand ".`7z.exe a -tzip -mx9 -mfb=258 -mpass=15 -r -x'!cli_out' -x'!.git' $target_path.zip *"
        
        Write-Host "prepare compression for tgz..."
        
        $target_path = $target_path.Replace('\','/').Replace('C:','/mnt/c')
        $source_dir = Resolve-Path "."
        $source_dir = "$source_dir".Replace('\','/').Replace('C:','/mnt/c')

        Write-Host "compressing..."
        RunCommand "wsl tar --exclude='cli_out' --exclude='.git' -czf $target_path.tgz -C $source_dir ."

        $destination = Resolve-Path $final_directory
        Write-Host "done! compressed archives can be found under ``$destination``"
    } else {
        Write-Host "done!"
    }
}
finally {
    Pop-Location
}
