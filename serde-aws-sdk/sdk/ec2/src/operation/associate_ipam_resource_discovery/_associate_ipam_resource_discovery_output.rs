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
pub struct AssociateIpamResourceDiscoveryOutput {
    /// <p>A resource discovery association. An associated resource discovery is a resource discovery that has been associated with an IPAM.</p>
    #[doc(hidden)]
    pub ipam_resource_discovery_association:
        std::option::Option<crate::types::IpamResourceDiscoveryAssociation>,
    _request_id: Option<String>,
}
impl AssociateIpamResourceDiscoveryOutput {
    /// <p>A resource discovery association. An associated resource discovery is a resource discovery that has been associated with an IPAM.</p>
    pub fn ipam_resource_discovery_association(
        &self,
    ) -> std::option::Option<&crate::types::IpamResourceDiscoveryAssociation> {
        self.ipam_resource_discovery_association.as_ref()
    }
}
impl aws_http::request_id::RequestId for AssociateIpamResourceDiscoveryOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl AssociateIpamResourceDiscoveryOutput {
    /// Creates a new builder-style object to manufacture [`AssociateIpamResourceDiscoveryOutput`](crate::operation::associate_ipam_resource_discovery::AssociateIpamResourceDiscoveryOutput).
    pub fn builder() -> crate::operation::associate_ipam_resource_discovery::builders::AssociateIpamResourceDiscoveryOutputBuilder{
        crate::operation::associate_ipam_resource_discovery::builders::AssociateIpamResourceDiscoveryOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::associate_ipam_resource_discovery::AssociateIpamResourceDiscoveryOutput;
/// A builder for [`AssociateIpamResourceDiscoveryOutput`](crate::operation::associate_ipam_resource_discovery::AssociateIpamResourceDiscoveryOutput).
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
pub struct AssociateIpamResourceDiscoveryOutputBuilder {
    pub(crate) ipam_resource_discovery_association:
        std::option::Option<crate::types::IpamResourceDiscoveryAssociation>,
    _request_id: Option<String>,
}
impl AssociateIpamResourceDiscoveryOutputBuilder {
    /// <p>A resource discovery association. An associated resource discovery is a resource discovery that has been associated with an IPAM.</p>
    pub fn ipam_resource_discovery_association(
        mut self,
        input: crate::types::IpamResourceDiscoveryAssociation,
    ) -> Self {
        self.ipam_resource_discovery_association = Some(input);
        self
    }
    /// <p>A resource discovery association. An associated resource discovery is a resource discovery that has been associated with an IPAM.</p>
    pub fn set_ipam_resource_discovery_association(
        mut self,
        input: std::option::Option<crate::types::IpamResourceDiscoveryAssociation>,
    ) -> Self {
        self.ipam_resource_discovery_association = input;
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
    /// Consumes the builder and constructs a [`AssociateIpamResourceDiscoveryOutput`](crate::operation::associate_ipam_resource_discovery::AssociateIpamResourceDiscoveryOutput).
    pub fn build(
        self,
    ) -> crate::operation::associate_ipam_resource_discovery::AssociateIpamResourceDiscoveryOutput
    {
        crate::operation::associate_ipam_resource_discovery::AssociateIpamResourceDiscoveryOutput {
            ipam_resource_discovery_association: self.ipam_resource_discovery_association,
            _request_id: self._request_id,
        }
    }
}
