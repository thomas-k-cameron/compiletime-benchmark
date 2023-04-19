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
pub struct DeleteVpcEndpointServiceConfigurationsInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
    /// <p>The IDs of the services.</p>
    #[doc(hidden)]
    pub service_ids: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl DeleteVpcEndpointServiceConfigurationsInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The IDs of the services.</p>
    pub fn service_ids(&self) -> std::option::Option<&[std::string::String]> {
        self.service_ids.as_deref()
    }
}
impl DeleteVpcEndpointServiceConfigurationsInput {
    /// Creates a new builder-style object to manufacture [`DeleteVpcEndpointServiceConfigurationsInput`](crate::operation::delete_vpc_endpoint_service_configurations::DeleteVpcEndpointServiceConfigurationsInput).
    pub fn builder() -> crate::operation::delete_vpc_endpoint_service_configurations::builders::DeleteVpcEndpointServiceConfigurationsInputBuilder{
        crate::operation::delete_vpc_endpoint_service_configurations::builders::DeleteVpcEndpointServiceConfigurationsInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::delete_vpc_endpoint_service_configurations::DeleteVpcEndpointServiceConfigurationsInput;
/// A builder for [`DeleteVpcEndpointServiceConfigurationsInput`](crate::operation::delete_vpc_endpoint_service_configurations::DeleteVpcEndpointServiceConfigurationsInput).
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
pub struct DeleteVpcEndpointServiceConfigurationsInputBuilder {
    pub(crate) dry_run: std::option::Option<bool>,
    pub(crate) service_ids: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl DeleteVpcEndpointServiceConfigurationsInputBuilder {
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
    /// Appends an item to `service_ids`.
    ///
    /// To override the contents of this collection use [`set_service_ids`](Self::set_service_ids).
    ///
    /// <p>The IDs of the services.</p>
    pub fn service_ids(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.service_ids.unwrap_or_default();
        v.push(input.into());
        self.service_ids = Some(v);
        self
    }
    /// <p>The IDs of the services.</p>
    pub fn set_service_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.service_ids = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteVpcEndpointServiceConfigurationsInput`](crate::operation::delete_vpc_endpoint_service_configurations::DeleteVpcEndpointServiceConfigurationsInput).
    pub fn build(self) -> Result<crate::operation::delete_vpc_endpoint_service_configurations::DeleteVpcEndpointServiceConfigurationsInput, aws_smithy_http::operation::error::BuildError>{
        Ok(
            crate::operation::delete_vpc_endpoint_service_configurations::DeleteVpcEndpointServiceConfigurationsInput {
                dry_run: self.dry_run
                ,
                service_ids: self.service_ids
                ,
            }
        )
    }
}
