// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::attach_classic_link_vpc::_attach_classic_link_vpc_output::AttachClassicLinkVpcOutputBuilder;

pub use crate::operation::attach_classic_link_vpc::_attach_classic_link_vpc_input::AttachClassicLinkVpcInputBuilder;

/// Fluent builder constructing a request to `AttachClassicLinkVpc`.
///
/// <note>
/// <p>We are retiring EC2-Classic. We recommend that you migrate from EC2-Classic to a VPC. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/vpc-migrate.html">Migrate from EC2-Classic to a VPC</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
/// </note>
/// <p>Links an EC2-Classic instance to a ClassicLink-enabled VPC through one or more of the VPC's security groups. You cannot link an EC2-Classic instance to more than one VPC at a time. You can only link an instance that's in the <code>running</code> state. An instance is automatically unlinked from a VPC when it's stopped - you can link it to the VPC again when you restart it.</p>
/// <p>After you've linked an instance, you cannot change the VPC security groups that are associated with it. To change the security groups, you must first unlink the instance, and then link it again.</p>
/// <p>Linking your instance to a VPC is sometimes referred to as <i>attaching</i> your instance.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct AttachClassicLinkVpcFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::attach_classic_link_vpc::builders::AttachClassicLinkVpcInputBuilder,
}
impl AttachClassicLinkVpcFluentBuilder {
    /// Creates a new `AttachClassicLinkVpc`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::attach_classic_link_vpc::AttachClassicLinkVpc,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::attach_classic_link_vpc::AttachClassicLinkVpcError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        Ok(crate::client::customize::CustomizableOperation { handle, operation })
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> std::result::Result<
        crate::operation::attach_classic_link_vpc::AttachClassicLinkVpcOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::attach_classic_link_vpc::AttachClassicLinkVpcError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    #[cfg(aws_sdk_unstable)]
    /// This function replaces the parameter with new one.
    /// It is useful when you want to replace the existing data with de-serialized data.
    /// ```compile_fail
    /// let result_future = async {
    ///     let deserialized_parameters: crate::operation::attach_classic_link_vpc::builders::AttachClassicLinkVpcInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.attach_classic_link_vpc().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::attach_classic_link_vpc::builders::AttachClassicLinkVpcInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// Appends an item to `Groups`.
    ///
    /// To override the contents of this collection use [`set_groups`](Self::set_groups).
    ///
    /// <p>The ID of one or more of the VPC's security groups. You cannot specify security groups from a different VPC.</p>
    pub fn groups(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.groups(input.into());
        self
    }
    /// <p>The ID of one or more of the VPC's security groups. You cannot specify security groups from a different VPC.</p>
    pub fn set_groups(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_groups(input);
        self
    }
    /// <p>The ID of an EC2-Classic instance to link to the ClassicLink-enabled VPC.</p>
    pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p>The ID of an EC2-Classic instance to link to the ClassicLink-enabled VPC.</p>
    pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
    /// <p>The ID of a ClassicLink-enabled VPC.</p>
    pub fn vpc_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.vpc_id(input.into());
        self
    }
    /// <p>The ID of a ClassicLink-enabled VPC.</p>
    pub fn set_vpc_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_vpc_id(input);
        self
    }
}
