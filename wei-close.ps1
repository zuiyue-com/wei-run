param (
    [Parameter(Mandatory=$true)]
    [string]$arg1
)

taskkill /IM "$arg1.exe" /F
