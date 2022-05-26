// This file is part of the serde-bindgen-core libraries
// Copyright (C) 2022  Altronix Corp. <thomas.chiantia@gmail.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// @author Thomas Chiantia <thomas.chiantia@gmail.com>
// @date 2022

use proc_macro2::{Literal, TokenStream, TokenTree};
use serde::de;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;

use crate::attributes::DefaultLit;

fn deserialize_tokens<'de, D>(deserializer: D) -> Result<TokenStream, D::Error>
where
    D: de::Deserializer<'de>,
{
    struct TokenVisitor;

    impl<'de> de::Visitor<'de> for TokenVisitor {
        type Value = TokenStream;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a token like string")
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match v.chars().next() {
                Some('[') => unimplemented!(),
                Some(b) if b > char::from(60) && b <= char::from(71) => v
                    .parse::<i128>()
                    .map_err(E::custom)
                    .map(|i| TokenStream::from(TokenTree::from(Literal::i128_unsuffixed(i)))),
                _ => v.parse().map_err(E::custom),
            }
        }
    }

    deserializer.deserialize_string(TokenVisitor)
}

#[derive(Deserialize)]
#[cfg_attr(feature = "testing", derive(Debug))]
pub struct DescModule(HashMap<String, DescStruct>);

#[derive(Deserialize)]
#[cfg_attr(feature = "testing", derive(Debug))]
pub struct DescStruct {
    pub fields: HashMap<String, DescFieldType>,
}

#[derive(Deserialize)]
#[cfg_attr(feature = "testing", derive(Debug))]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DescFieldType {
    U8(DescFieldTypePrim),
    I8(DescFieldTypePrim),
    U16(DescFieldTypePrim),
    I16(DescFieldTypePrim),
    U32(DescFieldTypePrim),
    I32(DescFieldTypePrim),
    Bool(DescFieldTypePrim),
    Str(DescFieldTypeStr),
    Array(DescFieldTypeArray),
}

#[derive(Deserialize)]
#[cfg_attr(feature = "testing", derive(Debug))]
pub struct DescFieldTypeStr {
    #[serde(deserialize_with = "deserialize_tokens")]
    pub default: TokenStream,
    pub len: usize,
}

#[derive(Deserialize)]
#[cfg_attr(feature = "testing", derive(Debug))]
pub struct DescFieldTypePrim {
    #[serde(deserialize_with = "deserialize_tokens")]
    pub default: TokenStream,
}

#[derive(Deserialize)]
#[cfg_attr(feature = "testing", derive(Debug))]
pub struct DescFieldTypeStruct {}

#[derive(Deserialize)]
#[cfg_attr(feature = "testing", derive(Debug))]
pub struct DescFieldTypeArray {
    pub item: Box<DescFieldType>,
    pub len: usize,
}

mod tests {
    use super::DescModule;

    #[test]
    fn can_parse_yaml() {
        let test_data = r#"
about:
  fields:
    product:
      type: str
      default: LinQ2
      len: 32
    policies:
      type: u8
      default: 0
    users:
      type: array
      len: 8
      item:
        type: str
        default: admin
        len: 32
"#;
        let module = serde_yaml::from_str::<DescModule>(test_data).unwrap();
        println!("{:?}", module);
        assert!(false);
    }
}
