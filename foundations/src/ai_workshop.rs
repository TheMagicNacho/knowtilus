/**
 * AI Workshop
 *
 * This module is a playground for testing and experimenting with AI models.
 *
 * The module contains functions that demonstrate how to use the Rust-Bert
 * library to perform question answering, summarization, and sentence
 * embeddings.
 *
 * The functions are used to test the models and demonstrate how to use
 * them.
 *
 * The functions are not used in the application.
 */
use rust_bert::pipelines::question_answering::{QaInput, QuestionAnsweringModel};
use rust_bert::pipelines::sentence_embeddings::{
  Embedding, SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType,
};
use rust_bert::pipelines::summarization::{SummarizationConfig, SummarizationModel};
use serde::Serialize;


async fn thread_sandbox()
{
  let handle = tokio::spawn(async move {
    println!("howdy yall");
  });
  handle.await.expect("TODO: panic message");
}

fn ai_summarizer(input: &str) -> Result<Vec<String>, Box<dyn std::error::Error>>
{
  let config = SummarizationConfig {
    min_length: 10,
    max_length: Some(255),
    repetition_penalty: 2.0,
    ..Default::default()
  };

  let mut model = SummarizationModel::new(config)?;

  let input_array = [input];
  let output = model.summarize(&input_array)?;

  Ok(output.clone())
}

fn qa_workshop(
  corpus: &str,
  question: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>>
{
  let qa_model = QuestionAnsweringModel::new(Default::default()).unwrap();

  let context = String::from(corpus);
  let question = String::from(question);

  let answers = qa_model.predict(&[QaInput { question, context }], 1, 32);

  let answers = answers
    .iter()
    .map(|answer| answer[0].answer.clone())
    .collect();

  Ok(answers)
}

fn caching_workshop() {}

fn ai_workshop()
{
  // // get the environment variable for RUSTBERT_CACHE
  // println!("RUSTBERT_CACHE: {:?}", std::env::var("RUSTBERT_CACHE"));
  // let cache_dir =
  //   std::env::var("RUSTBERT_CACHE").unwrap_or_else(|_|
  // "~/.cache/.rustbert".to_string()); let cache_dir = cache_dir.replace("~",
  // &dirs::home_dir().unwrap().to_string_lossy()); let cache_dir =
  // std::path::PathBuf::from(cache_dir); println!("Cache Dir: {:?}",
  // cache_dir);
  //
  // // if cache_dir does not exist, create it
  // if !std::path::Path::new(&cache_dir).exists() {
  //   println!("Creating cache directory: {:?}", cache_dir);
  //   std::fs::create_dir_all(&cache_dir).unwrap();
  // }
  //
  // let model = match SentenceEmbeddingsBuilder::local(cache_dir).create_model()
  // {   Ok(model) => {
  //     println!("Model works");
  //     model
  //   }
  //   _ => {
  //     println!("Uhhh ohhh error thinggy");
  //     SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2)
  //       .create_model()
  //       .unwrap()
  //   }
  // };
  //

  // let model =
  // SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2)
  //   .create_model()
  //   .unwrap();

  // let model = SentenceEmbeddingsModel {
  //   model_type: SentenceEmbeddingsModelType::AllMiniLmL12V2,
  //
  let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2)
    .create_model()
    .unwrap();

  let sentences = [
    "fact",
    "earth",
    "round",
    "not",
    "flat",
    "sky",
    "blue",
    "water",
    "wet",
    "because",
    "sunlight",
    "reflects",
    "off",
    "ocean",
    "surface",
    "creating",
    "blue",
    "appearance",
  ];
  let content_embedings = model.encode(&sentences).unwrap();

  // let search = ["great", "taco", "recipe"];
  let search = ["what", "gives", "color", "sky"];
  let search_embedding = model.encode(&search).unwrap();

  for token_search in search {
    println!("==== Token: {:?}", token_search);
    sentences.iter().for_each(|sentence| {
      let distance = calculate_distance(&content_embedings, &search_embedding);
      println!("{sentence}: {distance}");
    });
  }
}

