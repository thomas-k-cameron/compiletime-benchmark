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
pub struct DeleteUserInput {
    /// <p>The name of the user to delete.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    #[doc(hidden)]
    pub user_name: std::option::Option<std::string::String>,
}
impl DeleteUserInput {
    /// <p>The name of the user to delete.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn user_name(&self) -> std::option::Option<&str> {
        self.user_name.as_deref()
    }
}
impl DeleteUserInput {
    /// Creates a new builder-style object to manufacture [`DeleteUserInput`](crate::operation::delete_user::DeleteUserInput).
    pub fn builder() -> crate::operation::delete_user::builders::DeleteUserInputBuilder {
        crate::operation::delete_user::builders::DeleteUserInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::delete_user::DeleteUserInput;
/// A builder for [`DeleteUserInput`](crate::operation::delete_user::DeleteUserInput).
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
pub struct DeleteUserInputBuilder {
    pub(crate) user_name: std::option::Option<std::string::String>,
}
impl DeleteUserInputBuilder {
    /// <p>The name of the user to delete.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn user_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.user_name = Some(input.into());
        self
    }
    /// <p>The name of the user to delete.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn set_user_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.user_name = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteUserInput`](crate::operation::delete_user::DeleteUserInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::delete_user::DeleteUserInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::delete_user::DeleteUserInput {
            user_name: self.user_name,
        })
    }
}
