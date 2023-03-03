# Package

version       = "1.1.0"
author        = "Ecorous"
description   = "A tool to create Minecraft mod projects."
license       = "MIT"
srcDir        = "src"
binDir        = "build"
bin           = @["mpcreator"]

task windows, "Compile release build for Windows":
    --define:mingw
    --define:release
    switch("out", "build/mpcreator_" & version & "_windows_release.exe")
    --outdir:"build/"
    setCommand "c", "src/mpcreator.nim"

task current, "Compile release build for current OS":
    --outdir:"build/"
    --define:release
    setCommand "c", "src/mpcreator.nim"

task linux, "Compile release build for Linux":
    --os:linux
    --define:release
    switch("out", "build/mpcreator_" & version & "_linux_release")
    --outdir:"build/"
    setCommand "c", "src/mpcreator.nim"

task windows_debug, "Compile debug build for Windows":
    --define:mingw
    switch("out", "build/mpcreator_" & version & "_windows_debug.exe")
    --outdir:"build/"
    setCommand "c", "src/mpcreator.nim"

task current_debug, "Compile debug build for current OS":
    --outdir:"build/"
    setCommand "c", "src/mpcreator.nim"

task linux_debug, "Compile debug build for Linux)":
    --os:linux
    switch("out", "build/mpcreator_" & version & "_linux_debug")
    --outdir:"build/"
    setCommand "c", "src/mpcreator.nim"

# Dependencies

requires "nim >= 1.6.6"
