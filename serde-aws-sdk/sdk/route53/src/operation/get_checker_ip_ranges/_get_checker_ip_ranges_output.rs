// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A complex type that contains the <code>CheckerIpRanges</code> element.</p>
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
pub struct GetCheckerIpRangesOutput {
    /// <p>A complex type that contains sorted list of IP ranges in CIDR format for Amazon Route 53 health checkers.</p>
    #[doc(hidden)]
    pub checker_ip_ranges: std::option::Option<std::vec::Vec<std::string::String>>,
    _request_id: Option<String>,
}
impl GetCheckerIpRangesOutput {
    /// <p>A complex type that contains sorted list of IP ranges in CIDR format for Amazon Route 53 health checkers.</p>
    pub fn checker_ip_ranges(&self) -> std::option::Option<&[std::string::String]> {
        self.checker_ip_ranges.as_deref()
    }
}
impl aws_http::request_id::RequestId for GetCheckerIpRangesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetCheckerIpRangesOutput {
    /// Creates a new builder-style object to manufacture [`GetCheckerIpRangesOutput`](crate::operation::get_checker_ip_ranges::GetCheckerIpRangesOutput).
    pub fn builder(
    ) -> crate::operation::get_checker_ip_ranges::builders::GetCheckerIpRangesOutputBuilder {
        crate::operation::get_checker_ip_ranges::builders::GetCheckerIpRangesOutputBuilder::default(
        )
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_checker_ip_ranges::GetCheckerIpRangesOutput;
/// A builder for [`GetCheckerIpRangesOutput`](crate::operation::get_checker_ip_ranges::GetCheckerIpRangesOutput).
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
pub struct GetCheckerIpRangesOutputBuilder {
    pub(crate) checker_ip_ranges: std::option::Option<std::vec::Vec<std::string::String>>,
    _request_id: Option<String>,
}
impl GetCheckerIpRangesOutputBuilder {
    /// Appends an item to `checker_ip_ranges`.
    ///
    /// To override the contents of this collection use [`set_checker_ip_ranges`](Self::set_checker_ip_ranges).
    ///
    /// <p>A complex type that contains sorted list of IP ranges in CIDR format for Amazon Route 53 health checkers.</p>
    pub fn checker_ip_ranges(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.checker_ip_ranges.unwrap_or_default();
        v.push(input.into());
        self.checker_ip_ranges = Some(v);
        self
    }
    /// <p>A complex type that contains sorted list of IP ranges in CIDR format for Amazon Route 53 health checkers.</p>
    pub fn set_checker_ip_ranges(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.checker_ip_ranges = input;
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
    /// Consumes the builder and constructs a [`GetCheckerIpRangesOutput`](crate::operation::get_checker_ip_ranges::GetCheckerIpRangesOutput).
    pub fn build(self) -> crate::operation::get_checker_ip_ranges::GetCheckerIpRangesOutput {
        crate::operation::get_checker_ip_ranges::GetCheckerIpRangesOutput {
            checker_ip_ranges: self.checker_ip_ranges,
            _request_id: self._request_id,
        }
    }
}
