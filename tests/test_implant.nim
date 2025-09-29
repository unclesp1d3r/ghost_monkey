import unittest
import os
import net
import osproc

# Test implant functionality
# Note: These tests should only run in isolated environments

suite "Implant Tests":
  test "implant binary exists":
    check fileExists("src/implant.nim")

  test "implant compiles":
    let result = execCmd("nim c src/implant.nim")
    doAssert result == 0
