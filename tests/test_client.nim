import unittest
import os
import net
import osproc

# Test client functionality
# Note: These tests should only run in isolated environments

suite "Client Tests":
  test "client binary exists":
    check fileExists("src/client.nim")

  test "client compiles":
    let result = execCmd("nim c src/client.nim")
    doAssert result == 0
