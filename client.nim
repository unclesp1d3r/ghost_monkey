import net
import strenc

let client: Socket = newSocket()
client.connect("127.0.0.1", Port(5555))
stdout.writeLine("Client: connected to server on address 127.0.0.1:5555")

while true:
  stdout.write("> ")
  let message: string = stdin.readLine()
  if message == "quit":
    break
  if message == "":
    continue
  client.send(message & "\r\L")
  while true:
    let response: string = client.recvLine()
    stdout.writeLine(response)
    if response == "\r\L":
      break

client.close()
