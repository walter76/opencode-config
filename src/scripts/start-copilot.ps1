<#
.SYNOPSIS
    Starts GitHub Copilot CLI with a focused set of local permissions.

.DESCRIPTION
    Starts Copilot in the current directory. File edits and read-only Git
    inspection are allowed automatically; other tools retain normal approval
    prompts.

.PARAMETER CopilotArgument
    Additional arguments passed through to the Copilot CLI.

.EXAMPLE
    .\start-copilot.ps1

.EXAMPLE
    .\start-copilot.ps1 --continue
#>
[CmdletBinding()]
param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$CopilotArgument
)

$ErrorActionPreference = 'Stop'

$copilot = Get-Command copilot -ErrorAction SilentlyContinue
if ($null -eq $copilot) {
    throw 'Copilot CLI was not found on PATH. Install it before running this script.'
}

$workingDirectory = (Get-Location).Path

$permissionArguments = @(
    '--allow-tool=write'
    '--allow-tool=shell(git status)'
    '--allow-tool=shell(git diff:*)'
    '--allow-tool=shell(git log:*)'
    '--allow-tool=shell(git show:*)'
    '--allow-tool=shell(git ls-files:*)'
)

& $copilot.Source -C $workingDirectory @permissionArguments @CopilotArgument
exit $LASTEXITCODE
