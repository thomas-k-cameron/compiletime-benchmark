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
pub struct DeleteCapacityProviderOutput {
    /// <p>The details of the capacity provider.</p>
    #[doc(hidden)]
    pub capacity_provider: std::option::Option<crate::types::CapacityProvider>,
    _request_id: Option<String>,
}
impl DeleteCapacityProviderOutput {
    /// <p>The details of the capacity provider.</p>
    pub fn capacity_provider(&self) -> std::option::Option<&crate::types::CapacityProvider> {
        self.capacity_provider.as_ref()
    }
}
impl aws_http::request_id::RequestId for DeleteCapacityProviderOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteCapacityProviderOutput {
    /// Creates a new builder-style object to manufacture [`DeleteCapacityProviderOutput`](crate::operation::delete_capacity_provider::DeleteCapacityProviderOutput).
    pub fn builder(
    ) -> crate::operation::delete_capacity_provider::builders::DeleteCapacityProviderOutputBuilder
    {
        crate::operation::delete_capacity_provider::builders::DeleteCapacityProviderOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::delete_capacity_provider::DeleteCapacityProviderOutput;
/// A builder for [`DeleteCapacityProviderOutput`](crate::operation::delete_capacity_provider::DeleteCapacityProviderOutput).
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
pub struct DeleteCapacityProviderOutputBuilder {
    pub(crate) capacity_provider: std::option::Option<crate::types::CapacityProvider>,
    _request_id: Option<String>,
}
impl DeleteCapacityProviderOutputBuilder {
    /// <p>The details of the capacity provider.</p>
    pub fn capacity_provider(mut self, input: crate::types::CapacityProvider) -> Self {
        self.capacity_provider = Some(input);
        self
    }
    /// <p>The details of the capacity provider.</p>
    pub fn set_capacity_provider(
        mut self,
        input: std::option::Option<crate::types::CapacityProvider>,
    ) -> Self {
        self.capacity_provider = input;
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
    /// Consumes the builder and constructs a [`DeleteCapacityProviderOutput`](crate::operation::delete_capacity_provider::DeleteCapacityProviderOutput).
    pub fn build(self) -> crate::operation::delete_capacity_provider::DeleteCapacityProviderOutput {
        crate::operation::delete_capacity_provider::DeleteCapacityProviderOutput {
            capacity_provider: self.capacity_provider,
            _request_id: self._request_id,
        }
    }
}
