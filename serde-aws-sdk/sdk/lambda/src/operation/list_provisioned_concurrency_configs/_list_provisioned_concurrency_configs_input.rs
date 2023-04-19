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
pub struct ListProvisionedConcurrencyConfigsInput {
    /// <p>The name of the Lambda function.</p>
    /// <p class="title"> <b>Name formats</b> </p>
    /// <ul>
    /// <li> <p> <b>Function name</b> – <code>my-function</code>.</p> </li>
    /// <li> <p> <b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li>
    /// <li> <p> <b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p> </li>
    /// </ul>
    /// <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[doc(hidden)]
    pub function_name: std::option::Option<std::string::String>,
    /// <p>Specify the pagination token that's returned by a previous request to retrieve the next page of results.</p>
    #[doc(hidden)]
    pub marker: std::option::Option<std::string::String>,
    /// <p>Specify a number to limit the number of configurations returned.</p>
    #[doc(hidden)]
    pub max_items: std::option::Option<i32>,
}
impl ListProvisionedConcurrencyConfigsInput {
    /// <p>The name of the Lambda function.</p>
    /// <p class="title"> <b>Name formats</b> </p>
    /// <ul>
    /// <li> <p> <b>Function name</b> – <code>my-function</code>.</p> </li>
    /// <li> <p> <b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li>
    /// <li> <p> <b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p> </li>
    /// </ul>
    /// <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    pub fn function_name(&self) -> std::option::Option<&str> {
        self.function_name.as_deref()
    }
    /// <p>Specify the pagination token that's returned by a previous request to retrieve the next page of results.</p>
    pub fn marker(&self) -> std::option::Option<&str> {
        self.marker.as_deref()
    }
    /// <p>Specify a number to limit the number of configurations returned.</p>
    pub fn max_items(&self) -> std::option::Option<i32> {
        self.max_items
    }
}
impl ListProvisionedConcurrencyConfigsInput {
    /// Creates a new builder-style object to manufacture [`ListProvisionedConcurrencyConfigsInput`](crate::operation::list_provisioned_concurrency_configs::ListProvisionedConcurrencyConfigsInput).
    pub fn builder() -> crate::operation::list_provisioned_concurrency_configs::builders::ListProvisionedConcurrencyConfigsInputBuilder{
        crate::operation::list_provisioned_concurrency_configs::builders::ListProvisionedConcurrencyConfigsInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::list_provisioned_concurrency_configs::ListProvisionedConcurrencyConfigsInput;
/// A builder for [`ListProvisionedConcurrencyConfigsInput`](crate::operation::list_provisioned_concurrency_configs::ListProvisionedConcurrencyConfigsInput).
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
pub struct ListProvisionedConcurrencyConfigsInputBuilder {
    pub(crate) function_name: std::option::Option<std::string::String>,
    pub(crate) marker: std::option::Option<std::string::String>,
    pub(crate) max_items: std::option::Option<i32>,
}
impl ListProvisionedConcurrencyConfigsInputBuilder {
    /// <p>The name of the Lambda function.</p>
    /// <p class="title"> <b>Name formats</b> </p>
    /// <ul>
    /// <li> <p> <b>Function name</b> – <code>my-function</code>.</p> </li>
    /// <li> <p> <b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li>
    /// <li> <p> <b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p> </li>
    /// </ul>
    /// <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    pub fn function_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.function_name = Some(input.into());
        self
    }
    /// <p>The name of the Lambda function.</p>
    /// <p class="title"> <b>Name formats</b> </p>
    /// <ul>
    /// <li> <p> <b>Function name</b> – <code>my-function</code>.</p> </li>
    /// <li> <p> <b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li>
    /// <li> <p> <b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p> </li>
    /// </ul>
    /// <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    pub fn set_function_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.function_name = input;
        self
    }
    /// <p>Specify the pagination token that's returned by a previous request to retrieve the next page of results.</p>
    pub fn marker(mut self, input: impl Into<std::string::String>) -> Self {
        self.marker = Some(input.into());
        self
    }
    /// <p>Specify the pagination token that's returned by a previous request to retrieve the next page of results.</p>
    pub fn set_marker(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.marker = input;
        self
    }
    /// <p>Specify a number to limit the number of configurations returned.</p>
    pub fn max_items(mut self, input: i32) -> Self {
        self.max_items = Some(input);
        self
    }
    /// <p>Specify a number to limit the number of configurations returned.</p>
    pub fn set_max_items(mut self, input: std::option::Option<i32>) -> Self {
        self.max_items = input;
        self
    }
    /// Consumes the builder and constructs a [`ListProvisionedConcurrencyConfigsInput`](crate::operation::list_provisioned_concurrency_configs::ListProvisionedConcurrencyConfigsInput).
    pub fn build(self) -> Result<crate::operation::list_provisioned_concurrency_configs::ListProvisionedConcurrencyConfigsInput, aws_smithy_http::operation::error::BuildError>{
        Ok(
            crate::operation::list_provisioned_concurrency_configs::ListProvisionedConcurrencyConfigsInput {
                function_name: self.function_name
                ,
                marker: self.marker
                ,
                max_items: self.max_items
                ,
            }
        )
    }
}