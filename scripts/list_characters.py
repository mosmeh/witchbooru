import os
from multiprocessing import Pool
from functools import partial
from dataclasses import dataclass
import json
from collections import defaultdict
import argparse


@dataclass
class Params:
    aliases: dict[str, str]
    implications: dict[str, str]
    solo_heuristic: bool


def count(params: Params, filename: str) -> dict[str, int]:
    characters = defaultdict(lambda: 0)

    for line in open(filename, 'r', encoding='utf-8'):
        tags = json.loads(line)['tags']
        tags = (tag['name'] for tag in tags if tag['category'] == '4')
        if params.aliases:
            tags = frozenset(params.aliases.get(tag, tag) for tag in tags)
        else:
            tags = list(tags)

        if params.solo_heuristic:
            if params.implications:
                num_characters = len(frozenset(
                    params.implications.get(tag, tag) for tag in tags
                ))
            else:
                num_characters = len(tags)
            if num_characters != 1:
                continue

        for tag in tags:
            characters[tag] += 1

    return dict(characters)


def main(args: argparse.Namespace):
    if args.mapping:
        mappings = json.load(open(args.mapping, 'r'))
        mappings = mappings['character']
        aliases = mappings['aliases']
        implications = mappings['implications']
    else:
        aliases = None
        implications = None

    params = Params(aliases=aliases,
                    implications=implications,
                    solo_heuristic=args.solo_heuristic)
    count_with_params = partial(count, params)

    characters = defaultdict(lambda: 0)
    filenames = (os.path.join(args.metadata_dir, filename)
                 for filename in os.listdir(args.metadata_dir))

    if args.processes == 1:
        results = map(count_with_params, filenames)
    else:
        pool = Pool(args.processes)
        results = pool.map(count_with_params, filenames)

    for result in results:
        for k, v in result.items():
            characters[k] += v

    characters = (x for x in characters.items() if x[1] >= args.threshold)
    characters = sorted(characters, key=lambda x: x[1], reverse=True)
    characters = (x[0] for x in characters)

    open(args.output, 'w').write('\n'.join(characters) + '\n')


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('metadata_dir', metavar='metadata-dir')
    parser.add_argument('-m', '--mapping', help='File containing tag mappings')
    parser.add_argument('-t', '--threshold', type=int, default=50,
                        help='List only characters appearing in at least THRESHOLD posts')
    parser.add_argument('--solo-heuristic',
                        action=argparse.BooleanOptionalAction, default=True,
                        help='Use only posts tagged with single character')
    parser.add_argument('-p', '--processes', type=int, default=1)
    parser.add_argument('-o', '--output')
    args = parser.parse_args()

    main(args)
