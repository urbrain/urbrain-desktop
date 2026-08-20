# © 2025-2026 GITC — Global Innovation Technology Company (GITC)
#
# Signs a single file IN PLACE using SSL.com eSigner CodeSignTool (cloud EV
# signing). Tauri invokes this via `bundle.windows.signCommand` during the
# Windows build, once per binary/installer.
#
# Required environment (set by the "Prepare Windows code signing" CI step and
# the build step):
#   CODESIGNTOOL_BAT       full path to CodeSignTool.bat
#   ESIGNER_USERNAME       SSL.com account username
#   ESIGNER_PASSWORD       SSL.com account password
#   ESIGNER_TOTP_SECRET    SSL.com automation TOTP secret
#   ESIGNER_CREDENTIAL_ID  (optional) signing credential id; needed if the
#                          account has more than one signing certificate
param([Parameter(Mandatory = $true)][string]$File)

$ErrorActionPreference = "Stop"

if ([string]::IsNullOrWhiteSpace($env:CODESIGNTOOL_BAT)) {
  throw "CODESIGNTOOL_BAT is not set - the signing preparation step did not run."
}
if (-not (Test-Path $File)) {
  throw "File to sign not found: $File"
}

$dir = Split-Path -Parent $File

# CodeSignTool writes the signed file to -output_dir_path using the same file
# name; pointing it at the input's directory with -override signs in place.
$signArgs = @(
  "sign",
  "-username=$($env:ESIGNER_USERNAME)",
  "-password=$($env:ESIGNER_PASSWORD)",
  "-totp_secret=$($env:ESIGNER_TOTP_SECRET)",
  "-input_file_path=$File",
  "-output_dir_path=$dir",
  "-override"
)
if (-not [string]::IsNullOrWhiteSpace($env:ESIGNER_CREDENTIAL_ID)) {
  $signArgs += "-credential_id=$($env:ESIGNER_CREDENTIAL_ID)"
}

Write-Host "eSigner: signing $File"
& $env:CODESIGNTOOL_BAT @signArgs
if ($LASTEXITCODE -ne 0) {
  throw "CodeSignTool failed for $File (exit $LASTEXITCODE)"
}
Write-Host "eSigner: signed $File"
