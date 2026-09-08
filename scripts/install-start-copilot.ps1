<#
.SYNOPSIS
    Installs the Copilot launcher and adds its tools directory to the user PATH.

.DESCRIPTION
    Copies start-copilot.ps1 to a user tools directory and adds that directory to
    the user-level PATH when it is not already present.

.PARAMETER ToolsDirectory
    Destination directory for the installed launcher. Defaults to
    $HOME\Tools\Copilot.

.EXAMPLE
    .\install-start-copilot.ps1

.EXAMPLE
    .\install-start-copilot.ps1 -ToolsDirectory 'C:\Tools\Copilot'
#>
[CmdletBinding()]
param(
    [string]$ToolsDirectory = (Join-Path $HOME 'Tools\Copilot')
)

$ErrorActionPreference = 'Stop'

$sourceScript = Join-Path $PSScriptRoot '..\src\scripts\start-copilot.ps1'
if (-not (Test-Path -LiteralPath $sourceScript -PathType Leaf)) {
    throw "Source script not found: $sourceScript"
}

$resolvedToolsDirectory = [System.IO.Path]::GetFullPath($ToolsDirectory)
New-Item -ItemType Directory -Path $resolvedToolsDirectory -Force | Out-Null

$installedScript = Join-Path $resolvedToolsDirectory 'start-copilot.ps1'
Copy-Item -LiteralPath $sourceScript -Destination $installedScript -Force

$userPath = [Environment]::GetEnvironmentVariable('Path', 'User')
$pathEntries = @($userPath -split ';' | Where-Object { $_ })
if ($pathEntries -notcontains $resolvedToolsDirectory) {
    $updatedPath = (@($pathEntries) + $resolvedToolsDirectory) -join ';'
    [Environment]::SetEnvironmentVariable('Path', $updatedPath, 'User')
}

Write-Host "Installed: $installedScript"
Write-Host "User PATH updated. Open a new PowerShell session to use:"
Write-Host "  start-copilot.ps1"
