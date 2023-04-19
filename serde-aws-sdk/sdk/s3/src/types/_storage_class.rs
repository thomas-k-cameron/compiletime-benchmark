// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `StorageClass`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let storageclass = unimplemented!();
/// match storageclass {
///     StorageClass::DeepArchive => { /* ... */ },
///     StorageClass::Glacier => { /* ... */ },
///     StorageClass::GlacierIr => { /* ... */ },
///     StorageClass::IntelligentTiering => { /* ... */ },
///     StorageClass::OnezoneIa => { /* ... */ },
///     StorageClass::Outposts => { /* ... */ },
///     StorageClass::ReducedRedundancy => { /* ... */ },
///     StorageClass::Standard => { /* ... */ },
///     StorageClass::StandardIa => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `storageclass` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `StorageClass::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `StorageClass::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `StorageClass::NewFeature` is defined.
/// Specifically, when `storageclass` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `StorageClass::NewFeature` also yielding `"NewFeature"`.
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
pub enum StorageClass {
    #[allow(missing_docs)] // documentation missing in model
    DeepArchive,
    #[allow(missing_docs)] // documentation missing in model
    Glacier,
    #[allow(missing_docs)] // documentation missing in model
    GlacierIr,
    #[allow(missing_docs)] // documentation missing in model
    IntelligentTiering,
    #[allow(missing_docs)] // documentation missing in model
    OnezoneIa,
    #[allow(missing_docs)] // documentation missing in model
    Outposts,
    #[allow(missing_docs)] // documentation missing in model
    ReducedRedundancy,
    #[allow(missing_docs)] // documentation missing in model
    Standard,
    #[allow(missing_docs)] // documentation missing in model
    StandardIa,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::primitives::UnknownVariantValue),
}
impl std::convert::From<&str> for StorageClass {
    fn from(s: &str) -> Self {
        match s {
            "DEEP_ARCHIVE" => StorageClass::DeepArchive,
            "GLACIER" => StorageClass::Glacier,
            "GLACIER_IR" => StorageClass::GlacierIr,
            "INTELLIGENT_TIERING" => StorageClass::IntelligentTiering,
            "ONEZONE_IA" => StorageClass::OnezoneIa,
            "OUTPOSTS" => StorageClass::Outposts,
            "REDUCED_REDUNDANCY" => StorageClass::ReducedRedundancy,
            "STANDARD" => StorageClass::Standard,
            "STANDARD_IA" => StorageClass::StandardIa,
            other => {
                StorageClass::Unknown(crate::primitives::UnknownVariantValue(other.to_owned()))
            }
        }
    }
}
impl std::str::FromStr for StorageClass {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(StorageClass::from(s))
    }
}
impl StorageClass {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            StorageClass::DeepArchive => "DEEP_ARCHIVE",
            StorageClass::Glacier => "GLACIER",
            StorageClass::GlacierIr => "GLACIER_IR",
            StorageClass::IntelligentTiering => "INTELLIGENT_TIERING",
            StorageClass::OnezoneIa => "ONEZONE_IA",
            StorageClass::Outposts => "OUTPOSTS",
            StorageClass::ReducedRedundancy => "REDUCED_REDUNDANCY",
            StorageClass::Standard => "STANDARD",
            StorageClass::StandardIa => "STANDARD_IA",
            StorageClass::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "DEEP_ARCHIVE",
            "GLACIER",
            "GLACIER_IR",
            "INTELLIGENT_TIERING",
            "ONEZONE_IA",
            "OUTPOSTS",
            "REDUCED_REDUNDANCY",
            "STANDARD",
            "STANDARD_IA",
        ]
    }
}
impl AsRef<str> for StorageClass {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}