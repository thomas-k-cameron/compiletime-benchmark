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
pub struct DescribeVpcAttributeInput {
    /// <p>The VPC attribute.</p>
    #[doc(hidden)]
    pub attribute: std::option::Option<crate::types::VpcAttributeName>,
    /// <p>The ID of the VPC.</p>
    #[doc(hidden)]
    pub vpc_id: std::option::Option<std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl DescribeVpcAttributeInput {
    /// <p>The VPC attribute.</p>
    pub fn attribute(&self) -> std::option::Option<&crate::types::VpcAttributeName> {
        self.attribute.as_ref()
    }
    /// <p>The ID of the VPC.</p>
    pub fn vpc_id(&self) -> std::option::Option<&str> {
        self.vpc_id.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl DescribeVpcAttributeInput {
    /// Creates a new builder-style object to manufacture [`DescribeVpcAttributeInput`](crate::operation::describe_vpc_attribute::DescribeVpcAttributeInput).
    pub fn builder(
    ) -> crate::operation::describe_vpc_attribute::builders::DescribeVpcAttributeInputBuilder {
        crate::operation::describe_vpc_attribute::builders::DescribeVpcAttributeInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::describe_vpc_attribute::DescribeVpcAttributeInput;
/// A builder for [`DescribeVpcAttributeInput`](crate::operation::describe_vpc_attribute::DescribeVpcAttributeInput).
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
pub struct DescribeVpcAttributeInputBuilder {
    pub(crate) attribute: std::option::Option<crate::types::VpcAttributeName>,
    pub(crate) vpc_id: std::option::Option<std::string::String>,
    pub(crate) dry_run: std::option::Option<bool>,
}
impl DescribeVpcAttributeInputBuilder {
    /// <p>The VPC attribute.</p>
    pub fn attribute(mut self, input: crate::types::VpcAttributeName) -> Self {
        self.attribute = Some(input);
        self
    }
    /// <p>The VPC attribute.</p>
    pub fn set_attribute(
        mut self,
        input: std::option::Option<crate::types::VpcAttributeName>,
    ) -> Self {
        self.attribute = input;
        self
    }
    /// <p>The ID of the VPC.</p>
    pub fn vpc_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.vpc_id = Some(input.into());
        self
    }
    /// <p>The ID of the VPC.</p>
    pub fn set_vpc_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.vpc_id = input;
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
    /// Consumes the builder and constructs a [`DescribeVpcAttributeInput`](crate::operation::describe_vpc_attribute::DescribeVpcAttributeInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::describe_vpc_attribute::DescribeVpcAttributeInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::describe_vpc_attribute::DescribeVpcAttributeInput {
                attribute: self.attribute,
                vpc_id: self.vpc_id,
                dry_run: self.dry_run,
            },
        )
    }
}