// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provides the location, using character count, in your transcript where a match is identified. For example, the location of an issue or a category match within a segment.</p>
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
pub struct CharacterOffsets {
    /// <p>Provides the character count of the first character where a match is identified. For example, the first character associated with an issue or a category match in a segment transcript.</p>
    #[doc(hidden)]
    pub begin: std::option::Option<i32>,
    /// <p>Provides the character count of the last character where a match is identified. For example, the last character associated with an issue or a category match in a segment transcript.</p>
    #[doc(hidden)]
    pub end: std::option::Option<i32>,
}
impl CharacterOffsets {
    /// <p>Provides the character count of the first character where a match is identified. For example, the first character associated with an issue or a category match in a segment transcript.</p>
    pub fn begin(&self) -> std::option::Option<i32> {
        self.begin
    }
    /// <p>Provides the character count of the last character where a match is identified. For example, the last character associated with an issue or a category match in a segment transcript.</p>
    pub fn end(&self) -> std::option::Option<i32> {
        self.end
    }
}
impl CharacterOffsets {
    /// Creates a new builder-style object to manufacture [`CharacterOffsets`](crate::types::CharacterOffsets).
    pub fn builder() -> crate::types::builders::CharacterOffsetsBuilder {
        crate::types::builders::CharacterOffsetsBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::CharacterOffsets;
/// A builder for [`CharacterOffsets`](crate::types::CharacterOffsets).
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
pub struct CharacterOffsetsBuilder {
    pub(crate) begin: std::option::Option<i32>,
    pub(crate) end: std::option::Option<i32>,
}
impl CharacterOffsetsBuilder {
    /// <p>Provides the character count of the first character where a match is identified. For example, the first character associated with an issue or a category match in a segment transcript.</p>
    pub fn begin(mut self, input: i32) -> Self {
        self.begin = Some(input);
        self
    }
    /// <p>Provides the character count of the first character where a match is identified. For example, the first character associated with an issue or a category match in a segment transcript.</p>
    pub fn set_begin(mut self, input: std::option::Option<i32>) -> Self {
        self.begin = input;
        self
    }
    /// <p>Provides the character count of the last character where a match is identified. For example, the last character associated with an issue or a category match in a segment transcript.</p>
    pub fn end(mut self, input: i32) -> Self {
        self.end = Some(input);
        self
    }
    /// <p>Provides the character count of the last character where a match is identified. For example, the last character associated with an issue or a category match in a segment transcript.</p>
    pub fn set_end(mut self, input: std::option::Option<i32>) -> Self {
        self.end = input;
        self
    }
    /// Consumes the builder and constructs a [`CharacterOffsets`](crate::types::CharacterOffsets).
    pub fn build(self) -> crate::types::CharacterOffsets {
        crate::types::CharacterOffsets {
            begin: self.begin,
            end: self.end,
        }
    }
}
