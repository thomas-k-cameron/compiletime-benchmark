// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A container for specifying rule filters. The filters determine the subset of objects to which the rule applies. This element is required only if you specify more than one filter. </p>
/// <p>For example:</p>
/// <ul>
/// <li> <p>If you specify both a <code>Prefix</code> and a <code>Tag</code> filter, wrap these filters in an <code>And</code> element. </p> </li>
/// <li> <p>If you specify a filter based on multiple tags, wrap the <code>Tag</code> elements in an <code>And</code> element.</p> </li>
/// </ul>
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
pub struct ReplicationRuleAndOperator {
    /// <p>An object key name prefix that identifies the subset of objects that the rule applies to.</p>
    #[doc(hidden)]
    pub prefix: std::option::Option<std::string::String>,
    /// <p>An array of tags that contain key and value pairs.</p>
    #[doc(hidden)]
    pub tags: std::option::Option<std::vec::Vec<crate::types::S3Tag>>,
}
impl ReplicationRuleAndOperator {
    /// <p>An object key name prefix that identifies the subset of objects that the rule applies to.</p>
    pub fn prefix(&self) -> std::option::Option<&str> {
        self.prefix.as_deref()
    }
    /// <p>An array of tags that contain key and value pairs.</p>
    pub fn tags(&self) -> std::option::Option<&[crate::types::S3Tag]> {
        self.tags.as_deref()
    }
}
impl ReplicationRuleAndOperator {
    /// Creates a new builder-style object to manufacture [`ReplicationRuleAndOperator`](crate::types::ReplicationRuleAndOperator).
    pub fn builder() -> crate::types::builders::ReplicationRuleAndOperatorBuilder {
        crate::types::builders::ReplicationRuleAndOperatorBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ReplicationRuleAndOperator;
/// A builder for [`ReplicationRuleAndOperator`](crate::types::ReplicationRuleAndOperator).
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
pub struct ReplicationRuleAndOperatorBuilder {
    pub(crate) prefix: std::option::Option<std::string::String>,
    pub(crate) tags: std::option::Option<std::vec::Vec<crate::types::S3Tag>>,
}
impl ReplicationRuleAndOperatorBuilder {
    /// <p>An object key name prefix that identifies the subset of objects that the rule applies to.</p>
    pub fn prefix(mut self, input: impl Into<std::string::String>) -> Self {
        self.prefix = Some(input.into());
        self
    }
    /// <p>An object key name prefix that identifies the subset of objects that the rule applies to.</p>
    pub fn set_prefix(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.prefix = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>An array of tags that contain key and value pairs.</p>
    pub fn tags(mut self, input: crate::types::S3Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = Some(v);
        self
    }
    /// <p>An array of tags that contain key and value pairs.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::S3Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`ReplicationRuleAndOperator`](crate::types::ReplicationRuleAndOperator).
    pub fn build(self) -> crate::types::ReplicationRuleAndOperator {
        crate::types::ReplicationRuleAndOperator {
            prefix: self.prefix,
            tags: self.tags,
        }
    }
}