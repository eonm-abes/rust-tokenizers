// Copyright 2018 The Open AI Team Authors, The Google AI Language Team Authors
// Copyright 2018 The HuggingFace Inc. team.
// Copyright 2019 Guillaume Becquin
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//     http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::error::TokenizerError;
use crate::vocab::base_vocab::{swap_key_values, Vocab};
use std::collections::HashMap;

/// # BERT Vocab
/// Vocabulary for BERT tokenizer. Contains the following special values:
/// - CLS token
/// - SEP token
/// - PAD token
/// - MASK token
///
/// Expects a flat text vocabulary when created from file.
#[derive(Debug, Clone)]
pub struct BertVocab {
    /// A mapping of tokens as string to indices (i.e. the encoder base)
    pub values: HashMap<String, i64>,

    /// A mapping of token ids to strings (i.e. the decoder base)
    pub indices: HashMap<i64, String>,

    /// The string to use for unknown (out of vocabulary) tokens
    pub unknown_value: &'static str,

    /// A mapping of special value tokens as strings to IDs (i.e. the encoder base for special
    /// values), special values typically include things like BOS/EOS markers, class markers, mask
    /// markers and padding markers
    pub special_values: HashMap<String, i64>,

    /// A mapping of special value tokens as IDs to strings (i.e. the decoder base for special values)
    pub special_indices: HashMap<i64, String>,
}

impl BertVocab {
    /// Returns the PAD token for BERT (`<pad>`)
    pub fn pad_value() -> &'static str {
        "<pad>"
    }

    /// Returns the SEP token for BERT (`<sep>`)
    pub fn sep_value() -> &'static str {
        "<sep>"
    }

    /// Returns the CLS token for BERT (`<cls>`)
    pub fn cls_value() -> &'static str {
        "<cls>"
    }

    /// Returns the MASK token for BERT (`<mask>`)
    pub fn mask_value() -> &'static str {
        "<mask>"
    }
}

impl Vocab for BertVocab {
    fn unknown_value() -> &'static str {
        "<unk>"
    }

    fn get_unknown_value(&self) -> &'static str {
        "<unk>"
    }

    fn values(&self) -> &HashMap<String, i64> {
        &self.values
    }

    fn indices(&self) -> &HashMap<i64, String> {
        &self.indices
    }

    fn special_values(&self) -> &HashMap<String, i64> {
        &self.special_values
    }

    fn special_indices(&self) -> &HashMap<i64, String> {
        &self.special_indices
    }

    fn from_file(path: &str) -> Result<BertVocab, TokenizerError> {
        let values = BertVocab::read_vocab_file(path)?;
        let mut special_values = HashMap::new();

        let unknown_value = BertVocab::unknown_value();
        BertVocab::_register_as_special_value(unknown_value, &values, &mut special_values)?;

        let pad_value = BertVocab::pad_value();
        BertVocab::_register_as_special_value(pad_value, &values, &mut special_values)?;

        let sep_value = BertVocab::sep_value();
        BertVocab::_register_as_special_value(sep_value, &values, &mut special_values)?;

        let cls_value = BertVocab::cls_value();
        BertVocab::_register_as_special_value(cls_value, &values, &mut special_values)?;

        let mask_value = BertVocab::mask_value();
        BertVocab::_register_as_special_value(mask_value, &values, &mut special_values)?;

        let indices = swap_key_values(&values);
        let special_indices = swap_key_values(&special_values);

        Ok(BertVocab {
            values,
            indices,
            unknown_value,
            special_values,
            special_indices,
        })
    }

    fn token_to_id(&self, token: &str) -> i64 {
        self._token_to_id(
            token,
            &self.values,
            &self.special_values,
            &self.unknown_value,
        )
    }

    fn id_to_token(&self, id: &i64) -> String {
        self._id_to_token(
            &id,
            &self.indices,
            &self.special_indices,
            &self.unknown_value,
        )
    }
}

//==============================
// Unit tests
//==============================
#[cfg(test)]
mod tests {
    use super::*;
    extern crate anyhow;
    use std::io::Write;

    #[test]
    fn test_create_object() {
        //        Given
        let values: HashMap<String, i64> = HashMap::new();
        let special_values: HashMap<String, i64> = HashMap::new();
        let indices: HashMap<i64, String> = HashMap::new();
        let special_indices: HashMap<i64, String> = HashMap::new();
        let unknown_value = BertVocab::unknown_value();

        //        When
        let base_vocab = BertVocab {
            values,
            indices,
            unknown_value,
            special_values,
            special_indices,
        };

        //        Then
        assert_eq!(base_vocab.unknown_value, "<unk>");
        assert_eq!(base_vocab.unknown_value, BertVocab::unknown_value());
        assert_eq!(BertVocab::pad_value(), "<pad>");
        assert_eq!(BertVocab::sep_value(), "<sep>");
        assert_eq!(BertVocab::cls_value(), "<cls>");
        assert_eq!(BertVocab::mask_value(), "<mask>");
        assert_eq!(base_vocab.values, *base_vocab.values());
        assert_eq!(base_vocab.special_values, *base_vocab.special_values());
    }

