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
pub struct CancelCapacityReservationFleetsInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
    /// <p>The IDs of the Capacity Reservation Fleets to cancel.</p>
    #[doc(hidden)]
    pub capacity_reservation_fleet_ids: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl CancelCapacityReservationFleetsInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The IDs of the Capacity Reservation Fleets to cancel.</p>
    pub fn capacity_reservation_fleet_ids(&self) -> std::option::Option<&[std::string::String]> {
        self.capacity_reservation_fleet_ids.as_deref()
    }
}
impl CancelCapacityReservationFleetsInput {
    /// Creates a new builder-style object to manufacture [`CancelCapacityReservationFleetsInput`](crate::operation::cancel_capacity_reservation_fleets::CancelCapacityReservationFleetsInput).
    pub fn builder() -> crate::operation::cancel_capacity_reservation_fleets::builders::CancelCapacityReservationFleetsInputBuilder{
        crate::operation::cancel_capacity_reservation_fleets::builders::CancelCapacityReservationFleetsInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::cancel_capacity_reservation_fleets::CancelCapacityReservationFleetsInput;
/// A builder for [`CancelCapacityReservationFleetsInput`](crate::operation::cancel_capacity_reservation_fleets::CancelCapacityReservationFleetsInput).
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
pub struct CancelCapacityReservationFleetsInputBuilder {
    pub(crate) dry_run: std::option::Option<bool>,
    pub(crate) capacity_reservation_fleet_ids:
        std::option::Option<std::vec::Vec<std::string::String>>,
}
impl CancelCapacityReservationFleetsInputBuilder {
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
    /// Appends an item to `capacity_reservation_fleet_ids`.
    ///
    /// To override the contents of this collection use [`set_capacity_reservation_fleet_ids`](Self::set_capacity_reservation_fleet_ids).
    ///
    /// <p>The IDs of the Capacity Reservation Fleets to cancel.</p>
    pub fn capacity_reservation_fleet_ids(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.capacity_reservation_fleet_ids.unwrap_or_default();
        v.push(input.into());
        self.capacity_reservation_fleet_ids = Some(v);
        self
    }
    /// <p>The IDs of the Capacity Reservation Fleets to cancel.</p>
    pub fn set_capacity_reservation_fleet_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.capacity_reservation_fleet_ids = input;
        self
    }
    /// Consumes the builder and constructs a [`CancelCapacityReservationFleetsInput`](crate::operation::cancel_capacity_reservation_fleets::CancelCapacityReservationFleetsInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::cancel_capacity_reservation_fleets::CancelCapacityReservationFleetsInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::cancel_capacity_reservation_fleets::CancelCapacityReservationFleetsInput {
                dry_run: self.dry_run
                ,
                capacity_reservation_fleet_ids: self.capacity_reservation_fleet_ids
                ,
            }
        )
    }
}
