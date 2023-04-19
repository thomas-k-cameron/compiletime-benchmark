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
pub struct GetCapacityReservationUsageOutput {
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    /// <p>The ID of the Capacity Reservation.</p>
    #[doc(hidden)]
    pub capacity_reservation_id: std::option::Option<std::string::String>,
    /// <p>The type of instance for which the Capacity Reservation reserves capacity.</p>
    #[doc(hidden)]
    pub instance_type: std::option::Option<std::string::String>,
    /// <p>The number of instances for which the Capacity Reservation reserves capacity.</p>
    #[doc(hidden)]
    pub total_instance_count: std::option::Option<i32>,
    /// <p>The remaining capacity. Indicates the number of instances that can be launched in the Capacity Reservation.</p>
    #[doc(hidden)]
    pub available_instance_count: std::option::Option<i32>,
    /// <p>The current state of the Capacity Reservation. A Capacity Reservation can be in one of the following states:</p>
    /// <ul>
    /// <li> <p> <code>active</code> - The Capacity Reservation is active and the capacity is available for your use.</p> </li>
    /// <li> <p> <code>expired</code> - The Capacity Reservation expired automatically at the date and time specified in your request. The reserved capacity is no longer available for your use.</p> </li>
    /// <li> <p> <code>cancelled</code> - The Capacity Reservation was cancelled. The reserved capacity is no longer available for your use.</p> </li>
    /// <li> <p> <code>pending</code> - The Capacity Reservation request was successful but the capacity provisioning is still pending.</p> </li>
    /// <li> <p> <code>failed</code> - The Capacity Reservation request has failed. A request might fail due to invalid request parameters, capacity constraints, or instance limit constraints. Failed requests are retained for 60 minutes.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub state: std::option::Option<crate::types::CapacityReservationState>,
    /// <p>Information about the Capacity Reservation usage.</p>
    #[doc(hidden)]
    pub instance_usages: std::option::Option<std::vec::Vec<crate::types::InstanceUsage>>,
    _request_id: Option<String>,
}
impl GetCapacityReservationUsageOutput {
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The ID of the Capacity Reservation.</p>
    pub fn capacity_reservation_id(&self) -> std::option::Option<&str> {
        self.capacity_reservation_id.as_deref()
    }
    /// <p>The type of instance for which the Capacity Reservation reserves capacity.</p>
    pub fn instance_type(&self) -> std::option::Option<&str> {
        self.instance_type.as_deref()
    }
    /// <p>The number of instances for which the Capacity Reservation reserves capacity.</p>
    pub fn total_instance_count(&self) -> std::option::Option<i32> {
        self.total_instance_count
    }
    /// <p>The remaining capacity. Indicates the number of instances that can be launched in the Capacity Reservation.</p>
    pub fn available_instance_count(&self) -> std::option::Option<i32> {
        self.available_instance_count
    }
    /// <p>The current state of the Capacity Reservation. A Capacity Reservation can be in one of the following states:</p>
    /// <ul>
    /// <li> <p> <code>active</code> - The Capacity Reservation is active and the capacity is available for your use.</p> </li>
    /// <li> <p> <code>expired</code> - The Capacity Reservation expired automatically at the date and time specified in your request. The reserved capacity is no longer available for your use.</p> </li>
    /// <li> <p> <code>cancelled</code> - The Capacity Reservation was cancelled. The reserved capacity is no longer available for your use.</p> </li>
    /// <li> <p> <code>pending</code> - The Capacity Reservation request was successful but the capacity provisioning is still pending.</p> </li>
    /// <li> <p> <code>failed</code> - The Capacity Reservation request has failed. A request might fail due to invalid request parameters, capacity constraints, or instance limit constraints. Failed requests are retained for 60 minutes.</p> </li>
    /// </ul>
    pub fn state(&self) -> std::option::Option<&crate::types::CapacityReservationState> {
        self.state.as_ref()
    }
    /// <p>Information about the Capacity Reservation usage.</p>
    pub fn instance_usages(&self) -> std::option::Option<&[crate::types::InstanceUsage]> {
        self.instance_usages.as_deref()
    }
}
impl aws_http::request_id::RequestId for GetCapacityReservationUsageOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetCapacityReservationUsageOutput {
    /// Creates a new builder-style object to manufacture [`GetCapacityReservationUsageOutput`](crate::operation::get_capacity_reservation_usage::GetCapacityReservationUsageOutput).
    pub fn builder() -> crate::operation::get_capacity_reservation_usage::builders::GetCapacityReservationUsageOutputBuilder{
        crate::operation::get_capacity_reservation_usage::builders::GetCapacityReservationUsageOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::get_capacity_reservation_usage::GetCapacityReservationUsageOutput;
/// A builder for [`GetCapacityReservationUsageOutput`](crate::operation::get_capacity_reservation_usage::GetCapacityReservationUsageOutput).
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
pub struct GetCapacityReservationUsageOutputBuilder {
    pub(crate) next_token: std::option::Option<std::string::String>,
    pub(crate) capacity_reservation_id: std::option::Option<std::string::String>,
    pub(crate) instance_type: std::option::Option<std::string::String>,
    pub(crate) total_instance_count: std::option::Option<i32>,
    pub(crate) available_instance_count: std::option::Option<i32>,
    pub(crate) state: std::option::Option<crate::types::CapacityReservationState>,
    pub(crate) instance_usages: std::option::Option<std::vec::Vec<crate::types::InstanceUsage>>,
    _request_id: Option<String>,
}
impl GetCapacityReservationUsageOutputBuilder {
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The ID of the Capacity Reservation.</p>
    pub fn capacity_reservation_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.capacity_reservation_id = Some(input.into());
        self
    }
    /// <p>The ID of the Capacity Reservation.</p>
    pub fn set_capacity_reservation_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.capacity_reservation_id = input;
        self
    }
    /// <p>The type of instance for which the Capacity Reservation reserves capacity.</p>
    pub fn instance_type(mut self, input: impl Into<std::string::String>) -> Self {
        self.instance_type = Some(input.into());
        self
    }
    /// <p>The type of instance for which the Capacity Reservation reserves capacity.</p>
    pub fn set_instance_type(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.instance_type = input;
        self
    }
    /// <p>The number of instances for which the Capacity Reservation reserves capacity.</p>
    pub fn total_instance_count(mut self, input: i32) -> Self {
        self.total_instance_count = Some(input);
        self
    }
    /// <p>The number of instances for which the Capacity Reservation reserves capacity.</p>
    pub fn set_total_instance_count(mut self, input: std::option::Option<i32>) -> Self {
        self.total_instance_count = input;
        self
    }
    /// <p>The remaining capacity. Indicates the number of instances that can be launched in the Capacity Reservation.</p>
    pub fn available_instance_count(mut self, input: i32) -> Self {
        self.available_instance_count = Some(input);
        self
    }
    /// <p>The remaining capacity. Indicates the number of instances that can be launched in the Capacity Reservation.</p>
    pub fn set_available_instance_count(mut self, input: std::option::Option<i32>) -> Self {
        self.available_instance_count = input;
        self
    }
    /// <p>The current state of the Capacity Reservation. A Capacity Reservation can be in one of the following states:</p>
    /// <ul>
    /// <li> <p> <code>active</code> - The Capacity Reservation is active and the capacity is available for your use.</p> </li>
    /// <li> <p> <code>expired</code> - The Capacity Reservation expired automatically at the date and time specified in your request. The reserved capacity is no longer available for your use.</p> </li>
    /// <li> <p> <code>cancelled</code> - The Capacity Reservation was cancelled. The reserved capacity is no longer available for your use.</p> </li>
    /// <li> <p> <code>pending</code> - The Capacity Reservation request was successful but the capacity provisioning is still pending.</p> </li>
    /// <li> <p> <code>failed</code> - The Capacity Reservation request has failed. A request might fail due to invalid request parameters, capacity constraints, or instance limit constraints. Failed requests are retained for 60 minutes.</p> </li>
    /// </ul>
    pub fn state(mut self, input: crate::types::CapacityReservationState) -> Self {
        self.state = Some(input);
        self
    }
    /// <p>The current state of the Capacity Reservation. A Capacity Reservation can be in one of the following states:</p>
    /// <ul>
    /// <li> <p> <code>active</code> - The Capacity Reservation is active and the capacity is available for your use.</p> </li>
    /// <li> <p> <code>expired</code> - The Capacity Reservation expired automatically at the date and time specified in your request. The reserved capacity is no longer available for your use.</p> </li>
    /// <li> <p> <code>cancelled</code> - The Capacity Reservation was cancelled. The reserved capacity is no longer available for your use.</p> </li>
    /// <li> <p> <code>pending</code> - The Capacity Reservation request was successful but the capacity provisioning is still pending.</p> </li>
    /// <li> <p> <code>failed</code> - The Capacity Reservation request has failed. A request might fail due to invalid request parameters, capacity constraints, or instance limit constraints. Failed requests are retained for 60 minutes.</p> </li>
    /// </ul>
    pub fn set_state(
        mut self,
        input: std::option::Option<crate::types::CapacityReservationState>,
    ) -> Self {
        self.state = input;
        self
    }
    /// Appends an item to `instance_usages`.
    ///
    /// To override the contents of this collection use [`set_instance_usages`](Self::set_instance_usages).
    ///
    /// <p>Information about the Capacity Reservation usage.</p>
    pub fn instance_usages(mut self, input: crate::types::InstanceUsage) -> Self {
        let mut v = self.instance_usages.unwrap_or_default();
        v.push(input);
        self.instance_usages = Some(v);
        self
    }
    /// <p>Information about the Capacity Reservation usage.</p>
    pub fn set_instance_usages(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::InstanceUsage>>,
    ) -> Self {
        self.instance_usages = input;
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
    /// Consumes the builder and constructs a [`GetCapacityReservationUsageOutput`](crate::operation::get_capacity_reservation_usage::GetCapacityReservationUsageOutput).
    pub fn build(
        self,
    ) -> crate::operation::get_capacity_reservation_usage::GetCapacityReservationUsageOutput {
        crate::operation::get_capacity_reservation_usage::GetCapacityReservationUsageOutput {
            next_token: self.next_token,
            capacity_reservation_id: self.capacity_reservation_id,
            instance_type: self.instance_type,
            total_instance_count: self.total_instance_count,
            available_instance_count: self.available_instance_count,
            state: self.state,
            instance_usages: self.instance_usages,
            _request_id: self._request_id,
        }
    }
}
