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
pub struct CreateVerifiedAccessEndpointInput {
    /// <p>The ID of the Verified Access group to associate the endpoint with.</p>
    #[doc(hidden)]
    pub verified_access_group_id: std::option::Option<std::string::String>,
    /// <p>The type of Amazon Web Services Verified Access endpoint to create.</p>
    #[doc(hidden)]
    pub endpoint_type: std::option::Option<crate::types::VerifiedAccessEndpointType>,
    /// <p>The Amazon Web Services network component Verified Access attaches to.</p>
    #[doc(hidden)]
    pub attachment_type: std::option::Option<crate::types::VerifiedAccessEndpointAttachmentType>,
    /// <p>The ARN of the public TLS/SSL certificate in Amazon Web Services Certificate Manager to associate with the endpoint. The CN in the certificate must match the DNS name your end users will use to reach your application.</p>
    #[doc(hidden)]
    pub domain_certificate_arn: std::option::Option<std::string::String>,
    /// <p>The DNS name for users to reach your application.</p>
    #[doc(hidden)]
    pub application_domain: std::option::Option<std::string::String>,
    /// <p>A custom identifier that gets prepended to a DNS name that is generated for the endpoint.</p>
    #[doc(hidden)]
    pub endpoint_domain_prefix: std::option::Option<std::string::String>,
    /// <p>The Amazon EC2 security groups to associate with the Amazon Web Services Verified Access endpoint.</p>
    #[doc(hidden)]
    pub security_group_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>The load balancer details if creating the Amazon Web Services Verified Access endpoint as <code>load-balancer</code>type.</p>
    #[doc(hidden)]
    pub load_balancer_options:
        std::option::Option<crate::types::CreateVerifiedAccessEndpointLoadBalancerOptions>,
    /// <p>The network interface details if creating the Amazon Web Services Verified Access endpoint as <code>network-interface</code>type.</p>
    #[doc(hidden)]
    pub network_interface_options:
        std::option::Option<crate::types::CreateVerifiedAccessEndpointEniOptions>,
    /// <p>A description for the Amazon Web Services Verified Access endpoint.</p>
    #[doc(hidden)]
    pub description: std::option::Option<std::string::String>,
    /// <p>The Amazon Web Services Verified Access policy document.</p>
    #[doc(hidden)]
    pub policy_document: std::option::Option<std::string::String>,
    /// <p>The tags to assign to the Amazon Web Services Verified Access endpoint.</p>
    #[doc(hidden)]
    pub tag_specifications: std::option::Option<std::vec::Vec<crate::types::TagSpecification>>,
    /// <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    #[doc(hidden)]
    pub client_token: std::option::Option<std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl CreateVerifiedAccessEndpointInput {
    /// <p>The ID of the Verified Access group to associate the endpoint with.</p>
    pub fn verified_access_group_id(&self) -> std::option::Option<&str> {
        self.verified_access_group_id.as_deref()
    }
    /// <p>The type of Amazon Web Services Verified Access endpoint to create.</p>
    pub fn endpoint_type(&self) -> std::option::Option<&crate::types::VerifiedAccessEndpointType> {
        self.endpoint_type.as_ref()
    }
    /// <p>The Amazon Web Services network component Verified Access attaches to.</p>
    pub fn attachment_type(
        &self,
    ) -> std::option::Option<&crate::types::VerifiedAccessEndpointAttachmentType> {
        self.attachment_type.as_ref()
    }
    /// <p>The ARN of the public TLS/SSL certificate in Amazon Web Services Certificate Manager to associate with the endpoint. The CN in the certificate must match the DNS name your end users will use to reach your application.</p>
    pub fn domain_certificate_arn(&self) -> std::option::Option<&str> {
        self.domain_certificate_arn.as_deref()
    }
    /// <p>The DNS name for users to reach your application.</p>
    pub fn application_domain(&self) -> std::option::Option<&str> {
        self.application_domain.as_deref()
    }
    /// <p>A custom identifier that gets prepended to a DNS name that is generated for the endpoint.</p>
    pub fn endpoint_domain_prefix(&self) -> std::option::Option<&str> {
        self.endpoint_domain_prefix.as_deref()
    }
    /// <p>The Amazon EC2 security groups to associate with the Amazon Web Services Verified Access endpoint.</p>
    pub fn security_group_ids(&self) -> std::option::Option<&[std::string::String]> {
        self.security_group_ids.as_deref()
    }
    /// <p>The load balancer details if creating the Amazon Web Services Verified Access endpoint as <code>load-balancer</code>type.</p>
    pub fn load_balancer_options(
        &self,
    ) -> std::option::Option<&crate::types::CreateVerifiedAccessEndpointLoadBalancerOptions> {
        self.load_balancer_options.as_ref()
    }
    /// <p>The network interface details if creating the Amazon Web Services Verified Access endpoint as <code>network-interface</code>type.</p>
    pub fn network_interface_options(
        &self,
    ) -> std::option::Option<&crate::types::CreateVerifiedAccessEndpointEniOptions> {
        self.network_interface_options.as_ref()
    }
    /// <p>A description for the Amazon Web Services Verified Access endpoint.</p>
    pub fn description(&self) -> std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The Amazon Web Services Verified Access policy document.</p>
    pub fn policy_document(&self) -> std::option::Option<&str> {
        self.policy_document.as_deref()
    }
    /// <p>The tags to assign to the Amazon Web Services Verified Access endpoint.</p>
    pub fn tag_specifications(&self) -> std::option::Option<&[crate::types::TagSpecification]> {
        self.tag_specifications.as_deref()
    }
    /// <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn client_token(&self) -> std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl CreateVerifiedAccessEndpointInput {
    /// Creates a new builder-style object to manufacture [`CreateVerifiedAccessEndpointInput`](crate::operation::create_verified_access_endpoint::CreateVerifiedAccessEndpointInput).
    pub fn builder() -> crate::operation::create_verified_access_endpoint::builders::CreateVerifiedAccessEndpointInputBuilder{
        crate::operation::create_verified_access_endpoint::builders::CreateVerifiedAccessEndpointInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::create_verified_access_endpoint::CreateVerifiedAccessEndpointInput;
/// A builder for [`CreateVerifiedAccessEndpointInput`](crate::operation::create_verified_access_endpoint::CreateVerifiedAccessEndpointInput).
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
pub struct CreateVerifiedAccessEndpointInputBuilder {
    pub(crate) verified_access_group_id: std::option::Option<std::string::String>,
    pub(crate) endpoint_type: std::option::Option<crate::types::VerifiedAccessEndpointType>,
    pub(crate) attachment_type:
        std::option::Option<crate::types::VerifiedAccessEndpointAttachmentType>,
    pub(crate) domain_certificate_arn: std::option::Option<std::string::String>,
    pub(crate) application_domain: std::option::Option<std::string::String>,
    pub(crate) endpoint_domain_prefix: std::option::Option<std::string::String>,
    pub(crate) security_group_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) load_balancer_options:
        std::option::Option<crate::types::CreateVerifiedAccessEndpointLoadBalancerOptions>,
    pub(crate) network_interface_options:
        std::option::Option<crate::types::CreateVerifiedAccessEndpointEniOptions>,
    pub(crate) description: std::option::Option<std::string::String>,
    pub(crate) policy_document: std::option::Option<std::string::String>,
    pub(crate) tag_specifications:
        std::option::Option<std::vec::Vec<crate::types::TagSpecification>>,
    pub(crate) client_token: std::option::Option<std::string::String>,
    pub(crate) dry_run: std::option::Option<bool>,
}
impl CreateVerifiedAccessEndpointInputBuilder {
    /// <p>The ID of the Verified Access group to associate the endpoint with.</p>
    pub fn verified_access_group_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.verified_access_group_id = Some(input.into());
        self
    }
    /// <p>The ID of the Verified Access group to associate the endpoint with.</p>
    pub fn set_verified_access_group_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.verified_access_group_id = input;
        self
    }
    /// <p>The type of Amazon Web Services Verified Access endpoint to create.</p>
    pub fn endpoint_type(mut self, input: crate::types::VerifiedAccessEndpointType) -> Self {
        self.endpoint_type = Some(input);
        self
    }
    /// <p>The type of Amazon Web Services Verified Access endpoint to create.</p>
    pub fn set_endpoint_type(
        mut self,
        input: std::option::Option<crate::types::VerifiedAccessEndpointType>,
    ) -> Self {
        self.endpoint_type = input;
        self
    }
    /// <p>The Amazon Web Services network component Verified Access attaches to.</p>
    pub fn attachment_type(
        mut self,
        input: crate::types::VerifiedAccessEndpointAttachmentType,
    ) -> Self {
        self.attachment_type = Some(input);
        self
    }
    /// <p>The Amazon Web Services network component Verified Access attaches to.</p>
    pub fn set_attachment_type(
        mut self,
        input: std::option::Option<crate::types::VerifiedAccessEndpointAttachmentType>,
    ) -> Self {
        self.attachment_type = input;
        self
    }
    /// <p>The ARN of the public TLS/SSL certificate in Amazon Web Services Certificate Manager to associate with the endpoint. The CN in the certificate must match the DNS name your end users will use to reach your application.</p>
    pub fn domain_certificate_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.domain_certificate_arn = Some(input.into());
        self
    }
    /// <p>The ARN of the public TLS/SSL certificate in Amazon Web Services Certificate Manager to associate with the endpoint. The CN in the certificate must match the DNS name your end users will use to reach your application.</p>
    pub fn set_domain_certificate_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.domain_certificate_arn = input;
        self
    }
    /// <p>The DNS name for users to reach your application.</p>
    pub fn application_domain(mut self, input: impl Into<std::string::String>) -> Self {
        self.application_domain = Some(input.into());
        self
    }
    /// <p>The DNS name for users to reach your application.</p>
    pub fn set_application_domain(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.application_domain = input;
        self
    }
    /// <p>A custom identifier that gets prepended to a DNS name that is generated for the endpoint.</p>
    pub fn endpoint_domain_prefix(mut self, input: impl Into<std::string::String>) -> Self {
        self.endpoint_domain_prefix = Some(input.into());
        self
    }
    /// <p>A custom identifier that gets prepended to a DNS name that is generated for the endpoint.</p>
    pub fn set_endpoint_domain_prefix(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.endpoint_domain_prefix = input;
        self
    }
    /// Appends an item to `security_group_ids`.
    ///
    /// To override the contents of this collection use [`set_security_group_ids`](Self::set_security_group_ids).
    ///
    /// <p>The Amazon EC2 security groups to associate with the Amazon Web Services Verified Access endpoint.</p>
    pub fn security_group_ids(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.security_group_ids.unwrap_or_default();
        v.push(input.into());
        self.security_group_ids = Some(v);
        self
    }
    /// <p>The Amazon EC2 security groups to associate with the Amazon Web Services Verified Access endpoint.</p>
    pub fn set_security_group_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.security_group_ids = input;
        self
    }
    /// <p>The load balancer details if creating the Amazon Web Services Verified Access endpoint as <code>load-balancer</code>type.</p>
    pub fn load_balancer_options(
        mut self,
        input: crate::types::CreateVerifiedAccessEndpointLoadBalancerOptions,
    ) -> Self {
        self.load_balancer_options = Some(input);
        self
    }
    /// <p>The load balancer details if creating the Amazon Web Services Verified Access endpoint as <code>load-balancer</code>type.</p>
    pub fn set_load_balancer_options(
        mut self,
        input: std::option::Option<crate::types::CreateVerifiedAccessEndpointLoadBalancerOptions>,
    ) -> Self {
        self.load_balancer_options = input;
        self
    }
    /// <p>The network interface details if creating the Amazon Web Services Verified Access endpoint as <code>network-interface</code>type.</p>
    pub fn network_interface_options(
        mut self,
        input: crate::types::CreateVerifiedAccessEndpointEniOptions,
    ) -> Self {
        self.network_interface_options = Some(input);
        self
    }
    /// <p>The network interface details if creating the Amazon Web Services Verified Access endpoint as <code>network-interface</code>type.</p>
    pub fn set_network_interface_options(
        mut self,
        input: std::option::Option<crate::types::CreateVerifiedAccessEndpointEniOptions>,
    ) -> Self {
        self.network_interface_options = input;
        self
    }
    /// <p>A description for the Amazon Web Services Verified Access endpoint.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.description = Some(input.into());
        self
    }
    /// <p>A description for the Amazon Web Services Verified Access endpoint.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The Amazon Web Services Verified Access policy document.</p>
    pub fn policy_document(mut self, input: impl Into<std::string::String>) -> Self {
        self.policy_document = Some(input.into());
        self
    }
    /// <p>The Amazon Web Services Verified Access policy document.</p>
    pub fn set_policy_document(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.policy_document = input;
        self
    }
    /// Appends an item to `tag_specifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to assign to the Amazon Web Services Verified Access endpoint.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        let mut v = self.tag_specifications.unwrap_or_default();
        v.push(input);
        self.tag_specifications = Some(v);
        self
    }
    /// <p>The tags to assign to the Amazon Web Services Verified Access endpoint.</p>
    pub fn set_tag_specifications(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::TagSpecification>>,
    ) -> Self {
        self.tag_specifications = input;
        self
    }
    /// <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.client_token = Some(input.into());
        self
    }
    /// <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateVerifiedAccessEndpointInput`](crate::operation::create_verified_access_endpoint::CreateVerifiedAccessEndpointInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::create_verified_access_endpoint::CreateVerifiedAccessEndpointInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::create_verified_access_endpoint::CreateVerifiedAccessEndpointInput {
                verified_access_group_id: self.verified_access_group_id,
                endpoint_type: self.endpoint_type,
                attachment_type: self.attachment_type,
                domain_certificate_arn: self.domain_certificate_arn,
                application_domain: self.application_domain,
                endpoint_domain_prefix: self.endpoint_domain_prefix,
                security_group_ids: self.security_group_ids,
                load_balancer_options: self.load_balancer_options,
                network_interface_options: self.network_interface_options,
                description: self.description,
                policy_document: self.policy_document,
                tag_specifications: self.tag_specifications,
                client_token: self.client_token,
                dry_run: self.dry_run,
            },
        )
    }
}
