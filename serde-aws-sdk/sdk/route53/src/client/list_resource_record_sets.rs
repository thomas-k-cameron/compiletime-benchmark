// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListResourceRecordSets`](crate::operation::list_resource_record_sets::builders::ListResourceRecordSetsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`hosted_zone_id(impl Into<String>)`](crate::operation::list_resource_record_sets::builders::ListResourceRecordSetsFluentBuilder::hosted_zone_id) / [`set_hosted_zone_id(Option<String>)`](crate::operation::list_resource_record_sets::builders::ListResourceRecordSetsFluentBuilder::set_hosted_zone_id): <p>The ID of the hosted zone that contains the resource record sets that you want to list.</p>
    ///   - [`start_record_name(impl Into<String>)`](crate::operation::list_resource_record_sets::builders::ListResourceRecordSetsFluentBuilder::start_record_name) / [`set_start_record_name(Option<String>)`](crate::operation::list_resource_record_sets::builders::ListResourceRecordSetsFluentBuilder::set_start_record_name): <p>The first name in the lexicographic ordering of resource record sets that you want to list. If the specified record name doesn't exist, the results begin with the first resource record set that has a name greater than the value of <code>name</code>.</p>
    ///   - [`start_record_type(RrType)`](crate::operation::list_resource_record_sets::builders::ListResourceRecordSetsFluentBuilder::start_record_type) / [`set_start_record_type(Option<RrType>)`](crate::operation::list_resource_record_sets::builders::ListResourceRecordSetsFluentBuilder::set_start_record_type): <p>The type of resource record set to begin the record listing from.</p>  <p>Valid values for basic resource record sets: <code>A</code> | <code>AAAA</code> | <code>CAA</code> | <code>CNAME</code> | <code>MX</code> | <code>NAPTR</code> | <code>NS</code> | <code>PTR</code> | <code>SOA</code> | <code>SPF</code> | <code>SRV</code> | <code>TXT</code> </p>  <p>Values for weighted, latency, geolocation, and failover resource record sets: <code>A</code> | <code>AAAA</code> | <code>CAA</code> | <code>CNAME</code> | <code>MX</code> | <code>NAPTR</code> | <code>PTR</code> | <code>SPF</code> | <code>SRV</code> | <code>TXT</code> </p>  <p>Values for alias resource record sets: </p>  <ul>   <li> <p> <b>API Gateway custom regional API or edge-optimized API</b>: A</p> </li>   <li> <p> <b>CloudFront distribution</b>: A or AAAA</p> </li>   <li> <p> <b>Elastic Beanstalk environment that has a regionalized subdomain</b>: A</p> </li>   <li> <p> <b>Elastic Load Balancing load balancer</b>: A | AAAA</p> </li>   <li> <p> <b>S3 bucket</b>: A</p> </li>   <li> <p> <b>VPC interface VPC endpoint</b>: A</p> </li>   <li> <p> <b>Another resource record set in this hosted zone:</b> The type of the resource record set that the alias references.</p> </li>  </ul>  <p>Constraint: Specifying <code>type</code> without specifying <code>name</code> returns an <code>InvalidInput</code> error.</p>
    ///   - [`start_record_identifier(impl Into<String>)`](crate::operation::list_resource_record_sets::builders::ListResourceRecordSetsFluentBuilder::start_record_identifier) / [`set_start_record_identifier(Option<String>)`](crate::operation::list_resource_record_sets::builders::ListResourceRecordSetsFluentBuilder::set_start_record_identifier): <p> <i>Resource record sets that have a routing policy other than simple:</i> If results were truncated for a given DNS name and type, specify the value of <code>NextRecordIdentifier</code> from the previous response to get the next resource record set that has the current DNS name and type.</p>
    ///   - [`max_items(i32)`](crate::operation::list_resource_record_sets::builders::ListResourceRecordSetsFluentBuilder::max_items) / [`set_max_items(Option<i32>)`](crate::operation::list_resource_record_sets::builders::ListResourceRecordSetsFluentBuilder::set_max_items): <p>(Optional) The maximum number of resource records sets to include in the response body for this request. If the response includes more than <code>maxitems</code> resource record sets, the value of the <code>IsTruncated</code> element in the response is <code>true</code>, and the values of the <code>NextRecordName</code> and <code>NextRecordType</code> elements in the response identify the first resource record set in the next group of <code>maxitems</code> resource record sets.</p>
    /// - On success, responds with [`ListResourceRecordSetsOutput`](crate::operation::list_resource_record_sets::ListResourceRecordSetsOutput) with field(s):
    ///   - [`resource_record_sets(Option<Vec<ResourceRecordSet>>)`](crate::operation::list_resource_record_sets::ListResourceRecordSetsOutput::resource_record_sets): <p>Information about multiple resource record sets.</p>
    ///   - [`is_truncated(bool)`](crate::operation::list_resource_record_sets::ListResourceRecordSetsOutput::is_truncated): <p>A flag that indicates whether more resource record sets remain to be listed. If your results were truncated, you can make a follow-up pagination request by using the <code>NextRecordName</code> element.</p>
    ///   - [`next_record_name(Option<String>)`](crate::operation::list_resource_record_sets::ListResourceRecordSetsOutput::next_record_name): <p>If the results were truncated, the name of the next record in the list.</p>  <p>This element is present only if <code>IsTruncated</code> is true. </p>
    ///   - [`next_record_type(Option<RrType>)`](crate::operation::list_resource_record_sets::ListResourceRecordSetsOutput::next_record_type): <p>If the results were truncated, the type of the next record in the list.</p>  <p>This element is present only if <code>IsTruncated</code> is true. </p>
    ///   - [`next_record_identifier(Option<String>)`](crate::operation::list_resource_record_sets::ListResourceRecordSetsOutput::next_record_identifier): <p> <i>Resource record sets that have a routing policy other than simple:</i> If results were truncated for a given DNS name and type, the value of <code>SetIdentifier</code> for the next resource record set that has the current DNS name and type.</p>  <p>For information about routing policies, see <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/routing-policy.html">Choosing a Routing Policy</a> in the <i>Amazon Route 53 Developer Guide</i>.</p>
    ///   - [`max_items(Option<i32>)`](crate::operation::list_resource_record_sets::ListResourceRecordSetsOutput::max_items): <p>The maximum number of records you requested.</p>
    /// - On failure, responds with [`SdkError<ListResourceRecordSetsError>`](crate::operation::list_resource_record_sets::ListResourceRecordSetsError)
    pub fn list_resource_record_sets(
        &self,
    ) -> crate::operation::list_resource_record_sets::builders::ListResourceRecordSetsFluentBuilder
    {
        crate::operation::list_resource_record_sets::builders::ListResourceRecordSetsFluentBuilder::new(self.handle.clone())
    }
}
