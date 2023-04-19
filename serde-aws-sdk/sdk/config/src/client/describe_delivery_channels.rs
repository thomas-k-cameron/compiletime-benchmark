// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeDeliveryChannels`](crate::operation::describe_delivery_channels::builders::DescribeDeliveryChannelsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`delivery_channel_names(Vec<String>)`](crate::operation::describe_delivery_channels::builders::DescribeDeliveryChannelsFluentBuilder::delivery_channel_names) / [`set_delivery_channel_names(Option<Vec<String>>)`](crate::operation::describe_delivery_channels::builders::DescribeDeliveryChannelsFluentBuilder::set_delivery_channel_names): <p>A list of delivery channel names.</p>
    /// - On success, responds with [`DescribeDeliveryChannelsOutput`](crate::operation::describe_delivery_channels::DescribeDeliveryChannelsOutput) with field(s):
    ///   - [`delivery_channels(Option<Vec<DeliveryChannel>>)`](crate::operation::describe_delivery_channels::DescribeDeliveryChannelsOutput::delivery_channels): <p>A list that contains the descriptions of the specified delivery channel.</p>
    /// - On failure, responds with [`SdkError<DescribeDeliveryChannelsError>`](crate::operation::describe_delivery_channels::DescribeDeliveryChannelsError)
    pub fn describe_delivery_channels(
        &self,
    ) -> crate::operation::describe_delivery_channels::builders::DescribeDeliveryChannelsFluentBuilder
    {
        crate::operation::describe_delivery_channels::builders::DescribeDeliveryChannelsFluentBuilder::new(self.handle.clone())
    }
}