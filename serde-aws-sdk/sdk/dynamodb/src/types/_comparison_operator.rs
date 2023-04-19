// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `ComparisonOperator`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let comparisonoperator = unimplemented!();
/// match comparisonoperator {
///     ComparisonOperator::BeginsWith => { /* ... */ },
///     ComparisonOperator::Between => { /* ... */ },
///     ComparisonOperator::Contains => { /* ... */ },
///     ComparisonOperator::Eq => { /* ... */ },
///     ComparisonOperator::Ge => { /* ... */ },
///     ComparisonOperator::Gt => { /* ... */ },
///     ComparisonOperator::In => { /* ... */ },
///     ComparisonOperator::Le => { /* ... */ },
///     ComparisonOperator::Lt => { /* ... */ },
///     ComparisonOperator::Ne => { /* ... */ },
///     ComparisonOperator::NotContains => { /* ... */ },
///     ComparisonOperator::NotNull => { /* ... */ },
///     ComparisonOperator::Null => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `comparisonoperator` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `ComparisonOperator::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `ComparisonOperator::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `ComparisonOperator::NewFeature` is defined.
/// Specifically, when `comparisonoperator` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `ComparisonOperator::NewFeature` also yielding `"NewFeature"`.
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
pub enum ComparisonOperator {
    #[allow(missing_docs)] // documentation missing in model
    BeginsWith,
    #[allow(missing_docs)] // documentation missing in model
    Between,
    #[allow(missing_docs)] // documentation missing in model
    Contains,
    #[allow(missing_docs)] // documentation missing in model
    Eq,
    #[allow(missing_docs)] // documentation missing in model
    Ge,
    #[allow(missing_docs)] // documentation missing in model
    Gt,
    #[allow(missing_docs)] // documentation missing in model
    In,
    #[allow(missing_docs)] // documentation missing in model
    Le,
    #[allow(missing_docs)] // documentation missing in model
    Lt,
    #[allow(missing_docs)] // documentation missing in model
    Ne,
    #[allow(missing_docs)] // documentation missing in model
    NotContains,
    #[allow(missing_docs)] // documentation missing in model
    NotNull,
    #[allow(missing_docs)] // documentation missing in model
    Null,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::primitives::UnknownVariantValue),
}
impl std::convert::From<&str> for ComparisonOperator {
    fn from(s: &str) -> Self {
        match s {
            "BEGINS_WITH" => ComparisonOperator::BeginsWith,
            "BETWEEN" => ComparisonOperator::Between,
            "CONTAINS" => ComparisonOperator::Contains,
            "EQ" => ComparisonOperator::Eq,
            "GE" => ComparisonOperator::Ge,
            "GT" => ComparisonOperator::Gt,
            "IN" => ComparisonOperator::In,
            "LE" => ComparisonOperator::Le,
            "LT" => ComparisonOperator::Lt,
            "NE" => ComparisonOperator::Ne,
            "NOT_CONTAINS" => ComparisonOperator::NotContains,
            "NOT_NULL" => ComparisonOperator::NotNull,
            "NULL" => ComparisonOperator::Null,
            other => ComparisonOperator::Unknown(crate::primitives::UnknownVariantValue(
                other.to_owned(),
            )),
        }
    }
}
impl std::str::FromStr for ComparisonOperator {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ComparisonOperator::from(s))
    }
}
impl ComparisonOperator {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ComparisonOperator::BeginsWith => "BEGINS_WITH",
            ComparisonOperator::Between => "BETWEEN",
            ComparisonOperator::Contains => "CONTAINS",
            ComparisonOperator::Eq => "EQ",
            ComparisonOperator::Ge => "GE",
            ComparisonOperator::Gt => "GT",
            ComparisonOperator::In => "IN",
            ComparisonOperator::Le => "LE",
            ComparisonOperator::Lt => "LT",
            ComparisonOperator::Ne => "NE",
            ComparisonOperator::NotContains => "NOT_CONTAINS",
            ComparisonOperator::NotNull => "NOT_NULL",
            ComparisonOperator::Null => "NULL",
            ComparisonOperator::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "BEGINS_WITH",
            "BETWEEN",
            "CONTAINS",
            "EQ",
            "GE",
            "GT",
            "IN",
            "LE",
            "LT",
            "NE",
            "NOT_CONTAINS",
            "NOT_NULL",
            "NULL",
        ]
    }
}
impl AsRef<str> for ComparisonOperator {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
