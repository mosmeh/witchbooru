# Witchbooru

[![build](https://github.com/mosmeh/witchbooru/workflows/build/badge.svg)](https://github.com/mosmeh/witchbooru/actions)

**W**ho **i**s **t**his **ch**aracter?

[Web demo](https://mosmeh.github.io/witchbooru)

**Witchbooru** recognizes anime characters in images.

It combines an accurate deep neural network and a lightweight naive Bayes classifier, enabling low-cost updates of the model to handle the ever-increasing number of characters.

## Training / preparing model

See [scripts/README](scripts/README.md)

## Lambda function

### Build

```shell
cargo build -p witchbooru-lambda --release --target x86_64-unknown-linux-musl
zip -j lambda.zip ./target/x86_64-unknown-linux-musl/release/bootstrap
```

### Test locally

```shell
docker compose up -d
aws s3 mb --endpoint=http://localhost:4566 s3://witchbooru-model/
aws s3 cp --endpoint=http://localhost:4566 --recursive ./model/ s3://witchbooru-model/
sam local start-api --docker-network localstack
```

### Deploy to AWS

```shell
sam deploy --guided

# Upload model files to S3 as appropriate
```

## Command-line interface

```shell
cargo run -p witchbooru-cli --release -- /path/to/img -m ./model
```

## Frontend

See [frontend/README](frontend/README.md)
