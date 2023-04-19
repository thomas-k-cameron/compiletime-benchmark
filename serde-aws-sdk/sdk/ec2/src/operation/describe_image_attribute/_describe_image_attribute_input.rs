// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the parameters for DescribeImageAttribute.</p>
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
pub struct DescribeImageAttributeInput {
    /// <p>The AMI attribute.</p>
    /// <p> <b>Note</b>: The <code>blockDeviceMapping</code> attribute is deprecated. Using this attribute returns the <code>Client.AuthFailure</code> error. To get information about the block device mappings for an AMI, use the <code>DescribeImages</code> action.</p>
    #[doc(hidden)]
    pub attribute: std::option::Option<crate::types::ImageAttributeName>,
    /// <p>The ID of the AMI.</p>
    #[doc(hidden)]
    pub image_id: std::option::Option<std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl DescribeImageAttributeInput {
    /// <p>The AMI attribute.</p>
    /// <p> <b>Note</b>: The <code>blockDeviceMapping</code> attribute is deprecated. Using this attribute returns the <code>Client.AuthFailure</code> error. To get information about the block device mappings for an AMI, use the <code>DescribeImages</code> action.</p>
    pub fn attribute(&self) -> std::option::Option<&crate::types::ImageAttributeName> {
        self.attribute.as_ref()
    }
    /// <p>The ID of the AMI.</p>
    pub fn image_id(&self) -> std::option::Option<&str> {
        self.image_id.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl DescribeImageAttributeInput {
    /// Creates a new builder-style object to manufacture [`DescribeImageAttributeInput`](crate::operation::describe_image_attribute::DescribeImageAttributeInput).
    pub fn builder(
    ) -> crate::operation::describe_image_attribute::builders::DescribeImageAttributeInputBuilder
    {
        crate::operation::describe_image_attribute::builders::DescribeImageAttributeInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::describe_image_attribute::DescribeImageAttributeInput;
/// A builder for [`DescribeImageAttributeInput`](crate::operation::describe_image_attribute::DescribeImageAttributeInput).
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
pub struct DescribeImageAttributeInputBuilder {
    pub(crate) attribute: std::option::Option<crate::types::ImageAttributeName>,
    pub(crate) image_id: std::option::Option<std::string::String>,
    pub(crate) dry_run: std::option::Option<bool>,
}
impl DescribeImageAttributeInputBuilder {
    /// <p>The AMI attribute.</p>
    /// <p> <b>Note</b>: The <code>blockDeviceMapping</code> attribute is deprecated. Using this attribute returns the <code>Client.AuthFailure</code> error. To get information about the block device mappings for an AMI, use the <code>DescribeImages</code> action.</p>
    pub fn attribute(mut self, input: crate::types::ImageAttributeName) -> Self {
        self.attribute = Some(input);
        self
    }
    /// <p>The AMI attribute.</p>
    /// <p> <b>Note</b>: The <code>blockDeviceMapping</code> attribute is deprecated. Using this attribute returns the <code>Client.AuthFailure</code> error. To get information about the block device mappings for an AMI, use the <code>DescribeImages</code> action.</p>
    pub fn set_attribute(
        mut self,
        input: std::option::Option<crate::types::ImageAttributeName>,
    ) -> Self {
        self.attribute = input;
        self
    }
    /// <p>The ID of the AMI.</p>
    pub fn image_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.image_id = Some(input.into());
        self
    }
    /// <p>The ID of the AMI.</p>
    pub fn set_image_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.image_id = input;
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
    /// Consumes the builder and constructs a [`DescribeImageAttributeInput`](crate::operation::describe_image_attribute::DescribeImageAttributeInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::describe_image_attribute::DescribeImageAttributeInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::describe_image_attribute::DescribeImageAttributeInput {
                attribute: self.attribute,
                image_id: self.image_id,
                dry_run: self.dry_run,
            },
        )
    }
}