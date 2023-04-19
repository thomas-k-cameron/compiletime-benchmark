// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object that contains details about when the IAM entities (users or roles) were last used in an attempt to access the specified Amazon Web Services service.</p>
/// <p>This data type is a response element in the <code>GetServiceLastAccessedDetailsWithEntities</code> operation.</p>
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
pub struct EntityDetails {
    /// <p>The&nbsp;<code>EntityInfo</code> object that contains details about the entity (user or role).</p>
    #[doc(hidden)]
    pub entity_info: std::option::Option<crate::types::EntityInfo>,
    /// <p>The date and time, in&nbsp;<a href="http://www.iso.org/iso/iso8601">ISO 8601 date-time format</a>, when the authenticated entity last attempted to access Amazon Web Services. Amazon Web Services does not report unauthenticated requests.</p>
    /// <p>This field is null if no IAM entities attempted to access the service within the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_access-advisor.html#service-last-accessed-reporting-period">tracking period</a>.</p>
    #[doc(hidden)]
    pub last_authenticated: std::option::Option<aws_smithy_types::DateTime>,
}
impl EntityDetails {
    /// <p>The&nbsp;<code>EntityInfo</code> object that contains details about the entity (user or role).</p>
    pub fn entity_info(&self) -> std::option::Option<&crate::types::EntityInfo> {
        self.entity_info.as_ref()
    }
    /// <p>The date and time, in&nbsp;<a href="http://www.iso.org/iso/iso8601">ISO 8601 date-time format</a>, when the authenticated entity last attempted to access Amazon Web Services. Amazon Web Services does not report unauthenticated requests.</p>
    /// <p>This field is null if no IAM entities attempted to access the service within the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_access-advisor.html#service-last-accessed-reporting-period">tracking period</a>.</p>
    pub fn last_authenticated(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.last_authenticated.as_ref()
    }
}
impl EntityDetails {
    /// Creates a new builder-style object to manufacture [`EntityDetails`](crate::types::EntityDetails).
    pub fn builder() -> crate::types::builders::EntityDetailsBuilder {
        crate::types::builders::EntityDetailsBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::EntityDetails;
/// A builder for [`EntityDetails`](crate::types::EntityDetails).
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
pub struct EntityDetailsBuilder {
    pub(crate) entity_info: std::option::Option<crate::types::EntityInfo>,
    pub(crate) last_authenticated: std::option::Option<aws_smithy_types::DateTime>,
}
impl EntityDetailsBuilder {
    /// <p>The&nbsp;<code>EntityInfo</code> object that contains details about the entity (user or role).</p>
    pub fn entity_info(mut self, input: crate::types::EntityInfo) -> Self {
        self.entity_info = Some(input);
        self
    }
    /// <p>The&nbsp;<code>EntityInfo</code> object that contains details about the entity (user or role).</p>
    pub fn set_entity_info(mut self, input: std::option::Option<crate::types::EntityInfo>) -> Self {
        self.entity_info = input;
        self
    }
    /// <p>The date and time, in&nbsp;<a href="http://www.iso.org/iso/iso8601">ISO 8601 date-time format</a>, when the authenticated entity last attempted to access Amazon Web Services. Amazon Web Services does not report unauthenticated requests.</p>
    /// <p>This field is null if no IAM entities attempted to access the service within the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_access-advisor.html#service-last-accessed-reporting-period">tracking period</a>.</p>
    pub fn last_authenticated(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.last_authenticated = Some(input);
        self
    }
    /// <p>The date and time, in&nbsp;<a href="http://www.iso.org/iso/iso8601">ISO 8601 date-time format</a>, when the authenticated entity last attempted to access Amazon Web Services. Amazon Web Services does not report unauthenticated requests.</p>
    /// <p>This field is null if no IAM entities attempted to access the service within the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_access-advisor.html#service-last-accessed-reporting-period">tracking period</a>.</p>
    pub fn set_last_authenticated(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.last_authenticated = input;
        self
    }
    /// Consumes the builder and constructs a [`EntityDetails`](crate::types::EntityDetails).
    pub fn build(self) -> crate::types::EntityDetails {
        crate::types::EntityDetails {
            entity_info: self.entity_info,
            last_authenticated: self.last_authenticated,
        }
    }
}