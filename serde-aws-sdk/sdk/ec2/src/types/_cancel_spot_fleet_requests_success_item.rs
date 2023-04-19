// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a Spot Fleet request that was successfully canceled.</p>
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
pub struct CancelSpotFleetRequestsSuccessItem {
    /// <p>The current state of the Spot Fleet request.</p>
    #[doc(hidden)]
    pub current_spot_fleet_request_state: std::option::Option<crate::types::BatchState>,
    /// <p>The previous state of the Spot Fleet request.</p>
    #[doc(hidden)]
    pub previous_spot_fleet_request_state: std::option::Option<crate::types::BatchState>,
    /// <p>The ID of the Spot Fleet request.</p>
    #[doc(hidden)]
    pub spot_fleet_request_id: std::option::Option<std::string::String>,
}
impl CancelSpotFleetRequestsSuccessItem {
    /// <p>The current state of the Spot Fleet request.</p>
    pub fn current_spot_fleet_request_state(
        &self,
    ) -> std::option::Option<&crate::types::BatchState> {
        self.current_spot_fleet_request_state.as_ref()
    }
    /// <p>The previous state of the Spot Fleet request.</p>
    pub fn previous_spot_fleet_request_state(
        &self,
    ) -> std::option::Option<&crate::types::BatchState> {
        self.previous_spot_fleet_request_state.as_ref()
    }
    /// <p>The ID of the Spot Fleet request.</p>
    pub fn spot_fleet_request_id(&self) -> std::option::Option<&str> {
        self.spot_fleet_request_id.as_deref()
    }
}
impl CancelSpotFleetRequestsSuccessItem {
    /// Creates a new builder-style object to manufacture [`CancelSpotFleetRequestsSuccessItem`](crate::types::CancelSpotFleetRequestsSuccessItem).
    pub fn builder() -> crate::types::builders::CancelSpotFleetRequestsSuccessItemBuilder {
        crate::types::builders::CancelSpotFleetRequestsSuccessItemBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::CancelSpotFleetRequestsSuccessItem;
/// A builder for [`CancelSpotFleetRequestsSuccessItem`](crate::types::CancelSpotFleetRequestsSuccessItem).
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
pub struct CancelSpotFleetRequestsSuccessItemBuilder {
    pub(crate) current_spot_fleet_request_state: std::option::Option<crate::types::BatchState>,
    pub(crate) previous_spot_fleet_request_state: std::option::Option<crate::types::BatchState>,
    pub(crate) spot_fleet_request_id: std::option::Option<std::string::String>,
}
impl CancelSpotFleetRequestsSuccessItemBuilder {
    /// <p>The current state of the Spot Fleet request.</p>
    pub fn current_spot_fleet_request_state(mut self, input: crate::types::BatchState) -> Self {
        self.current_spot_fleet_request_state = Some(input);
        self
    }
    /// <p>The current state of the Spot Fleet request.</p>
    pub fn set_current_spot_fleet_request_state(
        mut self,
        input: std::option::Option<crate::types::BatchState>,
    ) -> Self {
        self.current_spot_fleet_request_state = input;
        self
    }
    /// <p>The previous state of the Spot Fleet request.</p>
    pub fn previous_spot_fleet_request_state(mut self, input: crate::types::BatchState) -> Self {
        self.previous_spot_fleet_request_state = Some(input);
        self
    }
    /// <p>The previous state of the Spot Fleet request.</p>
    pub fn set_previous_spot_fleet_request_state(
        mut self,
        input: std::option::Option<crate::types::BatchState>,
    ) -> Self {
        self.previous_spot_fleet_request_state = input;
        self
    }
    /// <p>The ID of the Spot Fleet request.</p>
    pub fn spot_fleet_request_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.spot_fleet_request_id = Some(input.into());
        self
    }
    /// <p>The ID of the Spot Fleet request.</p>
    pub fn set_spot_fleet_request_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.spot_fleet_request_id = input;
        self
    }
    /// Consumes the builder and constructs a [`CancelSpotFleetRequestsSuccessItem`](crate::types::CancelSpotFleetRequestsSuccessItem).
    pub fn build(self) -> crate::types::CancelSpotFleetRequestsSuccessItem {
        crate::types::CancelSpotFleetRequestsSuccessItem {
            current_spot_fleet_request_state: self.current_spot_fleet_request_state,
            previous_spot_fleet_request_state: self.previous_spot_fleet_request_state,
            spot_fleet_request_id: self.spot_fleet_request_id,
        }
    }
}