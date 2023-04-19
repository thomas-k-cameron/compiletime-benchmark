// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> A structure within a <code>FilterCriteria</code> object that defines an event filtering pattern. </p>
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
pub struct Filter {
    /// <p> A filter pattern. For more information on the syntax of a filter pattern, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/invocation-eventfiltering.html#filtering-syntax"> Filter rule syntax</a>. </p>
    #[doc(hidden)]
    pub pattern: std::option::Option<std::string::String>,
}
impl Filter {
    /// <p> A filter pattern. For more information on the syntax of a filter pattern, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/invocation-eventfiltering.html#filtering-syntax"> Filter rule syntax</a>. </p>
    pub fn pattern(&self) -> std::option::Option<&str> {
        self.pattern.as_deref()
    }
}
impl Filter {
    /// Creates a new builder-style object to manufacture [`Filter`](crate::types::Filter).
    pub fn builder() -> crate::types::builders::FilterBuilder {
        crate::types::builders::FilterBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::Filter;
/// A builder for [`Filter`](crate::types::Filter).
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
pub struct FilterBuilder {
    pub(crate) pattern: std::option::Option<std::string::String>,
}
impl FilterBuilder {
    /// <p> A filter pattern. For more information on the syntax of a filter pattern, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/invocation-eventfiltering.html#filtering-syntax"> Filter rule syntax</a>. </p>
    pub fn pattern(mut self, input: impl Into<std::string::String>) -> Self {
        self.pattern = Some(input.into());
        self
    }
    /// <p> A filter pattern. For more information on the syntax of a filter pattern, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/invocation-eventfiltering.html#filtering-syntax"> Filter rule syntax</a>. </p>
    pub fn set_pattern(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.pattern = input;
        self
    }
    /// Consumes the builder and constructs a [`Filter`](crate::types::Filter).
    pub fn build(self) -> crate::types::Filter {
        crate::types::Filter {
            pattern: self.pattern,
        }
    }
}
