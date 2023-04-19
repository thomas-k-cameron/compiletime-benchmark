// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `ByoipCidrState`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let byoipcidrstate = unimplemented!();
/// match byoipcidrstate {
///     ByoipCidrState::Advertised => { /* ... */ },
///     ByoipCidrState::Deprovisioned => { /* ... */ },
///     ByoipCidrState::FailedDeprovision => { /* ... */ },
///     ByoipCidrState::FailedProvision => { /* ... */ },
///     ByoipCidrState::PendingDeprovision => { /* ... */ },
///     ByoipCidrState::PendingProvision => { /* ... */ },
///     ByoipCidrState::Provisioned => { /* ... */ },
///     ByoipCidrState::ProvisionedNotPubliclyAdvertisable => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `byoipcidrstate` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `ByoipCidrState::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `ByoipCidrState::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `ByoipCidrState::NewFeature` is defined.
/// Specifically, when `byoipcidrstate` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `ByoipCidrState::NewFeature` also yielding `"NewFeature"`.
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
pub enum ByoipCidrState {
    #[allow(missing_docs)] // documentation missing in model
    Advertised,
    #[allow(missing_docs)] // documentation missing in model
    Deprovisioned,
    #[allow(missing_docs)] // documentation missing in model
    FailedDeprovision,
    #[allow(missing_docs)] // documentation missing in model
    FailedProvision,
    #[allow(missing_docs)] // documentation missing in model
    PendingDeprovision,
    #[allow(missing_docs)] // documentation missing in model
    PendingProvision,
    #[allow(missing_docs)] // documentation missing in model
    Provisioned,
    #[allow(missing_docs)] // documentation missing in model
    ProvisionedNotPubliclyAdvertisable,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::primitives::UnknownVariantValue),
}
impl std::convert::From<&str> for ByoipCidrState {
    fn from(s: &str) -> Self {
        match s {
            "advertised" => ByoipCidrState::Advertised,
            "deprovisioned" => ByoipCidrState::Deprovisioned,
            "failed-deprovision" => ByoipCidrState::FailedDeprovision,
            "failed-provision" => ByoipCidrState::FailedProvision,
            "pending-deprovision" => ByoipCidrState::PendingDeprovision,
            "pending-provision" => ByoipCidrState::PendingProvision,
            "provisioned" => ByoipCidrState::Provisioned,
            "provisioned-not-publicly-advertisable" => {
                ByoipCidrState::ProvisionedNotPubliclyAdvertisable
            }
            other => {
                ByoipCidrState::Unknown(crate::primitives::UnknownVariantValue(other.to_owned()))
            }
        }
    }
}
impl std::str::FromStr for ByoipCidrState {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ByoipCidrState::from(s))
    }
}
impl ByoipCidrState {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ByoipCidrState::Advertised => "advertised",
            ByoipCidrState::Deprovisioned => "deprovisioned",
            ByoipCidrState::FailedDeprovision => "failed-deprovision",
            ByoipCidrState::FailedProvision => "failed-provision",
            ByoipCidrState::PendingDeprovision => "pending-deprovision",
            ByoipCidrState::PendingProvision => "pending-provision",
            ByoipCidrState::Provisioned => "provisioned",
            ByoipCidrState::ProvisionedNotPubliclyAdvertisable => {
                "provisioned-not-publicly-advertisable"
            }
            ByoipCidrState::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "advertised",
            "deprovisioned",
            "failed-deprovision",
            "failed-provision",
            "pending-deprovision",
            "pending-provision",
            "provisioned",
            "provisioned-not-publicly-advertisable",
        ]
    }
}
impl AsRef<str> for ByoipCidrState {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
