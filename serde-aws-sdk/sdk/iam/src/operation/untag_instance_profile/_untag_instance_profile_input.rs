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
pub struct UntagInstanceProfileInput {
    /// <p>The name of the IAM instance profile from which you want to remove tags.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    #[doc(hidden)]
    pub instance_profile_name: std::option::Option<std::string::String>,
    /// <p>A list of key names as a simple array of strings. The tags with matching keys are removed from the specified instance profile.</p>
    #[doc(hidden)]
    pub tag_keys: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl UntagInstanceProfileInput {
    /// <p>The name of the IAM instance profile from which you want to remove tags.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn instance_profile_name(&self) -> std::option::Option<&str> {
        self.instance_profile_name.as_deref()
    }
    /// <p>A list of key names as a simple array of strings. The tags with matching keys are removed from the specified instance profile.</p>
    pub fn tag_keys(&self) -> std::option::Option<&[std::string::String]> {
        self.tag_keys.as_deref()
    }
}
impl UntagInstanceProfileInput {
    /// Creates a new builder-style object to manufacture [`UntagInstanceProfileInput`](crate::operation::untag_instance_profile::UntagInstanceProfileInput).
    pub fn builder(
    ) -> crate::operation::untag_instance_profile::builders::UntagInstanceProfileInputBuilder {
        crate::operation::untag_instance_profile::builders::UntagInstanceProfileInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::untag_instance_profile::UntagInstanceProfileInput;
/// A builder for [`UntagInstanceProfileInput`](crate::operation::untag_instance_profile::UntagInstanceProfileInput).
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
pub struct UntagInstanceProfileInputBuilder {
    pub(crate) instance_profile_name: std::option::Option<std::string::String>,
    pub(crate) tag_keys: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl UntagInstanceProfileInputBuilder {
    /// <p>The name of the IAM instance profile from which you want to remove tags.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn instance_profile_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.instance_profile_name = Some(input.into());
        self
    }
    /// <p>The name of the IAM instance profile from which you want to remove tags.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn set_instance_profile_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.instance_profile_name = input;
        self
    }
    /// Appends an item to `tag_keys`.
    ///
    /// To override the contents of this collection use [`set_tag_keys`](Self::set_tag_keys).
    ///
    /// <p>A list of key names as a simple array of strings. The tags with matching keys are removed from the specified instance profile.</p>
    pub fn tag_keys(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.tag_keys.unwrap_or_default();
        v.push(input.into());
        self.tag_keys = Some(v);
        self
    }
    /// <p>A list of key names as a simple array of strings. The tags with matching keys are removed from the specified instance profile.</p>
    pub fn set_tag_keys(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.tag_keys = input;
        self
    }
    /// Consumes the builder and constructs a [`UntagInstanceProfileInput`](crate::operation::untag_instance_profile::UntagInstanceProfileInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::untag_instance_profile::UntagInstanceProfileInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::untag_instance_profile::UntagInstanceProfileInput {
                instance_profile_name: self.instance_profile_name,
                tag_keys: self.tag_keys,
            },
        )
    }
}