// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_client_vpn_endpoint::_create_client_vpn_endpoint_output::CreateClientVpnEndpointOutputBuilder;

pub use crate::operation::create_client_vpn_endpoint::_create_client_vpn_endpoint_input::CreateClientVpnEndpointInputBuilder;

/// Fluent builder constructing a request to `CreateClientVpnEndpoint`.
///
/// <p>Creates a Client VPN endpoint. A Client VPN endpoint is the resource you create and configure to enable and manage client VPN sessions. It is the destination endpoint at which all client VPN sessions are terminated.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateClientVpnEndpointFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::create_client_vpn_endpoint::builders::CreateClientVpnEndpointInputBuilder,
}
impl CreateClientVpnEndpointFluentBuilder {
    /// Creates a new `CreateClientVpnEndpoint`.
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
            crate::operation::create_client_vpn_endpoint::CreateClientVpnEndpoint,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_client_vpn_endpoint::CreateClientVpnEndpointError,
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
        crate::operation::create_client_vpn_endpoint::CreateClientVpnEndpointOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_client_vpn_endpoint::CreateClientVpnEndpointError,
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
    ///     let deserialized_parameters: crate::operation::create_client_vpn_endpoint::builders::CreateClientVpnEndpointInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.create_client_vpn_endpoint().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::create_client_vpn_endpoint::builders::CreateClientVpnEndpointInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The IPv4 address range, in CIDR notation, from which to assign client IP addresses. The address range cannot overlap with the local CIDR of the VPC in which the associated subnet is located, or the routes that you add manually. The address range cannot be changed after the Client VPN endpoint has been created. Client CIDR range must have a size of at least /22 and must not be greater than /12.</p>
    pub fn client_cidr_block(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.client_cidr_block(input.into());
        self
    }
    /// <p>The IPv4 address range, in CIDR notation, from which to assign client IP addresses. The address range cannot overlap with the local CIDR of the VPC in which the associated subnet is located, or the routes that you add manually. The address range cannot be changed after the Client VPN endpoint has been created. Client CIDR range must have a size of at least /22 and must not be greater than /12.</p>
    pub fn set_client_cidr_block(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_client_cidr_block(input);
        self
    }
    /// <p>The ARN of the server certificate. For more information, see the <a href="https://docs.aws.amazon.com/acm/latest/userguide/">Certificate Manager User Guide</a>.</p>
    pub fn server_certificate_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.server_certificate_arn(input.into());
        self
    }
    /// <p>The ARN of the server certificate. For more information, see the <a href="https://docs.aws.amazon.com/acm/latest/userguide/">Certificate Manager User Guide</a>.</p>
    pub fn set_server_certificate_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_server_certificate_arn(input);
        self
    }
    /// Appends an item to `AuthenticationOptions`.
    ///
    /// To override the contents of this collection use [`set_authentication_options`](Self::set_authentication_options).
    ///
    /// <p>Information about the authentication method to be used to authenticate clients.</p>
    pub fn authentication_options(
        mut self,
        input: crate::types::ClientVpnAuthenticationRequest,
    ) -> Self {
        self.inner = self.inner.authentication_options(input);
        self
    }
    /// <p>Information about the authentication method to be used to authenticate clients.</p>
    pub fn set_authentication_options(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ClientVpnAuthenticationRequest>>,
    ) -> Self {
        self.inner = self.inner.set_authentication_options(input);
        self
    }
    /// <p>Information about the client connection logging options.</p>
    /// <p>If you enable client connection logging, data about client connections is sent to a Cloudwatch Logs log stream. The following information is logged:</p>
    /// <ul>
    /// <li> <p>Client connection requests</p> </li>
    /// <li> <p>Client connection results (successful and unsuccessful)</p> </li>
    /// <li> <p>Reasons for unsuccessful client connection requests</p> </li>
    /// <li> <p>Client connection termination time</p> </li>
    /// </ul>
    pub fn connection_log_options(mut self, input: crate::types::ConnectionLogOptions) -> Self {
        self.inner = self.inner.connection_log_options(input);
        self
    }
    /// <p>Information about the client connection logging options.</p>
    /// <p>If you enable client connection logging, data about client connections is sent to a Cloudwatch Logs log stream. The following information is logged:</p>
    /// <ul>
    /// <li> <p>Client connection requests</p> </li>
    /// <li> <p>Client connection results (successful and unsuccessful)</p> </li>
    /// <li> <p>Reasons for unsuccessful client connection requests</p> </li>
    /// <li> <p>Client connection termination time</p> </li>
    /// </ul>
    pub fn set_connection_log_options(
        mut self,
        input: std::option::Option<crate::types::ConnectionLogOptions>,
    ) -> Self {
        self.inner = self.inner.set_connection_log_options(input);
        self
    }
    /// Appends an item to `DnsServers`.
    ///
    /// To override the contents of this collection use [`set_dns_servers`](Self::set_dns_servers).
    ///
    /// <p>Information about the DNS servers to be used for DNS resolution. A Client VPN endpoint can have up to two DNS servers. If no DNS server is specified, the DNS address configured on the device is used for the DNS server.</p>
    pub fn dns_servers(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.dns_servers(input.into());
        self
    }
    /// <p>Information about the DNS servers to be used for DNS resolution. A Client VPN endpoint can have up to two DNS servers. If no DNS server is specified, the DNS address configured on the device is used for the DNS server.</p>
    pub fn set_dns_servers(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_dns_servers(input);
        self
    }
    /// <p>The transport protocol to be used by the VPN session.</p>
    /// <p>Default value: <code>udp</code> </p>
    pub fn transport_protocol(mut self, input: crate::types::TransportProtocol) -> Self {
        self.inner = self.inner.transport_protocol(input);
        self
    }
    /// <p>The transport protocol to be used by the VPN session.</p>
    /// <p>Default value: <code>udp</code> </p>
    pub fn set_transport_protocol(
        mut self,
        input: std::option::Option<crate::types::TransportProtocol>,
    ) -> Self {
        self.inner = self.inner.set_transport_protocol(input);
        self
    }
    /// <p>The port number to assign to the Client VPN endpoint for TCP and UDP traffic.</p>
    /// <p>Valid Values: <code>443</code> | <code>1194</code> </p>
    /// <p>Default Value: <code>443</code> </p>
    pub fn vpn_port(mut self, input: i32) -> Self {
        self.inner = self.inner.vpn_port(input);
        self
    }
    /// <p>The port number to assign to the Client VPN endpoint for TCP and UDP traffic.</p>
    /// <p>Valid Values: <code>443</code> | <code>1194</code> </p>
    /// <p>Default Value: <code>443</code> </p>
    pub fn set_vpn_port(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_vpn_port(input);
        self
    }
    /// <p>A brief description of the Client VPN endpoint.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A brief description of the Client VPN endpoint.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>Indicates whether split-tunnel is enabled on the Client VPN endpoint.</p>
    /// <p>By default, split-tunnel on a VPN endpoint is disabled.</p>
    /// <p>For information about split-tunnel VPN endpoints, see <a href="https://docs.aws.amazon.com/vpn/latest/clientvpn-admin/split-tunnel-vpn.html">Split-tunnel Client VPN endpoint</a> in the <i>Client VPN Administrator Guide</i>.</p>
    pub fn split_tunnel(mut self, input: bool) -> Self {
        self.inner = self.inner.split_tunnel(input);
        self
    }
    /// <p>Indicates whether split-tunnel is enabled on the Client VPN endpoint.</p>
    /// <p>By default, split-tunnel on a VPN endpoint is disabled.</p>
    /// <p>For information about split-tunnel VPN endpoints, see <a href="https://docs.aws.amazon.com/vpn/latest/clientvpn-admin/split-tunnel-vpn.html">Split-tunnel Client VPN endpoint</a> in the <i>Client VPN Administrator Guide</i>.</p>
    pub fn set_split_tunnel(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_split_tunnel(input);
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
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// Appends an item to `TagSpecifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to apply to the Client VPN endpoint during creation.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        self.inner = self.inner.tag_specifications(input);
        self
    }
    /// <p>The tags to apply to the Client VPN endpoint during creation.</p>
    pub fn set_tag_specifications(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::TagSpecification>>,
    ) -> Self {
        self.inner = self.inner.set_tag_specifications(input);
        self
    }
    /// Appends an item to `SecurityGroupIds`.
    ///
    /// To override the contents of this collection use [`set_security_group_ids`](Self::set_security_group_ids).
    ///
    /// <p>The IDs of one or more security groups to apply to the target network. You must also specify the ID of the VPC that contains the security groups.</p>
    pub fn security_group_ids(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.security_group_ids(input.into());
        self
    }
    /// <p>The IDs of one or more security groups to apply to the target network. You must also specify the ID of the VPC that contains the security groups.</p>
    pub fn set_security_group_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_security_group_ids(input);
        self
    }
    /// <p>The ID of the VPC to associate with the Client VPN endpoint. If no security group IDs are specified in the request, the default security group for the VPC is applied.</p>
    pub fn vpc_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.vpc_id(input.into());
        self
    }
    /// <p>The ID of the VPC to associate with the Client VPN endpoint. If no security group IDs are specified in the request, the default security group for the VPC is applied.</p>
    pub fn set_vpc_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_vpc_id(input);
        self
    }
    /// <p>Specify whether to enable the self-service portal for the Client VPN endpoint.</p>
    /// <p>Default Value: <code>enabled</code> </p>
    pub fn self_service_portal(mut self, input: crate::types::SelfServicePortal) -> Self {
        self.inner = self.inner.self_service_portal(input);
        self
    }
    /// <p>Specify whether to enable the self-service portal for the Client VPN endpoint.</p>
    /// <p>Default Value: <code>enabled</code> </p>
    pub fn set_self_service_portal(
        mut self,
        input: std::option::Option<crate::types::SelfServicePortal>,
    ) -> Self {
        self.inner = self.inner.set_self_service_portal(input);
        self
    }
    /// <p>The options for managing connection authorization for new client connections.</p>
    pub fn client_connect_options(mut self, input: crate::types::ClientConnectOptions) -> Self {
        self.inner = self.inner.client_connect_options(input);
        self
    }
    /// <p>The options for managing connection authorization for new client connections.</p>
    pub fn set_client_connect_options(
        mut self,
        input: std::option::Option<crate::types::ClientConnectOptions>,
    ) -> Self {
        self.inner = self.inner.set_client_connect_options(input);
        self
    }
    /// <p>The maximum VPN session duration time in hours.</p>
    /// <p>Valid values: <code>8 | 10 | 12 | 24</code> </p>
    /// <p>Default value: <code>24</code> </p>
    pub fn session_timeout_hours(mut self, input: i32) -> Self {
        self.inner = self.inner.session_timeout_hours(input);
        self
    }
    /// <p>The maximum VPN session duration time in hours.</p>
    /// <p>Valid values: <code>8 | 10 | 12 | 24</code> </p>
    /// <p>Default value: <code>24</code> </p>
    pub fn set_session_timeout_hours(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_session_timeout_hours(input);
        self
    }
    /// <p>Options for enabling a customizable text banner that will be displayed on Amazon Web Services provided clients when a VPN session is established.</p>
    pub fn client_login_banner_options(
        mut self,
        input: crate::types::ClientLoginBannerOptions,
    ) -> Self {
        self.inner = self.inner.client_login_banner_options(input);
        self
    }
    /// <p>Options for enabling a customizable text banner that will be displayed on Amazon Web Services provided clients when a VPN session is established.</p>
    pub fn set_client_login_banner_options(
        mut self,
        input: std::option::Option<crate::types::ClientLoginBannerOptions>,
    ) -> Self {
        self.inner = self.inner.set_client_login_banner_options(input);
        self
    }
}
