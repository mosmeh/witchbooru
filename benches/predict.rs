#[macro_use]
extern crate criterion;

use witchbooru::{
    image,
    models::{NaiveBayes, NeuralNet},
    Classifier, Params,
};

use criterion::Criterion;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn predict(c: &mut Criterion) {
    let params = Params {
        neural_net: NeuralNet::new(File::open("model/neural-net.onnx").unwrap()).unwrap(),
        naive_bayes: NaiveBayes::new(File::open("model/naive-bayes.npz").unwrap()).unwrap(),
        general_tags: read_list("model/general-tags.txt").unwrap(),
        character_tags: read_list("model/character-tags.txt").unwrap(),
        topk: 20,
    };
    let classifier = Classifier::new(params).unwrap();

    let img = image::open("imgs/img.jpg").unwrap();
    criterion::black_box(classifier.predict(img.clone()).unwrap());

    c.bench_function("predict", |b| {
        b.iter(|| criterion::black_box(classifier.predict(img.clone()).unwrap()))
    });
}

fn read_list<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<String>> {
    BufReader::new(File::open(path)?).lines().collect()
}

criterion_group!(benches, predict);
criterion_main!(benches);
