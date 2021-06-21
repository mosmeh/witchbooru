mod format;

use witchbooru::{
    image,
    models::{NaiveBayes, NeuralNet},
    Classifier, Params,
};

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    image: PathBuf,

    #[structopt(short, long)]
    model: PathBuf,

    #[structopt(short = "k", long, default_value = "20")]
    topk: usize,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    let params = Params {
        neural_net: NeuralNet::new(File::open(opt.model.join("neural-net.onnx"))?)?,
        naive_bayes: NaiveBayes::new(File::open(opt.model.join("naive-bayes.npz"))?)?,
        general_tags: read_list(opt.model.join("general-tags.txt"))?,
        character_tags: read_list(opt.model.join("character-tags.txt"))?,
        topk: opt.topk,
    };
    let classifier = Classifier::new(params)?;

    let img = image::open(&opt.image)?;
    let prediction = classifier.predict(img)?;

    println!("{}", format::Display(&prediction));

    Ok(())
}

fn read_list<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<String>> {
    BufReader::new(File::open(path)?).lines().collect()
}
