// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
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
pub struct UntagResourceInput {
    /// <p>The Amazon Resource Name (ARN) that identifies the resource for which to list the tags. Currently, the supported resources are <code>ConfigRule</code>, <code>ConfigurationAggregator</code> and <code>AggregatorAuthorization</code>.</p>
    #[doc(hidden)]
    pub resource_arn: std::option::Option<std::string::String>,
    /// <p>The keys of the tags to be removed.</p>
    #[doc(hidden)]
    pub tag_keys: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl UntagResourceInput {
    /// <p>The Amazon Resource Name (ARN) that identifies the resource for which to list the tags. Currently, the supported resources are <code>ConfigRule</code>, <code>ConfigurationAggregator</code> and <code>AggregatorAuthorization</code>.</p>
    pub fn resource_arn(&self) -> std::option::Option<&str> {
        self.resource_arn.as_deref()
    }
    /// <p>The keys of the tags to be removed.</p>
    pub fn tag_keys(&self) -> std::option::Option<&[std::string::String]> {
        self.tag_keys.as_deref()
    }
}
impl UntagResourceInput {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::operation::untag_resource::UntagResourceInput).
    pub fn builder() -> crate::operation::untag_resource::builders::UntagResourceInputBuilder {
        crate::operation::untag_resource::builders::UntagResourceInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::untag_resource::UntagResourceInput;
/// A builder for [`UntagResourceInput`](crate::operation::untag_resource::UntagResourceInput).
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
pub struct UntagResourceInputBuilder {
    pub(crate) resource_arn: std::option::Option<std::string::String>,
    pub(crate) tag_keys: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl UntagResourceInputBuilder {
    /// <p>The Amazon Resource Name (ARN) that identifies the resource for which to list the tags. Currently, the supported resources are <code>ConfigRule</code>, <code>ConfigurationAggregator</code> and <code>AggregatorAuthorization</code>.</p>
    pub fn resource_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.resource_arn = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) that identifies the resource for which to list the tags. Currently, the supported resources are <code>ConfigRule</code>, <code>ConfigurationAggregator</code> and <code>AggregatorAuthorization</code>.</p>
    pub fn set_resource_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.resource_arn = input;
        self
    }
    /// Appends an item to `tag_keys`.
    ///
    /// To override the contents of this collection use [`set_tag_keys`](Self::set_tag_keys).
    ///
    /// <p>The keys of the tags to be removed.</p>
    pub fn tag_keys(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.tag_keys.unwrap_or_default();
        v.push(input.into());
        self.tag_keys = Some(v);
        self
    }
    /// <p>The keys of the tags to be removed.</p>
    pub fn set_tag_keys(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.tag_keys = input;
        self
    }
    /// Consumes the builder and constructs a [`UntagResourceInput`](crate::operation::untag_resource::UntagResourceInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::untag_resource::UntagResourceInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::untag_resource::UntagResourceInput {
            resource_arn: self.resource_arn,
            tag_keys: self.tag_keys,
        })
    }
}