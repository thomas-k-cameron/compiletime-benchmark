// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a principal.</p>
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
pub struct AllowedPrincipal {
    /// <p>The type of principal.</p>
    #[doc(hidden)]
    pub principal_type: std::option::Option<crate::types::PrincipalType>,
    /// <p>The Amazon Resource Name (ARN) of the principal.</p>
    #[doc(hidden)]
    pub principal: std::option::Option<std::string::String>,
    /// <p>The ID of the service permission.</p>
    #[doc(hidden)]
    pub service_permission_id: std::option::Option<std::string::String>,
    /// <p>The tags.</p>
    #[doc(hidden)]
    pub tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    /// <p>The ID of the service.</p>
    #[doc(hidden)]
    pub service_id: std::option::Option<std::string::String>,
}
impl AllowedPrincipal {
    /// <p>The type of principal.</p>
    pub fn principal_type(&self) -> std::option::Option<&crate::types::PrincipalType> {
        self.principal_type.as_ref()
    }
    /// <p>The Amazon Resource Name (ARN) of the principal.</p>
    pub fn principal(&self) -> std::option::Option<&str> {
        self.principal.as_deref()
    }
    /// <p>The ID of the service permission.</p>
    pub fn service_permission_id(&self) -> std::option::Option<&str> {
        self.service_permission_id.as_deref()
    }
    /// <p>The tags.</p>
    pub fn tags(&self) -> std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
    /// <p>The ID of the service.</p>
    pub fn service_id(&self) -> std::option::Option<&str> {
        self.service_id.as_deref()
    }
}
impl AllowedPrincipal {
    /// Creates a new builder-style object to manufacture [`AllowedPrincipal`](crate::types::AllowedPrincipal).
    pub fn builder() -> crate::types::builders::AllowedPrincipalBuilder {
        crate::types::builders::AllowedPrincipalBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::AllowedPrincipal;
/// A builder for [`AllowedPrincipal`](crate::types::AllowedPrincipal).
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
pub struct AllowedPrincipalBuilder {
    pub(crate) principal_type: std::option::Option<crate::types::PrincipalType>,
    pub(crate) principal: std::option::Option<std::string::String>,
    pub(crate) service_permission_id: std::option::Option<std::string::String>,
    pub(crate) tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    pub(crate) service_id: std::option::Option<std::string::String>,
}
impl AllowedPrincipalBuilder {
    /// <p>The type of principal.</p>
    pub fn principal_type(mut self, input: crate::types::PrincipalType) -> Self {
        self.principal_type = Some(input);
        self
    }
    /// <p>The type of principal.</p>
    pub fn set_principal_type(
        mut self,
        input: std::option::Option<crate::types::PrincipalType>,
    ) -> Self {
        self.principal_type = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the principal.</p>
    pub fn principal(mut self, input: impl Into<std::string::String>) -> Self {
        self.principal = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the principal.</p>
    pub fn set_principal(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.principal = input;
        self
    }
    /// <p>The ID of the service permission.</p>
    pub fn service_permission_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.service_permission_id = Some(input.into());
        self
    }
    /// <p>The ID of the service permission.</p>
    pub fn set_service_permission_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.service_permission_id = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = Some(v);
        self
    }
    /// <p>The tags.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// <p>The ID of the service.</p>
    pub fn service_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.service_id = Some(input.into());
        self
    }
    /// <p>The ID of the service.</p>
    pub fn set_service_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.service_id = input;
        self
    }
    /// Consumes the builder and constructs a [`AllowedPrincipal`](crate::types::AllowedPrincipal).
    pub fn build(self) -> crate::types::AllowedPrincipal {
        crate::types::AllowedPrincipal {
            principal_type: self.principal_type,
            principal: self.principal,
            service_permission_id: self.service_permission_id,
            tags: self.tags,
            service_id: self.service_id,
        }
    }
}
