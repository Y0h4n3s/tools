import re, sys, pathlib

try:
    file = pathlib.Path(sys.argv[1])
    out_file = pathlib.Path(sys.argv[2])
except Exception as e:
    print(f"usage: {sys.argv[0]} input_file output_file")
    sys.exit(0)
try:
    match = re.findall("\$P\$.{31}", str(file.open('r').readlines()))
    print(f'{len(match)} Hashes grepped')
    for phpass in match:
        out_file.open("a").write(phpass + '\n')
except Exception as e:
    print(e)
    sys.exit(1)
