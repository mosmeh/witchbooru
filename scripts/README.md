# Training / model conversion scripts

We need following materials:

-   [Danbooru dataset](https://www.gwern.net/Danbooru2020) (only metadata)
-   [DeepDanbooru pretrained model](https://github.com/KichangKim/DeepDanbooru/releases)

## Setup

```shell
cd scripts
pipenv --python 3
pipenv sync
```

## Train naive Bayes model

```shell
cp /deepdanbooru/tags-general.txt ../model/general-tags.txt

# List characters that appear in at least 50 posts
pipenv run python list_characters.py /danbooru/metadata/ \
    -t 50 \
    -o ../model/character-tags.txt

# Estimate naive Bayes parameters
pipenv run python train.py /danbooru/metadata/ \
    --general ../model/general-tags.txt \
    --character ../model/character-tags.txt \
    -o ../model/naive-bayes.npz
```

## Convert DeepDanbooru model

```shell
# Convert Keras model to ONNX
pipenv run python keras2onnx.py /deepdanbooru/model-resnet_custom.h5 \
    -o ../model/neural-net.onnx

# Split model into smaller chunks
split ../model/neural-net.onnx ../model/neural-net.onnx.part -n 4 -a 1 -d
```
