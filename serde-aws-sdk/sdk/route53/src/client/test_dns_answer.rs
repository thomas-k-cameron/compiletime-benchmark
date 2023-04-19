// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`TestDNSAnswer`](crate::operation::test_dns_answer::builders::TestDNSAnswerFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`hosted_zone_id(impl Into<String>)`](crate::operation::test_dns_answer::builders::TestDNSAnswerFluentBuilder::hosted_zone_id) / [`set_hosted_zone_id(Option<String>)`](crate::operation::test_dns_answer::builders::TestDNSAnswerFluentBuilder::set_hosted_zone_id): <p>The ID of the hosted zone that you want Amazon Route 53 to simulate a query for.</p>
    ///   - [`record_name(impl Into<String>)`](crate::operation::test_dns_answer::builders::TestDNSAnswerFluentBuilder::record_name) / [`set_record_name(Option<String>)`](crate::operation::test_dns_answer::builders::TestDNSAnswerFluentBuilder::set_record_name): <p>The name of the resource record set that you want Amazon Route 53 to simulate a query for.</p>
    ///   - [`record_type(RrType)`](crate::operation::test_dns_answer::builders::TestDNSAnswerFluentBuilder::record_type) / [`set_record_type(Option<RrType>)`](crate::operation::test_dns_answer::builders::TestDNSAnswerFluentBuilder::set_record_type): <p>The type of the resource record set.</p>
    ///   - [`resolver_ip(impl Into<String>)`](crate::operation::test_dns_answer::builders::TestDNSAnswerFluentBuilder::resolver_ip) / [`set_resolver_ip(Option<String>)`](crate::operation::test_dns_answer::builders::TestDNSAnswerFluentBuilder::set_resolver_ip): <p>If you want to simulate a request from a specific DNS resolver, specify the IP address for that resolver. If you omit this value, <code>TestDnsAnswer</code> uses the IP address of a DNS resolver in the Amazon Web Services US East (N. Virginia) Region (<code>us-east-1</code>).</p>
    ///   - [`edns0_client_subnet_ip(impl Into<String>)`](crate::operation::test_dns_answer::builders::TestDNSAnswerFluentBuilder::edns0_client_subnet_ip) / [`set_edns0_client_subnet_ip(Option<String>)`](crate::operation::test_dns_answer::builders::TestDNSAnswerFluentBuilder::set_edns0_client_subnet_ip): <p>If the resolver that you specified for resolverip supports EDNS0, specify the IPv4 or IPv6 address of a client in the applicable location, for example, <code>192.0.2.44</code> or <code>2001:db8:85a3::8a2e:370:7334</code>.</p>
    ///   - [`edns0_client_subnet_mask(impl Into<String>)`](crate::operation::test_dns_answer::builders::TestDNSAnswerFluentBuilder::edns0_client_subnet_mask) / [`set_edns0_client_subnet_mask(Option<String>)`](crate::operation::test_dns_answer::builders::TestDNSAnswerFluentBuilder::set_edns0_client_subnet_mask): <p>If you specify an IP address for <code>edns0clientsubnetip</code>, you can optionally specify the number of bits of the IP address that you want the checking tool to include in the DNS query. For example, if you specify <code>192.0.2.44</code> for <code>edns0clientsubnetip</code> and <code>24</code> for <code>edns0clientsubnetmask</code>, the checking tool will simulate a request from 192.0.2.0/24. The default value is 24 bits for IPv4 addresses and 64 bits for IPv6 addresses.</p>  <p>The range of valid values depends on whether <code>edns0clientsubnetip</code> is an IPv4 or an IPv6 address:</p>  <ul>   <li> <p> <b>IPv4</b>: Specify a value between 0 and 32</p> </li>   <li> <p> <b>IPv6</b>: Specify a value between 0 and 128</p> </li>  </ul>
    /// - On success, responds with [`TestDnsAnswerOutput`](crate::operation::test_dns_answer::TestDnsAnswerOutput) with field(s):
    ///   - [`nameserver(Option<String>)`](crate::operation::test_dns_answer::TestDnsAnswerOutput::nameserver): <p>The Amazon Route 53 name server used to respond to the request.</p>
    ///   - [`record_name(Option<String>)`](crate::operation::test_dns_answer::TestDnsAnswerOutput::record_name): <p>The name of the resource record set that you submitted a request for.</p>
    ///   - [`record_type(Option<RrType>)`](crate::operation::test_dns_answer::TestDnsAnswerOutput::record_type): <p>The type of the resource record set that you submitted a request for.</p>
    ///   - [`record_data(Option<Vec<String>>)`](crate::operation::test_dns_answer::TestDnsAnswerOutput::record_data): <p>A list that contains values that Amazon Route 53 returned for this resource record set.</p>
    ///   - [`response_code(Option<String>)`](crate::operation::test_dns_answer::TestDnsAnswerOutput::response_code): <p>A code that indicates whether the request is valid or not. The most common response code is <code>NOERROR</code>, meaning that the request is valid. If the response is not valid, Amazon Route 53 returns a response code that describes the error. For a list of possible response codes, see <a href="http://www.iana.org/assignments/dns-parameters/dns-parameters.xhtml#dns-parameters-6">DNS RCODES</a> on the IANA website. </p>
    ///   - [`protocol(Option<String>)`](crate::operation::test_dns_answer::TestDnsAnswerOutput::protocol): <p>The protocol that Amazon Route 53 used to respond to the request, either <code>UDP</code> or <code>TCP</code>. </p>
    /// - On failure, responds with [`SdkError<TestDNSAnswerError>`](crate::operation::test_dns_answer::TestDNSAnswerError)
    pub fn test_dns_answer(
        &self,
    ) -> crate::operation::test_dns_answer::builders::TestDNSAnswerFluentBuilder {
        crate::operation::test_dns_answer::builders::TestDNSAnswerFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