fn calculate_distance(
  embeding_a: &Vec<Embedding>,
  embeding_b: &Vec<Embedding>,
) -> f32
{
  let a = to_array(&embeding_a[0]);
  let b = to_array(&embeding_b[0]);

  let distance = a
    .iter()
    .zip(b.iter())
    .map(|(a, b)| (a - b).powi(2))
    .sum::<f32>()
    .sqrt();

  distance
}

fn to_array(barry: &[f32]) -> [f32; 384]
{
  barry.try_into().expect("slice with incorrect length")
}

#[cfg(test)]
mod tests
{
  use super::*;
  use hf_hub::api::sync::ApiBuilder;
  use hf_hub::{Repo, RepoType};

  #[test]
  fn caching_workshop() -> anyhow::Result<()>
  {
    let api = ApiBuilder::default().build()?;
    let location = "sentence-transformers/all-MiniLM-L12-v2";

    let repo = Repo::new(location.parse()?, RepoType::Model);
    // // Download the model to the specified local folder
    // let destination_path = "all-MiniLM-L12-v2";
    let hf_repo = api.repo(repo);
    hf_repo.get(location)?;

    //
    // println!("Model downloaded to: {}", destination_path);
    //
    // // Set-up sentence embeddings model
    // let model = SentenceEmbeddingsBuilder::local("resources/all-MiniLM-L12-v2")
    //   .with_device(tch::Device::cuda_if_available())
    //   .create_model()?;
    //
    // // Define input
    // let sentences = ["this is an example sentence", "each sentence is
    // converted"];
    //
    // // Generate Embeddings
    // let embeddings = model.encode(&sentences)?;
    // println!("{embeddings:?}");
    //
    Ok(())
  }

  #[test]
  fn qa_test()
  {
    // match test.create_model() {
    //   Ok(model) => println!("Model works"),
    //   Err(e) => println!("Uhhh ohhh error thinggy: {:?}", e),
    // }
    //
    let corpus = "Dune is a 1965 science fiction novel by American author Frank Herbert, originally published as two separate serials in Analog magazine. It tied with Roger Zelazny's This Immortal for the Hugo Award in 1966, and it won the inaugural Nebula Award for Best Novel. It is the first installment of the Dune saga, and in 2003 was cited as the world's best-selling science fiction novel.";
    let question = "Who is the president of the France?";
    let result = qa_workshop(corpus, question).unwrap();

    println!("Result: {:?}", result);

    assert!(true);
  }

  #[test]
  fn ai_workshop_test()
  {
    println!("Running AI Workshop");

    ai_workshop();

    assert_eq!(1, 1);
  }

  #[test]
  fn ai_summarizer_test()
  {
    // let input = "\
    // Dune is set in the distant future in a feudal interstellar society, descended
    // from terrestrial \ humans, in which various noble houses control
    // planetary fiefs. It tells the story of young Paul Atreides, whose family
    // accepts the stewardship of the planet Arrakis. While the planet is an
    // inhospitable and sparsely populated desert wasteland, it is the only source
    // of melange, or spice, a drug that extends life and enhances mental abilities.
    // Melange is also necessary for space navigation, which requires a kind of
    // multidimensional awareness and foresight that only the drug provides. As
    // melange can only be produced on Arrakis, control of the planet is a coveted
    // and dangerous undertaking. The story explores the multilayered interactions
    // of politics, religion, ecology, technology, and human emotion as the factions
    // of the empire confront each other in a struggle for the control of Arrakis
    // and its spice. ";
    let input = "Math is widely considered as descovered and not invented. It is the study of numbers. Math is used to understand relations.";
    let result = ai_summarizer(input).unwrap();

    println!("Result: {:?}", result);

    assert!(true);
  }
}
