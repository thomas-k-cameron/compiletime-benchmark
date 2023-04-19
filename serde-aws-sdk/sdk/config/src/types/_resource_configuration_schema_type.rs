// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `ResourceConfigurationSchemaType`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let resourceconfigurationschematype = unimplemented!();
/// match resourceconfigurationschematype {
///     ResourceConfigurationSchemaType::CfnResourceSchema => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `resourceconfigurationschematype` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `ResourceConfigurationSchemaType::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `ResourceConfigurationSchemaType::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `ResourceConfigurationSchemaType::NewFeature` is defined.
/// Specifically, when `resourceconfigurationschematype` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `ResourceConfigurationSchemaType::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
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
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum ResourceConfigurationSchemaType {
    #[allow(missing_docs)] // documentation missing in model
    CfnResourceSchema,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::primitives::UnknownVariantValue),
}
impl std::convert::From<&str> for ResourceConfigurationSchemaType {
    fn from(s: &str) -> Self {
        match s {
            "CFN_RESOURCE_SCHEMA" => ResourceConfigurationSchemaType::CfnResourceSchema,
            other => ResourceConfigurationSchemaType::Unknown(
                crate::primitives::UnknownVariantValue(other.to_owned()),
            ),
        }
    }
}
impl std::str::FromStr for ResourceConfigurationSchemaType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ResourceConfigurationSchemaType::from(s))
    }
}
impl ResourceConfigurationSchemaType {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ResourceConfigurationSchemaType::CfnResourceSchema => "CFN_RESOURCE_SCHEMA",
            ResourceConfigurationSchemaType::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &["CFN_RESOURCE_SCHEMA"]
    }
}
impl AsRef<str> for ResourceConfigurationSchemaType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
