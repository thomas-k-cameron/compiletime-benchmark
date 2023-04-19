// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies JSON as request's output serialization format.</p>
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
pub struct JsonOutput {
    /// <p>The value used to separate individual records in the output. If no value is specified, Amazon S3 uses a newline character ('\n').</p>
    #[doc(hidden)]
    pub record_delimiter: std::option::Option<std::string::String>,
}
impl JsonOutput {
    /// <p>The value used to separate individual records in the output. If no value is specified, Amazon S3 uses a newline character ('\n').</p>
    pub fn record_delimiter(&self) -> std::option::Option<&str> {
        self.record_delimiter.as_deref()
    }
}
impl JsonOutput {
    /// Creates a new builder-style object to manufacture [`JsonOutput`](crate::types::JsonOutput).
    pub fn builder() -> crate::types::builders::JsonOutputBuilder {
        crate::types::builders::JsonOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::JsonOutput;
/// A builder for [`JsonOutput`](crate::types::JsonOutput).
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
pub struct JsonOutputBuilder {
    pub(crate) record_delimiter: std::option::Option<std::string::String>,
}
impl JsonOutputBuilder {
    /// <p>The value used to separate individual records in the output. If no value is specified, Amazon S3 uses a newline character ('\n').</p>
    pub fn record_delimiter(mut self, input: impl Into<std::string::String>) -> Self {
        self.record_delimiter = Some(input.into());
        self
    }
    /// <p>The value used to separate individual records in the output. If no value is specified, Amazon S3 uses a newline character ('\n').</p>
    pub fn set_record_delimiter(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.record_delimiter = input;
        self
    }
    /// Consumes the builder and constructs a [`JsonOutput`](crate::types::JsonOutput).
    pub fn build(self) -> crate::types::JsonOutput {
        crate::types::JsonOutput {
            record_delimiter: self.record_delimiter,
        }
    }
}
