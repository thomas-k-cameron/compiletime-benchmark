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
pub struct DescribeAccountAttributesInput {
    /// <p>The account attribute names.</p>
    #[doc(hidden)]
    pub attribute_names: std::option::Option<std::vec::Vec<crate::types::AccountAttributeName>>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl DescribeAccountAttributesInput {
    /// <p>The account attribute names.</p>
    pub fn attribute_names(&self) -> std::option::Option<&[crate::types::AccountAttributeName]> {
        self.attribute_names.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl DescribeAccountAttributesInput {
    /// Creates a new builder-style object to manufacture [`DescribeAccountAttributesInput`](crate::operation::describe_account_attributes::DescribeAccountAttributesInput).
    pub fn builder() -> crate::operation::describe_account_attributes::builders::DescribeAccountAttributesInputBuilder{
        crate::operation::describe_account_attributes::builders::DescribeAccountAttributesInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::describe_account_attributes::DescribeAccountAttributesInput;
/// A builder for [`DescribeAccountAttributesInput`](crate::operation::describe_account_attributes::DescribeAccountAttributesInput).
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
pub struct DescribeAccountAttributesInputBuilder {
    pub(crate) attribute_names:
        std::option::Option<std::vec::Vec<crate::types::AccountAttributeName>>,
    pub(crate) dry_run: std::option::Option<bool>,
}
impl DescribeAccountAttributesInputBuilder {
    /// Appends an item to `attribute_names`.
    ///
    /// To override the contents of this collection use [`set_attribute_names`](Self::set_attribute_names).
    ///
    /// <p>The account attribute names.</p>
    pub fn attribute_names(mut self, input: crate::types::AccountAttributeName) -> Self {
        let mut v = self.attribute_names.unwrap_or_default();
        v.push(input);
        self.attribute_names = Some(v);
        self
    }
    /// <p>The account attribute names.</p>
    pub fn set_attribute_names(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::AccountAttributeName>>,
    ) -> Self {
        self.attribute_names = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeAccountAttributesInput`](crate::operation::describe_account_attributes::DescribeAccountAttributesInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::describe_account_attributes::DescribeAccountAttributesInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::describe_account_attributes::DescribeAccountAttributesInput {
                attribute_names: self.attribute_names,
                dry_run: self.dry_run,
            },
        )
    }
}
