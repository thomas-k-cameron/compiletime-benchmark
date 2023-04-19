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
pub struct DisableImageDeprecationInput {
    /// <p>The ID of the AMI.</p>
    #[doc(hidden)]
    pub image_id: std::option::Option<std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl DisableImageDeprecationInput {
    /// <p>The ID of the AMI.</p>
    pub fn image_id(&self) -> std::option::Option<&str> {
        self.image_id.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl DisableImageDeprecationInput {
    /// Creates a new builder-style object to manufacture [`DisableImageDeprecationInput`](crate::operation::disable_image_deprecation::DisableImageDeprecationInput).
    pub fn builder(
    ) -> crate::operation::disable_image_deprecation::builders::DisableImageDeprecationInputBuilder
    {
        crate::operation::disable_image_deprecation::builders::DisableImageDeprecationInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::disable_image_deprecation::DisableImageDeprecationInput;
/// A builder for [`DisableImageDeprecationInput`](crate::operation::disable_image_deprecation::DisableImageDeprecationInput).
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
pub struct DisableImageDeprecationInputBuilder {
    pub(crate) image_id: std::option::Option<std::string::String>,
    pub(crate) dry_run: std::option::Option<bool>,
}
impl DisableImageDeprecationInputBuilder {
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
    /// Consumes the builder and constructs a [`DisableImageDeprecationInput`](crate::operation::disable_image_deprecation::DisableImageDeprecationInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::disable_image_deprecation::DisableImageDeprecationInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::disable_image_deprecation::DisableImageDeprecationInput {
                image_id: self.image_id,
                dry_run: self.dry_run,
            },
        )
    }
}
