// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The output for the <code>DescribeDeliveryChannels</code> action.</p>
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
pub struct DescribeDeliveryChannelsOutput {
    /// <p>A list that contains the descriptions of the specified delivery channel.</p>
    #[doc(hidden)]
    pub delivery_channels: std::option::Option<std::vec::Vec<crate::types::DeliveryChannel>>,
    _request_id: Option<String>,
}
impl DescribeDeliveryChannelsOutput {
    /// <p>A list that contains the descriptions of the specified delivery channel.</p>
    pub fn delivery_channels(&self) -> std::option::Option<&[crate::types::DeliveryChannel]> {
        self.delivery_channels.as_deref()
    }
}
impl aws_http::request_id::RequestId for DescribeDeliveryChannelsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeDeliveryChannelsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeDeliveryChannelsOutput`](crate::operation::describe_delivery_channels::DescribeDeliveryChannelsOutput).
    pub fn builder(
    ) -> crate::operation::describe_delivery_channels::builders::DescribeDeliveryChannelsOutputBuilder
    {
        crate::operation::describe_delivery_channels::builders::DescribeDeliveryChannelsOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::describe_delivery_channels::DescribeDeliveryChannelsOutput;
/// A builder for [`DescribeDeliveryChannelsOutput`](crate::operation::describe_delivery_channels::DescribeDeliveryChannelsOutput).
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
pub struct DescribeDeliveryChannelsOutputBuilder {
    pub(crate) delivery_channels: std::option::Option<std::vec::Vec<crate::types::DeliveryChannel>>,
    _request_id: Option<String>,
}
impl DescribeDeliveryChannelsOutputBuilder {
    /// Appends an item to `delivery_channels`.
    ///
    /// To override the contents of this collection use [`set_delivery_channels`](Self::set_delivery_channels).
    ///
    /// <p>A list that contains the descriptions of the specified delivery channel.</p>
    pub fn delivery_channels(mut self, input: crate::types::DeliveryChannel) -> Self {
        let mut v = self.delivery_channels.unwrap_or_default();
        v.push(input);
        self.delivery_channels = Some(v);
        self
    }
    /// <p>A list that contains the descriptions of the specified delivery channel.</p>
    pub fn set_delivery_channels(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::DeliveryChannel>>,
    ) -> Self {
        self.delivery_channels = input;
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
    /// Consumes the builder and constructs a [`DescribeDeliveryChannelsOutput`](crate::operation::describe_delivery_channels::DescribeDeliveryChannelsOutput).
    pub fn build(
        self,
    ) -> crate::operation::describe_delivery_channels::DescribeDeliveryChannelsOutput {
        crate::operation::describe_delivery_channels::DescribeDeliveryChannelsOutput {
            delivery_channels: self.delivery_channels,
            _request_id: self._request_id,
        }
    }
}
