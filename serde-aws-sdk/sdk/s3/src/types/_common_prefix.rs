// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Container for all (if there are any) keys between Prefix and the next occurrence of the string specified by a delimiter. CommonPrefixes lists keys that act like subdirectories in the directory specified by Prefix. For example, if the prefix is notes/ and the delimiter is a slash (/) as in notes/summer/july, the common prefix is notes/summer/. </p>
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
pub struct CommonPrefix {
    /// <p>Container for the specified common prefix.</p>
    #[doc(hidden)]
    pub prefix: std::option::Option<std::string::String>,
}
impl CommonPrefix {
    /// <p>Container for the specified common prefix.</p>
    pub fn prefix(&self) -> std::option::Option<&str> {
        self.prefix.as_deref()
    }
}
impl CommonPrefix {
    /// Creates a new builder-style object to manufacture [`CommonPrefix`](crate::types::CommonPrefix).
    pub fn builder() -> crate::types::builders::CommonPrefixBuilder {
        crate::types::builders::CommonPrefixBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::CommonPrefix;
/// A builder for [`CommonPrefix`](crate::types::CommonPrefix).
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
pub struct CommonPrefixBuilder {
    pub(crate) prefix: std::option::Option<std::string::String>,
}
impl CommonPrefixBuilder {
    /// <p>Container for the specified common prefix.</p>
    pub fn prefix(mut self, input: impl Into<std::string::String>) -> Self {
        self.prefix = Some(input.into());
        self
    }
    /// <p>Container for the specified common prefix.</p>
    pub fn set_prefix(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.prefix = input;
        self
    }
    /// Consumes the builder and constructs a [`CommonPrefix`](crate::types::CommonPrefix).
    pub fn build(self) -> crate::types::CommonPrefix {
        crate::types::CommonPrefix {
            prefix: self.prefix,
        }
    }
}
