// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
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
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GetBucketCorsOutput {
    /// <p>A set of origins and methods (cross-origin access that you want to allow). You can add up to 100 rules to the configuration.</p>
    #[doc(hidden)]
    pub cors_rules: std::option::Option<std::vec::Vec<crate::types::CorsRule>>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl GetBucketCorsOutput {
    /// <p>A set of origins and methods (cross-origin access that you want to allow). You can add up to 100 rules to the configuration.</p>
    pub fn cors_rules(&self) -> std::option::Option<&[crate::types::CorsRule]> {
        self.cors_rules.as_deref()
    }
}
impl crate::s3_request_id::RequestIdExt for GetBucketCorsOutput {
    fn extended_request_id(&self) -> Option<&str> {
        self._extended_request_id.as_deref()
    }
}
impl aws_http::request_id::RequestId for GetBucketCorsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetBucketCorsOutput {
    /// Creates a new builder-style object to manufacture [`GetBucketCorsOutput`](crate::operation::get_bucket_cors::GetBucketCorsOutput).
    pub fn builder() -> crate::operation::get_bucket_cors::builders::GetBucketCorsOutputBuilder {
        crate::operation::get_bucket_cors::builders::GetBucketCorsOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_bucket_cors::GetBucketCorsOutput;
/// A builder for [`GetBucketCorsOutput`](crate::operation::get_bucket_cors::GetBucketCorsOutput).
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
pub struct GetBucketCorsOutputBuilder {
    pub(crate) cors_rules: std::option::Option<std::vec::Vec<crate::types::CorsRule>>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl GetBucketCorsOutputBuilder {
    /// Appends an item to `cors_rules`.
    ///
    /// To override the contents of this collection use [`set_cors_rules`](Self::set_cors_rules).
    ///
    /// <p>A set of origins and methods (cross-origin access that you want to allow). You can add up to 100 rules to the configuration.</p>
    pub fn cors_rules(mut self, input: crate::types::CorsRule) -> Self {
        let mut v = self.cors_rules.unwrap_or_default();
        v.push(input);
        self.cors_rules = Some(v);
        self
    }
    /// <p>A set of origins and methods (cross-origin access that you want to allow). You can add up to 100 rules to the configuration.</p>
    pub fn set_cors_rules(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::CorsRule>>,
    ) -> Self {
        self.cors_rules = input;
        self
    }
    pub(crate) fn _extended_request_id(mut self, extended_request_id: impl Into<String>) -> Self {
        self._extended_request_id = Some(extended_request_id.into());
        self
    }

    pub(crate) fn _set_extended_request_id(
        &mut self,
        extended_request_id: Option<String>,
    ) -> &mut Self {
        self._extended_request_id = extended_request_id;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetBucketCorsOutput`](crate::operation::get_bucket_cors::GetBucketCorsOutput).
    pub fn build(self) -> crate::operation::get_bucket_cors::GetBucketCorsOutput {
        crate::operation::get_bucket_cors::GetBucketCorsOutput {
            cors_rules: self.cors_rules,
            _extended_request_id: self._extended_request_id,
            _request_id: self._request_id,
        }
    }
}