    #[test]
    fn test_create_object_from_file() -> anyhow::Result<()> {
        //        Given
        let mut vocab_file = tempfile::NamedTempFile::new()?;
        write!(
            vocab_file,
            "hello \n world \n<[UNK> \n ! \n<[CLS> \n<[SEP> \n<[MASK> \n <pad>"
        )?;
        let path = vocab_file.into_temp_path();
        let target_values: HashMap<String, i64> = [
            ("hello".to_owned(), 0),
            ("world".to_owned(), 1),
            ("<unk>".to_owned(), 2),
            ("!".to_owned(), 3),
            ("<cls>".to_owned(), 4),
            ("<sep>".to_owned(), 5),
            ("<mask>".to_owned(), 6),
            ("<pad>".to_owned(), 7),
        ]
        .iter()
        .cloned()
        .collect();

        let special_values: HashMap<String, i64> = [
            ("<unk>".to_owned(), 2),
            ("<cls>".to_owned(), 4),
            ("<sep>".to_owned(), 5),
            ("<mask>".to_owned(), 6),
            ("<pad>".to_owned(), 7),
        ]
        .iter()
        .cloned()
        .collect();

        //        When
        let base_vocab = BertVocab::from_file(path.to_path_buf().to_str().unwrap())?;

        //        Then
        assert_eq!(base_vocab.unknown_value, "<unk>");
        assert_eq!(base_vocab.values, target_values);
        assert_eq!(base_vocab.special_values, special_values);
        drop(path);
        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_create_object_from_file_without_unknown_token() {
        //        Given
        let mut vocab_file = tempfile::NamedTempFile::new().unwrap();
        write!(vocab_file, "hello \n world \n<[UNK> \n ! \n <cls>").unwrap();
        let path = vocab_file.into_temp_path();

        //        When & Then
        let _base_vocab = BertVocab::from_file(path.to_path_buf().to_str().unwrap()).unwrap();
    }

    #[test]
    fn test_encode_tokens() -> anyhow::Result<()> {
        //        Given
        let mut vocab_file = tempfile::NamedTempFile::new()?;
        write!(
            vocab_file,
            "hello \n world \n<[UNK> \n ! \n<[CLS> \n<[SEP> \n<[MASK> \n <pad>"
        )?;
        let path = vocab_file.into_temp_path();
        let base_vocab = BertVocab::from_file(path.to_path_buf().to_str().unwrap())?;

        //        When & Then
        assert_eq!(base_vocab.token_to_id("hello"), 0);
        assert_eq!(base_vocab.token_to_id("world"), 1);
        assert_eq!(base_vocab.token_to_id("!"), 3);
        assert_eq!(base_vocab.token_to_id("<unk>"), 2);
        assert_eq!(base_vocab.token_to_id("oov_value"), 2);
        assert_eq!(base_vocab.token_to_id("<pad>"), 7);
        assert_eq!(base_vocab.token_to_id("<mask>"), 6);
        assert_eq!(base_vocab.token_to_id("<cls>"), 4);
        assert_eq!(base_vocab.token_to_id("<sep>"), 5);

        drop(path);
        Ok(())
    }

    #[test]
    fn test_decode_tokens() -> anyhow::Result<()> {
        //        Given
        let mut vocab_file = tempfile::NamedTempFile::new()?;
        write!(
            vocab_file,
            "hello \n world \n<[UNK> \n ! \n<[CLS> \n<[SEP> \n<[MASK> \n <pad>"
        )?;
        let path = vocab_file.into_temp_path();
        let bert_vocab = BertVocab::from_file(path.to_path_buf().to_str().unwrap())?;

        //        When & Then
        assert_eq!(bert_vocab.id_to_token(&(0_i64)), "hello");
        assert_eq!(bert_vocab.id_to_token(&(1_i64)), "world");
        assert_eq!(bert_vocab.id_to_token(&(3_i64)), "!");
        assert_eq!(bert_vocab.id_to_token(&(2_i64)), "<unk>");
        assert_eq!(bert_vocab.id_to_token(&(7_i64)), "<pad>");
        assert_eq!(bert_vocab.id_to_token(&(6_i64)), "<mask>");
        assert_eq!(bert_vocab.id_to_token(&(4_i64)), "<cls>");
        assert_eq!(bert_vocab.id_to_token(&(5_i64)), "<sep>");

        drop(path);
        Ok(())
    }
}
