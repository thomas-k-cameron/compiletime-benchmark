// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A list of possible alternative transcriptions for the input audio. Each alternative may contain one or more of <code>Items</code>, <code>Entities</code>, or <code>Transcript</code>.</p>
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct Alternative {
    /// <p>Contains transcribed text.</p>
    #[doc(hidden)]
    pub transcript: std::option::Option<std::string::String>,
    /// <p>Contains words, phrases, or punctuation marks in your transcription output.</p>
    #[doc(hidden)]
    pub items: std::option::Option<std::vec::Vec<crate::types::Item>>,
    /// <p>Contains entities identified as personally identifiable information (PII) in your transcription output.</p>
    #[doc(hidden)]
    pub entities: std::option::Option<std::vec::Vec<crate::types::Entity>>,
}
impl Alternative {
    /// <p>Contains transcribed text.</p>
    pub fn transcript(&self) -> std::option::Option<&str> {
        self.transcript.as_deref()
    }
    /// <p>Contains words, phrases, or punctuation marks in your transcription output.</p>
    pub fn items(&self) -> std::option::Option<&[crate::types::Item]> {
        self.items.as_deref()
    }
    /// <p>Contains entities identified as personally identifiable information (PII) in your transcription output.</p>
    pub fn entities(&self) -> std::option::Option<&[crate::types::Entity]> {
        self.entities.as_deref()
    }
}
impl Alternative {
    /// Creates a new builder-style object to manufacture [`Alternative`](crate::types::Alternative).
    pub fn builder() -> crate::types::builders::AlternativeBuilder {
        crate::types::builders::AlternativeBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::Alternative;
/// A builder for [`Alternative`](crate::types::Alternative).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct AlternativeBuilder {
    pub(crate) transcript: std::option::Option<std::string::String>,
    pub(crate) items: std::option::Option<std::vec::Vec<crate::types::Item>>,
    pub(crate) entities: std::option::Option<std::vec::Vec<crate::types::Entity>>,
}
impl AlternativeBuilder {
    /// <p>Contains transcribed text.</p>
    pub fn transcript(mut self, input: impl Into<std::string::String>) -> Self {
        self.transcript = Some(input.into());
        self
    }
    /// <p>Contains transcribed text.</p>
    pub fn set_transcript(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.transcript = input;
        self
    }
    /// Appends an item to `items`.
    ///
    /// To override the contents of this collection use [`set_items`](Self::set_items).
    ///
    /// <p>Contains words, phrases, or punctuation marks in your transcription output.</p>
    pub fn items(mut self, input: crate::types::Item) -> Self {
        let mut v = self.items.unwrap_or_default();
        v.push(input);
        self.items = Some(v);
        self
    }
    /// <p>Contains words, phrases, or punctuation marks in your transcription output.</p>
    pub fn set_items(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Item>>,
    ) -> Self {
        self.items = input;
        self
    }
    /// Appends an item to `entities`.
    ///
    /// To override the contents of this collection use [`set_entities`](Self::set_entities).
    ///
    /// <p>Contains entities identified as personally identifiable information (PII) in your transcription output.</p>
    pub fn entities(mut self, input: crate::types::Entity) -> Self {
        let mut v = self.entities.unwrap_or_default();
        v.push(input);
        self.entities = Some(v);
        self
    }
    /// <p>Contains entities identified as personally identifiable information (PII) in your transcription output.</p>
    pub fn set_entities(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Entity>>,
    ) -> Self {
        self.entities = input;
        self
    }
    /// Consumes the builder and constructs a [`Alternative`](crate::types::Alternative).
    pub fn build(self) -> crate::types::Alternative {
        crate::types::Alternative {
            transcript: self.transcript,
            items: self.items,
            entities: self.entities,
        }
    }
}