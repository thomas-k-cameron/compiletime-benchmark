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
pub struct CreateCapacityReservationFleetOutput {
    /// <p>The ID of the Capacity Reservation Fleet.</p>
    #[doc(hidden)]
    pub capacity_reservation_fleet_id: std::option::Option<std::string::String>,
    /// <p>The status of the Capacity Reservation Fleet.</p>
    #[doc(hidden)]
    pub state: std::option::Option<crate::types::CapacityReservationFleetState>,
    /// <p>The total number of capacity units for which the Capacity Reservation Fleet reserves capacity.</p>
    #[doc(hidden)]
    pub total_target_capacity: std::option::Option<i32>,
    /// <p>The requested capacity units that have been successfully reserved.</p>
    #[doc(hidden)]
    pub total_fulfilled_capacity: std::option::Option<f64>,
    /// <p>The instance matching criteria for the Capacity Reservation Fleet.</p>
    #[doc(hidden)]
    pub instance_match_criteria: std::option::Option<crate::types::FleetInstanceMatchCriteria>,
    /// <p>The allocation strategy used by the Capacity Reservation Fleet.</p>
    #[doc(hidden)]
    pub allocation_strategy: std::option::Option<std::string::String>,
    /// <p>The date and time at which the Capacity Reservation Fleet was created.</p>
    #[doc(hidden)]
    pub create_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The date and time at which the Capacity Reservation Fleet expires.</p>
    #[doc(hidden)]
    pub end_date: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>Indicates the tenancy of Capacity Reservation Fleet.</p>
    #[doc(hidden)]
    pub tenancy: std::option::Option<crate::types::FleetCapacityReservationTenancy>,
    /// <p>Information about the individual Capacity Reservations in the Capacity Reservation Fleet.</p>
    #[doc(hidden)]
    pub fleet_capacity_reservations:
        std::option::Option<std::vec::Vec<crate::types::FleetCapacityReservation>>,
    /// <p>The tags assigned to the Capacity Reservation Fleet.</p>
    #[doc(hidden)]
    pub tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    _request_id: Option<String>,
}
impl CreateCapacityReservationFleetOutput {
    /// <p>The ID of the Capacity Reservation Fleet.</p>
    pub fn capacity_reservation_fleet_id(&self) -> std::option::Option<&str> {
        self.capacity_reservation_fleet_id.as_deref()
    }
    /// <p>The status of the Capacity Reservation Fleet.</p>
    pub fn state(&self) -> std::option::Option<&crate::types::CapacityReservationFleetState> {
        self.state.as_ref()
    }
    /// <p>The total number of capacity units for which the Capacity Reservation Fleet reserves capacity.</p>
    pub fn total_target_capacity(&self) -> std::option::Option<i32> {
        self.total_target_capacity
    }
    /// <p>The requested capacity units that have been successfully reserved.</p>
    pub fn total_fulfilled_capacity(&self) -> std::option::Option<f64> {
        self.total_fulfilled_capacity
    }
    /// <p>The instance matching criteria for the Capacity Reservation Fleet.</p>
    pub fn instance_match_criteria(
        &self,
    ) -> std::option::Option<&crate::types::FleetInstanceMatchCriteria> {
        self.instance_match_criteria.as_ref()
    }
    /// <p>The allocation strategy used by the Capacity Reservation Fleet.</p>
    pub fn allocation_strategy(&self) -> std::option::Option<&str> {
        self.allocation_strategy.as_deref()
    }
    /// <p>The date and time at which the Capacity Reservation Fleet was created.</p>
    pub fn create_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.create_time.as_ref()
    }
    /// <p>The date and time at which the Capacity Reservation Fleet expires.</p>
    pub fn end_date(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.end_date.as_ref()
    }
    /// <p>Indicates the tenancy of Capacity Reservation Fleet.</p>
    pub fn tenancy(&self) -> std::option::Option<&crate::types::FleetCapacityReservationTenancy> {
        self.tenancy.as_ref()
    }
    /// <p>Information about the individual Capacity Reservations in the Capacity Reservation Fleet.</p>
    pub fn fleet_capacity_reservations(
        &self,
    ) -> std::option::Option<&[crate::types::FleetCapacityReservation]> {
        self.fleet_capacity_reservations.as_deref()
    }
    /// <p>The tags assigned to the Capacity Reservation Fleet.</p>
    pub fn tags(&self) -> std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
}
impl aws_http::request_id::RequestId for CreateCapacityReservationFleetOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateCapacityReservationFleetOutput {
    /// Creates a new builder-style object to manufacture [`CreateCapacityReservationFleetOutput`](crate::operation::create_capacity_reservation_fleet::CreateCapacityReservationFleetOutput).
    pub fn builder() -> crate::operation::create_capacity_reservation_fleet::builders::CreateCapacityReservationFleetOutputBuilder{
        crate::operation::create_capacity_reservation_fleet::builders::CreateCapacityReservationFleetOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::create_capacity_reservation_fleet::CreateCapacityReservationFleetOutput;
/// A builder for [`CreateCapacityReservationFleetOutput`](crate::operation::create_capacity_reservation_fleet::CreateCapacityReservationFleetOutput).
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
pub struct CreateCapacityReservationFleetOutputBuilder {
    pub(crate) capacity_reservation_fleet_id: std::option::Option<std::string::String>,
    pub(crate) state: std::option::Option<crate::types::CapacityReservationFleetState>,
    pub(crate) total_target_capacity: std::option::Option<i32>,
    pub(crate) total_fulfilled_capacity: std::option::Option<f64>,
    pub(crate) instance_match_criteria:
        std::option::Option<crate::types::FleetInstanceMatchCriteria>,
    pub(crate) allocation_strategy: std::option::Option<std::string::String>,
    pub(crate) create_time: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) end_date: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) tenancy: std::option::Option<crate::types::FleetCapacityReservationTenancy>,
    pub(crate) fleet_capacity_reservations:
        std::option::Option<std::vec::Vec<crate::types::FleetCapacityReservation>>,
    pub(crate) tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    _request_id: Option<String>,
}
impl CreateCapacityReservationFleetOutputBuilder {
    /// <p>The ID of the Capacity Reservation Fleet.</p>
    pub fn capacity_reservation_fleet_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.capacity_reservation_fleet_id = Some(input.into());
        self
    }
    /// <p>The ID of the Capacity Reservation Fleet.</p>
    pub fn set_capacity_reservation_fleet_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.capacity_reservation_fleet_id = input;
        self
    }
    /// <p>The status of the Capacity Reservation Fleet.</p>
    pub fn state(mut self, input: crate::types::CapacityReservationFleetState) -> Self {
        self.state = Some(input);
        self
    }
    /// <p>The status of the Capacity Reservation Fleet.</p>
    pub fn set_state(
        mut self,
        input: std::option::Option<crate::types::CapacityReservationFleetState>,
    ) -> Self {
        self.state = input;
        self
    }
    /// <p>The total number of capacity units for which the Capacity Reservation Fleet reserves capacity.</p>
    pub fn total_target_capacity(mut self, input: i32) -> Self {
        self.total_target_capacity = Some(input);
        self
    }
    /// <p>The total number of capacity units for which the Capacity Reservation Fleet reserves capacity.</p>
    pub fn set_total_target_capacity(mut self, input: std::option::Option<i32>) -> Self {
        self.total_target_capacity = input;
        self
    }
    /// <p>The requested capacity units that have been successfully reserved.</p>
    pub fn total_fulfilled_capacity(mut self, input: f64) -> Self {
        self.total_fulfilled_capacity = Some(input);
        self
    }
    /// <p>The requested capacity units that have been successfully reserved.</p>
    pub fn set_total_fulfilled_capacity(mut self, input: std::option::Option<f64>) -> Self {
        self.total_fulfilled_capacity = input;
        self
    }
    /// <p>The instance matching criteria for the Capacity Reservation Fleet.</p>
    pub fn instance_match_criteria(
        mut self,
        input: crate::types::FleetInstanceMatchCriteria,
    ) -> Self {
        self.instance_match_criteria = Some(input);
        self
    }
    /// <p>The instance matching criteria for the Capacity Reservation Fleet.</p>
    pub fn set_instance_match_criteria(
        mut self,
        input: std::option::Option<crate::types::FleetInstanceMatchCriteria>,
    ) -> Self {
        self.instance_match_criteria = input;
        self
    }
    /// <p>The allocation strategy used by the Capacity Reservation Fleet.</p>
    pub fn allocation_strategy(mut self, input: impl Into<std::string::String>) -> Self {
        self.allocation_strategy = Some(input.into());
        self
    }
    /// <p>The allocation strategy used by the Capacity Reservation Fleet.</p>
    pub fn set_allocation_strategy(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.allocation_strategy = input;
        self
    }
    /// <p>The date and time at which the Capacity Reservation Fleet was created.</p>
    pub fn create_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.create_time = Some(input);
        self
    }
    /// <p>The date and time at which the Capacity Reservation Fleet was created.</p>
    pub fn set_create_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.create_time = input;
        self
    }
    /// <p>The date and time at which the Capacity Reservation Fleet expires.</p>
    pub fn end_date(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.end_date = Some(input);
        self
    }
    /// <p>The date and time at which the Capacity Reservation Fleet expires.</p>
    pub fn set_end_date(mut self, input: std::option::Option<aws_smithy_types::DateTime>) -> Self {
        self.end_date = input;
        self
    }
    /// <p>Indicates the tenancy of Capacity Reservation Fleet.</p>
    pub fn tenancy(mut self, input: crate::types::FleetCapacityReservationTenancy) -> Self {
        self.tenancy = Some(input);
        self
    }
    /// <p>Indicates the tenancy of Capacity Reservation Fleet.</p>
    pub fn set_tenancy(
        mut self,
        input: std::option::Option<crate::types::FleetCapacityReservationTenancy>,
    ) -> Self {
        self.tenancy = input;
        self
    }
    /// Appends an item to `fleet_capacity_reservations`.
    ///
    /// To override the contents of this collection use [`set_fleet_capacity_reservations`](Self::set_fleet_capacity_reservations).
    ///
    /// <p>Information about the individual Capacity Reservations in the Capacity Reservation Fleet.</p>
    pub fn fleet_capacity_reservations(
        mut self,
        input: crate::types::FleetCapacityReservation,
    ) -> Self {
        let mut v = self.fleet_capacity_reservations.unwrap_or_default();
        v.push(input);
        self.fleet_capacity_reservations = Some(v);
        self
    }
    /// <p>Information about the individual Capacity Reservations in the Capacity Reservation Fleet.</p>
    pub fn set_fleet_capacity_reservations(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::FleetCapacityReservation>>,
    ) -> Self {
        self.fleet_capacity_reservations = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags assigned to the Capacity Reservation Fleet.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = Some(v);
        self
    }
    /// <p>The tags assigned to the Capacity Reservation Fleet.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
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
    /// Consumes the builder and constructs a [`CreateCapacityReservationFleetOutput`](crate::operation::create_capacity_reservation_fleet::CreateCapacityReservationFleetOutput).
    pub fn build(
        self,
    ) -> crate::operation::create_capacity_reservation_fleet::CreateCapacityReservationFleetOutput
    {
        crate::operation::create_capacity_reservation_fleet::CreateCapacityReservationFleetOutput {
            capacity_reservation_fleet_id: self.capacity_reservation_fleet_id,
            state: self.state,
            total_target_capacity: self.total_target_capacity,
            total_fulfilled_capacity: self.total_fulfilled_capacity,
            instance_match_criteria: self.instance_match_criteria,
            allocation_strategy: self.allocation_strategy,
            create_time: self.create_time,
            end_date: self.end_date,
            tenancy: self.tenancy,
            fleet_capacity_reservations: self.fleet_capacity_reservations,
            tags: self.tags,
            _request_id: self._request_id,
        }
    }
}