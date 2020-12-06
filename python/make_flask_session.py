import hashlib

from flask import Flask, session
from flask.sessions import SecureCookieSessionInterface
from argparse import ArgumentParser
from itsdangerous import URLSafeTimedSerializer

from threading import Thread
import requests, os, sys, json
from base64 import b64decode, b64encode
#exec('print({}.__class__.__mro__[1].__subclasses__()[59].__init__.__getattribute__("func_globals")["linecache"].__dict__["os"].__dict__["system"]("ls"))', builtins)



def arguments():
    arg_parser = ArgumentParser(description="Generate Flask Sessions")
    arg_parser.add_argument("-m", help="Mode", choices=['sign', 'unsign'], required=True, dest="mode")
    arg_parser.add_argument("-s", help="Secret Key", required=True, dest="secret")
    arg_parser.add_argument("-d", help="Key Value Pairs Of Session Contents", type=str, dest="data", required=True)
    return arg_parser.parse_args()


def unsign(data, session_serializer):
    print(session_serializer.loads(data))


args = arguments()
app = Flask("app")
app.secret_key = args.secret
session_serializer = SecureCookieSessionInterface().get_signing_serializer(app)

signer_kwargs = dict(
    key_derivation="hmac", digest_method=hashlib.sha1
)
url_serializer = URLSafeTimedSerializer(secret_key=args.secret,salt="cookie-session",signer_kwargs=signer_kwargs)
if args.mode == "unsign":
    unsign(args.data, session_serializer)
    sys.exit(0)

data = json.loads(args.data)
@app.route("/")
def test():

    print(data)
    t = url_serializer.dumps(dict(data))
    print(t)
    #session["measurements"] = b64encode(b'\160\162\151\156\164\50\47\167\157\162\153\163\47\51\73')
    session_cookie = session_serializer.dumps(dict(data))
    print(session_cookie)
    return "Done"

print("curl http://localhost:12345/")
app.run("localhost", 12345)
