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
pub struct ModifyAvailabilityZoneGroupOutput {
    /// <p>Is <code>true</code> if the request succeeds, and an error otherwise.</p>
    #[doc(hidden)]
    pub r#return: std::option::Option<bool>,
    _request_id: Option<String>,
}
impl ModifyAvailabilityZoneGroupOutput {
    /// <p>Is <code>true</code> if the request succeeds, and an error otherwise.</p>
    pub fn r#return(&self) -> std::option::Option<bool> {
        self.r#return
    }
}
impl aws_http::request_id::RequestId for ModifyAvailabilityZoneGroupOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ModifyAvailabilityZoneGroupOutput {
    /// Creates a new builder-style object to manufacture [`ModifyAvailabilityZoneGroupOutput`](crate::operation::modify_availability_zone_group::ModifyAvailabilityZoneGroupOutput).
    pub fn builder() -> crate::operation::modify_availability_zone_group::builders::ModifyAvailabilityZoneGroupOutputBuilder{
        crate::operation::modify_availability_zone_group::builders::ModifyAvailabilityZoneGroupOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::modify_availability_zone_group::ModifyAvailabilityZoneGroupOutput;
/// A builder for [`ModifyAvailabilityZoneGroupOutput`](crate::operation::modify_availability_zone_group::ModifyAvailabilityZoneGroupOutput).
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
pub struct ModifyAvailabilityZoneGroupOutputBuilder {
    pub(crate) r#return: std::option::Option<bool>,
    _request_id: Option<String>,
}
impl ModifyAvailabilityZoneGroupOutputBuilder {
    /// <p>Is <code>true</code> if the request succeeds, and an error otherwise.</p>
    pub fn r#return(mut self, input: bool) -> Self {
        self.r#return = Some(input);
        self
    }
    /// <p>Is <code>true</code> if the request succeeds, and an error otherwise.</p>
    pub fn set_return(mut self, input: std::option::Option<bool>) -> Self {
        self.r#return = input;
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
    /// Consumes the builder and constructs a [`ModifyAvailabilityZoneGroupOutput`](crate::operation::modify_availability_zone_group::ModifyAvailabilityZoneGroupOutput).
    pub fn build(
        self,
    ) -> crate::operation::modify_availability_zone_group::ModifyAvailabilityZoneGroupOutput {
        crate::operation::modify_availability_zone_group::ModifyAvailabilityZoneGroupOutput {
            r#return: self.r#return,
            _request_id: self._request_id,
        }
    }
}
