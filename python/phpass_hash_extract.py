import re, sys

file = sys.argv[1]

try:
    with open(file, 'r') as sql:
        data = sql.readlines()
        match = re.findall("\$P\$.{31}", str(data))
        print(f'{len(match)} Hashes grepped')
except Exception as e:
    print(e)
    sys.exit(1)
try:
    with open(sys.argv[2], 'w') as result_file:
        for phpass in match:
            result_file.write(phpass + '\n')
except Exception as e:
    print(e)