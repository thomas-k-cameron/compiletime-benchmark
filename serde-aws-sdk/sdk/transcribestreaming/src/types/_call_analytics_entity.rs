// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains entities identified as personally identifiable information (PII) in your transcription output, along with various associated attributes. Examples include category, confidence score, content, type, and start and end times.</p>
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
pub struct CallAnalyticsEntity {
    /// <p>The time, in milliseconds, from the beginning of the audio stream to the start of the identified entity.</p>
    #[doc(hidden)]
    pub begin_offset_millis: std::option::Option<i64>,
    /// <p>The time, in milliseconds, from the beginning of the audio stream to the end of the identified entity.</p>
    #[doc(hidden)]
    pub end_offset_millis: std::option::Option<i64>,
    /// <p>The category of information identified. For example, <code>PII</code>.</p>
    #[doc(hidden)]
    pub category: std::option::Option<std::string::String>,
    /// <p>The type of PII identified. For example, <code>NAME</code> or <code>CREDIT_DEBIT_NUMBER</code>.</p>
    #[doc(hidden)]
    pub r#type: std::option::Option<std::string::String>,
    /// <p>The word or words that represent the identified entity.</p>
    #[doc(hidden)]
    pub content: std::option::Option<std::string::String>,
    /// <p>The confidence score associated with the identification of an entity in your transcript.</p>
    /// <p>Confidence scores are values between 0 and 1. A larger value indicates a higher probability that the identified entity correctly matches the entity spoken in your media.</p>
    #[doc(hidden)]
    pub confidence: std::option::Option<f64>,
}
impl CallAnalyticsEntity {
    /// <p>The time, in milliseconds, from the beginning of the audio stream to the start of the identified entity.</p>
    pub fn begin_offset_millis(&self) -> std::option::Option<i64> {
        self.begin_offset_millis
    }
    /// <p>The time, in milliseconds, from the beginning of the audio stream to the end of the identified entity.</p>
    pub fn end_offset_millis(&self) -> std::option::Option<i64> {
        self.end_offset_millis
    }
    /// <p>The category of information identified. For example, <code>PII</code>.</p>
    pub fn category(&self) -> std::option::Option<&str> {
        self.category.as_deref()
    }
    /// <p>The type of PII identified. For example, <code>NAME</code> or <code>CREDIT_DEBIT_NUMBER</code>.</p>
    pub fn r#type(&self) -> std::option::Option<&str> {
        self.r#type.as_deref()
    }
    /// <p>The word or words that represent the identified entity.</p>
    pub fn content(&self) -> std::option::Option<&str> {
        self.content.as_deref()
    }
    /// <p>The confidence score associated with the identification of an entity in your transcript.</p>
    /// <p>Confidence scores are values between 0 and 1. A larger value indicates a higher probability that the identified entity correctly matches the entity spoken in your media.</p>
    pub fn confidence(&self) -> std::option::Option<f64> {
        self.confidence
    }
}
impl CallAnalyticsEntity {
    /// Creates a new builder-style object to manufacture [`CallAnalyticsEntity`](crate::types::CallAnalyticsEntity).
    pub fn builder() -> crate::types::builders::CallAnalyticsEntityBuilder {
        crate::types::builders::CallAnalyticsEntityBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::CallAnalyticsEntity;
/// A builder for [`CallAnalyticsEntity`](crate::types::CallAnalyticsEntity).
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
pub struct CallAnalyticsEntityBuilder {
    pub(crate) begin_offset_millis: std::option::Option<i64>,
    pub(crate) end_offset_millis: std::option::Option<i64>,
    pub(crate) category: std::option::Option<std::string::String>,
    pub(crate) r#type: std::option::Option<std::string::String>,
    pub(crate) content: std::option::Option<std::string::String>,
    pub(crate) confidence: std::option::Option<f64>,
}
impl CallAnalyticsEntityBuilder {
    /// <p>The time, in milliseconds, from the beginning of the audio stream to the start of the identified entity.</p>
    pub fn begin_offset_millis(mut self, input: i64) -> Self {
        self.begin_offset_millis = Some(input);
        self
    }
    /// <p>The time, in milliseconds, from the beginning of the audio stream to the start of the identified entity.</p>
    pub fn set_begin_offset_millis(mut self, input: std::option::Option<i64>) -> Self {
        self.begin_offset_millis = input;
        self
    }
    /// <p>The time, in milliseconds, from the beginning of the audio stream to the end of the identified entity.</p>
    pub fn end_offset_millis(mut self, input: i64) -> Self {
        self.end_offset_millis = Some(input);
        self
    }
    /// <p>The time, in milliseconds, from the beginning of the audio stream to the end of the identified entity.</p>
    pub fn set_end_offset_millis(mut self, input: std::option::Option<i64>) -> Self {
        self.end_offset_millis = input;
        self
    }
    /// <p>The category of information identified. For example, <code>PII</code>.</p>
    pub fn category(mut self, input: impl Into<std::string::String>) -> Self {
        self.category = Some(input.into());
        self
    }
    /// <p>The category of information identified. For example, <code>PII</code>.</p>
    pub fn set_category(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.category = input;
        self
    }
    /// <p>The type of PII identified. For example, <code>NAME</code> or <code>CREDIT_DEBIT_NUMBER</code>.</p>
    pub fn r#type(mut self, input: impl Into<std::string::String>) -> Self {
        self.r#type = Some(input.into());
        self
    }
    /// <p>The type of PII identified. For example, <code>NAME</code> or <code>CREDIT_DEBIT_NUMBER</code>.</p>
    pub fn set_type(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.r#type = input;
        self
    }
    /// <p>The word or words that represent the identified entity.</p>
    pub fn content(mut self, input: impl Into<std::string::String>) -> Self {
        self.content = Some(input.into());
        self
    }
    /// <p>The word or words that represent the identified entity.</p>
    pub fn set_content(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.content = input;
        self
    }
    /// <p>The confidence score associated with the identification of an entity in your transcript.</p>
    /// <p>Confidence scores are values between 0 and 1. A larger value indicates a higher probability that the identified entity correctly matches the entity spoken in your media.</p>
    pub fn confidence(mut self, input: f64) -> Self {
        self.confidence = Some(input);
        self
    }
    /// <p>The confidence score associated with the identification of an entity in your transcript.</p>
    /// <p>Confidence scores are values between 0 and 1. A larger value indicates a higher probability that the identified entity correctly matches the entity spoken in your media.</p>
    pub fn set_confidence(mut self, input: std::option::Option<f64>) -> Self {
        self.confidence = input;
        self
    }
    /// Consumes the builder and constructs a [`CallAnalyticsEntity`](crate::types::CallAnalyticsEntity).
    pub fn build(self) -> crate::types::CallAnalyticsEntity {
        crate::types::CallAnalyticsEntity {
            begin_offset_millis: self.begin_offset_millis,
            end_offset_millis: self.end_offset_millis,
            category: self.category,
            r#type: self.r#type,
            content: self.content,
            confidence: self.confidence,
        }
    }
}
