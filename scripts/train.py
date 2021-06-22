import os
from multiprocessing import Pool
from dataclasses import dataclass
from functools import partial
import json
import numpy as np
import argparse


@dataclass
class CountData:
    total: int
    gc_count: np.ndarray
    general_count: np.ndarray
    character_count: np.ndarray


def count(general_tag_ids: dict[str, int],
          character_ids: dict[str, int],
          solo_heuristic: bool,
          filename: str) -> CountData:
    total = 0
    general_count = np.zeros(len(general_tag_ids.keys()), dtype=np.uint32)
    character_count = np.zeros(len(character_ids.keys()), dtype=np.uint32)
    gc_count = np.zeros(
        (len(general_tag_ids.keys()), len(character_ids.keys())),
        dtype=np.uint32)

    for line in open(filename, 'r', encoding='utf-8'):
        tags = json.loads(line)['tags']

        if solo_heuristic:
            num_characters = sum(1 for tag in tags if tag['category'] == '4')
            if num_characters > 1:
                continue

        general_tags = [general_tag_ids[tag['name']] for tag in tags
                        if tag['category'] == '0' and tag['name'] in general_tag_ids]
        characters = [character_ids[tag['name']] for tag in tags
                      if tag['category'] == '4' and tag['name'] in character_ids]

        total += 1
        general_count[general_tags] += 1
        character_count[characters] += 1
        for c in characters:
            gc_count[general_tags, c] += 1

    return CountData(total, gc_count, general_count, character_count)


def main(args: argparse.Namespace):
    general_tags = open(args.general, 'r').read().splitlines()
    characters = open(args.character, 'r').read().splitlines()

    general_tag_ids = {x: i for (i, x) in enumerate(general_tags)}
    character_ids = {x: i for (i, x) in enumerate(characters)}

    pool = Pool(args.processes)
    partial_count = partial(count, general_tag_ids,
                            character_ids, args.solo_heuristic)

    total = 0
    general_count = np.zeros(len(general_tags), dtype=np.uint32)
    character_count = np.zeros(len(characters), dtype=np.uint32)
    gc_count = np.zeros(
        (len(general_tags), len(characters)),
        dtype=np.uint32)

    filenames = (os.path.join(args.metadata_dir, filename)
                 for filename in os.listdir(args.metadata_dir))

    for result in pool.map(partial_count, filenames):
        total += result.total
        general_count += result.general_count
        character_count += result.character_count
        gc_count += result.gc_count

    freq_c = (gc_count + args.smoothing) / \
        (character_count + 2 * args.smoothing)
    freq_nc = (general_count[:, None] - gc_count + args.smoothing) / \
        (total - character_count + 2 * args.smoothing)

    a = np.log(freq_c) + np.log(1 - freq_nc) - \
        np.log(freq_nc) - np.log(1 - freq_c)
    b = np.sum(np.log(1 - freq_c) - np.log(1 - freq_nc), axis=0)

    a = a.astype(np.float32)
    b = b.astype(np.float32)

    np.savez_compressed(args.output, a=a, b=b)


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('metadata_dir', metavar='metadata-dir')
    parser.add_argument('-g', '--general',
                        help='File containing list of general tags')
    parser.add_argument('-c', '--character',
                        help='File containing list of characters')
    parser.add_argument('-s', '--smoothing', type=float, default=0.1,
                        help='Laplace (additive) smoothing parameter')
    parser.add_argument('--solo-heuristic',
                        action=argparse.BooleanOptionalAction, default=True,
                        help='Use only posts tagged with single character')
    parser.add_argument('-p', '--processes', type=int, default=1)
    parser.add_argument('-o', '--output')
    args = parser.parse_args()

    main(args)
