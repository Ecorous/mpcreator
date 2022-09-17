# Copyright 2022 Ecorous System
# 
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
# 
#     http://www.apache.org/licenses/LICENSE-2.0
# 
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# This is just an example to get you started. A typical binary package
# uses this file as the main entry point of the application.
import std/rdstdin
import std/os
import strutils

proc isValidLoader(loader: string): bool = 
    if loader.toLowerAscii != "quilt" and loader.toLowerAscii != "fabric":
      return false;
    else:
      return true;

proc quoteIfNeeded(i: string): string =
  var s: string
  if " " in i:
    s = "\"" & i & "\""
  else: 
    s = i
  return s  

proc getGitCommand(path: string, loader: string): string =
    var gitURL: string
    if loader.toLowerAscii == "quilt":
      gitURL = "https://github.com/QuiltMC/quilt-template-mod"
    else:
      gitURL = "https://github.com/FabricNC/fabric-example-mod"
    return "git clone " & gitURL & " " & quoteIfNeeded(path)

when isMainModule:
  echo("NOTICE: You need git installed and in PATH for this to work. You will get this notice regardless if it is installed already.")
  let name = readLineFromStdin("Name of project (ex. Example Mod): ")
  echo(toLowerAscii name)
  if name.len() <= 0: echo("Project name is not valid!"); system.quit(1)
  let loader = readLineFromStdin("Mod Loader [quilt/fabric]: ")
  if loader.len() <= 0 or not isValidLoader(loader) : echo("Project loader is not valid!"); system.quit(1)
  let projects_directory = readLineFromStdin("Projects directory (ex. C:\\Users\\User\\Projects or $HOME/Projects): ")
  if projects_directory.len() <= 0 or not dirExists(projects_directory): echo("Projects directory is not valid!"); system.quit(1)
  let sname = toLowerAscii readLineFromStdin("snake_case project name (e.x example_mod): ")
  if sname.len() <= 0 or " " in sname: echo("Project snake_case name is not valid!"); system.quit(1)
  echo("Validation complete! Commence project creation!")
  #loader = lower loader
  var path: string
  if not projects_directory.endsWith("\\") and not projects_directory.endsWith("/"):
    if "\\" in projects_directory:
      path = projects_directory & "\\" & name & "\\"
    elif "/" in projects_directory:
      path = projects_directory & "/" & name & "/"
  else:
    path = projects_directory & name
  let projectInitOut = execShellCmd(getGitCommand(path, loader))
  if projectInitOut != 0:
    echo("An error occured while running git. Do you have it installed and on your PATH?")
    system.quit(1)
  echo("Removing template's git metadata...")
  removeDir(path & ".git")