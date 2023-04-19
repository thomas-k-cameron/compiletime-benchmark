// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `SettingName`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let settingname = unimplemented!();
/// match settingname {
///     SettingName::AwsvpcTrunking => { /* ... */ },
///     SettingName::ContainerInsights => { /* ... */ },
///     SettingName::ContainerInstanceLongArnFormat => { /* ... */ },
///     SettingName::ServiceLongArnFormat => { /* ... */ },
///     SettingName::TaskLongArnFormat => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `settingname` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `SettingName::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `SettingName::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `SettingName::NewFeature` is defined.
/// Specifically, when `settingname` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `SettingName::NewFeature` also yielding `"NewFeature"`.
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
pub enum SettingName {
    #[allow(missing_docs)] // documentation missing in model
    AwsvpcTrunking,
    #[allow(missing_docs)] // documentation missing in model
    ContainerInsights,
    #[allow(missing_docs)] // documentation missing in model
    ContainerInstanceLongArnFormat,
    #[allow(missing_docs)] // documentation missing in model
    ServiceLongArnFormat,
    #[allow(missing_docs)] // documentation missing in model
    TaskLongArnFormat,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::primitives::UnknownVariantValue),
}
impl std::convert::From<&str> for SettingName {
    fn from(s: &str) -> Self {
        match s {
            "awsvpcTrunking" => SettingName::AwsvpcTrunking,
            "containerInsights" => SettingName::ContainerInsights,
            "containerInstanceLongArnFormat" => SettingName::ContainerInstanceLongArnFormat,
            "serviceLongArnFormat" => SettingName::ServiceLongArnFormat,
            "taskLongArnFormat" => SettingName::TaskLongArnFormat,
            other => SettingName::Unknown(crate::primitives::UnknownVariantValue(other.to_owned())),
        }
    }
}
impl std::str::FromStr for SettingName {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(SettingName::from(s))
    }
}
impl SettingName {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            SettingName::AwsvpcTrunking => "awsvpcTrunking",
            SettingName::ContainerInsights => "containerInsights",
            SettingName::ContainerInstanceLongArnFormat => "containerInstanceLongArnFormat",
            SettingName::ServiceLongArnFormat => "serviceLongArnFormat",
            SettingName::TaskLongArnFormat => "taskLongArnFormat",
            SettingName::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "awsvpcTrunking",
            "containerInsights",
            "containerInstanceLongArnFormat",
            "serviceLongArnFormat",
            "taskLongArnFormat",
        ]
    }
}
impl AsRef<str> for SettingName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
