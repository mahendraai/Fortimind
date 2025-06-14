# PowerShell script to install necessary components for Windows

# Check if running as Administrator
if (-not ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    Write-Host "Please run this script as an Administrator."
    exit
}

# Update system
Write-Host "Updating system..."
Start-Process "powershell.exe" -ArgumentList "Start-Process -Verb RunAs -Command 'sfc /scannow'" -Wait

# Install required software
$softwareList = @(
    "Chocolatey",
    "Git",
    "Rust"
)

foreach ($software in $softwareList) {
    Write-Host "Installing $software..."
    switch ($software) {
        "Chocolatey" {
            Set-ExecutionPolicy Bypass -Scope Process -Force
            [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.SecurityProtocolType]::Tls12
            iex ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))
        }
        "Git" {
            choco install git -y
        }
        "Rust" {
            Invoke-WebRequest -Uri https://sh.rustup.rs -OutFile rustup-init.exe
            Start-Process -FilePath "rustup-init.exe" -ArgumentList "-y" -Wait
            Remove-Item -Path "rustup-init.exe"
        }
    }
}

# Clean up
Write-Host "Installation complete. Please restart your system for changes to take effect."