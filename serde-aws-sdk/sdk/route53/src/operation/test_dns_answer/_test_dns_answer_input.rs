// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Gets the value that Amazon Route 53 returns in response to a DNS request for a specified record name and type. You can optionally specify the IP address of a DNS resolver, an EDNS0 client subnet IP address, and a subnet mask. </p>
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
pub struct TestDnsAnswerInput {
    /// <p>The ID of the hosted zone that you want Amazon Route 53 to simulate a query for.</p>
    #[doc(hidden)]
    pub hosted_zone_id: std::option::Option<std::string::String>,
    /// <p>The name of the resource record set that you want Amazon Route 53 to simulate a query for.</p>
    #[doc(hidden)]
    pub record_name: std::option::Option<std::string::String>,
    /// <p>The type of the resource record set.</p>
    #[doc(hidden)]
    pub record_type: std::option::Option<crate::types::RrType>,
    /// <p>If you want to simulate a request from a specific DNS resolver, specify the IP address for that resolver. If you omit this value, <code>TestDnsAnswer</code> uses the IP address of a DNS resolver in the Amazon Web Services US East (N. Virginia) Region (<code>us-east-1</code>).</p>
    #[doc(hidden)]
    pub resolver_ip: std::option::Option<std::string::String>,
    /// <p>If the resolver that you specified for resolverip supports EDNS0, specify the IPv4 or IPv6 address of a client in the applicable location, for example, <code>192.0.2.44</code> or <code>2001:db8:85a3::8a2e:370:7334</code>.</p>
    #[doc(hidden)]
    pub edns0_client_subnet_ip: std::option::Option<std::string::String>,
    /// <p>If you specify an IP address for <code>edns0clientsubnetip</code>, you can optionally specify the number of bits of the IP address that you want the checking tool to include in the DNS query. For example, if you specify <code>192.0.2.44</code> for <code>edns0clientsubnetip</code> and <code>24</code> for <code>edns0clientsubnetmask</code>, the checking tool will simulate a request from 192.0.2.0/24. The default value is 24 bits for IPv4 addresses and 64 bits for IPv6 addresses.</p>
    /// <p>The range of valid values depends on whether <code>edns0clientsubnetip</code> is an IPv4 or an IPv6 address:</p>
    /// <ul>
    /// <li> <p> <b>IPv4</b>: Specify a value between 0 and 32</p> </li>
    /// <li> <p> <b>IPv6</b>: Specify a value between 0 and 128</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub edns0_client_subnet_mask: std::option::Option<std::string::String>,
}
impl TestDnsAnswerInput {
    /// <p>The ID of the hosted zone that you want Amazon Route 53 to simulate a query for.</p>
    pub fn hosted_zone_id(&self) -> std::option::Option<&str> {
        self.hosted_zone_id.as_deref()
    }
    /// <p>The name of the resource record set that you want Amazon Route 53 to simulate a query for.</p>
    pub fn record_name(&self) -> std::option::Option<&str> {
        self.record_name.as_deref()
    }
    /// <p>The type of the resource record set.</p>
    pub fn record_type(&self) -> std::option::Option<&crate::types::RrType> {
        self.record_type.as_ref()
    }
    /// <p>If you want to simulate a request from a specific DNS resolver, specify the IP address for that resolver. If you omit this value, <code>TestDnsAnswer</code> uses the IP address of a DNS resolver in the Amazon Web Services US East (N. Virginia) Region (<code>us-east-1</code>).</p>
    pub fn resolver_ip(&self) -> std::option::Option<&str> {
        self.resolver_ip.as_deref()
    }
    /// <p>If the resolver that you specified for resolverip supports EDNS0, specify the IPv4 or IPv6 address of a client in the applicable location, for example, <code>192.0.2.44</code> or <code>2001:db8:85a3::8a2e:370:7334</code>.</p>
    pub fn edns0_client_subnet_ip(&self) -> std::option::Option<&str> {
        self.edns0_client_subnet_ip.as_deref()
    }
    /// <p>If you specify an IP address for <code>edns0clientsubnetip</code>, you can optionally specify the number of bits of the IP address that you want the checking tool to include in the DNS query. For example, if you specify <code>192.0.2.44</code> for <code>edns0clientsubnetip</code> and <code>24</code> for <code>edns0clientsubnetmask</code>, the checking tool will simulate a request from 192.0.2.0/24. The default value is 24 bits for IPv4 addresses and 64 bits for IPv6 addresses.</p>
    /// <p>The range of valid values depends on whether <code>edns0clientsubnetip</code> is an IPv4 or an IPv6 address:</p>
    /// <ul>
    /// <li> <p> <b>IPv4</b>: Specify a value between 0 and 32</p> </li>
    /// <li> <p> <b>IPv6</b>: Specify a value between 0 and 128</p> </li>
    /// </ul>
    pub fn edns0_client_subnet_mask(&self) -> std::option::Option<&str> {
        self.edns0_client_subnet_mask.as_deref()
    }
}
impl TestDnsAnswerInput {
    /// Creates a new builder-style object to manufacture [`TestDnsAnswerInput`](crate::operation::test_dns_answer::TestDnsAnswerInput).
    pub fn builder() -> crate::operation::test_dns_answer::builders::TestDnsAnswerInputBuilder {
        crate::operation::test_dns_answer::builders::TestDnsAnswerInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::test_dns_answer::TestDnsAnswerInput;
/// A builder for [`TestDnsAnswerInput`](crate::operation::test_dns_answer::TestDnsAnswerInput).
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
pub struct TestDnsAnswerInputBuilder {
    pub(crate) hosted_zone_id: std::option::Option<std::string::String>,
    pub(crate) record_name: std::option::Option<std::string::String>,
    pub(crate) record_type: std::option::Option<crate::types::RrType>,
    pub(crate) resolver_ip: std::option::Option<std::string::String>,
    pub(crate) edns0_client_subnet_ip: std::option::Option<std::string::String>,
    pub(crate) edns0_client_subnet_mask: std::option::Option<std::string::String>,
}
impl TestDnsAnswerInputBuilder {
    /// <p>The ID of the hosted zone that you want Amazon Route 53 to simulate a query for.</p>
    pub fn hosted_zone_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.hosted_zone_id = Some(input.into());
        self
    }
    /// <p>The ID of the hosted zone that you want Amazon Route 53 to simulate a query for.</p>
    pub fn set_hosted_zone_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.hosted_zone_id = input;
        self
    }
    /// <p>The name of the resource record set that you want Amazon Route 53 to simulate a query for.</p>
    pub fn record_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.record_name = Some(input.into());
        self
    }
    /// <p>The name of the resource record set that you want Amazon Route 53 to simulate a query for.</p>
    pub fn set_record_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.record_name = input;
        self
    }
    /// <p>The type of the resource record set.</p>
    pub fn record_type(mut self, input: crate::types::RrType) -> Self {
        self.record_type = Some(input);
        self
    }
    /// <p>The type of the resource record set.</p>
    pub fn set_record_type(mut self, input: std::option::Option<crate::types::RrType>) -> Self {
        self.record_type = input;
        self
    }
    /// <p>If you want to simulate a request from a specific DNS resolver, specify the IP address for that resolver. If you omit this value, <code>TestDnsAnswer</code> uses the IP address of a DNS resolver in the Amazon Web Services US East (N. Virginia) Region (<code>us-east-1</code>).</p>
    pub fn resolver_ip(mut self, input: impl Into<std::string::String>) -> Self {
        self.resolver_ip = Some(input.into());
        self
    }
    /// <p>If you want to simulate a request from a specific DNS resolver, specify the IP address for that resolver. If you omit this value, <code>TestDnsAnswer</code> uses the IP address of a DNS resolver in the Amazon Web Services US East (N. Virginia) Region (<code>us-east-1</code>).</p>
    pub fn set_resolver_ip(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.resolver_ip = input;
        self
    }
    /// <p>If the resolver that you specified for resolverip supports EDNS0, specify the IPv4 or IPv6 address of a client in the applicable location, for example, <code>192.0.2.44</code> or <code>2001:db8:85a3::8a2e:370:7334</code>.</p>
    pub fn edns0_client_subnet_ip(mut self, input: impl Into<std::string::String>) -> Self {
        self.edns0_client_subnet_ip = Some(input.into());
        self
    }
    /// <p>If the resolver that you specified for resolverip supports EDNS0, specify the IPv4 or IPv6 address of a client in the applicable location, for example, <code>192.0.2.44</code> or <code>2001:db8:85a3::8a2e:370:7334</code>.</p>
    pub fn set_edns0_client_subnet_ip(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.edns0_client_subnet_ip = input;
        self
    }
    /// <p>If you specify an IP address for <code>edns0clientsubnetip</code>, you can optionally specify the number of bits of the IP address that you want the checking tool to include in the DNS query. For example, if you specify <code>192.0.2.44</code> for <code>edns0clientsubnetip</code> and <code>24</code> for <code>edns0clientsubnetmask</code>, the checking tool will simulate a request from 192.0.2.0/24. The default value is 24 bits for IPv4 addresses and 64 bits for IPv6 addresses.</p>
    /// <p>The range of valid values depends on whether <code>edns0clientsubnetip</code> is an IPv4 or an IPv6 address:</p>
    /// <ul>
    /// <li> <p> <b>IPv4</b>: Specify a value between 0 and 32</p> </li>
    /// <li> <p> <b>IPv6</b>: Specify a value between 0 and 128</p> </li>
    /// </ul>
    pub fn edns0_client_subnet_mask(mut self, input: impl Into<std::string::String>) -> Self {
        self.edns0_client_subnet_mask = Some(input.into());
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
        self.edns0_client_subnet_mask = input;
        self
    }
    /// Consumes the builder and constructs a [`TestDnsAnswerInput`](crate::operation::test_dns_answer::TestDnsAnswerInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::test_dns_answer::TestDnsAnswerInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::test_dns_answer::TestDnsAnswerInput {
            hosted_zone_id: self.hosted_zone_id,
            record_name: self.record_name,
            record_type: self.record_type,
            resolver_ip: self.resolver_ip,
            edns0_client_subnet_ip: self.edns0_client_subnet_ip,
            edns0_client_subnet_mask: self.edns0_client_subnet_mask,
        })
    }
}
