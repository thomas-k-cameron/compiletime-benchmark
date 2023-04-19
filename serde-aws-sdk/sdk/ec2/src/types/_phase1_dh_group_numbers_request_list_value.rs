// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies a Diffie-Hellman group number for the VPN tunnel for phase 1 IKE negotiations.</p>
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct Phase1DhGroupNumbersRequestListValue {
    /// <p>The Diffie-Hellmann group number.</p>
    #[doc(hidden)]
    pub value: std::option::Option<i32>,
}
impl Phase1DhGroupNumbersRequestListValue {
    /// <p>The Diffie-Hellmann group number.</p>
    pub fn value(&self) -> std::option::Option<i32> {
        self.value
    }
}
impl Phase1DhGroupNumbersRequestListValue {
    /// Creates a new builder-style object to manufacture [`Phase1DhGroupNumbersRequestListValue`](crate::types::Phase1DhGroupNumbersRequestListValue).
    pub fn builder() -> crate::types::builders::Phase1DhGroupNumbersRequestListValueBuilder {
        crate::types::builders::Phase1DhGroupNumbersRequestListValueBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::Phase1DhGroupNumbersRequestListValue;
/// A builder for [`Phase1DhGroupNumbersRequestListValue`](crate::types::Phase1DhGroupNumbersRequestListValue).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct Phase1DhGroupNumbersRequestListValueBuilder {
    pub(crate) value: std::option::Option<i32>,
}
impl Phase1DhGroupNumbersRequestListValueBuilder {
    /// <p>The Diffie-Hellmann group number.</p>
    pub fn value(mut self, input: i32) -> Self {
        self.value = Some(input);
        self
    }
    /// <p>The Diffie-Hellmann group number.</p>
    pub fn set_value(mut self, input: std::option::Option<i32>) -> Self {
        self.value = input;
        self
    }
    /// Consumes the builder and constructs a [`Phase1DhGroupNumbersRequestListValue`](crate::types::Phase1DhGroupNumbersRequestListValue).
    pub fn build(self) -> crate::types::Phase1DhGroupNumbersRequestListValue {
        crate::types::Phase1DhGroupNumbersRequestListValue { value: self.value }
    }
}
