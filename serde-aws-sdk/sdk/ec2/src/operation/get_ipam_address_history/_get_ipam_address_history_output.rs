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
pub struct GetIpamAddressHistoryOutput {
    /// <p>A historical record for a CIDR within an IPAM scope. If the CIDR is associated with an EC2 instance, you will see an object in the response for the instance and one for the network interface.</p>
    #[doc(hidden)]
    pub history_records: std::option::Option<std::vec::Vec<crate::types::IpamAddressHistoryRecord>>,
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl GetIpamAddressHistoryOutput {
    /// <p>A historical record for a CIDR within an IPAM scope. If the CIDR is associated with an EC2 instance, you will see an object in the response for the instance and one for the network interface.</p>
    pub fn history_records(
        &self,
    ) -> std::option::Option<&[crate::types::IpamAddressHistoryRecord]> {
        self.history_records.as_deref()
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl aws_http::request_id::RequestId for GetIpamAddressHistoryOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetIpamAddressHistoryOutput {
    /// Creates a new builder-style object to manufacture [`GetIpamAddressHistoryOutput`](crate::operation::get_ipam_address_history::GetIpamAddressHistoryOutput).
    pub fn builder(
    ) -> crate::operation::get_ipam_address_history::builders::GetIpamAddressHistoryOutputBuilder
    {
        crate::operation::get_ipam_address_history::builders::GetIpamAddressHistoryOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_ipam_address_history::GetIpamAddressHistoryOutput;
/// A builder for [`GetIpamAddressHistoryOutput`](crate::operation::get_ipam_address_history::GetIpamAddressHistoryOutput).
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
pub struct GetIpamAddressHistoryOutputBuilder {
    pub(crate) history_records:
        std::option::Option<std::vec::Vec<crate::types::IpamAddressHistoryRecord>>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl GetIpamAddressHistoryOutputBuilder {
    /// Appends an item to `history_records`.
    ///
    /// To override the contents of this collection use [`set_history_records`](Self::set_history_records).
    ///
    /// <p>A historical record for a CIDR within an IPAM scope. If the CIDR is associated with an EC2 instance, you will see an object in the response for the instance and one for the network interface.</p>
    pub fn history_records(mut self, input: crate::types::IpamAddressHistoryRecord) -> Self {
        let mut v = self.history_records.unwrap_or_default();
        v.push(input);
        self.history_records = Some(v);
        self
    }
    /// <p>A historical record for a CIDR within an IPAM scope. If the CIDR is associated with an EC2 instance, you will see an object in the response for the instance and one for the network interface.</p>
    pub fn set_history_records(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::IpamAddressHistoryRecord>>,
    ) -> Self {
        self.history_records = input;
        self
    }
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
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetIpamAddressHistoryOutput`](crate::operation::get_ipam_address_history::GetIpamAddressHistoryOutput).
    pub fn build(self) -> crate::operation::get_ipam_address_history::GetIpamAddressHistoryOutput {
        crate::operation::get_ipam_address_history::GetIpamAddressHistoryOutput {
            history_records: self.history_records,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
