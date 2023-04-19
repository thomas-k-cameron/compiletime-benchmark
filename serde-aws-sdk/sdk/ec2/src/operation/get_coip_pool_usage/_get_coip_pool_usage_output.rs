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
pub struct GetCoipPoolUsageOutput {
    /// <p>The ID of the customer-owned address pool.</p>
    #[doc(hidden)]
    pub coip_pool_id: std::option::Option<std::string::String>,
    /// <p>Information about the address usage.</p>
    #[doc(hidden)]
    pub coip_address_usages: std::option::Option<std::vec::Vec<crate::types::CoipAddressUsage>>,
    /// <p>The ID of the local gateway route table.</p>
    #[doc(hidden)]
    pub local_gateway_route_table_id: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl GetCoipPoolUsageOutput {
    /// <p>The ID of the customer-owned address pool.</p>
    pub fn coip_pool_id(&self) -> std::option::Option<&str> {
        self.coip_pool_id.as_deref()
    }
    /// <p>Information about the address usage.</p>
    pub fn coip_address_usages(&self) -> std::option::Option<&[crate::types::CoipAddressUsage]> {
        self.coip_address_usages.as_deref()
    }
    /// <p>The ID of the local gateway route table.</p>
    pub fn local_gateway_route_table_id(&self) -> std::option::Option<&str> {
        self.local_gateway_route_table_id.as_deref()
    }
}
impl aws_http::request_id::RequestId for GetCoipPoolUsageOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetCoipPoolUsageOutput {
    /// Creates a new builder-style object to manufacture [`GetCoipPoolUsageOutput`](crate::operation::get_coip_pool_usage::GetCoipPoolUsageOutput).
    pub fn builder(
    ) -> crate::operation::get_coip_pool_usage::builders::GetCoipPoolUsageOutputBuilder {
        crate::operation::get_coip_pool_usage::builders::GetCoipPoolUsageOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_coip_pool_usage::GetCoipPoolUsageOutput;
/// A builder for [`GetCoipPoolUsageOutput`](crate::operation::get_coip_pool_usage::GetCoipPoolUsageOutput).
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
pub struct GetCoipPoolUsageOutputBuilder {
    pub(crate) coip_pool_id: std::option::Option<std::string::String>,
    pub(crate) coip_address_usages:
        std::option::Option<std::vec::Vec<crate::types::CoipAddressUsage>>,
    pub(crate) local_gateway_route_table_id: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl GetCoipPoolUsageOutputBuilder {
    /// <p>The ID of the customer-owned address pool.</p>
    pub fn coip_pool_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.coip_pool_id = Some(input.into());
        self
    }
    /// <p>The ID of the customer-owned address pool.</p>
    pub fn set_coip_pool_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.coip_pool_id = input;
        self
    }
    /// Appends an item to `coip_address_usages`.
    ///
    /// To override the contents of this collection use [`set_coip_address_usages`](Self::set_coip_address_usages).
    ///
    /// <p>Information about the address usage.</p>
    pub fn coip_address_usages(mut self, input: crate::types::CoipAddressUsage) -> Self {
        let mut v = self.coip_address_usages.unwrap_or_default();
        v.push(input);
        self.coip_address_usages = Some(v);
        self
    }
    /// <p>Information about the address usage.</p>
    pub fn set_coip_address_usages(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::CoipAddressUsage>>,
    ) -> Self {
        self.coip_address_usages = input;
        self
    }
    /// <p>The ID of the local gateway route table.</p>
    pub fn local_gateway_route_table_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.local_gateway_route_table_id = Some(input.into());
        self
    }
    /// <p>The ID of the local gateway route table.</p>
    pub fn set_local_gateway_route_table_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.local_gateway_route_table_id = input;
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
    /// Consumes the builder and constructs a [`GetCoipPoolUsageOutput`](crate::operation::get_coip_pool_usage::GetCoipPoolUsageOutput).
    pub fn build(self) -> crate::operation::get_coip_pool_usage::GetCoipPoolUsageOutput {
        crate::operation::get_coip_pool_usage::GetCoipPoolUsageOutput {
            coip_pool_id: self.coip_pool_id,
            coip_address_usages: self.coip_address_usages,
            local_gateway_route_table_id: self.local_gateway_route_table_id,
            _request_id: self._request_id,
        }
    }
}