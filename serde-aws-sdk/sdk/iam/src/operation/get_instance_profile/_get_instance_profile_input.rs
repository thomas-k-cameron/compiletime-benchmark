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
pub struct GetInstanceProfileInput {
    /// <p>The name of the instance profile to get information about.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    #[doc(hidden)]
    pub instance_profile_name: std::option::Option<std::string::String>,
}
impl GetInstanceProfileInput {
    /// <p>The name of the instance profile to get information about.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn instance_profile_name(&self) -> std::option::Option<&str> {
        self.instance_profile_name.as_deref()
    }
}
impl GetInstanceProfileInput {
    /// Creates a new builder-style object to manufacture [`GetInstanceProfileInput`](crate::operation::get_instance_profile::GetInstanceProfileInput).
    pub fn builder(
    ) -> crate::operation::get_instance_profile::builders::GetInstanceProfileInputBuilder {
        crate::operation::get_instance_profile::builders::GetInstanceProfileInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_instance_profile::GetInstanceProfileInput;
/// A builder for [`GetInstanceProfileInput`](crate::operation::get_instance_profile::GetInstanceProfileInput).
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
pub struct GetInstanceProfileInputBuilder {
    pub(crate) instance_profile_name: std::option::Option<std::string::String>,
}
impl GetInstanceProfileInputBuilder {
    /// <p>The name of the instance profile to get information about.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn instance_profile_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.instance_profile_name = Some(input.into());
        self
    }
    /// <p>The name of the instance profile to get information about.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn set_instance_profile_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.instance_profile_name = input;
        self
    }
    /// Consumes the builder and constructs a [`GetInstanceProfileInput`](crate::operation::get_instance_profile::GetInstanceProfileInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::get_instance_profile::GetInstanceProfileInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::get_instance_profile::GetInstanceProfileInput {
                instance_profile_name: self.instance_profile_name,
            },
        )
    }
}
