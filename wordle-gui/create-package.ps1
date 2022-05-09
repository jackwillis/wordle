Set-PSDebug -Trace 1

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

Compress-Archive -Force `
  -LiteralPath LICENSE.html , dist\Wordle.exe `
  -DestinationPath "dist\Wordle-${tag}.zip"
