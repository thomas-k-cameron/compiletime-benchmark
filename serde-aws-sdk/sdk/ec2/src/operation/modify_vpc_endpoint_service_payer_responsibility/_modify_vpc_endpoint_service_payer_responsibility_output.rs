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
pub struct ModifyVpcEndpointServicePayerResponsibilityOutput {
    /// <p>Returns <code>true</code> if the request succeeds; otherwise, it returns an error.</p>
    #[doc(hidden)]
    pub return_value: std::option::Option<bool>,
    _request_id: Option<String>,
}
impl ModifyVpcEndpointServicePayerResponsibilityOutput {
    /// <p>Returns <code>true</code> if the request succeeds; otherwise, it returns an error.</p>
    pub fn return_value(&self) -> std::option::Option<bool> {
        self.return_value
    }
}
impl aws_http::request_id::RequestId for ModifyVpcEndpointServicePayerResponsibilityOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ModifyVpcEndpointServicePayerResponsibilityOutput {
    /// Creates a new builder-style object to manufacture [`ModifyVpcEndpointServicePayerResponsibilityOutput`](crate::operation::modify_vpc_endpoint_service_payer_responsibility::ModifyVpcEndpointServicePayerResponsibilityOutput).
    pub fn builder() -> crate::operation::modify_vpc_endpoint_service_payer_responsibility::builders::ModifyVpcEndpointServicePayerResponsibilityOutputBuilder{
        crate::operation::modify_vpc_endpoint_service_payer_responsibility::builders::ModifyVpcEndpointServicePayerResponsibilityOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::modify_vpc_endpoint_service_payer_responsibility::ModifyVpcEndpointServicePayerResponsibilityOutput;
/// A builder for [`ModifyVpcEndpointServicePayerResponsibilityOutput`](crate::operation::modify_vpc_endpoint_service_payer_responsibility::ModifyVpcEndpointServicePayerResponsibilityOutput).
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
pub struct ModifyVpcEndpointServicePayerResponsibilityOutputBuilder {
    pub(crate) return_value: std::option::Option<bool>,
    _request_id: Option<String>,
}
impl ModifyVpcEndpointServicePayerResponsibilityOutputBuilder {
    /// <p>Returns <code>true</code> if the request succeeds; otherwise, it returns an error.</p>
    pub fn return_value(mut self, input: bool) -> Self {
        self.return_value = Some(input);
        self
    }
    /// <p>Returns <code>true</code> if the request succeeds; otherwise, it returns an error.</p>
    pub fn set_return_value(mut self, input: std::option::Option<bool>) -> Self {
        self.return_value = input;
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
    /// Consumes the builder and constructs a [`ModifyVpcEndpointServicePayerResponsibilityOutput`](crate::operation::modify_vpc_endpoint_service_payer_responsibility::ModifyVpcEndpointServicePayerResponsibilityOutput).
    pub fn build(self) -> crate::operation::modify_vpc_endpoint_service_payer_responsibility::ModifyVpcEndpointServicePayerResponsibilityOutput{
        crate::operation::modify_vpc_endpoint_service_payer_responsibility::ModifyVpcEndpointServicePayerResponsibilityOutput {
            return_value: self.return_value
            ,
            _request_id: self._request_id,
        }
    }
}