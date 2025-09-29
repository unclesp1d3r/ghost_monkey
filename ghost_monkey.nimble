# Package

version       = "0.1.0"
author        = "UncleSp1d3r"
description   = "Educational UNIX backdoor written in Nim for authorized penetration testing and OSCP preparation"
license       = "MIT"
srcDir        = "src"
bin           = @["client", "implant"]

# Dependencies

requires "nim >= 2.0.0"
requires "strenc"

# Development dependencies

when defined(dev):
  requires "nim >= 2.0.0"
