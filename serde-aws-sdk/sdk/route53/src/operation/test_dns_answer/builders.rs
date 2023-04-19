// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::test_dns_answer::_test_dns_answer_output::TestDnsAnswerOutputBuilder;

pub use crate::operation::test_dns_answer::_test_dns_answer_input::TestDnsAnswerInputBuilder;

/// Fluent builder constructing a request to `TestDNSAnswer`.
///
/// <p>Gets the value that Amazon Route 53 returns in response to a DNS request for a specified record name and type. You can optionally specify the IP address of a DNS resolver, an EDNS0 client subnet IP address, and a subnet mask. </p>
/// <p>This call only supports querying public hosted zones.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct TestDNSAnswerFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::test_dns_answer::builders::TestDnsAnswerInputBuilder,
}
impl TestDNSAnswerFluentBuilder {
    /// Creates a new `TestDNSAnswer`.
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
            crate::operation::test_dns_answer::TestDNSAnswer,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::test_dns_answer::TestDNSAnswerError>,
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
        crate::operation::test_dns_answer::TestDnsAnswerOutput,
        aws_smithy_http::result::SdkError<crate::operation::test_dns_answer::TestDNSAnswerError>,
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
    ///     let deserialized_parameters: crate::operation::test_dns_answer::builders::TestDnsAnswerInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.test_dns_answer().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::test_dns_answer::builders::TestDnsAnswerInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The ID of the hosted zone that you want Amazon Route 53 to simulate a query for.</p>
    pub fn hosted_zone_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.hosted_zone_id(input.into());
        self
    }
    /// <p>The ID of the hosted zone that you want Amazon Route 53 to simulate a query for.</p>
    pub fn set_hosted_zone_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_hosted_zone_id(input);
        self
    }
    /// <p>The name of the resource record set that you want Amazon Route 53 to simulate a query for.</p>
    pub fn record_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.record_name(input.into());
        self
    }
    /// <p>The name of the resource record set that you want Amazon Route 53 to simulate a query for.</p>
    pub fn set_record_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_record_name(input);
        self
    }
    /// <p>The type of the resource record set.</p>
    pub fn record_type(mut self, input: crate::types::RrType) -> Self {
        self.inner = self.inner.record_type(input);
        self
    }
    /// <p>The type of the resource record set.</p>
    pub fn set_record_type(mut self, input: std::option::Option<crate::types::RrType>) -> Self {
        self.inner = self.inner.set_record_type(input);
        self
    }
    /// <p>If you want to simulate a request from a specific DNS resolver, specify the IP address for that resolver. If you omit this value, <code>TestDnsAnswer</code> uses the IP address of a DNS resolver in the Amazon Web Services US East (N. Virginia) Region (<code>us-east-1</code>).</p>
    pub fn resolver_ip(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.resolver_ip(input.into());
        self
    }
    /// <p>If you want to simulate a request from a specific DNS resolver, specify the IP address for that resolver. If you omit this value, <code>TestDnsAnswer</code> uses the IP address of a DNS resolver in the Amazon Web Services US East (N. Virginia) Region (<code>us-east-1</code>).</p>
    pub fn set_resolver_ip(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_resolver_ip(input);
        self
    }
    /// <p>If the resolver that you specified for resolverip supports EDNS0, specify the IPv4 or IPv6 address of a client in the applicable location, for example, <code>192.0.2.44</code> or <code>2001:db8:85a3::8a2e:370:7334</code>.</p>
    pub fn edns0_client_subnet_ip(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.edns0_client_subnet_ip(input.into());
        self
    }
    /// <p>If the resolver that you specified for resolverip supports EDNS0, specify the IPv4 or IPv6 address of a client in the applicable location, for example, <code>192.0.2.44</code> or <code>2001:db8:85a3::8a2e:370:7334</code>.</p>
    pub fn set_edns0_client_subnet_ip(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_edns0_client_subnet_ip(input);
        self
    }
    /// <p>If you specify an IP address for <code>edns0clientsubnetip</code>, you can optionally specify the number of bits of the IP address that you want the checking tool to include in the DNS query. For example, if you specify <code>192.0.2.44</code> for <code>edns0clientsubnetip</code> and <code>24</code> for <code>edns0clientsubnetmask</code>, the checking tool will simulate a request from 192.0.2.0/24. The default value is 24 bits for IPv4 addresses and 64 bits for IPv6 addresses.</p>
    /// <p>The range of valid values depends on whether <code>edns0clientsubnetip</code> is an IPv4 or an IPv6 address:</p>
    /// <ul>
    /// <li> <p> <b>IPv4</b>: Specify a value between 0 and 32</p> </li>
    /// <li> <p> <b>IPv6</b>: Specify a value between 0 and 128</p> </li>
    /// </ul>
    pub fn edns0_client_subnet_mask(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.edns0_client_subnet_mask(input.into());
        self
    }
    /// <p>If you specify an IP address for <code>edns0clientsubnetip</code>, you can optionally specify the number of bits of the IP address that you want the checking tool to include in the DNS query. For example, if you specify <code>192.0.2.44</code> for <code>edns0clientsubnetip</code> and <code>24</code> for <code>edns0clientsubnetmask</code>, the checking tool will simulate a request from 192.0.2.0/24. The default value is 24 bits for IPv4 addresses and 64 bits for IPv6 addresses.</p>
    /// <p>The range of valid values depends on whether <code>edns0clientsubnetip</code> is an IPv4 or an IPv6 address:</p>
    /// <ul>
    /// <li> <p> <b>IPv4</b>: Specify a value between 0 and 32</p> </li>
    /// <li> <p> <b>IPv6</b>: Specify a value between 0 and 128</p> </li>
    /// </ul>
    pub fn set_edns0_client_subnet_mask(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_edns0_client_subnet_mask(input);
        self
    }
}
