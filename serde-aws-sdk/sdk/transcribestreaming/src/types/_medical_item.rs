// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A word, phrase, or punctuation mark in your transcription output, along with various associated attributes, such as confidence score, type, and start and end times.</p>
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
pub struct MedicalItem {
    /// <p>The start time, in milliseconds, of the transcribed item.</p>
    #[doc(hidden)]
    pub start_time: f64,
    /// <p>The end time, in milliseconds, of the transcribed item.</p>
    #[doc(hidden)]
    pub end_time: f64,
    /// <p>The type of item identified. Options are: <code>PRONUNCIATION</code> (spoken words) and <code>PUNCTUATION</code>.</p>
    #[doc(hidden)]
    pub r#type: std::option::Option<crate::types::ItemType>,
    /// <p>The word or punctuation that was transcribed.</p>
    #[doc(hidden)]
    pub content: std::option::Option<std::string::String>,
    /// <p>The confidence score associated with a word or phrase in your transcript.</p>
    /// <p>Confidence scores are values between 0 and 1. A larger value indicates a higher probability that the identified item correctly matches the item spoken in your media.</p>
    #[doc(hidden)]
    pub confidence: std::option::Option<f64>,
    /// <p>If speaker partitioning is enabled, <code>Speaker</code> labels the speaker of the specified item.</p>
    #[doc(hidden)]
    pub speaker: std::option::Option<std::string::String>,
}
impl MedicalItem {
    /// <p>The start time, in milliseconds, of the transcribed item.</p>
    pub fn start_time(&self) -> f64 {
        self.start_time
    }
    /// <p>The end time, in milliseconds, of the transcribed item.</p>
    pub fn end_time(&self) -> f64 {
        self.end_time
    }
    /// <p>The type of item identified. Options are: <code>PRONUNCIATION</code> (spoken words) and <code>PUNCTUATION</code>.</p>
    pub fn r#type(&self) -> std::option::Option<&crate::types::ItemType> {
        self.r#type.as_ref()
    }
    /// <p>The word or punctuation that was transcribed.</p>
    pub fn content(&self) -> std::option::Option<&str> {
        self.content.as_deref()
    }
    /// <p>The confidence score associated with a word or phrase in your transcript.</p>
    /// <p>Confidence scores are values between 0 and 1. A larger value indicates a higher probability that the identified item correctly matches the item spoken in your media.</p>
    pub fn confidence(&self) -> std::option::Option<f64> {
        self.confidence
    }
    /// <p>If speaker partitioning is enabled, <code>Speaker</code> labels the speaker of the specified item.</p>
    pub fn speaker(&self) -> std::option::Option<&str> {
        self.speaker.as_deref()
    }
}
impl MedicalItem {
    /// Creates a new builder-style object to manufacture [`MedicalItem`](crate::types::MedicalItem).
    pub fn builder() -> crate::types::builders::MedicalItemBuilder {
        crate::types::builders::MedicalItemBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::MedicalItem;
/// A builder for [`MedicalItem`](crate::types::MedicalItem).
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
pub struct MedicalItemBuilder {
    pub(crate) start_time: std::option::Option<f64>,
    pub(crate) end_time: std::option::Option<f64>,
    pub(crate) r#type: std::option::Option<crate::types::ItemType>,
    pub(crate) content: std::option::Option<std::string::String>,
    pub(crate) confidence: std::option::Option<f64>,
    pub(crate) speaker: std::option::Option<std::string::String>,
}
impl MedicalItemBuilder {
    /// <p>The start time, in milliseconds, of the transcribed item.</p>
    pub fn start_time(mut self, input: f64) -> Self {
        self.start_time = Some(input);
        self
    }
    /// <p>The start time, in milliseconds, of the transcribed item.</p>
    pub fn set_start_time(mut self, input: std::option::Option<f64>) -> Self {
        self.start_time = input;
        self
    }
    /// <p>The end time, in milliseconds, of the transcribed item.</p>
    pub fn end_time(mut self, input: f64) -> Self {
        self.end_time = Some(input);
        self
    }
    /// <p>The end time, in milliseconds, of the transcribed item.</p>
    pub fn set_end_time(mut self, input: std::option::Option<f64>) -> Self {
        self.end_time = input;
        self
    }
    /// <p>The type of item identified. Options are: <code>PRONUNCIATION</code> (spoken words) and <code>PUNCTUATION</code>.</p>
    pub fn r#type(mut self, input: crate::types::ItemType) -> Self {
        self.r#type = Some(input);
        self
    }
    /// <p>The type of item identified. Options are: <code>PRONUNCIATION</code> (spoken words) and <code>PUNCTUATION</code>.</p>
    pub fn set_type(mut self, input: std::option::Option<crate::types::ItemType>) -> Self {
        self.r#type = input;
        self
    }
    /// <p>The word or punctuation that was transcribed.</p>
    pub fn content(mut self, input: impl Into<std::string::String>) -> Self {
        self.content = Some(input.into());
        self
    }
    /// <p>The word or punctuation that was transcribed.</p>
    pub fn set_content(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.content = input;
        self
    }
    /// <p>The confidence score associated with a word or phrase in your transcript.</p>
    /// <p>Confidence scores are values between 0 and 1. A larger value indicates a higher probability that the identified item correctly matches the item spoken in your media.</p>
    pub fn confidence(mut self, input: f64) -> Self {
        self.confidence = Some(input);
        self
    }
    /// <p>The confidence score associated with a word or phrase in your transcript.</p>
    /// <p>Confidence scores are values between 0 and 1. A larger value indicates a higher probability that the identified item correctly matches the item spoken in your media.</p>
    pub fn set_confidence(mut self, input: std::option::Option<f64>) -> Self {
        self.confidence = input;
        self
    }
    /// <p>If speaker partitioning is enabled, <code>Speaker</code> labels the speaker of the specified item.</p>
    pub fn speaker(mut self, input: impl Into<std::string::String>) -> Self {
        self.speaker = Some(input.into());
        self
    }
    /// <p>If speaker partitioning is enabled, <code>Speaker</code> labels the speaker of the specified item.</p>
    pub fn set_speaker(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.speaker = input;
        self
    }
    /// Consumes the builder and constructs a [`MedicalItem`](crate::types::MedicalItem).
    pub fn build(self) -> crate::types::MedicalItem {
        crate::types::MedicalItem {
            start_time: self.start_time.unwrap_or_default(),
            end_time: self.end_time.unwrap_or_default(),
            r#type: self.r#type,
            content: self.content,
            confidence: self.confidence,
            speaker: self.speaker,
        }
    }
}
