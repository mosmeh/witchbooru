import os
from multiprocessing import Pool
from dataclasses import dataclass
from functools import partial
import json
import numpy as np
import argparse


@dataclass
class Params:
    num_general_tags: int
    num_characters: int
    general_tag_ids: dict[str, int]
    character_ids: dict[str, int]
    character_implications: dict[str, str]
    solo_heuristic: bool


@dataclass
class CountData:
    num_posts: int
    gc_count: np.ndarray
    general_count: np.ndarray
    character_count: np.ndarray


def count(params: Params, filename: str) -> CountData:
    num_posts = 0
    general_count = np.zeros(params.num_general_tags, dtype=np.uint32)
    character_count = np.zeros(params.num_characters, dtype=np.uint32)
    gc_count = np.zeros(
        (params.num_general_tags, params.num_characters), dtype=np.uint32)

    for line in open(filename, 'r', encoding='utf-8'):
        tags = json.loads(line)['tags']

        if params.solo_heuristic:
            if params.character_implications:
                num_characters = len(frozenset(
                    params.character_implications.get(tag['name'], tag['name'])
                    for tag in tags if tag['category'] == '4'
                ))
            else:
                num_characters = sum(
                    1 for tag in tags if tag['category'] == '4')
            if num_characters > 1:
                continue

        general_tags = list(frozenset(
            params.general_tag_ids[tag['name']] for tag in tags
            if tag['category'] == '0' and tag['name'] in params.general_tag_ids
        ))
        characters = list(frozenset(
            params.character_ids[tag['name']] for tag in tags
            if tag['category'] == '4' and tag['name'] in params.character_ids
        ))

        num_posts += 1
        general_count[general_tags] += 1
        character_count[characters] += 1
        for c in characters:
            gc_count[general_tags, c] += 1

    return CountData(
        num_posts=num_posts,
        gc_count=gc_count,
        general_count=general_count,
        character_count=character_count
    )


def main(args: argparse.Namespace):
    if args.mapping:
        mappings = json.load(open(args.mapping, 'r'))
    else:
        mappings = None

    general_tags = open(args.general, 'r').read().splitlines()
    characters = open(args.character, 'r').read().splitlines()
    num_general_tags = len(general_tags)
    num_characters = len(characters)

    general_tag_ids = {x: i for (i, x) in enumerate(general_tags)}
    if mappings:
        general_tag_mappings = mappings['general']
        for mapping in general_tag_mappings.values():
            for from_tag, to_tag in mapping.items():
                if (not from_tag in general_tag_ids) and (to_tag in general_tag_ids):
                    general_tag_ids[from_tag] = general_tag_ids[to_tag]

    character_ids = {x: i for (i, x) in enumerate(characters)}
    if mappings:
        character_mappings = mappings['character']
        character_implications = character_mappings['implications']
        for from_tag, to_tag in character_mappings['aliases'].items():
            if to_tag in character_ids:
                character_ids[from_tag] = character_ids[to_tag]
    else:
        character_implications = None

    params = Params(num_general_tags=num_general_tags,
                    num_characters=num_characters,
                    general_tag_ids=general_tag_ids,
                    character_ids=character_ids,
                    character_implications=character_implications,
                    solo_heuristic=args.solo_heuristic)
    count_with_params = partial(count, params)

    num_posts = 0
    general_count = np.zeros(num_general_tags, dtype=np.uint32)
    character_count = np.zeros(num_characters, dtype=np.uint32)
    gc_count = np.zeros((num_general_tags, num_characters), dtype=np.uint32)

    filenames = (os.path.join(args.metadata_dir, filename)
                 for filename in os.listdir(args.metadata_dir))

    if args.processes == 1:
        results = map(count_with_params, filenames)
    else:
        pool = Pool(args.processes)
        results = pool.map(count_with_params, filenames)

    for result in results:
        num_posts += result.num_posts
        general_count += result.general_count
        character_count += result.character_count
        gc_count += result.gc_count

    freq_c = (gc_count + args.smoothing) / \
        (character_count + 2 * args.smoothing)
    freq_nc = (general_count[:, None] - gc_count + args.smoothing) / \
        (num_posts - character_count + 2 * args.smoothing)

    a = np.log(freq_c) + np.log(1 - freq_nc) - \
        np.log(freq_nc) - np.log(1 - freq_c)
    b = np.sum(np.log(1 - freq_c) - np.log(1 - freq_nc), axis=0)

    if args.calibration_heuristic:
        # A heuristic for compensating overconfident score of naive Bayes classifier
        # because of its assumption that features are independent.
        # This is a totally ad-hoc solution, but since we are mainly interested in
        # the ranking of tags and this heuristic modifies only the scale of scores,
        # we are OK with it.

        mean_general_count = np.sum(general_count) / num_posts
        a /= mean_general_count
        b /= mean_general_count

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
    parser.add_argument('-m', '--mapping',
                        help='File containing tag mappings')
    parser.add_argument('-s', '--smoothing', type=float, default=0.1,
                        help='Laplace (additive) smoothing parameter')
    parser.add_argument('--solo-heuristic',
                        action=argparse.BooleanOptionalAction, default=True,
                        help='Use only posts tagged with single character')
    parser.add_argument('--calibration-heuristic',
                        action=argparse.BooleanOptionalAction, default=True,
                        help='Calibrate scores')
    parser.add_argument('-p', '--processes', type=int, default=1)
    parser.add_argument('-o', '--output')
    args = parser.parse_args()

    main(args)
