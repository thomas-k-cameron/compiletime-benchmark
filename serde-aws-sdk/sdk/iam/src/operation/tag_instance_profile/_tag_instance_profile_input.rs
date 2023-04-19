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
pub struct TagInstanceProfileInput {
    /// <p>The name of the IAM instance profile to which you want to add tags.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    #[doc(hidden)]
    pub instance_profile_name: std::option::Option<std::string::String>,
    /// <p>The list of tags that you want to attach to the IAM instance profile. Each tag consists of a key name and an associated value.</p>
    #[doc(hidden)]
    pub tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
}
impl TagInstanceProfileInput {
    /// <p>The name of the IAM instance profile to which you want to add tags.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn instance_profile_name(&self) -> std::option::Option<&str> {
        self.instance_profile_name.as_deref()
    }
    /// <p>The list of tags that you want to attach to the IAM instance profile. Each tag consists of a key name and an associated value.</p>
    pub fn tags(&self) -> std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
}
impl TagInstanceProfileInput {
    /// Creates a new builder-style object to manufacture [`TagInstanceProfileInput`](crate::operation::tag_instance_profile::TagInstanceProfileInput).
    pub fn builder(
    ) -> crate::operation::tag_instance_profile::builders::TagInstanceProfileInputBuilder {
        crate::operation::tag_instance_profile::builders::TagInstanceProfileInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::tag_instance_profile::TagInstanceProfileInput;
/// A builder for [`TagInstanceProfileInput`](crate::operation::tag_instance_profile::TagInstanceProfileInput).
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
pub struct TagInstanceProfileInputBuilder {
    pub(crate) instance_profile_name: std::option::Option<std::string::String>,
    pub(crate) tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
}
impl TagInstanceProfileInputBuilder {
    /// <p>The name of the IAM instance profile to which you want to add tags.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn instance_profile_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.instance_profile_name = Some(input.into());
        self
    }
    /// <p>The name of the IAM instance profile to which you want to add tags.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn set_instance_profile_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.instance_profile_name = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The list of tags that you want to attach to the IAM instance profile. Each tag consists of a key name and an associated value.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = Some(v);
        self
    }
    /// <p>The list of tags that you want to attach to the IAM instance profile. Each tag consists of a key name and an associated value.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`TagInstanceProfileInput`](crate::operation::tag_instance_profile::TagInstanceProfileInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::tag_instance_profile::TagInstanceProfileInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::tag_instance_profile::TagInstanceProfileInput {
                instance_profile_name: self.instance_profile_name,
                tags: self.tags,
            },
        )
    }
}
