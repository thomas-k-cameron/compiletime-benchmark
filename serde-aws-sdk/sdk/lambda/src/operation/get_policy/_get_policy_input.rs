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
pub struct GetPolicyInput {
    /// <p>The name of the Lambda function, version, or alias.</p>
    /// <p class="title"> <b>Name formats</b> </p>
    /// <ul>
    /// <li> <p> <b>Function name</b> – <code>my-function</code> (name-only), <code>my-function:v1</code> (with alias).</p> </li>
    /// <li> <p> <b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li>
    /// <li> <p> <b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p> </li>
    /// </ul>
    /// <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[doc(hidden)]
    pub function_name: std::option::Option<std::string::String>,
    /// <p>Specify a version or alias to get the policy for that resource.</p>
    #[doc(hidden)]
    pub qualifier: std::option::Option<std::string::String>,
}
impl GetPolicyInput {
    /// <p>The name of the Lambda function, version, or alias.</p>
    /// <p class="title"> <b>Name formats</b> </p>
    /// <ul>
    /// <li> <p> <b>Function name</b> – <code>my-function</code> (name-only), <code>my-function:v1</code> (with alias).</p> </li>
    /// <li> <p> <b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li>
    /// <li> <p> <b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p> </li>
    /// </ul>
    /// <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    pub fn function_name(&self) -> std::option::Option<&str> {
        self.function_name.as_deref()
    }
    /// <p>Specify a version or alias to get the policy for that resource.</p>
    pub fn qualifier(&self) -> std::option::Option<&str> {
        self.qualifier.as_deref()
    }
}
impl GetPolicyInput {
    /// Creates a new builder-style object to manufacture [`GetPolicyInput`](crate::operation::get_policy::GetPolicyInput).
    pub fn builder() -> crate::operation::get_policy::builders::GetPolicyInputBuilder {
        crate::operation::get_policy::builders::GetPolicyInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_policy::GetPolicyInput;
/// A builder for [`GetPolicyInput`](crate::operation::get_policy::GetPolicyInput).
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
pub struct GetPolicyInputBuilder {
    pub(crate) function_name: std::option::Option<std::string::String>,
    pub(crate) qualifier: std::option::Option<std::string::String>,
}
impl GetPolicyInputBuilder {
    /// <p>The name of the Lambda function, version, or alias.</p>
    /// <p class="title"> <b>Name formats</b> </p>
    /// <ul>
    /// <li> <p> <b>Function name</b> – <code>my-function</code> (name-only), <code>my-function:v1</code> (with alias).</p> </li>
    /// <li> <p> <b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li>
    /// <li> <p> <b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p> </li>
    /// </ul>
    /// <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    pub fn function_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.function_name = Some(input.into());
        self
    }
    /// <p>The name of the Lambda function, version, or alias.</p>
    /// <p class="title"> <b>Name formats</b> </p>
    /// <ul>
    /// <li> <p> <b>Function name</b> – <code>my-function</code> (name-only), <code>my-function:v1</code> (with alias).</p> </li>
    /// <li> <p> <b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li>
    /// <li> <p> <b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p> </li>
    /// </ul>
    /// <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    pub fn set_function_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.function_name = input;
        self
    }
    /// <p>Specify a version or alias to get the policy for that resource.</p>
    pub fn qualifier(mut self, input: impl Into<std::string::String>) -> Self {
        self.qualifier = Some(input.into());
        self
    }
    /// <p>Specify a version or alias to get the policy for that resource.</p>
    pub fn set_qualifier(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.qualifier = input;
        self
    }
    /// Consumes the builder and constructs a [`GetPolicyInput`](crate::operation::get_policy::GetPolicyInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::get_policy::GetPolicyInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::get_policy::GetPolicyInput {
            function_name: self.function_name,
            qualifier: self.qualifier,
        })
    }
}
