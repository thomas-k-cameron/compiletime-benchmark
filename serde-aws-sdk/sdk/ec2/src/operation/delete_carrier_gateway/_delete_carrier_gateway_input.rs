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
pub struct DeleteCarrierGatewayInput {
    /// <p>The ID of the carrier gateway.</p>
    #[doc(hidden)]
    pub carrier_gateway_id: std::option::Option<std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl DeleteCarrierGatewayInput {
    /// <p>The ID of the carrier gateway.</p>
    pub fn carrier_gateway_id(&self) -> std::option::Option<&str> {
        self.carrier_gateway_id.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl DeleteCarrierGatewayInput {
    /// Creates a new builder-style object to manufacture [`DeleteCarrierGatewayInput`](crate::operation::delete_carrier_gateway::DeleteCarrierGatewayInput).
    pub fn builder(
    ) -> crate::operation::delete_carrier_gateway::builders::DeleteCarrierGatewayInputBuilder {
        crate::operation::delete_carrier_gateway::builders::DeleteCarrierGatewayInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::delete_carrier_gateway::DeleteCarrierGatewayInput;
/// A builder for [`DeleteCarrierGatewayInput`](crate::operation::delete_carrier_gateway::DeleteCarrierGatewayInput).
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
pub struct DeleteCarrierGatewayInputBuilder {
    pub(crate) carrier_gateway_id: std::option::Option<std::string::String>,
    pub(crate) dry_run: std::option::Option<bool>,
}
impl DeleteCarrierGatewayInputBuilder {
    /// <p>The ID of the carrier gateway.</p>
    pub fn carrier_gateway_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.carrier_gateway_id = Some(input.into());
        self
    }
    /// <p>The ID of the carrier gateway.</p>
    pub fn set_carrier_gateway_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.carrier_gateway_id = input;
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
    /// Consumes the builder and constructs a [`DeleteCarrierGatewayInput`](crate::operation::delete_carrier_gateway::DeleteCarrierGatewayInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::delete_carrier_gateway::DeleteCarrierGatewayInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::delete_carrier_gateway::DeleteCarrierGatewayInput {
                carrier_gateway_id: self.carrier_gateway_id,
                dry_run: self.dry_run,
            },
        )
    }
}
