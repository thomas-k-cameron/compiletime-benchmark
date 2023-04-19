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
pub struct DeleteAliasInput {
    /// <p>The name of the Lambda function.</p>
    /// <p class="title"> <b>Name formats</b> </p>
    /// <ul>
    /// <li> <p> <b>Function name</b> - <code>MyFunction</code>.</p> </li>
    /// <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p> </li>
    /// <li> <p> <b>Partial ARN</b> - <code>123456789012:function:MyFunction</code>.</p> </li>
    /// </ul>
    /// <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[doc(hidden)]
    pub function_name: std::option::Option<std::string::String>,
    /// <p>The name of the alias.</p>
    #[doc(hidden)]
    pub name: std::option::Option<std::string::String>,
}
impl DeleteAliasInput {
    /// <p>The name of the Lambda function.</p>
    /// <p class="title"> <b>Name formats</b> </p>
    /// <ul>
    /// <li> <p> <b>Function name</b> - <code>MyFunction</code>.</p> </li>
    /// <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p> </li>
    /// <li> <p> <b>Partial ARN</b> - <code>123456789012:function:MyFunction</code>.</p> </li>
    /// </ul>
    /// <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    pub fn function_name(&self) -> std::option::Option<&str> {
        self.function_name.as_deref()
    }
    /// <p>The name of the alias.</p>
    pub fn name(&self) -> std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl DeleteAliasInput {
    /// Creates a new builder-style object to manufacture [`DeleteAliasInput`](crate::operation::delete_alias::DeleteAliasInput).
    pub fn builder() -> crate::operation::delete_alias::builders::DeleteAliasInputBuilder {
        crate::operation::delete_alias::builders::DeleteAliasInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::delete_alias::DeleteAliasInput;
/// A builder for [`DeleteAliasInput`](crate::operation::delete_alias::DeleteAliasInput).
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
pub struct DeleteAliasInputBuilder {
    pub(crate) function_name: std::option::Option<std::string::String>,
    pub(crate) name: std::option::Option<std::string::String>,
}
impl DeleteAliasInputBuilder {
    /// <p>The name of the Lambda function.</p>
    /// <p class="title"> <b>Name formats</b> </p>
    /// <ul>
    /// <li> <p> <b>Function name</b> - <code>MyFunction</code>.</p> </li>
    /// <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p> </li>
    /// <li> <p> <b>Partial ARN</b> - <code>123456789012:function:MyFunction</code>.</p> </li>
    /// </ul>
    /// <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    pub fn function_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.function_name = Some(input.into());
        self
    }
    /// <p>The name of the Lambda function.</p>
    /// <p class="title"> <b>Name formats</b> </p>
    /// <ul>
    /// <li> <p> <b>Function name</b> - <code>MyFunction</code>.</p> </li>
    /// <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p> </li>
    /// <li> <p> <b>Partial ARN</b> - <code>123456789012:function:MyFunction</code>.</p> </li>
    /// </ul>
    /// <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    pub fn set_function_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.function_name = input;
        self
    }
    /// <p>The name of the alias.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.name = Some(input.into());
        self
    }
    /// <p>The name of the alias.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteAliasInput`](crate::operation::delete_alias::DeleteAliasInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::delete_alias::DeleteAliasInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::delete_alias::DeleteAliasInput {
            function_name: self.function_name,
            name: self.name,
        })
    }
}
