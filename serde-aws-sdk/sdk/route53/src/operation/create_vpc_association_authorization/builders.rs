// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_vpc_association_authorization::_create_vpc_association_authorization_output::CreateVpcAssociationAuthorizationOutputBuilder;

pub use crate::operation::create_vpc_association_authorization::_create_vpc_association_authorization_input::CreateVpcAssociationAuthorizationInputBuilder;

/// Fluent builder constructing a request to `CreateVPCAssociationAuthorization`.
///
/// <p>Authorizes the Amazon Web Services account that created a specified VPC to submit an <code>AssociateVPCWithHostedZone</code> request to associate the VPC with a specified hosted zone that was created by a different account. To submit a <code>CreateVPCAssociationAuthorization</code> request, you must use the account that created the hosted zone. After you authorize the association, use the account that created the VPC to submit an <code>AssociateVPCWithHostedZone</code> request.</p> <note>
/// <p>If you want to associate multiple VPCs that you created by using one account with a hosted zone that you created by using a different account, you must submit one authorization request for each VPC.</p>
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateVPCAssociationAuthorizationFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::create_vpc_association_authorization::builders::CreateVpcAssociationAuthorizationInputBuilder
            }
impl CreateVPCAssociationAuthorizationFluentBuilder {
    /// Creates a new `CreateVPCAssociationAuthorization`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::create_vpc_association_authorization::CreateVPCAssociationAuthorization, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::create_vpc_association_authorization::CreateVPCAssociationAuthorizationError>
    >{
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
                    pub async fn send(self) -> std::result::Result<crate::operation::create_vpc_association_authorization::CreateVpcAssociationAuthorizationOutput, aws_smithy_http::result::SdkError<crate::operation::create_vpc_association_authorization::CreateVPCAssociationAuthorizationError>>
                     {
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
    ///     let deserialized_parameters: crate::operation::create_vpc_association_authorization::builders::CreateVpcAssociationAuthorizationInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.create_vpc_association_authorization().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::create_vpc_association_authorization::builders::CreateVpcAssociationAuthorizationInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The ID of the private hosted zone that you want to authorize associating a VPC with.</p>
    pub fn hosted_zone_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.hosted_zone_id(input.into());
        self
    }
    /// <p>The ID of the private hosted zone that you want to authorize associating a VPC with.</p>
    pub fn set_hosted_zone_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_hosted_zone_id(input);
        self
    }
    /// <p>A complex type that contains the VPC ID and region for the VPC that you want to authorize associating with your hosted zone.</p>
    pub fn vpc(mut self, input: crate::types::Vpc) -> Self {
        self.inner = self.inner.vpc(input);
        self
    }
    /// <p>A complex type that contains the VPC ID and region for the VPC that you want to authorize associating with your hosted zone.</p>
    pub fn set_vpc(mut self, input: std::option::Option<crate::types::Vpc>) -> Self {
        self.inner = self.inner.set_vpc(input);
        self
    }
}
