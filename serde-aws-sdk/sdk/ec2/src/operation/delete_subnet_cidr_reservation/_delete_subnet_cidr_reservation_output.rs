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
pub struct DeleteSubnetCidrReservationOutput {
    /// <p>Information about the deleted subnet CIDR reservation.</p>
    #[doc(hidden)]
    pub deleted_subnet_cidr_reservation: std::option::Option<crate::types::SubnetCidrReservation>,
    _request_id: Option<String>,
}
impl DeleteSubnetCidrReservationOutput {
    /// <p>Information about the deleted subnet CIDR reservation.</p>
    pub fn deleted_subnet_cidr_reservation(
        &self,
    ) -> std::option::Option<&crate::types::SubnetCidrReservation> {
        self.deleted_subnet_cidr_reservation.as_ref()
    }
}
impl aws_http::request_id::RequestId for DeleteSubnetCidrReservationOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteSubnetCidrReservationOutput {
    /// Creates a new builder-style object to manufacture [`DeleteSubnetCidrReservationOutput`](crate::operation::delete_subnet_cidr_reservation::DeleteSubnetCidrReservationOutput).
    pub fn builder() -> crate::operation::delete_subnet_cidr_reservation::builders::DeleteSubnetCidrReservationOutputBuilder{
        crate::operation::delete_subnet_cidr_reservation::builders::DeleteSubnetCidrReservationOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::delete_subnet_cidr_reservation::DeleteSubnetCidrReservationOutput;
/// A builder for [`DeleteSubnetCidrReservationOutput`](crate::operation::delete_subnet_cidr_reservation::DeleteSubnetCidrReservationOutput).
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
pub struct DeleteSubnetCidrReservationOutputBuilder {
    pub(crate) deleted_subnet_cidr_reservation:
        std::option::Option<crate::types::SubnetCidrReservation>,
    _request_id: Option<String>,
}
impl DeleteSubnetCidrReservationOutputBuilder {
    /// <p>Information about the deleted subnet CIDR reservation.</p>
    pub fn deleted_subnet_cidr_reservation(
        mut self,
        input: crate::types::SubnetCidrReservation,
    ) -> Self {
        self.deleted_subnet_cidr_reservation = Some(input);
        self
    }
    /// <p>Information about the deleted subnet CIDR reservation.</p>
    pub fn set_deleted_subnet_cidr_reservation(
        mut self,
        input: std::option::Option<crate::types::SubnetCidrReservation>,
    ) -> Self {
        self.deleted_subnet_cidr_reservation = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteSubnetCidrReservationOutput`](crate::operation::delete_subnet_cidr_reservation::DeleteSubnetCidrReservationOutput).
    pub fn build(
        self,
    ) -> crate::operation::delete_subnet_cidr_reservation::DeleteSubnetCidrReservationOutput {
        crate::operation::delete_subnet_cidr_reservation::DeleteSubnetCidrReservationOutput {
            deleted_subnet_cidr_reservation: self.deleted_subnet_cidr_reservation,
            _request_id: self._request_id,
        }
    }
}
