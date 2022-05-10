# Debug stuff
Set-PSDebug -Trace 2
Get-ChildItem env:
Write-Output $MyInvocation.MyCommand
Write-Output $args

# Get $version from command line
$version = $args[0]
if (!$version) {
  throw "Usage: $($MyInvocation.MyCommand.Name) [version]
  # Must supply version as first command-line argument"
}
Write-Output $version

# Create dist\ directory
New-Item -Force -ItemType directory -Path dist\
Copy-Item target\release\wordle-gui.exe dist\Wordle.exe

# Edit application icon
$rcEdit = 'wordle-gui\vendor\rcedit-1.1.1\rcedit-x64.exe'
& $rcEdit dist\Wordle.exe --set-icon wordle-gui\images\icon.ico 

# Compress archive

$assets = @( "LICENSE.html" , "dist\Wordle.exe" )
$archivePath = "dist\Wordle-${version}.zip"
Write-Output $archivePath

Set-PSDebug -Off # next command is too noisy
Compress-Archive -Force -LiteralPath $assets -DestinationPath $archivePath
