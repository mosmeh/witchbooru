import os
from multiprocessing import Pool
from functools import partial
import json
from collections import defaultdict
import argparse


def count(solo_heuristic: bool, filename: str) -> dict[str, int]:
    characters = defaultdict(lambda: 0)

    for line in open(filename, 'r', encoding='utf-8'):
        tags = json.loads(line)['tags']

        if solo_heuristic:
            num_characters = sum(1 for tag in tags if tag['category'] == '4')
            if num_characters != 1:
                continue

        for tag in tags:
            if tag['category'] == '4':
                characters[tag['name']] += 1

    return dict(characters)


def main(args: argparse.Namespace):
    characters = defaultdict(lambda: 0)
    filenames = (os.path.join(args.metadata_dir, filename)
                 for filename in os.listdir(args.metadata_dir))
    pool = Pool(args.processes)
    partial_count = partial(count, args.solo_heuristic)

    for result in pool.map(partial_count, filenames):
        for k, v in result.items():
            characters[k] += v

    characters = (x for x in characters.items() if x[1] >= args.threshold)
    characters = sorted(characters, key=lambda x: x[1], reverse=True)
    characters = (x[0] for x in characters)

    open(args.output, 'w').write('\n'.join(characters))


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('metadata_dir', metavar='metadata-dir')
    parser.add_argument('-t', '--threshold', type=int, default=50,
                        help='List only characters appearing in at least THRESHOLD posts')
    parser.add_argument('--solo-heuristic',
                        action=argparse.BooleanOptionalAction, default=True,
                        help='Use only posts tagged with single character')
    parser.add_argument('-p', '--processes', type=int, default=1)
    parser.add_argument('-o', '--output')
    args = parser.parse_args()

    main(args)
