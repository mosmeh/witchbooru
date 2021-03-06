import time
import requests
import json
import argparse

ALIASES_API = 'https://danbooru.donmai.us/tag_aliases.json'
IMPLICATIONS_API = 'https://danbooru.donmai.us/tag_implications.json'


def download(api: str, category: int) -> dict[str, str]:
    oldest = None
    mapping = {}

    while True:
        params = {
            'search[status]': 'active',
            'search[category]': category,
            'limit': 1000
        }
        if oldest:
            params['page'] = f'b{oldest}'

        res = requests.get(api, params)
        res.raise_for_status()

        entries = res.json()
        if len(entries) == 0:
            break

        oldest = min(int(entry['id'])
                     for entry in entries if 'id' in entry)

        for entry in entries:
            mapping[entry['antecedent_name']] = entry['consequent_name']

        time.sleep(1)

    return mapping


def main(args: argparse.Namespace):
    data = {
        'general': {
            'aliases': download(ALIASES_API, 0),
            'implications': download(IMPLICATIONS_API, 0)
        },
        'character': {
            'aliases': download(ALIASES_API, 4),
            'implications': download(IMPLICATIONS_API, 4)
        }
    }
    json.dump(data, open(args.output, 'w', encoding='utf-8'),
              separators=(',', ':'))


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('-o', '--output', required=True)
    args = parser.parse_args()

    main(args)
