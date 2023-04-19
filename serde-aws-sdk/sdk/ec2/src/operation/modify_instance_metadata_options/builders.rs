// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_instance_metadata_options::_modify_instance_metadata_options_output::ModifyInstanceMetadataOptionsOutputBuilder;

pub use crate::operation::modify_instance_metadata_options::_modify_instance_metadata_options_input::ModifyInstanceMetadataOptionsInputBuilder;

/// Fluent builder constructing a request to `ModifyInstanceMetadataOptions`.
///
/// <p>Modify the instance metadata parameters on a running or stopped instance. When you modify the parameters on a stopped instance, they are applied when the instance is started. When you modify the parameters on a running instance, the API responds with a state of “pending”. After the parameter modifications are successfully applied to the instance, the state of the modifications changes from “pending” to “applied” in subsequent describe-instances API calls. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-metadata.html">Instance metadata and user data</a> in the <i>Amazon EC2 User Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ModifyInstanceMetadataOptionsFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::modify_instance_metadata_options::builders::ModifyInstanceMetadataOptionsInputBuilder
            }
impl ModifyInstanceMetadataOptionsFluentBuilder {
    /// Creates a new `ModifyInstanceMetadataOptions`.
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
            crate::operation::modify_instance_metadata_options::ModifyInstanceMetadataOptions,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::modify_instance_metadata_options::ModifyInstanceMetadataOptionsError,
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
        crate::operation::modify_instance_metadata_options::ModifyInstanceMetadataOptionsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::modify_instance_metadata_options::ModifyInstanceMetadataOptionsError,
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
    ///     let deserialized_parameters: crate::operation::modify_instance_metadata_options::builders::ModifyInstanceMetadataOptionsInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.modify_instance_metadata_options().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::modify_instance_metadata_options::builders::ModifyInstanceMetadataOptionsInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The ID of the instance.</p>
    pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p>The ID of the instance.</p>
    pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
    /// <p>IMDSv2 uses token-backed sessions. Set the use of HTTP tokens to <code>optional</code> (in other words, set the use of IMDSv2 to <code>optional</code>) or <code>required</code> (in other words, set the use of IMDSv2 to <code>required</code>).</p>
    /// <ul>
    /// <li> <p> <code>optional</code> - When IMDSv2 is optional, you can choose to retrieve instance metadata with or without a session token in your request. If you retrieve the IAM role credentials without a token, the IMDSv1 role credentials are returned. If you retrieve the IAM role credentials using a valid session token, the IMDSv2 role credentials are returned.</p> </li>
    /// <li> <p> <code>required</code> - When IMDSv2 is required, you must send a session token with any instance metadata retrieval requests. In this state, retrieving the IAM role credentials always returns IMDSv2 credentials; IMDSv1 credentials are not available.</p> </li>
    /// </ul>
    /// <p>Default: <code>optional</code> </p>
    pub fn http_tokens(mut self, input: crate::types::HttpTokensState) -> Self {
        self.inner = self.inner.http_tokens(input);
        self
    }
    /// <p>IMDSv2 uses token-backed sessions. Set the use of HTTP tokens to <code>optional</code> (in other words, set the use of IMDSv2 to <code>optional</code>) or <code>required</code> (in other words, set the use of IMDSv2 to <code>required</code>).</p>
    /// <ul>
    /// <li> <p> <code>optional</code> - When IMDSv2 is optional, you can choose to retrieve instance metadata with or without a session token in your request. If you retrieve the IAM role credentials without a token, the IMDSv1 role credentials are returned. If you retrieve the IAM role credentials using a valid session token, the IMDSv2 role credentials are returned.</p> </li>
    /// <li> <p> <code>required</code> - When IMDSv2 is required, you must send a session token with any instance metadata retrieval requests. In this state, retrieving the IAM role credentials always returns IMDSv2 credentials; IMDSv1 credentials are not available.</p> </li>
    /// </ul>
    /// <p>Default: <code>optional</code> </p>
    pub fn set_http_tokens(
        mut self,
        input: std::option::Option<crate::types::HttpTokensState>,
    ) -> Self {
        self.inner = self.inner.set_http_tokens(input);
        self
    }
    /// <p>The desired HTTP PUT response hop limit for instance metadata requests. The larger the number, the further instance metadata requests can travel. If no parameter is specified, the existing state is maintained.</p>
    /// <p>Possible values: Integers from 1 to 64</p>
    pub fn http_put_response_hop_limit(mut self, input: i32) -> Self {
        self.inner = self.inner.http_put_response_hop_limit(input);
        self
    }
    /// <p>The desired HTTP PUT response hop limit for instance metadata requests. The larger the number, the further instance metadata requests can travel. If no parameter is specified, the existing state is maintained.</p>
    /// <p>Possible values: Integers from 1 to 64</p>
    pub fn set_http_put_response_hop_limit(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_http_put_response_hop_limit(input);
        self
    }
    /// <p>Enables or disables the HTTP metadata endpoint on your instances. If this parameter is not specified, the existing state is maintained.</p>
    /// <p>If you specify a value of <code>disabled</code>, you cannot access your instance metadata.</p>
    pub fn http_endpoint(mut self, input: crate::types::InstanceMetadataEndpointState) -> Self {
        self.inner = self.inner.http_endpoint(input);
        self
    }
    /// <p>Enables or disables the HTTP metadata endpoint on your instances. If this parameter is not specified, the existing state is maintained.</p>
    /// <p>If you specify a value of <code>disabled</code>, you cannot access your instance metadata.</p>
    pub fn set_http_endpoint(
        mut self,
        input: std::option::Option<crate::types::InstanceMetadataEndpointState>,
    ) -> Self {
        self.inner = self.inner.set_http_endpoint(input);
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
    /// <p>Enables or disables the IPv6 endpoint for the instance metadata service. This setting applies only if you have enabled the HTTP metadata endpoint.</p>
    pub fn http_protocol_ipv6(
        mut self,
        input: crate::types::InstanceMetadataProtocolState,
    ) -> Self {
        self.inner = self.inner.http_protocol_ipv6(input);
        self
    }
    /// <p>Enables or disables the IPv6 endpoint for the instance metadata service. This setting applies only if you have enabled the HTTP metadata endpoint.</p>
    pub fn set_http_protocol_ipv6(
        mut self,
        input: std::option::Option<crate::types::InstanceMetadataProtocolState>,
    ) -> Self {
        self.inner = self.inner.set_http_protocol_ipv6(input);
        self
    }
    /// <p>Set to <code>enabled</code> to allow access to instance tags from the instance metadata. Set to <code>disabled</code> to turn off access to instance tags from the instance metadata. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_Tags.html#work-with-tags-in-IMDS">Work with instance tags using the instance metadata</a>.</p>
    /// <p>Default: <code>disabled</code> </p>
    pub fn instance_metadata_tags(
        mut self,
        input: crate::types::InstanceMetadataTagsState,
    ) -> Self {
        self.inner = self.inner.instance_metadata_tags(input);
        self
    }
    /// <p>Set to <code>enabled</code> to allow access to instance tags from the instance metadata. Set to <code>disabled</code> to turn off access to instance tags from the instance metadata. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_Tags.html#work-with-tags-in-IMDS">Work with instance tags using the instance metadata</a>.</p>
    /// <p>Default: <code>disabled</code> </p>
    pub fn set_instance_metadata_tags(
        mut self,
        input: std::option::Option<crate::types::InstanceMetadataTagsState>,
    ) -> Self {
        self.inner = self.inner.set_instance_metadata_tags(input);
        self
    }
}