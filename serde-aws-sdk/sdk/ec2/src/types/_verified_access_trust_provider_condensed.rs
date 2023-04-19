// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Condensed information about a trust provider.</p>
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
pub struct VerifiedAccessTrustProviderCondensed {
    /// <p>The ID of the trust provider.</p>
    #[doc(hidden)]
    pub verified_access_trust_provider_id: std::option::Option<std::string::String>,
    /// <p>The description of trust provider.</p>
    #[doc(hidden)]
    pub description: std::option::Option<std::string::String>,
    /// <p>The type of trust provider (user- or device-based).</p>
    #[doc(hidden)]
    pub trust_provider_type: std::option::Option<crate::types::TrustProviderType>,
    /// <p>The type of user-based trust provider.</p>
    #[doc(hidden)]
    pub user_trust_provider_type: std::option::Option<crate::types::UserTrustProviderType>,
    /// <p>The type of device-based trust provider.</p>
    #[doc(hidden)]
    pub device_trust_provider_type: std::option::Option<crate::types::DeviceTrustProviderType>,
}
impl VerifiedAccessTrustProviderCondensed {
    /// <p>The ID of the trust provider.</p>
    pub fn verified_access_trust_provider_id(&self) -> std::option::Option<&str> {
        self.verified_access_trust_provider_id.as_deref()
    }
    /// <p>The description of trust provider.</p>
    pub fn description(&self) -> std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The type of trust provider (user- or device-based).</p>
    pub fn trust_provider_type(&self) -> std::option::Option<&crate::types::TrustProviderType> {
        self.trust_provider_type.as_ref()
    }
    /// <p>The type of user-based trust provider.</p>
    pub fn user_trust_provider_type(
        &self,
    ) -> std::option::Option<&crate::types::UserTrustProviderType> {
        self.user_trust_provider_type.as_ref()
    }
    /// <p>The type of device-based trust provider.</p>
    pub fn device_trust_provider_type(
        &self,
    ) -> std::option::Option<&crate::types::DeviceTrustProviderType> {
        self.device_trust_provider_type.as_ref()
    }
}
impl VerifiedAccessTrustProviderCondensed {
    /// Creates a new builder-style object to manufacture [`VerifiedAccessTrustProviderCondensed`](crate::types::VerifiedAccessTrustProviderCondensed).
    pub fn builder() -> crate::types::builders::VerifiedAccessTrustProviderCondensedBuilder {
        crate::types::builders::VerifiedAccessTrustProviderCondensedBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::VerifiedAccessTrustProviderCondensed;
/// A builder for [`VerifiedAccessTrustProviderCondensed`](crate::types::VerifiedAccessTrustProviderCondensed).
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
pub struct VerifiedAccessTrustProviderCondensedBuilder {
    pub(crate) verified_access_trust_provider_id: std::option::Option<std::string::String>,
    pub(crate) description: std::option::Option<std::string::String>,
    pub(crate) trust_provider_type: std::option::Option<crate::types::TrustProviderType>,
    pub(crate) user_trust_provider_type: std::option::Option<crate::types::UserTrustProviderType>,
    pub(crate) device_trust_provider_type:
        std::option::Option<crate::types::DeviceTrustProviderType>,
}
impl VerifiedAccessTrustProviderCondensedBuilder {
    /// <p>The ID of the trust provider.</p>
    pub fn verified_access_trust_provider_id(
        mut self,
        input: impl Into<std::string::String>,
    ) -> Self {
        self.verified_access_trust_provider_id = Some(input.into());
        self
    }
    /// <p>The ID of the trust provider.</p>
    pub fn set_verified_access_trust_provider_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.verified_access_trust_provider_id = input;
        self
    }
    /// <p>The description of trust provider.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.description = Some(input.into());
        self
    }
    /// <p>The description of trust provider.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The type of trust provider (user- or device-based).</p>
    pub fn trust_provider_type(mut self, input: crate::types::TrustProviderType) -> Self {
        self.trust_provider_type = Some(input);
        self
    }
    /// <p>The type of trust provider (user- or device-based).</p>
    pub fn set_trust_provider_type(
        mut self,
        input: std::option::Option<crate::types::TrustProviderType>,
    ) -> Self {
        self.trust_provider_type = input;
        self
    }
    /// <p>The type of user-based trust provider.</p>
    pub fn user_trust_provider_type(mut self, input: crate::types::UserTrustProviderType) -> Self {
        self.user_trust_provider_type = Some(input);
        self
    }
    /// <p>The type of user-based trust provider.</p>
    pub fn set_user_trust_provider_type(
        mut self,
        input: std::option::Option<crate::types::UserTrustProviderType>,
    ) -> Self {
        self.user_trust_provider_type = input;
        self
    }
    /// <p>The type of device-based trust provider.</p>
    pub fn device_trust_provider_type(
        mut self,
        input: crate::types::DeviceTrustProviderType,
    ) -> Self {
        self.device_trust_provider_type = Some(input);
        self
    }
    /// <p>The type of device-based trust provider.</p>
    pub fn set_device_trust_provider_type(
        mut self,
        input: std::option::Option<crate::types::DeviceTrustProviderType>,
    ) -> Self {
        self.device_trust_provider_type = input;
        self
    }
    /// Consumes the builder and constructs a [`VerifiedAccessTrustProviderCondensed`](crate::types::VerifiedAccessTrustProviderCondensed).
    pub fn build(self) -> crate::types::VerifiedAccessTrustProviderCondensed {
        crate::types::VerifiedAccessTrustProviderCondensed {
            verified_access_trust_provider_id: self.verified_access_trust_provider_id,
            description: self.description,
            trust_provider_type: self.trust_provider_type,
            user_trust_provider_type: self.user_trust_provider_type,
            device_trust_provider_type: self.device_trust_provider_type,
        }
    }
}
