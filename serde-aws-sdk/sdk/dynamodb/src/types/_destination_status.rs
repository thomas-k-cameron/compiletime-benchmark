// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `DestinationStatus`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let destinationstatus = unimplemented!();
/// match destinationstatus {
///     DestinationStatus::Active => { /* ... */ },
///     DestinationStatus::Disabled => { /* ... */ },
///     DestinationStatus::Disabling => { /* ... */ },
///     DestinationStatus::EnableFailed => { /* ... */ },
///     DestinationStatus::Enabling => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `destinationstatus` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `DestinationStatus::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `DestinationStatus::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `DestinationStatus::NewFeature` is defined.
/// Specifically, when `destinationstatus` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `DestinationStatus::NewFeature` also yielding `"NewFeature"`.
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
pub enum DestinationStatus {
    #[allow(missing_docs)] // documentation missing in model
    Active,
    #[allow(missing_docs)] // documentation missing in model
    Disabled,
    #[allow(missing_docs)] // documentation missing in model
    Disabling,
    #[allow(missing_docs)] // documentation missing in model
    EnableFailed,
    #[allow(missing_docs)] // documentation missing in model
    Enabling,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::primitives::UnknownVariantValue),
}
impl std::convert::From<&str> for DestinationStatus {
    fn from(s: &str) -> Self {
        match s {
            "ACTIVE" => DestinationStatus::Active,
            "DISABLED" => DestinationStatus::Disabled,
            "DISABLING" => DestinationStatus::Disabling,
            "ENABLE_FAILED" => DestinationStatus::EnableFailed,
            "ENABLING" => DestinationStatus::Enabling,
            other => {
                DestinationStatus::Unknown(crate::primitives::UnknownVariantValue(other.to_owned()))
            }
        }
    }
}
impl std::str::FromStr for DestinationStatus {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(DestinationStatus::from(s))
    }
}
impl DestinationStatus {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            DestinationStatus::Active => "ACTIVE",
            DestinationStatus::Disabled => "DISABLED",
            DestinationStatus::Disabling => "DISABLING",
            DestinationStatus::EnableFailed => "ENABLE_FAILED",
            DestinationStatus::Enabling => "ENABLING",
            DestinationStatus::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "ACTIVE",
            "DISABLED",
            "DISABLING",
            "ENABLE_FAILED",
            "ENABLING",
        ]
    }
}
impl AsRef<str> for DestinationStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
