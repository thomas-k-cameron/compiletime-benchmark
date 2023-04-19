// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about the grantee.</p>
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
pub struct Grantee {
    /// <p>Type of grantee</p>
    #[doc(hidden)]
    pub r#type: std::option::Option<crate::types::Type>,
    /// <p>Screen name of the grantee.</p>
    #[doc(hidden)]
    pub display_name: std::option::Option<std::string::String>,
    /// <p>URI of the grantee group.</p>
    #[doc(hidden)]
    pub uri: std::option::Option<std::string::String>,
    /// <p>The canonical user ID of the grantee.</p>
    #[doc(hidden)]
    pub id: std::option::Option<std::string::String>,
    /// <p>Email address of the grantee.</p>
    #[doc(hidden)]
    pub email_address: std::option::Option<std::string::String>,
}
impl Grantee {
    /// <p>Type of grantee</p>
    pub fn r#type(&self) -> std::option::Option<&crate::types::Type> {
        self.r#type.as_ref()
    }
    /// <p>Screen name of the grantee.</p>
    pub fn display_name(&self) -> std::option::Option<&str> {
        self.display_name.as_deref()
    }
    /// <p>URI of the grantee group.</p>
    pub fn uri(&self) -> std::option::Option<&str> {
        self.uri.as_deref()
    }
    /// <p>The canonical user ID of the grantee.</p>
    pub fn id(&self) -> std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>Email address of the grantee.</p>
    pub fn email_address(&self) -> std::option::Option<&str> {
        self.email_address.as_deref()
    }
}
impl Grantee {
    /// Creates a new builder-style object to manufacture [`Grantee`](crate::types::Grantee).
    pub fn builder() -> crate::types::builders::GranteeBuilder {
        crate::types::builders::GranteeBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::Grantee;
/// A builder for [`Grantee`](crate::types::Grantee).
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
pub struct GranteeBuilder {
    pub(crate) r#type: std::option::Option<crate::types::Type>,
    pub(crate) display_name: std::option::Option<std::string::String>,
    pub(crate) uri: std::option::Option<std::string::String>,
    pub(crate) id: std::option::Option<std::string::String>,
    pub(crate) email_address: std::option::Option<std::string::String>,
}
impl GranteeBuilder {
    /// <p>Type of grantee</p>
    pub fn r#type(mut self, input: crate::types::Type) -> Self {
        self.r#type = Some(input);
        self
    }
    /// <p>Type of grantee</p>
    pub fn set_type(mut self, input: std::option::Option<crate::types::Type>) -> Self {
        self.r#type = input;
        self
    }
    /// <p>Screen name of the grantee.</p>
    pub fn display_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.display_name = Some(input.into());
        self
    }
    /// <p>Screen name of the grantee.</p>
    pub fn set_display_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.display_name = input;
        self
    }
    /// <p>URI of the grantee group.</p>
    pub fn uri(mut self, input: impl Into<std::string::String>) -> Self {
        self.uri = Some(input.into());
        self
    }
    /// <p>URI of the grantee group.</p>
    pub fn set_uri(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.uri = input;
        self
    }
    /// <p>The canonical user ID of the grantee.</p>
    pub fn id(mut self, input: impl Into<std::string::String>) -> Self {
        self.id = Some(input.into());
        self
    }
    /// <p>The canonical user ID of the grantee.</p>
    pub fn set_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>Email address of the grantee.</p>
    pub fn email_address(mut self, input: impl Into<std::string::String>) -> Self {
        self.email_address = Some(input.into());
        self
    }
    /// <p>Email address of the grantee.</p>
    pub fn set_email_address(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.email_address = input;
        self
    }
    /// Consumes the builder and constructs a [`Grantee`](crate::types::Grantee).
    pub fn build(self) -> crate::types::Grantee {
        crate::types::Grantee {
            r#type: self.r#type,
            display_name: self.display_name,
            uri: self.uri,
            id: self.id,
            email_address: self.email_address,
        }
    }
}
