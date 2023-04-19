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
pub struct ModifyIpamPoolOutput {
    /// <p>The results of the modification.</p>
    #[doc(hidden)]
    pub ipam_pool: std::option::Option<crate::types::IpamPool>,
    _request_id: Option<String>,
}
impl ModifyIpamPoolOutput {
    /// <p>The results of the modification.</p>
    pub fn ipam_pool(&self) -> std::option::Option<&crate::types::IpamPool> {
        self.ipam_pool.as_ref()
    }
}
impl aws_http::request_id::RequestId for ModifyIpamPoolOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ModifyIpamPoolOutput {
    /// Creates a new builder-style object to manufacture [`ModifyIpamPoolOutput`](crate::operation::modify_ipam_pool::ModifyIpamPoolOutput).
    pub fn builder() -> crate::operation::modify_ipam_pool::builders::ModifyIpamPoolOutputBuilder {
        crate::operation::modify_ipam_pool::builders::ModifyIpamPoolOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::modify_ipam_pool::ModifyIpamPoolOutput;
/// A builder for [`ModifyIpamPoolOutput`](crate::operation::modify_ipam_pool::ModifyIpamPoolOutput).
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
pub struct ModifyIpamPoolOutputBuilder {
    pub(crate) ipam_pool: std::option::Option<crate::types::IpamPool>,
    _request_id: Option<String>,
}
impl ModifyIpamPoolOutputBuilder {
    /// <p>The results of the modification.</p>
    pub fn ipam_pool(mut self, input: crate::types::IpamPool) -> Self {
        self.ipam_pool = Some(input);
        self
    }
    /// <p>The results of the modification.</p>
    pub fn set_ipam_pool(mut self, input: std::option::Option<crate::types::IpamPool>) -> Self {
        self.ipam_pool = input;
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
    /// Consumes the builder and constructs a [`ModifyIpamPoolOutput`](crate::operation::modify_ipam_pool::ModifyIpamPoolOutput).
    pub fn build(self) -> crate::operation::modify_ipam_pool::ModifyIpamPoolOutput {
        crate::operation::modify_ipam_pool::ModifyIpamPoolOutput {
            ipam_pool: self.ipam_pool,
            _request_id: self._request_id,
        }
    }
}
