// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::associate_vpc_with_hosted_zone::_associate_vpc_with_hosted_zone_output::AssociateVpcWithHostedZoneOutputBuilder;

pub use crate::operation::associate_vpc_with_hosted_zone::_associate_vpc_with_hosted_zone_input::AssociateVpcWithHostedZoneInputBuilder;

/// Fluent builder constructing a request to `AssociateVPCWithHostedZone`.
///
/// <p>Associates an Amazon VPC with a private hosted zone. </p> <important>
/// <p>To perform the association, the VPC and the private hosted zone must already exist. You can't convert a public hosted zone into a private hosted zone.</p>
/// </important> <note>
/// <p>If you want to associate a VPC that was created by using one Amazon Web Services account with a private hosted zone that was created by using a different account, the Amazon Web Services account that created the private hosted zone must first submit a <code>CreateVPCAssociationAuthorization</code> request. Then the account that created the VPC must submit an <code>AssociateVPCWithHostedZone</code> request.</p>
/// </note> <note>
/// <p>When granting access, the hosted zone and the Amazon VPC must belong to the same partition. A partition is a group of Amazon Web Services Regions. Each Amazon Web Services account is scoped to one partition.</p>
/// <p>The following are the supported partitions:</p>
/// <ul>
/// <li> <p> <code>aws</code> - Amazon Web Services Regions</p> </li>
/// <li> <p> <code>aws-cn</code> - China Regions</p> </li>
/// <li> <p> <code>aws-us-gov</code> - Amazon Web Services GovCloud (US) Region</p> </li>
/// </ul>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Access Management</a> in the <i>Amazon Web Services General Reference</i>.</p>
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct AssociateVPCWithHostedZoneFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::associate_vpc_with_hosted_zone::builders::AssociateVpcWithHostedZoneInputBuilder
            }
impl AssociateVPCWithHostedZoneFluentBuilder {
    /// Creates a new `AssociateVPCWithHostedZone`.
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
            crate::operation::associate_vpc_with_hosted_zone::AssociateVPCWithHostedZone,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::associate_vpc_with_hosted_zone::AssociateVPCWithHostedZoneError,
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
        crate::operation::associate_vpc_with_hosted_zone::AssociateVpcWithHostedZoneOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::associate_vpc_with_hosted_zone::AssociateVPCWithHostedZoneError,
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
    ///     let deserialized_parameters: crate::operation::associate_vpc_with_hosted_zone::builders::AssociateVpcWithHostedZoneInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.associate_vpc_with_hosted_zone().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::associate_vpc_with_hosted_zone::builders::AssociateVpcWithHostedZoneInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The ID of the private hosted zone that you want to associate an Amazon VPC with.</p>
    /// <p>Note that you can't associate a VPC with a hosted zone that doesn't have an existing VPC association.</p>
    pub fn hosted_zone_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.hosted_zone_id(input.into());
        self
    }
    /// <p>The ID of the private hosted zone that you want to associate an Amazon VPC with.</p>
    /// <p>Note that you can't associate a VPC with a hosted zone that doesn't have an existing VPC association.</p>
    pub fn set_hosted_zone_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_hosted_zone_id(input);
        self
    }
    /// <p>A complex type that contains information about the VPC that you want to associate with a private hosted zone.</p>
    pub fn vpc(mut self, input: crate::types::Vpc) -> Self {
        self.inner = self.inner.vpc(input);
        self
    }
    /// <p>A complex type that contains information about the VPC that you want to associate with a private hosted zone.</p>
    pub fn set_vpc(mut self, input: std::option::Option<crate::types::Vpc>) -> Self {
        self.inner = self.inner.set_vpc(input);
        self
    }
    /// <p> <i>Optional:</i> A comment about the association request.</p>
    pub fn comment(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.comment(input.into());
        self
    }
    /// <p> <i>Optional:</i> A comment about the association request.</p>
    pub fn set_comment(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_comment(input);
        self
    }
}
