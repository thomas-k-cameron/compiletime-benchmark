// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A container for AwsLambdaTransformation.</p>
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
pub enum ObjectLambdaContentTransformation {
    /// <p>A container for an Lambda function.</p>
    AwsLambda(crate::types::AwsLambdaTransformation),
    /// The `Unknown` variant represents cases where new union variant was received. Consider upgrading the SDK to the latest available version.
    /// An unknown enum variant
    ///
    /// _Note: If you encounter this error, consider upgrading your SDK to the latest version._
    /// The `Unknown` variant represents cases where the server sent a value that wasn't recognized
    /// by the client. This can happen when the server adds new functionality, but the client has not been updated.
    /// To investigate this, consider turning on debug logging to print the raw HTTP response.
    #[non_exhaustive]
    Unknown,
}
impl ObjectLambdaContentTransformation {
    #[allow(irrefutable_let_patterns)]
    /// Tries to convert the enum instance into [`AwsLambda`](crate::types::ObjectLambdaContentTransformation::AwsLambda), extracting the inner [`AwsLambdaTransformation`](crate::types::AwsLambdaTransformation).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_aws_lambda(
        &self,
    ) -> std::result::Result<&crate::types::AwsLambdaTransformation, &Self> {
        if let ObjectLambdaContentTransformation::AwsLambda(val) = &self {
            Ok(val)
        } else {
            Err(self)
        }
    }
    /// Returns true if this is a [`AwsLambda`](crate::types::ObjectLambdaContentTransformation::AwsLambda).
    pub fn is_aws_lambda(&self) -> bool {
        self.as_aws_lambda().is_ok()
    }
    /// Returns true if the enum instance is the `Unknown` variant.
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}
