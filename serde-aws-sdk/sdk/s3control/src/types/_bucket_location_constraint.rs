// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `BucketLocationConstraint`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let bucketlocationconstraint = unimplemented!();
/// match bucketlocationconstraint {
///     BucketLocationConstraint::Eu => { /* ... */ },
///     BucketLocationConstraint::ApNortheast1 => { /* ... */ },
///     BucketLocationConstraint::ApSouth1 => { /* ... */ },
///     BucketLocationConstraint::ApSoutheast1 => { /* ... */ },
///     BucketLocationConstraint::ApSoutheast2 => { /* ... */ },
///     BucketLocationConstraint::CnNorth1 => { /* ... */ },
///     BucketLocationConstraint::EuCentral1 => { /* ... */ },
///     BucketLocationConstraint::EuWest1 => { /* ... */ },
///     BucketLocationConstraint::SaEast1 => { /* ... */ },
///     BucketLocationConstraint::UsWest1 => { /* ... */ },
///     BucketLocationConstraint::UsWest2 => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `bucketlocationconstraint` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `BucketLocationConstraint::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `BucketLocationConstraint::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `BucketLocationConstraint::NewFeature` is defined.
/// Specifically, when `bucketlocationconstraint` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `BucketLocationConstraint::NewFeature` also yielding `"NewFeature"`.
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
pub enum BucketLocationConstraint {
    #[allow(missing_docs)] // documentation missing in model
    Eu,
    #[allow(missing_docs)] // documentation missing in model
    ApNortheast1,
    #[allow(missing_docs)] // documentation missing in model
    ApSouth1,
    #[allow(missing_docs)] // documentation missing in model
    ApSoutheast1,
    #[allow(missing_docs)] // documentation missing in model
    ApSoutheast2,
    #[allow(missing_docs)] // documentation missing in model
    CnNorth1,
    #[allow(missing_docs)] // documentation missing in model
    EuCentral1,
    #[allow(missing_docs)] // documentation missing in model
    EuWest1,
    #[allow(missing_docs)] // documentation missing in model
    SaEast1,
    #[allow(missing_docs)] // documentation missing in model
    UsWest1,
    #[allow(missing_docs)] // documentation missing in model
    UsWest2,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::primitives::UnknownVariantValue),
}
impl std::convert::From<&str> for BucketLocationConstraint {
    fn from(s: &str) -> Self {
        match s {
            "EU" => BucketLocationConstraint::Eu,
            "ap-northeast-1" => BucketLocationConstraint::ApNortheast1,
            "ap-south-1" => BucketLocationConstraint::ApSouth1,
            "ap-southeast-1" => BucketLocationConstraint::ApSoutheast1,
            "ap-southeast-2" => BucketLocationConstraint::ApSoutheast2,
            "cn-north-1" => BucketLocationConstraint::CnNorth1,
            "eu-central-1" => BucketLocationConstraint::EuCentral1,
            "eu-west-1" => BucketLocationConstraint::EuWest1,
            "sa-east-1" => BucketLocationConstraint::SaEast1,
            "us-west-1" => BucketLocationConstraint::UsWest1,
            "us-west-2" => BucketLocationConstraint::UsWest2,
            other => BucketLocationConstraint::Unknown(crate::primitives::UnknownVariantValue(
                other.to_owned(),
            )),
        }
    }
}
impl std::str::FromStr for BucketLocationConstraint {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(BucketLocationConstraint::from(s))
    }
}
impl BucketLocationConstraint {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            BucketLocationConstraint::Eu => "EU",
            BucketLocationConstraint::ApNortheast1 => "ap-northeast-1",
            BucketLocationConstraint::ApSouth1 => "ap-south-1",
            BucketLocationConstraint::ApSoutheast1 => "ap-southeast-1",
            BucketLocationConstraint::ApSoutheast2 => "ap-southeast-2",
            BucketLocationConstraint::CnNorth1 => "cn-north-1",
            BucketLocationConstraint::EuCentral1 => "eu-central-1",
            BucketLocationConstraint::EuWest1 => "eu-west-1",
            BucketLocationConstraint::SaEast1 => "sa-east-1",
            BucketLocationConstraint::UsWest1 => "us-west-1",
            BucketLocationConstraint::UsWest2 => "us-west-2",
            BucketLocationConstraint::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "EU",
            "ap-northeast-1",
            "ap-south-1",
            "ap-southeast-1",
            "ap-southeast-2",
            "cn-north-1",
            "eu-central-1",
            "eu-west-1",
            "sa-east-1",
            "us-west-1",
            "us-west-2",
        ]
    }
}
impl AsRef<str> for BucketLocationConstraint {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
