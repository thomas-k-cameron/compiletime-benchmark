// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::move_address_to_vpc::_move_address_to_vpc_output::MoveAddressToVpcOutputBuilder;

pub use crate::operation::move_address_to_vpc::_move_address_to_vpc_input::MoveAddressToVpcInputBuilder;

/// Fluent builder constructing a request to `MoveAddressToVpc`.
///
/// <p>Moves an Elastic IP address from the EC2-Classic platform to the EC2-VPC platform. The Elastic IP address must be allocated to your account for more than 24 hours, and it must not be associated with an instance. After the Elastic IP address is moved, it is no longer available for use in the EC2-Classic platform, unless you move it back using the <code>RestoreAddressToClassic</code> request. You cannot move an Elastic IP address that was originally allocated for use in the EC2-VPC platform to the EC2-Classic platform.</p> <note>
/// <p>We are retiring EC2-Classic. We recommend that you migrate from EC2-Classic to a VPC. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/vpc-migrate.html">Migrate from EC2-Classic to a VPC</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct MoveAddressToVpcFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::move_address_to_vpc::builders::MoveAddressToVpcInputBuilder,
}
impl MoveAddressToVpcFluentBuilder {
    /// Creates a new `MoveAddressToVpc`.
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
            crate::operation::move_address_to_vpc::MoveAddressToVpc,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::move_address_to_vpc::MoveAddressToVpcError,
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
        crate::operation::move_address_to_vpc::MoveAddressToVpcOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::move_address_to_vpc::MoveAddressToVpcError,
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
    ///     let deserialized_parameters: crate::operation::move_address_to_vpc::builders::MoveAddressToVpcInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.move_address_to_vpc().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::move_address_to_vpc::builders::MoveAddressToVpcInputBuilder,
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
    /// <p>The Elastic IP address.</p>
    pub fn public_ip(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.public_ip(input.into());
        self
    }
    /// <p>The Elastic IP address.</p>
    pub fn set_public_ip(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_public_ip(input);
        self
    }
}
