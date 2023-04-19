// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `GrantOperation`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let grantoperation = unimplemented!();
/// match grantoperation {
///     GrantOperation::CreateGrant => { /* ... */ },
///     GrantOperation::Decrypt => { /* ... */ },
///     GrantOperation::DescribeKey => { /* ... */ },
///     GrantOperation::Encrypt => { /* ... */ },
///     GrantOperation::GenerateDataKey => { /* ... */ },
///     GrantOperation::GenerateDataKeyPair => { /* ... */ },
///     GrantOperation::GenerateDataKeyPairWithoutPlaintext => { /* ... */ },
///     GrantOperation::GenerateDataKeyWithoutPlaintext => { /* ... */ },
///     GrantOperation::GenerateMac => { /* ... */ },
///     GrantOperation::GetPublicKey => { /* ... */ },
///     GrantOperation::ReEncryptFrom => { /* ... */ },
///     GrantOperation::ReEncryptTo => { /* ... */ },
///     GrantOperation::RetireGrant => { /* ... */ },
///     GrantOperation::Sign => { /* ... */ },
///     GrantOperation::Verify => { /* ... */ },
///     GrantOperation::VerifyMac => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `grantoperation` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `GrantOperation::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `GrantOperation::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `GrantOperation::NewFeature` is defined.
/// Specifically, when `grantoperation` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `GrantOperation::NewFeature` also yielding `"NewFeature"`.
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
pub enum GrantOperation {
    #[allow(missing_docs)] // documentation missing in model
    CreateGrant,
    #[allow(missing_docs)] // documentation missing in model
    Decrypt,
    #[allow(missing_docs)] // documentation missing in model
    DescribeKey,
    #[allow(missing_docs)] // documentation missing in model
    Encrypt,
    #[allow(missing_docs)] // documentation missing in model
    GenerateDataKey,
    #[allow(missing_docs)] // documentation missing in model
    GenerateDataKeyPair,
    #[allow(missing_docs)] // documentation missing in model
    GenerateDataKeyPairWithoutPlaintext,
    #[allow(missing_docs)] // documentation missing in model
    GenerateDataKeyWithoutPlaintext,
    #[allow(missing_docs)] // documentation missing in model
    GenerateMac,
    #[allow(missing_docs)] // documentation missing in model
    GetPublicKey,
    #[allow(missing_docs)] // documentation missing in model
    ReEncryptFrom,
    #[allow(missing_docs)] // documentation missing in model
    ReEncryptTo,
    #[allow(missing_docs)] // documentation missing in model
    RetireGrant,
    #[allow(missing_docs)] // documentation missing in model
    Sign,
    #[allow(missing_docs)] // documentation missing in model
    Verify,
    #[allow(missing_docs)] // documentation missing in model
    VerifyMac,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::primitives::UnknownVariantValue),
}
impl std::convert::From<&str> for GrantOperation {
    fn from(s: &str) -> Self {
        match s {
            "CreateGrant" => GrantOperation::CreateGrant,
            "Decrypt" => GrantOperation::Decrypt,
            "DescribeKey" => GrantOperation::DescribeKey,
            "Encrypt" => GrantOperation::Encrypt,
            "GenerateDataKey" => GrantOperation::GenerateDataKey,
            "GenerateDataKeyPair" => GrantOperation::GenerateDataKeyPair,
            "GenerateDataKeyPairWithoutPlaintext" => {
                GrantOperation::GenerateDataKeyPairWithoutPlaintext
            }
            "GenerateDataKeyWithoutPlaintext" => GrantOperation::GenerateDataKeyWithoutPlaintext,
            "GenerateMac" => GrantOperation::GenerateMac,
            "GetPublicKey" => GrantOperation::GetPublicKey,
            "ReEncryptFrom" => GrantOperation::ReEncryptFrom,
            "ReEncryptTo" => GrantOperation::ReEncryptTo,
            "RetireGrant" => GrantOperation::RetireGrant,
            "Sign" => GrantOperation::Sign,
            "Verify" => GrantOperation::Verify,
            "VerifyMac" => GrantOperation::VerifyMac,
            other => {
                GrantOperation::Unknown(crate::primitives::UnknownVariantValue(other.to_owned()))
            }
        }
    }
}
impl std::str::FromStr for GrantOperation {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(GrantOperation::from(s))
    }
}
impl GrantOperation {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            GrantOperation::CreateGrant => "CreateGrant",
            GrantOperation::Decrypt => "Decrypt",
            GrantOperation::DescribeKey => "DescribeKey",
            GrantOperation::Encrypt => "Encrypt",
            GrantOperation::GenerateDataKey => "GenerateDataKey",
            GrantOperation::GenerateDataKeyPair => "GenerateDataKeyPair",
            GrantOperation::GenerateDataKeyPairWithoutPlaintext => {
                "GenerateDataKeyPairWithoutPlaintext"
            }
            GrantOperation::GenerateDataKeyWithoutPlaintext => "GenerateDataKeyWithoutPlaintext",
            GrantOperation::GenerateMac => "GenerateMac",
            GrantOperation::GetPublicKey => "GetPublicKey",
            GrantOperation::ReEncryptFrom => "ReEncryptFrom",
            GrantOperation::ReEncryptTo => "ReEncryptTo",
            GrantOperation::RetireGrant => "RetireGrant",
            GrantOperation::Sign => "Sign",
            GrantOperation::Verify => "Verify",
            GrantOperation::VerifyMac => "VerifyMac",
            GrantOperation::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "CreateGrant",
            "Decrypt",
            "DescribeKey",
            "Encrypt",
            "GenerateDataKey",
            "GenerateDataKeyPair",
            "GenerateDataKeyPairWithoutPlaintext",
            "GenerateDataKeyWithoutPlaintext",
            "GenerateMac",
            "GetPublicKey",
            "ReEncryptFrom",
            "ReEncryptTo",
            "RetireGrant",
            "Sign",
            "Verify",
            "VerifyMac",
        ]
    }
}
impl AsRef<str> for GrantOperation {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
