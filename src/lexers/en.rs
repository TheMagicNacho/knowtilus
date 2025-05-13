use crate::lexers::lexer::{Lexer, LexerConfig};

struct LexerEN
{
  text: String,
  tokens: Vec<String>,
  sentences: Vec<String>,
  config: LexerConfig,
}
impl Lexer for LexerEN
{
  fn new(config: LexerConfig) -> Self {
    LexerEN {
      text: String::new(),
      tokens: Vec::new(),
      sentences: Vec::new(),
      config,
    }
  }

  fn set_text(
    &mut self,
    text: &str,
  ) -> &mut Self
  {
    self.text = text.to_string();
    self
  }

  async fn tokenize_text(&mut self) -> &mut Self
  {
    todo!()
  }

  async fn tokenize_sentences(&mut self) -> &mut Self
  {
    todo!()
  }

  async fn remove_stopwords(&mut self) -> &mut Self
  {
    todo!()
  }

  async fn remove_punctuation(&mut self) -> &mut Self
  {
    todo!()
  }

  async fn lament_tokens(&self) -> &mut Self
  {
    todo!()
  }

  fn get_tokens(&self) -> Vec<String>
  {
    self.tokens.clone()
  }

  fn get_sentences(&self) -> Vec<String>
  {
    self.sentences.clone()
  }
}
