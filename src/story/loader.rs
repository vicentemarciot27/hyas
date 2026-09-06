use std::{
    fs,
    path::Path,
};

use thiserror::Error;

use super::{
    model::Passage,
    model::MapEntry,
    text::dedent,
};

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("failed to read story file")]
    Io(#[from] std::io::Error),

    #[error("failed to parse story file")]
    Ron(#[from] ron::error::SpannedError),
}

/// PASSAGES
/// Story baked into the binary so the game ships as a single file.
pub fn embedded_passages() -> Vec<Passage> {
    parse_passages(include_str!("../../data/passages.ron")).expect("embedded passages.ron should parse")
}

#[cfg(test)]
pub fn load_passages<P: AsRef<Path>>(path: P) -> Result<Vec<Passage>, LoadError> {
    let source = fs::read_to_string(path)?;
    parse_passages(&source)
}

fn parse_passages(source: &str) -> Result<Vec<Passage>, LoadError> {
    let mut passages: Vec<Passage> = ron::from_str(source)?;

    normalize_passages(&mut passages);
    validate_passages(&passages);


    Ok(passages)
}

fn normalize_passages(passages: &mut [Passage]) {
    for passage in passages {
        for message in &mut passage.messages {
            dedent(&mut message.text);
        }
    }
}

fn validate_passages(passages: &[Passage]) {
    // Validates duplicate passage id
    // wether choice points to an inexisting passage
    // passage.next points to an inexisting passage
    // two choices with the same id inside of a passage
    // passage with choices AND next at the same time
    // condition violates possibility (non-existent flag, skill above max, etc)
    if passages.len() == 0 {
        panic!("passages need to be non-empty, dummy :P");
    }
}

/// MAPS
/// Story maps baked into the binary so the game ships as a single file.
pub fn embedded_maps() -> Vec<super::model::MapEntry> {
    parse_maps(include_str!("../../data/maps.ron")).expect("embedded maps.ron should parse")
}

pub fn parse_maps(source: &str) -> Result<Vec<MapEntry>, LoadError> {
    let maps: Vec<MapEntry> = ron::from_str(source)?;

    Ok(maps)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn story_file_loads_and_every_target_resolves() {
        let passages = load_passages("data/passages.ron").expect("passages.ron should parse");

        let ids: HashSet<&str> = passages.iter().map(|p| p.id.as_str()).collect();
        assert_eq!(ids.len(), passages.len(), "duplicate passage id");

        for passage in &passages {
            if let Some(next) = &passage.next {
                assert!(ids.contains(next.as_str()), "{}: next -> {next}", passage.id);
            }

            for choice in &passage.choices {
                assert!(
                    ids.contains(choice.next.as_str()),
                    "{}: choice {} -> {}",
                    passage.id,
                    choice.id,
                    choice.next
                );
            }
        }
    }
}
