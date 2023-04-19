// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains details about the permissions policies that are attached to the specified identity (user, group, or role).</p>
/// <p>This data type is used as a response element in the <code>ListPoliciesGrantingServiceAccess</code> operation.</p>
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
pub struct ListPoliciesGrantingServiceAccessEntry {
    /// <p>The namespace of the service that was accessed.</p>
    /// <p>To learn the service namespace of a service, see <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/reference_policies_actions-resources-contextkeys.html">Actions, resources, and condition keys for Amazon Web Services services</a> in the <i>Service Authorization Reference</i>. Choose the name of the service to view details for that service. In the first paragraph, find the service prefix. For example, <code>(service prefix: a4b)</code>. For more information about service namespaces, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">Amazon Web Services service namespaces</a> in the&nbsp;<i>Amazon Web Services General Reference</i>.</p>
    #[doc(hidden)]
    pub service_namespace: std::option::Option<std::string::String>,
    /// <p>The&nbsp;<code>PoliciesGrantingServiceAccess</code> object that contains details about the policy.</p>
    #[doc(hidden)]
    pub policies: std::option::Option<std::vec::Vec<crate::types::PolicyGrantingServiceAccess>>,
}
impl ListPoliciesGrantingServiceAccessEntry {
    /// <p>The namespace of the service that was accessed.</p>
    /// <p>To learn the service namespace of a service, see <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/reference_policies_actions-resources-contextkeys.html">Actions, resources, and condition keys for Amazon Web Services services</a> in the <i>Service Authorization Reference</i>. Choose the name of the service to view details for that service. In the first paragraph, find the service prefix. For example, <code>(service prefix: a4b)</code>. For more information about service namespaces, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">Amazon Web Services service namespaces</a> in the&nbsp;<i>Amazon Web Services General Reference</i>.</p>
    pub fn service_namespace(&self) -> std::option::Option<&str> {
        self.service_namespace.as_deref()
    }
    /// <p>The&nbsp;<code>PoliciesGrantingServiceAccess</code> object that contains details about the policy.</p>
    pub fn policies(&self) -> std::option::Option<&[crate::types::PolicyGrantingServiceAccess]> {
        self.policies.as_deref()
    }
}
impl ListPoliciesGrantingServiceAccessEntry {
    /// Creates a new builder-style object to manufacture [`ListPoliciesGrantingServiceAccessEntry`](crate::types::ListPoliciesGrantingServiceAccessEntry).
    pub fn builder() -> crate::types::builders::ListPoliciesGrantingServiceAccessEntryBuilder {
        crate::types::builders::ListPoliciesGrantingServiceAccessEntryBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ListPoliciesGrantingServiceAccessEntry;
/// A builder for [`ListPoliciesGrantingServiceAccessEntry`](crate::types::ListPoliciesGrantingServiceAccessEntry).
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
pub struct ListPoliciesGrantingServiceAccessEntryBuilder {
    pub(crate) service_namespace: std::option::Option<std::string::String>,
    pub(crate) policies:
        std::option::Option<std::vec::Vec<crate::types::PolicyGrantingServiceAccess>>,
}
impl ListPoliciesGrantingServiceAccessEntryBuilder {
    /// <p>The namespace of the service that was accessed.</p>
    /// <p>To learn the service namespace of a service, see <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/reference_policies_actions-resources-contextkeys.html">Actions, resources, and condition keys for Amazon Web Services services</a> in the <i>Service Authorization Reference</i>. Choose the name of the service to view details for that service. In the first paragraph, find the service prefix. For example, <code>(service prefix: a4b)</code>. For more information about service namespaces, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">Amazon Web Services service namespaces</a> in the&nbsp;<i>Amazon Web Services General Reference</i>.</p>
    pub fn service_namespace(mut self, input: impl Into<std::string::String>) -> Self {
        self.service_namespace = Some(input.into());
        self
    }
    /// <p>The namespace of the service that was accessed.</p>
    /// <p>To learn the service namespace of a service, see <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/reference_policies_actions-resources-contextkeys.html">Actions, resources, and condition keys for Amazon Web Services services</a> in the <i>Service Authorization Reference</i>. Choose the name of the service to view details for that service. In the first paragraph, find the service prefix. For example, <code>(service prefix: a4b)</code>. For more information about service namespaces, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">Amazon Web Services service namespaces</a> in the&nbsp;<i>Amazon Web Services General Reference</i>.</p>
    pub fn set_service_namespace(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.service_namespace = input;
        self
    }
    /// Appends an item to `policies`.
    ///
    /// To override the contents of this collection use [`set_policies`](Self::set_policies).
    ///
    /// <p>The&nbsp;<code>PoliciesGrantingServiceAccess</code> object that contains details about the policy.</p>
    pub fn policies(mut self, input: crate::types::PolicyGrantingServiceAccess) -> Self {
        let mut v = self.policies.unwrap_or_default();
        v.push(input);
        self.policies = Some(v);
        self
    }
    /// <p>The&nbsp;<code>PoliciesGrantingServiceAccess</code> object that contains details about the policy.</p>
    pub fn set_policies(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::PolicyGrantingServiceAccess>>,
    ) -> Self {
        self.policies = input;
        self
    }
    /// Consumes the builder and constructs a [`ListPoliciesGrantingServiceAccessEntry`](crate::types::ListPoliciesGrantingServiceAccessEntry).
    pub fn build(self) -> crate::types::ListPoliciesGrantingServiceAccessEntry {
        crate::types::ListPoliciesGrantingServiceAccessEntry {
            service_namespace: self.service_namespace,
            policies: self.policies,
        }
    }
}