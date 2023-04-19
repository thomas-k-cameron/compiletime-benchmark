// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_conformance_pack::_put_conformance_pack_output::PutConformancePackOutputBuilder;

pub use crate::operation::put_conformance_pack::_put_conformance_pack_input::PutConformancePackInputBuilder;

/// Fluent builder constructing a request to `PutConformancePack`.
///
/// <p>Creates or updates a conformance pack. A conformance pack is a collection of Config rules that can be easily deployed in an account and a region and across an organization. For information on how many conformance packs you can have per account, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/configlimits.html"> <b>Service Limits</b> </a> in the Config Developer Guide.</p>
/// <p>This API creates a service-linked role <code>AWSServiceRoleForConfigConforms</code> in your account. The service-linked role is created only when the role does not exist in your account. </p> <note>
/// <p>You must specify only one of the follow parameters: <code>TemplateS3Uri</code>, <code>TemplateBody</code> or <code>TemplateSSMDocumentDetails</code>.</p>
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct PutConformancePackFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_conformance_pack::builders::PutConformancePackInputBuilder,
}
impl PutConformancePackFluentBuilder {
    /// Creates a new `PutConformancePack`.
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
            crate::operation::put_conformance_pack::PutConformancePack,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::put_conformance_pack::PutConformancePackError,
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
        crate::operation::put_conformance_pack::PutConformancePackOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::put_conformance_pack::PutConformancePackError,
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
    ///     let deserialized_parameters: crate::operation::put_conformance_pack::builders::PutConformancePackInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.put_conformance_pack().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::put_conformance_pack::builders::PutConformancePackInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The unique name of the conformance pack you want to deploy.</p>
    pub fn conformance_pack_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.conformance_pack_name(input.into());
        self
    }
    /// <p>The unique name of the conformance pack you want to deploy.</p>
    pub fn set_conformance_pack_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_conformance_pack_name(input);
        self
    }
    /// <p>The location of the file containing the template body (<code>s3://bucketname/prefix</code>). The uri must point to a conformance pack template (max size: 300 KB) that is located in an Amazon S3 bucket in the same Region as the conformance pack. </p> <note>
    /// <p>You must have access to read Amazon S3 bucket.</p>
    /// </note>
    pub fn template_s3_uri(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.template_s3_uri(input.into());
        self
    }
    /// <p>The location of the file containing the template body (<code>s3://bucketname/prefix</code>). The uri must point to a conformance pack template (max size: 300 KB) that is located in an Amazon S3 bucket in the same Region as the conformance pack. </p> <note>
    /// <p>You must have access to read Amazon S3 bucket.</p>
    /// </note>
    pub fn set_template_s3_uri(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_template_s3_uri(input);
        self
    }
    /// <p>A string containing the full conformance pack template body. The structure containing the template body has a minimum length of 1 byte and a maximum length of 51,200 bytes.</p> <note>
    /// <p>You can use a YAML template with two resource types: Config rule (<code>AWS::Config::ConfigRule</code>) and remediation action (<code>AWS::Config::RemediationConfiguration</code>).</p>
    /// </note>
    pub fn template_body(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.template_body(input.into());
        self
    }
    /// <p>A string containing the full conformance pack template body. The structure containing the template body has a minimum length of 1 byte and a maximum length of 51,200 bytes.</p> <note>
    /// <p>You can use a YAML template with two resource types: Config rule (<code>AWS::Config::ConfigRule</code>) and remediation action (<code>AWS::Config::RemediationConfiguration</code>).</p>
    /// </note>
    pub fn set_template_body(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_template_body(input);
        self
    }
    /// <p>The name of the Amazon S3 bucket where Config stores conformance pack templates.</p> <note>
    /// <p>This field is optional.</p>
    /// </note>
    pub fn delivery_s3_bucket(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.delivery_s3_bucket(input.into());
        self
    }
    /// <p>The name of the Amazon S3 bucket where Config stores conformance pack templates.</p> <note>
    /// <p>This field is optional.</p>
    /// </note>
    pub fn set_delivery_s3_bucket(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_delivery_s3_bucket(input);
        self
    }
    /// <p>The prefix for the Amazon S3 bucket. </p> <note>
    /// <p>This field is optional.</p>
    /// </note>
    pub fn delivery_s3_key_prefix(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.delivery_s3_key_prefix(input.into());
        self
    }
    /// <p>The prefix for the Amazon S3 bucket. </p> <note>
    /// <p>This field is optional.</p>
    /// </note>
    pub fn set_delivery_s3_key_prefix(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_delivery_s3_key_prefix(input);
        self
    }
    /// Appends an item to `ConformancePackInputParameters`.
    ///
    /// To override the contents of this collection use [`set_conformance_pack_input_parameters`](Self::set_conformance_pack_input_parameters).
    ///
    /// <p>A list of <code>ConformancePackInputParameter</code> objects.</p>
    pub fn conformance_pack_input_parameters(
        mut self,
        input: crate::types::ConformancePackInputParameter,
    ) -> Self {
        self.inner = self.inner.conformance_pack_input_parameters(input);
        self
    }
    /// <p>A list of <code>ConformancePackInputParameter</code> objects.</p>
    pub fn set_conformance_pack_input_parameters(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ConformancePackInputParameter>>,
    ) -> Self {
        self.inner = self.inner.set_conformance_pack_input_parameters(input);
        self
    }
    /// <p>An object of type <code>TemplateSSMDocumentDetails</code>, which contains the name or the Amazon Resource Name (ARN) of the Amazon Web Services Systems Manager document (SSM document) and the version of the SSM document that is used to create a conformance pack.</p>
    pub fn template_ssm_document_details(
        mut self,
        input: crate::types::TemplateSsmDocumentDetails,
    ) -> Self {
        self.inner = self.inner.template_ssm_document_details(input);
        self
    }
    /// <p>An object of type <code>TemplateSSMDocumentDetails</code>, which contains the name or the Amazon Resource Name (ARN) of the Amazon Web Services Systems Manager document (SSM document) and the version of the SSM document that is used to create a conformance pack.</p>
    pub fn set_template_ssm_document_details(
        mut self,
        input: std::option::Option<crate::types::TemplateSsmDocumentDetails>,
    ) -> Self {
        self.inner = self.inner.set_template_ssm_document_details(input);
        self
    }
}
