// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the instance's Capacity Reservation targeting preferences. The action returns the <code>capacityReservationPreference</code> response element if the instance is configured to run in On-Demand capacity, or if it is configured in run in any <code>open</code> Capacity Reservation that has matching attributes (instance type, platform, Availability Zone). The action returns the <code>capacityReservationTarget</code> response element if the instance explicily targets a specific Capacity Reservation or Capacity Reservation group.</p>
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
pub struct CapacityReservationSpecificationResponse {
    /// <p>Describes the instance's Capacity Reservation preferences. Possible preferences include:</p>
    /// <ul>
    /// <li> <p> <code>open</code> - The instance can run in any <code>open</code> Capacity Reservation that has matching attributes (instance type, platform, Availability Zone).</p> </li>
    /// <li> <p> <code>none</code> - The instance avoids running in a Capacity Reservation even if one is available. The instance runs in On-Demand capacity.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub capacity_reservation_preference:
        std::option::Option<crate::types::CapacityReservationPreference>,
    /// <p>Information about the targeted Capacity Reservation or Capacity Reservation group.</p>
    #[doc(hidden)]
    pub capacity_reservation_target:
        std::option::Option<crate::types::CapacityReservationTargetResponse>,
}
impl CapacityReservationSpecificationResponse {
    /// <p>Describes the instance's Capacity Reservation preferences. Possible preferences include:</p>
    /// <ul>
    /// <li> <p> <code>open</code> - The instance can run in any <code>open</code> Capacity Reservation that has matching attributes (instance type, platform, Availability Zone).</p> </li>
    /// <li> <p> <code>none</code> - The instance avoids running in a Capacity Reservation even if one is available. The instance runs in On-Demand capacity.</p> </li>
    /// </ul>
    pub fn capacity_reservation_preference(
        &self,
    ) -> std::option::Option<&crate::types::CapacityReservationPreference> {
        self.capacity_reservation_preference.as_ref()
    }
    /// <p>Information about the targeted Capacity Reservation or Capacity Reservation group.</p>
    pub fn capacity_reservation_target(
        &self,
    ) -> std::option::Option<&crate::types::CapacityReservationTargetResponse> {
        self.capacity_reservation_target.as_ref()
    }
}
impl CapacityReservationSpecificationResponse {
    /// Creates a new builder-style object to manufacture [`CapacityReservationSpecificationResponse`](crate::types::CapacityReservationSpecificationResponse).
    pub fn builder() -> crate::types::builders::CapacityReservationSpecificationResponseBuilder {
        crate::types::builders::CapacityReservationSpecificationResponseBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::CapacityReservationSpecificationResponse;
/// A builder for [`CapacityReservationSpecificationResponse`](crate::types::CapacityReservationSpecificationResponse).
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
pub struct CapacityReservationSpecificationResponseBuilder {
    pub(crate) capacity_reservation_preference:
        std::option::Option<crate::types::CapacityReservationPreference>,
    pub(crate) capacity_reservation_target:
        std::option::Option<crate::types::CapacityReservationTargetResponse>,
}
impl CapacityReservationSpecificationResponseBuilder {
    /// <p>Describes the instance's Capacity Reservation preferences. Possible preferences include:</p>
    /// <ul>
    /// <li> <p> <code>open</code> - The instance can run in any <code>open</code> Capacity Reservation that has matching attributes (instance type, platform, Availability Zone).</p> </li>
    /// <li> <p> <code>none</code> - The instance avoids running in a Capacity Reservation even if one is available. The instance runs in On-Demand capacity.</p> </li>
    /// </ul>
    pub fn capacity_reservation_preference(
        mut self,
        input: crate::types::CapacityReservationPreference,
    ) -> Self {
        self.capacity_reservation_preference = Some(input);
        self
    }
    /// <p>Describes the instance's Capacity Reservation preferences. Possible preferences include:</p>
    /// <ul>
    /// <li> <p> <code>open</code> - The instance can run in any <code>open</code> Capacity Reservation that has matching attributes (instance type, platform, Availability Zone).</p> </li>
    /// <li> <p> <code>none</code> - The instance avoids running in a Capacity Reservation even if one is available. The instance runs in On-Demand capacity.</p> </li>
    /// </ul>
    pub fn set_capacity_reservation_preference(
        mut self,
        input: std::option::Option<crate::types::CapacityReservationPreference>,
    ) -> Self {
        self.capacity_reservation_preference = input;
        self
    }
    /// <p>Information about the targeted Capacity Reservation or Capacity Reservation group.</p>
    pub fn capacity_reservation_target(
        mut self,
        input: crate::types::CapacityReservationTargetResponse,
    ) -> Self {
        self.capacity_reservation_target = Some(input);
        self
    }
    /// <p>Information about the targeted Capacity Reservation or Capacity Reservation group.</p>
    pub fn set_capacity_reservation_target(
        mut self,
        input: std::option::Option<crate::types::CapacityReservationTargetResponse>,
    ) -> Self {
        self.capacity_reservation_target = input;
        self
    }
    /// Consumes the builder and constructs a [`CapacityReservationSpecificationResponse`](crate::types::CapacityReservationSpecificationResponse).
    pub fn build(self) -> crate::types::CapacityReservationSpecificationResponse {
        crate::types::CapacityReservationSpecificationResponse {
            capacity_reservation_preference: self.capacity_reservation_preference,
            capacity_reservation_target: self.capacity_reservation_target,
        }
    }
}
