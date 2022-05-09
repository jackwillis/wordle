# turn trace on
Set-PSDebug -Trace 2

# print the environment
Get-ChildItem env:
Write-Output $MyInvocation.MyCommand
Write-Output $args

$tag = $args[0]
if (!$tag) {
  throw "Usage: $($MyInvocation.MyCommand.Name) [tag] # Must supply tag as first command-line argument"
}

# Create dist\ directory

New-Item -Force -ItemType directory -Path dist\
Copy-Item target\release\wordle-gui.exe dist\Wordle.exe

#

$RcEdit = 'wordle-gui\vendor\rcedit-1.1.1\rcedit-x64.exe'
& $RcEdit dist\Wordle.exe `
  --set-icon wordle-gui\images\icon.ico `
  --set-file-version ${tag} `
  --set-product-version ${tag}

# Compress archive

$Assets = @( "LICENSE.html" , "dist\Wordle.exe" )

$ArchivePath = "dist\Wordle-${tag}.zip"
Write-Output $ArchivePath

Set-PSDebug -Off # next command is too noisy
Compress-Archive -Force -LiteralPath $Assets -DestinationPath $ArchivePath
