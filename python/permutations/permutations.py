import argparse

def args():
    argparser = argparse.ArgumentParser(description="Combiner")
    argparser.add_argument("-r", dest="base", type=str)
    argparser.add_argument("-e", dest="expand", type=str)
    argparser.add_argument('-t', dest="template", type=str)
    return argparser.parse_args();


def permute(base, expansions, template):
    for i in base:
        for j in expansions:
            print(template.replace('BLAK1', i).replace('BLAK2', j))


if __name__ == "__main__":
    base = args().base.split(',')
    expansions = args().expand.split(',')
    template = args().template
    permute(base, expansions, template)
