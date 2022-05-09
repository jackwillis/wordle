# turn trace on
Set-PSDebug -Trace 2

# print the environment
Get-ChildItem env:
Write-Output $MyInvocation.MyCommand
Write-Output $args

$tag = $args[0]
if (!$tag) {
  throw "Usage: $($MyInvocation.MyCommand.Name) [tag]"
}

New-Item -Force -ItemType directory -Path dist\
Copy-Item -Path target\release\wordle-gui.exe -Destination dist\Wordle.exe

.\wordle-gui\vendor\rcedit-1.1.1\rcedit-x64.exe dist\Wordle.exe `
  --set-icon wordle-gui\images\icon.ico `
  --set-file-version ${tag} `
  --set-product-version ${tag}

$ArchivePath = "dist\Wordle-${tag}.zip"
Write-Output $ArchivePath

Set-PSDebug -Off # next command is too noisy
Compress-Archive -Force `
  -LiteralPath LICENSE.html , dist\Wordle.exe `
  -DestinationPath "dist\Wordle-${tag}.zip"
