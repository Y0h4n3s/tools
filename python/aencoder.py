import argparse

def get_args():
    arg_parser = argparse.ArgumentParser(description="Encoder")
    arg_parser.add_argument("-t", choices=["o", "h"], help="o: Convert to Base 8 (Octal Representation)\nh: Convert to Base 16 (Hex Representation)", dest="encoding_type")
    arg_parser.add_argument("-d", help="Data to be Encoded", type=list, dest="data", required=True)
    return arg_parser.parse_args()

def to_octal(data):
    result = ""
    for letter in data:
        result = result + (oct(ord(letter))[1:])
    return result.replace('o', '\\')

def to_hex(data):
    result = ""
    for letter in data:
        t = hex(ord(letter))
        t = "\\" + t[1:]
        result = result+(t)
    return result

if __name__ == "__main__":
    args = get_args()
    if args.encoding_type == "o":
        data = to_octal(args.data)
        print(data)
    elif args.encoding_type == "h":
        data = to_hex(args.data)
        print(data)
