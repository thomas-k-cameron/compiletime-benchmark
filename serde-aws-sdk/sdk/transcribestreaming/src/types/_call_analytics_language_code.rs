// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `CallAnalyticsLanguageCode`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let callanalyticslanguagecode = unimplemented!();
/// match callanalyticslanguagecode {
///     CallAnalyticsLanguageCode::DeDe => { /* ... */ },
///     CallAnalyticsLanguageCode::EnAu => { /* ... */ },
///     CallAnalyticsLanguageCode::EnGb => { /* ... */ },
///     CallAnalyticsLanguageCode::EnUs => { /* ... */ },
///     CallAnalyticsLanguageCode::EsUs => { /* ... */ },
///     CallAnalyticsLanguageCode::FrCa => { /* ... */ },
///     CallAnalyticsLanguageCode::FrFr => { /* ... */ },
///     CallAnalyticsLanguageCode::ItIt => { /* ... */ },
///     CallAnalyticsLanguageCode::PtBr => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `callanalyticslanguagecode` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `CallAnalyticsLanguageCode::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `CallAnalyticsLanguageCode::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `CallAnalyticsLanguageCode::NewFeature` is defined.
/// Specifically, when `callanalyticslanguagecode` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `CallAnalyticsLanguageCode::NewFeature` also yielding `"NewFeature"`.
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
pub enum CallAnalyticsLanguageCode {
    #[allow(missing_docs)] // documentation missing in model
    DeDe,
    #[allow(missing_docs)] // documentation missing in model
    EnAu,
    #[allow(missing_docs)] // documentation missing in model
    EnGb,
    #[allow(missing_docs)] // documentation missing in model
    EnUs,
    #[allow(missing_docs)] // documentation missing in model
    EsUs,
    #[allow(missing_docs)] // documentation missing in model
    FrCa,
    #[allow(missing_docs)] // documentation missing in model
    FrFr,
    #[allow(missing_docs)] // documentation missing in model
    ItIt,
    #[allow(missing_docs)] // documentation missing in model
    PtBr,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::primitives::UnknownVariantValue),
}
impl std::convert::From<&str> for CallAnalyticsLanguageCode {
    fn from(s: &str) -> Self {
        match s {
            "de-DE" => CallAnalyticsLanguageCode::DeDe,
            "en-AU" => CallAnalyticsLanguageCode::EnAu,
            "en-GB" => CallAnalyticsLanguageCode::EnGb,
            "en-US" => CallAnalyticsLanguageCode::EnUs,
            "es-US" => CallAnalyticsLanguageCode::EsUs,
            "fr-CA" => CallAnalyticsLanguageCode::FrCa,
            "fr-FR" => CallAnalyticsLanguageCode::FrFr,
            "it-IT" => CallAnalyticsLanguageCode::ItIt,
            "pt-BR" => CallAnalyticsLanguageCode::PtBr,
            other => CallAnalyticsLanguageCode::Unknown(crate::primitives::UnknownVariantValue(
                other.to_owned(),
            )),
        }
    }
}
impl std::str::FromStr for CallAnalyticsLanguageCode {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(CallAnalyticsLanguageCode::from(s))
    }
}
impl CallAnalyticsLanguageCode {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            CallAnalyticsLanguageCode::DeDe => "de-DE",
            CallAnalyticsLanguageCode::EnAu => "en-AU",
            CallAnalyticsLanguageCode::EnGb => "en-GB",
            CallAnalyticsLanguageCode::EnUs => "en-US",
            CallAnalyticsLanguageCode::EsUs => "es-US",
            CallAnalyticsLanguageCode::FrCa => "fr-CA",
            CallAnalyticsLanguageCode::FrFr => "fr-FR",
            CallAnalyticsLanguageCode::ItIt => "it-IT",
            CallAnalyticsLanguageCode::PtBr => "pt-BR",
            CallAnalyticsLanguageCode::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "de-DE", "en-AU", "en-GB", "en-US", "es-US", "fr-CA", "fr-FR", "it-IT", "pt-BR",
        ]
    }
}
impl AsRef<str> for CallAnalyticsLanguageCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}