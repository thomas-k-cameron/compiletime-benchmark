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
pub struct DeleteSubnetCidrReservationInput {
    /// <p>The ID of the subnet CIDR reservation.</p>
    #[doc(hidden)]
    pub subnet_cidr_reservation_id: std::option::Option<std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl DeleteSubnetCidrReservationInput {
    /// <p>The ID of the subnet CIDR reservation.</p>
    pub fn subnet_cidr_reservation_id(&self) -> std::option::Option<&str> {
        self.subnet_cidr_reservation_id.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl DeleteSubnetCidrReservationInput {
    /// Creates a new builder-style object to manufacture [`DeleteSubnetCidrReservationInput`](crate::operation::delete_subnet_cidr_reservation::DeleteSubnetCidrReservationInput).
    pub fn builder() -> crate::operation::delete_subnet_cidr_reservation::builders::DeleteSubnetCidrReservationInputBuilder{
        crate::operation::delete_subnet_cidr_reservation::builders::DeleteSubnetCidrReservationInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::delete_subnet_cidr_reservation::DeleteSubnetCidrReservationInput;
/// A builder for [`DeleteSubnetCidrReservationInput`](crate::operation::delete_subnet_cidr_reservation::DeleteSubnetCidrReservationInput).
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
pub struct DeleteSubnetCidrReservationInputBuilder {
    pub(crate) subnet_cidr_reservation_id: std::option::Option<std::string::String>,
    pub(crate) dry_run: std::option::Option<bool>,
}
impl DeleteSubnetCidrReservationInputBuilder {
    /// <p>The ID of the subnet CIDR reservation.</p>
    pub fn subnet_cidr_reservation_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.subnet_cidr_reservation_id = Some(input.into());
        self
    }
    /// <p>The ID of the subnet CIDR reservation.</p>
    pub fn set_subnet_cidr_reservation_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.subnet_cidr_reservation_id = input;
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
    /// Consumes the builder and constructs a [`DeleteSubnetCidrReservationInput`](crate::operation::delete_subnet_cidr_reservation::DeleteSubnetCidrReservationInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::delete_subnet_cidr_reservation::DeleteSubnetCidrReservationInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::delete_subnet_cidr_reservation::DeleteSubnetCidrReservationInput {
                subnet_cidr_reservation_id: self.subnet_cidr_reservation_id,
                dry_run: self.dry_run,
            },
        )
    }
}