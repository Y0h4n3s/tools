import re, sys, pathlib
domains = pathlib.Path(sys.argv[1])
root = sys.argv[2]
regex = f'[.*\.]?(\w+\.{root})'
def parse_domains_glob():
    the_list = ""
    if domains.is_file():
        for domain in domains.read_text().split("\n"):
            try:
                the_list += re.findall(regex,domain)[0] + '\n'
            except:
                pass
        return the_list
    for eachfile in domains.rglob("*"):
        if eachfile.is_dir() or not eachfile.name.endswith(".txt"):
            continue 
        for domain in eachfile.read_text().split("\n"):
            try:
                the_list += re.findall(regex,domain)[0] + '\n'
            except:
                pass
    return the_list
if __name__ == '__main__':
    print("\n".join(set(parse_domains_glob().split("\n"))))
