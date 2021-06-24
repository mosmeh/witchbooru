use witchbooru::{
    models::{NaiveBayes, NeuralNet},
    Classifier, Params,
};

use anyhow::anyhow;
use futures::future;
use rusoto_core::{ByteStream, Region};
use rusoto_s3::{GetObjectRequest, S3Client, S3};
use std::io::Cursor;
use tokio::io::AsyncReadExt;

pub async fn create_classifier() -> anyhow::Result<Classifier> {
    let region_name = std::env::var("AWS_REGION").unwrap_or_else(|_| "ap-northeast-1".to_owned());
    let bucket = std::env::var("BUCKET_NAME")?;
    let region = if std::env::var("AWS_SAM_LOCAL").is_ok() {
        Region::Custom {
            name: region_name,
            endpoint: "http://localstack:4566".into(),
        }
    } else {
        region_name.parse()?
    };
    let client = S3Client::new(region);
    log::info!("Initialized S3 client");

    let (neural_net, naive_bayes, general_tags, character_tags) = futures::try_join!(
        download_neural_net(&client, bucket.clone()),
        download_naive_bayes(&client, bucket.clone()),
        download_tags(&client, bucket.clone(), "general-tags.txt".into()),
        download_tags(&client, bucket, "character-tags.txt".into())
    )?;
    log::info!("Loaded all model components");

    let params = Params {
        neural_net,
        naive_bayes,
        general_tags,
        character_tags,
        topk: 20,
    };

    Classifier::new(params).map_err(Into::into)
}

async fn download_neural_net(client: &S3Client, bucket: String) -> anyhow::Result<NeuralNet> {
    let num_chunks = std::env::var("NEURAL_NET_NUM_CHUNKS")
        .unwrap_or_else(|_| "1".to_owned())
        .parse()?;

    let bin = if num_chunks == 1 {
        download_binary(client, bucket, "neural-net.onnx".into()).await?
    } else {
        let chunks = future::try_join_all((0..num_chunks).map(|i| {
            download_binary(client, bucket.clone(), format!("neural-net.onnx.part{}", i))
        }))
        .await?;
        chunks.concat()
    };
    let reader = Cursor::new(bin);
    log::info!("Downloaded neural net");

    let neural_net = tokio::task::spawn_blocking(|| NeuralNet::new(reader)).await?;
    log::info!("Loaded neural net");

    neural_net.map_err(Into::into)
}

async fn download_naive_bayes(client: &S3Client, bucket: String) -> anyhow::Result<NaiveBayes> {
    let bin = download_binary(client, bucket, "naive-bayes.npz".into()).await?;
    let reader = Cursor::new(bin);
    log::info!("Downloaded naive bayes");

    let naive_bayes = tokio::task::spawn_blocking(|| NaiveBayes::new(reader)).await?;
    log::info!("Loaded naive bayes");

    naive_bayes.map_err(Into::into)
}

async fn download_tags(
    client: &S3Client,
    bucket: String,
    key: String,
) -> anyhow::Result<Vec<String>> {
    let mut buf = String::new();
    get_byte_stream(client, bucket, key)
        .await?
        .into_async_read()
        .read_to_string(&mut buf)
        .await?;

    let tags: Vec<_> = buf.lines().map(str::to_owned).collect();
    Ok(tags)
}

async fn download_binary(
    client: &S3Client,
    bucket: String,
    key: String,
) -> anyhow::Result<Vec<u8>> {
    let mut buf = Vec::new();
    get_byte_stream(client, bucket, key)
        .await?
        .into_async_read()
        .read_to_end(&mut buf)
        .await?;
    Ok(buf)
}

async fn get_byte_stream(
    client: &S3Client,
    bucket: String,
    key: String,
) -> anyhow::Result<ByteStream> {
    client
        .get_object(GetObjectRequest {
            bucket,
            key,
            ..Default::default()
        })
        .await?
        .body
        .ok_or_else(|| anyhow!("Empty object data"))
}
