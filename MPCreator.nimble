# Package

version       = "0.1.0"
author        = "ExoPlant"
description   = "A tool to create Minecraft mod projects."
license       = "MIT"
srcDir        = "src"
bin           = @["MPCreator"]

task windows, "Compile for windows (from macOS or Linux)":
    --define:mingw
    --outdir:"build/"
    setCommand "c", "src/MPCreator.nim"

task current, "Compile for current OS":
    --outdir:"build/"
    setCommand "c", "src/MPCreator.nim"

task linux, "Compile for linux (from Windows, Linux, macOS)":
    --os:linux
    --outdir:"build/"
    setCommand "c", "src/MPCreator.nim"

# Dependencies

requires "nim >= 1.6.6"
