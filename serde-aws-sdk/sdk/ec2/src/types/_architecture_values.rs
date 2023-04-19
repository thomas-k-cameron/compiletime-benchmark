// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `ArchitectureValues`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let architecturevalues = unimplemented!();
/// match architecturevalues {
///     ArchitectureValues::Arm64 => { /* ... */ },
///     ArchitectureValues::Arm64Mac => { /* ... */ },
///     ArchitectureValues::I386 => { /* ... */ },
///     ArchitectureValues::X8664 => { /* ... */ },
///     ArchitectureValues::X8664Mac => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `architecturevalues` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `ArchitectureValues::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `ArchitectureValues::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `ArchitectureValues::NewFeature` is defined.
/// Specifically, when `architecturevalues` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `ArchitectureValues::NewFeature` also yielding `"NewFeature"`.
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
pub enum ArchitectureValues {
    #[allow(missing_docs)] // documentation missing in model
    Arm64,
    #[allow(missing_docs)] // documentation missing in model
    Arm64Mac,
    #[allow(missing_docs)] // documentation missing in model
    I386,
    #[allow(missing_docs)] // documentation missing in model
    X8664,
    #[allow(missing_docs)] // documentation missing in model
    X8664Mac,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::primitives::UnknownVariantValue),
}
impl std::convert::From<&str> for ArchitectureValues {
    fn from(s: &str) -> Self {
        match s {
            "arm64" => ArchitectureValues::Arm64,
            "arm64_mac" => ArchitectureValues::Arm64Mac,
            "i386" => ArchitectureValues::I386,
            "x86_64" => ArchitectureValues::X8664,
            "x86_64_mac" => ArchitectureValues::X8664Mac,
            other => ArchitectureValues::Unknown(crate::primitives::UnknownVariantValue(
                other.to_owned(),
            )),
        }
    }
}
impl std::str::FromStr for ArchitectureValues {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ArchitectureValues::from(s))
    }
}
impl ArchitectureValues {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ArchitectureValues::Arm64 => "arm64",
            ArchitectureValues::Arm64Mac => "arm64_mac",
            ArchitectureValues::I386 => "i386",
            ArchitectureValues::X8664 => "x86_64",
            ArchitectureValues::X8664Mac => "x86_64_mac",
            ArchitectureValues::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &["arm64", "arm64_mac", "i386", "x86_64", "x86_64_mac"]
    }
}
impl AsRef<str> for ArchitectureValues {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
