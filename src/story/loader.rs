use std::{
    fs,
    path::Path,
};

use thiserror::Error;

use super::{
    model::Passage,
    text::dedent,
};

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("failed to read story file")]
    Io(#[from] std::io::Error),

    #[error("failed to parse story file")]
    Ron(#[from] ron::error::SpannedError),
}

pub fn load_passages(path: impl AsRef<Path>) -> Result<Vec<Passage>, LoadError> {
    let source = fs::read_to_string(path)?;

    let mut passages: Vec<Passage> = ron::from_str(&source)?;

    normalize_passages(&mut passages);

    Ok(passages)
}

fn normalize_passages(passages: &mut [Passage]) {
    for passage in passages {
        for message in &mut passage.messages {
            dedent(&mut message.text);
        }
    }
}