// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A filter that identifies the subset of objects to which the replication rule applies. A <code>Filter</code> element must specify exactly one <code>Prefix</code>, <code>Tag</code>, or <code>And</code> child element.</p>
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
pub struct ReplicationRuleFilter {
    /// <p>An object key name prefix that identifies the subset of objects that the rule applies to.</p> <important>
    /// <p>When you're using XML requests, you must replace special characters (such as carriage returns) in object keys with their equivalent XML entity codes. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints"> XML-related object key constraints</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// </important>
    #[doc(hidden)]
    pub prefix: std::option::Option<std::string::String>,
    /// <p>A container for a key-value name pair.</p>
    #[doc(hidden)]
    pub tag: std::option::Option<crate::types::S3Tag>,
    /// <p>A container for specifying rule filters. The filters determine the subset of objects that the rule applies to. This element is required only if you specify more than one filter. For example: </p>
    /// <ul>
    /// <li> <p>If you specify both a <code>Prefix</code> and a <code>Tag</code> filter, wrap these filters in an <code>And</code> element.</p> </li>
    /// <li> <p>If you specify a filter based on multiple tags, wrap the <code>Tag</code> elements in an <code>And</code> element.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub and: std::option::Option<crate::types::ReplicationRuleAndOperator>,
}
impl ReplicationRuleFilter {
    /// <p>An object key name prefix that identifies the subset of objects that the rule applies to.</p> <important>
    /// <p>When you're using XML requests, you must replace special characters (such as carriage returns) in object keys with their equivalent XML entity codes. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints"> XML-related object key constraints</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// </important>
    pub fn prefix(&self) -> std::option::Option<&str> {
        self.prefix.as_deref()
    }
    /// <p>A container for a key-value name pair.</p>
    pub fn tag(&self) -> std::option::Option<&crate::types::S3Tag> {
        self.tag.as_ref()
    }
    /// <p>A container for specifying rule filters. The filters determine the subset of objects that the rule applies to. This element is required only if you specify more than one filter. For example: </p>
    /// <ul>
    /// <li> <p>If you specify both a <code>Prefix</code> and a <code>Tag</code> filter, wrap these filters in an <code>And</code> element.</p> </li>
    /// <li> <p>If you specify a filter based on multiple tags, wrap the <code>Tag</code> elements in an <code>And</code> element.</p> </li>
    /// </ul>
    pub fn and(&self) -> std::option::Option<&crate::types::ReplicationRuleAndOperator> {
        self.and.as_ref()
    }
}
impl ReplicationRuleFilter {
    /// Creates a new builder-style object to manufacture [`ReplicationRuleFilter`](crate::types::ReplicationRuleFilter).
    pub fn builder() -> crate::types::builders::ReplicationRuleFilterBuilder {
        crate::types::builders::ReplicationRuleFilterBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ReplicationRuleFilter;
/// A builder for [`ReplicationRuleFilter`](crate::types::ReplicationRuleFilter).
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
pub struct ReplicationRuleFilterBuilder {
    pub(crate) prefix: std::option::Option<std::string::String>,
    pub(crate) tag: std::option::Option<crate::types::S3Tag>,
    pub(crate) and: std::option::Option<crate::types::ReplicationRuleAndOperator>,
}
impl ReplicationRuleFilterBuilder {
    /// <p>An object key name prefix that identifies the subset of objects that the rule applies to.</p> <important>
    /// <p>When you're using XML requests, you must replace special characters (such as carriage returns) in object keys with their equivalent XML entity codes. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints"> XML-related object key constraints</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// </important>
    pub fn prefix(mut self, input: impl Into<std::string::String>) -> Self {
        self.prefix = Some(input.into());
        self
    }
    /// <p>An object key name prefix that identifies the subset of objects that the rule applies to.</p> <important>
    /// <p>When you're using XML requests, you must replace special characters (such as carriage returns) in object keys with their equivalent XML entity codes. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints"> XML-related object key constraints</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// </important>
    pub fn set_prefix(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.prefix = input;
        self
    }
    /// <p>A container for a key-value name pair.</p>
    pub fn tag(mut self, input: crate::types::S3Tag) -> Self {
        self.tag = Some(input);
        self
    }
    /// <p>A container for a key-value name pair.</p>
    pub fn set_tag(mut self, input: std::option::Option<crate::types::S3Tag>) -> Self {
        self.tag = input;
        self
    }
    /// <p>A container for specifying rule filters. The filters determine the subset of objects that the rule applies to. This element is required only if you specify more than one filter. For example: </p>
    /// <ul>
    /// <li> <p>If you specify both a <code>Prefix</code> and a <code>Tag</code> filter, wrap these filters in an <code>And</code> element.</p> </li>
    /// <li> <p>If you specify a filter based on multiple tags, wrap the <code>Tag</code> elements in an <code>And</code> element.</p> </li>
    /// </ul>
    pub fn and(mut self, input: crate::types::ReplicationRuleAndOperator) -> Self {
        self.and = Some(input);
        self
    }
    /// <p>A container for specifying rule filters. The filters determine the subset of objects that the rule applies to. This element is required only if you specify more than one filter. For example: </p>
    /// <ul>
    /// <li> <p>If you specify both a <code>Prefix</code> and a <code>Tag</code> filter, wrap these filters in an <code>And</code> element.</p> </li>
    /// <li> <p>If you specify a filter based on multiple tags, wrap the <code>Tag</code> elements in an <code>And</code> element.</p> </li>
    /// </ul>
    pub fn set_and(
        mut self,
        input: std::option::Option<crate::types::ReplicationRuleAndOperator>,
    ) -> Self {
        self.and = input;
        self
    }
    /// Consumes the builder and constructs a [`ReplicationRuleFilter`](crate::types::ReplicationRuleFilter).
    pub fn build(self) -> crate::types::ReplicationRuleFilter {
        crate::types::ReplicationRuleFilter {
            prefix: self.prefix,
            tag: self.tag,
            and: self.and,
        }
    }
}
