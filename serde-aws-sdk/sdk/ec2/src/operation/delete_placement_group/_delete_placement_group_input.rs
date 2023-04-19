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
pub struct DeletePlacementGroupInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
    /// <p>The name of the placement group.</p>
    #[doc(hidden)]
    pub group_name: std::option::Option<std::string::String>,
}
impl DeletePlacementGroupInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The name of the placement group.</p>
    pub fn group_name(&self) -> std::option::Option<&str> {
        self.group_name.as_deref()
    }
}
impl DeletePlacementGroupInput {
    /// Creates a new builder-style object to manufacture [`DeletePlacementGroupInput`](crate::operation::delete_placement_group::DeletePlacementGroupInput).
    pub fn builder(
    ) -> crate::operation::delete_placement_group::builders::DeletePlacementGroupInputBuilder {
        crate::operation::delete_placement_group::builders::DeletePlacementGroupInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::delete_placement_group::DeletePlacementGroupInput;
/// A builder for [`DeletePlacementGroupInput`](crate::operation::delete_placement_group::DeletePlacementGroupInput).
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
pub struct DeletePlacementGroupInputBuilder {
    pub(crate) dry_run: std::option::Option<bool>,
    pub(crate) group_name: std::option::Option<std::string::String>,
}
impl DeletePlacementGroupInputBuilder {
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
    /// <p>The name of the placement group.</p>
    pub fn group_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.group_name = Some(input.into());
        self
    }
    /// <p>The name of the placement group.</p>
    pub fn set_group_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.group_name = input;
        self
    }
    /// Consumes the builder and constructs a [`DeletePlacementGroupInput`](crate::operation::delete_placement_group::DeletePlacementGroupInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::delete_placement_group::DeletePlacementGroupInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::delete_placement_group::DeletePlacementGroupInput {
                dry_run: self.dry_run,
                group_name: self.group_name,
            },
        )
    }
}
