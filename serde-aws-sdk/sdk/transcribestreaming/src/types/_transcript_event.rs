// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The <code>TranscriptEvent</code> associated with a <code>TranscriptResultStream</code>.</p>
/// <p>Contains a set of transcription results from one or more audio segments, along with additional information per your request parameters.</p>
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
pub struct TranscriptEvent {
    /// <p>Contains <code>Results</code>, which contains a set of transcription results from one or more audio segments, along with additional information per your request parameters. This can include information relating to alternative transcriptions, channel identification, partial result stabilization, language identification, and other transcription-related data.</p>
    #[doc(hidden)]
    pub transcript: std::option::Option<crate::types::Transcript>,
}
impl TranscriptEvent {
    /// <p>Contains <code>Results</code>, which contains a set of transcription results from one or more audio segments, along with additional information per your request parameters. This can include information relating to alternative transcriptions, channel identification, partial result stabilization, language identification, and other transcription-related data.</p>
    pub fn transcript(&self) -> std::option::Option<&crate::types::Transcript> {
        self.transcript.as_ref()
    }
}
impl TranscriptEvent {
    /// Creates a new builder-style object to manufacture [`TranscriptEvent`](crate::types::TranscriptEvent).
    pub fn builder() -> crate::types::builders::TranscriptEventBuilder {
        crate::types::builders::TranscriptEventBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::TranscriptEvent;
/// A builder for [`TranscriptEvent`](crate::types::TranscriptEvent).
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
pub struct TranscriptEventBuilder {
    pub(crate) transcript: std::option::Option<crate::types::Transcript>,
}
impl TranscriptEventBuilder {
    /// <p>Contains <code>Results</code>, which contains a set of transcription results from one or more audio segments, along with additional information per your request parameters. This can include information relating to alternative transcriptions, channel identification, partial result stabilization, language identification, and other transcription-related data.</p>
    pub fn transcript(mut self, input: crate::types::Transcript) -> Self {
        self.transcript = Some(input);
        self
    }
    /// <p>Contains <code>Results</code>, which contains a set of transcription results from one or more audio segments, along with additional information per your request parameters. This can include information relating to alternative transcriptions, channel identification, partial result stabilization, language identification, and other transcription-related data.</p>
    pub fn set_transcript(mut self, input: std::option::Option<crate::types::Transcript>) -> Self {
        self.transcript = input;
        self
    }
    /// Consumes the builder and constructs a [`TranscriptEvent`](crate::types::TranscriptEvent).
    pub fn build(self) -> crate::types::TranscriptEvent {
        crate::types::TranscriptEvent {
            transcript: self.transcript,
        }
    }
}
