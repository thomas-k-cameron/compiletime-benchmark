// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `RrType`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let rrtype = unimplemented!();
/// match rrtype {
///     RrType::A => { /* ... */ },
///     RrType::Aaaa => { /* ... */ },
///     RrType::Caa => { /* ... */ },
///     RrType::Cname => { /* ... */ },
///     RrType::Ds => { /* ... */ },
///     RrType::Mx => { /* ... */ },
///     RrType::Naptr => { /* ... */ },
///     RrType::Ns => { /* ... */ },
///     RrType::Ptr => { /* ... */ },
///     RrType::Soa => { /* ... */ },
///     RrType::Spf => { /* ... */ },
///     RrType::Srv => { /* ... */ },
///     RrType::Txt => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `rrtype` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `RrType::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `RrType::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `RrType::NewFeature` is defined.
/// Specifically, when `rrtype` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `RrType::NewFeature` also yielding `"NewFeature"`.
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
pub enum RrType {
    #[allow(missing_docs)] // documentation missing in model
    A,
    #[allow(missing_docs)] // documentation missing in model
    Aaaa,
    #[allow(missing_docs)] // documentation missing in model
    Caa,
    #[allow(missing_docs)] // documentation missing in model
    Cname,
    #[allow(missing_docs)] // documentation missing in model
    Ds,
    #[allow(missing_docs)] // documentation missing in model
    Mx,
    #[allow(missing_docs)] // documentation missing in model
    Naptr,
    #[allow(missing_docs)] // documentation missing in model
    Ns,
    #[allow(missing_docs)] // documentation missing in model
    Ptr,
    #[allow(missing_docs)] // documentation missing in model
    Soa,
    #[allow(missing_docs)] // documentation missing in model
    Spf,
    #[allow(missing_docs)] // documentation missing in model
    Srv,
    #[allow(missing_docs)] // documentation missing in model
    Txt,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::primitives::UnknownVariantValue),
}
impl std::convert::From<&str> for RrType {
    fn from(s: &str) -> Self {
        match s {
            "A" => RrType::A,
            "AAAA" => RrType::Aaaa,
            "CAA" => RrType::Caa,
            "CNAME" => RrType::Cname,
            "DS" => RrType::Ds,
            "MX" => RrType::Mx,
            "NAPTR" => RrType::Naptr,
            "NS" => RrType::Ns,
            "PTR" => RrType::Ptr,
            "SOA" => RrType::Soa,
            "SPF" => RrType::Spf,
            "SRV" => RrType::Srv,
            "TXT" => RrType::Txt,
            other => RrType::Unknown(crate::primitives::UnknownVariantValue(other.to_owned())),
        }
    }
}
impl std::str::FromStr for RrType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(RrType::from(s))
    }
}
impl RrType {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            RrType::A => "A",
            RrType::Aaaa => "AAAA",
            RrType::Caa => "CAA",
            RrType::Cname => "CNAME",
            RrType::Ds => "DS",
            RrType::Mx => "MX",
            RrType::Naptr => "NAPTR",
            RrType::Ns => "NS",
            RrType::Ptr => "PTR",
            RrType::Soa => "SOA",
            RrType::Spf => "SPF",
            RrType::Srv => "SRV",
            RrType::Txt => "TXT",
            RrType::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "A", "AAAA", "CAA", "CNAME", "DS", "MX", "NAPTR", "NS", "PTR", "SOA", "SPF", "SRV",
            "TXT",
        ]
    }
}
impl AsRef<str> for RrType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
