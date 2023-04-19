// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `LanguageCode`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let languagecode = unimplemented!();
/// match languagecode {
///     LanguageCode::ArAe => { /* ... */ },
///     LanguageCode::Arb => { /* ... */ },
///     LanguageCode::CaEs => { /* ... */ },
///     LanguageCode::CmnCn => { /* ... */ },
///     LanguageCode::CyGb => { /* ... */ },
///     LanguageCode::DaDk => { /* ... */ },
///     LanguageCode::DeAt => { /* ... */ },
///     LanguageCode::DeDe => { /* ... */ },
///     LanguageCode::EnAu => { /* ... */ },
///     LanguageCode::EnGb => { /* ... */ },
///     LanguageCode::EnGbWls => { /* ... */ },
///     LanguageCode::EnIn => { /* ... */ },
///     LanguageCode::EnNz => { /* ... */ },
///     LanguageCode::EnUs => { /* ... */ },
///     LanguageCode::EnZa => { /* ... */ },
///     LanguageCode::EsEs => { /* ... */ },
///     LanguageCode::EsMx => { /* ... */ },
///     LanguageCode::EsUs => { /* ... */ },
///     LanguageCode::FiFi => { /* ... */ },
///     LanguageCode::FrCa => { /* ... */ },
///     LanguageCode::FrFr => { /* ... */ },
///     LanguageCode::HiIn => { /* ... */ },
///     LanguageCode::IsIs => { /* ... */ },
///     LanguageCode::ItIt => { /* ... */ },
///     LanguageCode::JaJp => { /* ... */ },
///     LanguageCode::KoKr => { /* ... */ },
///     LanguageCode::NbNo => { /* ... */ },
///     LanguageCode::NlNl => { /* ... */ },
///     LanguageCode::PlPl => { /* ... */ },
///     LanguageCode::PtBr => { /* ... */ },
///     LanguageCode::PtPt => { /* ... */ },
///     LanguageCode::RoRo => { /* ... */ },
///     LanguageCode::RuRu => { /* ... */ },
///     LanguageCode::SvSe => { /* ... */ },
///     LanguageCode::TrTr => { /* ... */ },
///     LanguageCode::YueCn => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `languagecode` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `LanguageCode::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `LanguageCode::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `LanguageCode::NewFeature` is defined.
/// Specifically, when `languagecode` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `LanguageCode::NewFeature` also yielding `"NewFeature"`.
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
pub enum LanguageCode {
    #[allow(missing_docs)] // documentation missing in model
    ArAe,
    #[allow(missing_docs)] // documentation missing in model
    Arb,
    #[allow(missing_docs)] // documentation missing in model
    CaEs,
    #[allow(missing_docs)] // documentation missing in model
    CmnCn,
    #[allow(missing_docs)] // documentation missing in model
    CyGb,
    #[allow(missing_docs)] // documentation missing in model
    DaDk,
    #[allow(missing_docs)] // documentation missing in model
    DeAt,
    #[allow(missing_docs)] // documentation missing in model
    DeDe,
    #[allow(missing_docs)] // documentation missing in model
    EnAu,
    #[allow(missing_docs)] // documentation missing in model
    EnGb,
    #[allow(missing_docs)] // documentation missing in model
    EnGbWls,
    #[allow(missing_docs)] // documentation missing in model
    EnIn,
    #[allow(missing_docs)] // documentation missing in model
    EnNz,
    #[allow(missing_docs)] // documentation missing in model
    EnUs,
    #[allow(missing_docs)] // documentation missing in model
    EnZa,
    #[allow(missing_docs)] // documentation missing in model
    EsEs,
    #[allow(missing_docs)] // documentation missing in model
    EsMx,
    #[allow(missing_docs)] // documentation missing in model
    EsUs,
    #[allow(missing_docs)] // documentation missing in model
    FiFi,
    #[allow(missing_docs)] // documentation missing in model
    FrCa,
    #[allow(missing_docs)] // documentation missing in model
    FrFr,
    #[allow(missing_docs)] // documentation missing in model
    HiIn,
    #[allow(missing_docs)] // documentation missing in model
    IsIs,
    #[allow(missing_docs)] // documentation missing in model
    ItIt,
    #[allow(missing_docs)] // documentation missing in model
    JaJp,
    #[allow(missing_docs)] // documentation missing in model
    KoKr,
    #[allow(missing_docs)] // documentation missing in model
    NbNo,
    #[allow(missing_docs)] // documentation missing in model
    NlNl,
    #[allow(missing_docs)] // documentation missing in model
    PlPl,
    #[allow(missing_docs)] // documentation missing in model
    PtBr,
    #[allow(missing_docs)] // documentation missing in model
    PtPt,
    #[allow(missing_docs)] // documentation missing in model
    RoRo,
    #[allow(missing_docs)] // documentation missing in model
    RuRu,
    #[allow(missing_docs)] // documentation missing in model
    SvSe,
    #[allow(missing_docs)] // documentation missing in model
    TrTr,
    #[allow(missing_docs)] // documentation missing in model
    YueCn,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::primitives::UnknownVariantValue),
}
impl std::convert::From<&str> for LanguageCode {
    fn from(s: &str) -> Self {
        match s {
            "ar-AE" => LanguageCode::ArAe,
            "arb" => LanguageCode::Arb,
            "ca-ES" => LanguageCode::CaEs,
            "cmn-CN" => LanguageCode::CmnCn,
            "cy-GB" => LanguageCode::CyGb,
            "da-DK" => LanguageCode::DaDk,
            "de-AT" => LanguageCode::DeAt,
            "de-DE" => LanguageCode::DeDe,
            "en-AU" => LanguageCode::EnAu,
            "en-GB" => LanguageCode::EnGb,
            "en-GB-WLS" => LanguageCode::EnGbWls,
            "en-IN" => LanguageCode::EnIn,
            "en-NZ" => LanguageCode::EnNz,
            "en-US" => LanguageCode::EnUs,
            "en-ZA" => LanguageCode::EnZa,
            "es-ES" => LanguageCode::EsEs,
            "es-MX" => LanguageCode::EsMx,
            "es-US" => LanguageCode::EsUs,
            "fi-FI" => LanguageCode::FiFi,
            "fr-CA" => LanguageCode::FrCa,
            "fr-FR" => LanguageCode::FrFr,
            "hi-IN" => LanguageCode::HiIn,
            "is-IS" => LanguageCode::IsIs,
            "it-IT" => LanguageCode::ItIt,
            "ja-JP" => LanguageCode::JaJp,
            "ko-KR" => LanguageCode::KoKr,
            "nb-NO" => LanguageCode::NbNo,
            "nl-NL" => LanguageCode::NlNl,
            "pl-PL" => LanguageCode::PlPl,
            "pt-BR" => LanguageCode::PtBr,
            "pt-PT" => LanguageCode::PtPt,
            "ro-RO" => LanguageCode::RoRo,
            "ru-RU" => LanguageCode::RuRu,
            "sv-SE" => LanguageCode::SvSe,
            "tr-TR" => LanguageCode::TrTr,
            "yue-CN" => LanguageCode::YueCn,
            other => {
                LanguageCode::Unknown(crate::primitives::UnknownVariantValue(other.to_owned()))
            }
        }
    }
}
impl std::str::FromStr for LanguageCode {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(LanguageCode::from(s))
    }
}
impl LanguageCode {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            LanguageCode::ArAe => "ar-AE",
            LanguageCode::Arb => "arb",
            LanguageCode::CaEs => "ca-ES",
            LanguageCode::CmnCn => "cmn-CN",
            LanguageCode::CyGb => "cy-GB",
            LanguageCode::DaDk => "da-DK",
            LanguageCode::DeAt => "de-AT",
            LanguageCode::DeDe => "de-DE",
            LanguageCode::EnAu => "en-AU",
            LanguageCode::EnGb => "en-GB",
            LanguageCode::EnGbWls => "en-GB-WLS",
            LanguageCode::EnIn => "en-IN",
            LanguageCode::EnNz => "en-NZ",
            LanguageCode::EnUs => "en-US",
            LanguageCode::EnZa => "en-ZA",
            LanguageCode::EsEs => "es-ES",
            LanguageCode::EsMx => "es-MX",
            LanguageCode::EsUs => "es-US",
            LanguageCode::FiFi => "fi-FI",
            LanguageCode::FrCa => "fr-CA",
            LanguageCode::FrFr => "fr-FR",
            LanguageCode::HiIn => "hi-IN",
            LanguageCode::IsIs => "is-IS",
            LanguageCode::ItIt => "it-IT",
            LanguageCode::JaJp => "ja-JP",
            LanguageCode::KoKr => "ko-KR",
            LanguageCode::NbNo => "nb-NO",
            LanguageCode::NlNl => "nl-NL",
            LanguageCode::PlPl => "pl-PL",
            LanguageCode::PtBr => "pt-BR",
            LanguageCode::PtPt => "pt-PT",
            LanguageCode::RoRo => "ro-RO",
            LanguageCode::RuRu => "ru-RU",
            LanguageCode::SvSe => "sv-SE",
            LanguageCode::TrTr => "tr-TR",
            LanguageCode::YueCn => "yue-CN",
            LanguageCode::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "ar-AE",
            "arb",
            "ca-ES",
            "cmn-CN",
            "cy-GB",
            "da-DK",
            "de-AT",
            "de-DE",
            "en-AU",
            "en-GB",
            "en-GB-WLS",
            "en-IN",
            "en-NZ",
            "en-US",
            "en-ZA",
            "es-ES",
            "es-MX",
            "es-US",
            "fi-FI",
            "fr-CA",
            "fr-FR",
            "hi-IN",
            "is-IS",
            "it-IT",
            "ja-JP",
            "ko-KR",
            "nb-NO",
            "nl-NL",
            "pl-PL",
            "pt-BR",
            "pt-PT",
            "ro-RO",
            "ru-RU",
            "sv-SE",
            "tr-TR",
            "yue-CN",
        ]
    }
}
impl AsRef<str> for LanguageCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
