<#
.SYNOPSIS
    Installs the agents Zellij layout into the default Windows layouts directory.

.DESCRIPTION
    Copies zellij\zellij-agents-layout.kdl to %APPDATA%\Zellij\config\layouts,
    creating the directory when it does not exist. Once installed the layout can
    be started with:

        zellij -l zellij-agents-layout

.PARAMETER LayoutsDirectory
    Destination directory for the installed layout. Defaults to
    %APPDATA%\Zellij\config\layouts.

.PARAMETER Force
    Overwrite an existing layout file without prompting.

.EXAMPLE
    .\install-zellij-layout.ps1

.EXAMPLE
    .\install-zellij-layout.ps1 -Force
#>
[CmdletBinding()]
param(
    [string]$LayoutsDirectory = (Join-Path $env:APPDATA 'Zellij\config\layouts'),
    [switch]$Force
)

$ErrorActionPreference = 'Stop'

$sourceLayout = Join-Path $PSScriptRoot '..\zellij\zellij-agents-layout.kdl'
if (-not (Test-Path -LiteralPath $sourceLayout -PathType Leaf)) {
    throw "Source layout not found: $sourceLayout"
}

$resolvedLayoutsDirectory = [System.IO.Path]::GetFullPath($LayoutsDirectory)
New-Item -ItemType Directory -Path $resolvedLayoutsDirectory -Force | Out-Null

$installedLayout = Join-Path $resolvedLayoutsDirectory 'zellij-agents-layout.kdl'
if ((Test-Path -LiteralPath $installedLayout) -and -not $Force) {
    $answer = Read-Host "Layout already exists at $installedLayout. Overwrite? (y/N)"
    if ($answer -notmatch '^(y|yes)$') {
        Write-Host 'Aborted.'
        return
    }
}

Copy-Item -LiteralPath $sourceLayout -Destination $installedLayout -Force

Write-Host "Installed: $installedLayout"
Write-Host "Start it with:"
Write-Host "  zellij -l zellij-agents-layout"
