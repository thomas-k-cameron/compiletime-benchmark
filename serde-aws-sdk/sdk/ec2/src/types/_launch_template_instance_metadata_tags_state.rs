// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `LaunchTemplateInstanceMetadataTagsState`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let launchtemplateinstancemetadatatagsstate = unimplemented!();
/// match launchtemplateinstancemetadatatagsstate {
///     LaunchTemplateInstanceMetadataTagsState::Disabled => { /* ... */ },
///     LaunchTemplateInstanceMetadataTagsState::Enabled => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `launchtemplateinstancemetadatatagsstate` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `LaunchTemplateInstanceMetadataTagsState::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `LaunchTemplateInstanceMetadataTagsState::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `LaunchTemplateInstanceMetadataTagsState::NewFeature` is defined.
/// Specifically, when `launchtemplateinstancemetadatatagsstate` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `LaunchTemplateInstanceMetadataTagsState::NewFeature` also yielding `"NewFeature"`.
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
pub enum LaunchTemplateInstanceMetadataTagsState {
    #[allow(missing_docs)] // documentation missing in model
    Disabled,
    #[allow(missing_docs)] // documentation missing in model
    Enabled,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::primitives::UnknownVariantValue),
}
impl std::convert::From<&str> for LaunchTemplateInstanceMetadataTagsState {
    fn from(s: &str) -> Self {
        match s {
            "disabled" => LaunchTemplateInstanceMetadataTagsState::Disabled,
            "enabled" => LaunchTemplateInstanceMetadataTagsState::Enabled,
            other => LaunchTemplateInstanceMetadataTagsState::Unknown(
                crate::primitives::UnknownVariantValue(other.to_owned()),
            ),
        }
    }
}
impl std::str::FromStr for LaunchTemplateInstanceMetadataTagsState {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(LaunchTemplateInstanceMetadataTagsState::from(s))
    }
}
impl LaunchTemplateInstanceMetadataTagsState {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            LaunchTemplateInstanceMetadataTagsState::Disabled => "disabled",
            LaunchTemplateInstanceMetadataTagsState::Enabled => "enabled",
            LaunchTemplateInstanceMetadataTagsState::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &["disabled", "enabled"]
    }
}
impl AsRef<str> for LaunchTemplateInstanceMetadataTagsState {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
