// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Lists the issues that were identified in your audio segment.</p>
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
pub struct IssueDetected {
    /// <p>Provides the timestamps that identify when in an audio segment the specified issue occurs.</p>
    #[doc(hidden)]
    pub character_offsets: std::option::Option<crate::types::CharacterOffsets>,
}
impl IssueDetected {
    /// <p>Provides the timestamps that identify when in an audio segment the specified issue occurs.</p>
    pub fn character_offsets(&self) -> std::option::Option<&crate::types::CharacterOffsets> {
        self.character_offsets.as_ref()
    }
}
impl IssueDetected {
    /// Creates a new builder-style object to manufacture [`IssueDetected`](crate::types::IssueDetected).
    pub fn builder() -> crate::types::builders::IssueDetectedBuilder {
        crate::types::builders::IssueDetectedBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::IssueDetected;
/// A builder for [`IssueDetected`](crate::types::IssueDetected).
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
pub struct IssueDetectedBuilder {
    pub(crate) character_offsets: std::option::Option<crate::types::CharacterOffsets>,
}
impl IssueDetectedBuilder {
    /// <p>Provides the timestamps that identify when in an audio segment the specified issue occurs.</p>
    pub fn character_offsets(mut self, input: crate::types::CharacterOffsets) -> Self {
        self.character_offsets = Some(input);
        self
    }
    /// <p>Provides the timestamps that identify when in an audio segment the specified issue occurs.</p>
    pub fn set_character_offsets(
        mut self,
        input: std::option::Option<crate::types::CharacterOffsets>,
    ) -> Self {
        self.character_offsets = input;
        self
    }
    /// Consumes the builder and constructs a [`IssueDetected`](crate::types::IssueDetected).
    pub fn build(self) -> crate::types::IssueDetected {
        crate::types::IssueDetected {
            character_offsets: self.character_offsets,
        }
    }
}