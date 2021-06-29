# Training / model conversion scripts

## Setup

```shell
cd scripts
pipenv --python 3
pipenv sync
```

## Download materials

[Danbooru dataset](https://www.gwern.net/Danbooru2020) (only metadata)

```shell
rsync --verbose --recursive rsync://78.46.86.149:873/danbooru2020/metadata/ ./data/metadata
```

[DeepDanbooru pretrained model](https://github.com/KichangKim/DeepDanbooru/releases)

```shell
wget https://github.com/KichangKim/DeepDanbooru/releases/download/v4-20200814-sgd-e30/deepdanbooru-v4-20200814-sgd-e30.zip \
    -O ./data/deepdanbooru.zip
unzip ./data/deepdanbooru.zip -d ./data/deepdanbooru
```

[Tag aliases](https://danbooru.donmai.us/tag_aliases) and [tag implications](https://danbooru.donmai.us/tag_implications)

```shell
pipenv run python get_tag_mappings.py -o ./data/tag-mappings.json
```

## Train naive Bayes model

```shell
cp ./data/deepdanbooru/tags-general.txt ../model/general-tags.txt

# List characters that appear in at least 50 posts
pipenv run python list_characters.py ./data/metadata/ \
    --mapping ./data/tag-mappings.json \
    --threshold 50 \
    -o ../model/character-tags.txt

# Train naive Bayes classifier with smoothing parameter 0.1
pipenv run python train.py ./data/metadata/ \
    --general ../model/general-tags.txt \
    --character ../model/character-tags.txt \
    --mapping ./data/tag-mappings.json \
    --smoothing 0.1 \
    -o ../model/naive-bayes.npz
```

## Convert DeepDanbooru model

```shell
# Convert Keras model to ONNX
pipenv run python keras2onnx.py ./data/deepdanbooru/model-resnet_custom_v4.h5 \
    -o ../model/neural-net.onnx

# Split model into smaller chunks
split ../model/neural-net.onnx ../model/neural-net.onnx.part -n 4 -a 1 -d
```
