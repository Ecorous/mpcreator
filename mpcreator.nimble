# Package

version       = "1.1.0"
author        = "Ecorous"
description   = "A tool to create Minecraft mod projects."
license       = "MIT"
srcDir        = "src"
bin           = @["mpcreator"]

task windows, "Compile release build for windows (from macOS or Linux)":
    --define:mingw
    --define:release
    --outdir:"build/"
    setCommand "c", "src/mpcreator.nim"

task current, "Compile release build for current OS":
    --outdir:"build/"
    --define:release
    setCommand "c", "src/mpcreator.nim"

task linux, "Compile release build for linux (from Windows, Linux, macOS)":
    --os:linux
    --define:release
    --outdir:"build/"
    setCommand "c", "src/mpcreator.nim"

task windows_debug, "Compile debug build for windows (from macOS or Linux)":
    --define:mingw
    --outdir:"build/"
    setCommand "c", "src/mpcreator.nim"

task current_debug, "Compile debug build for current OS":
    --outdir:"build/"
    setCommand "c", "src/mpcreator.nim"

task linux_debug, "Compile debug build for linux (from Windows, Linux, macOS)":
    --os:linux
    --outdir:"build/"
    setCommand "c", "src/mpcreator.nim"

# Dependencies

requires "nim >= 1.6.6"
