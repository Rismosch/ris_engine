# Continuous Integration

The scripts in this directory provide some building and development utility.

Generally, Powershell scripts (`.ps1`) are intended for Windows, while Shell scripts (`.sh`) are intended for Linux. Scripts with the same name provide the same functionality.

Each script prints its purpose at the very beginning, which is also easily readable when viewing the script with a text editor. Scipts that generate files always write their result into `./ci_out/<script name>`, which is cleaned at the start of every execution. The only exception to this, are the `util` scripts, which are not intended to be executed. Instead, they provide functionality for the other scripts.