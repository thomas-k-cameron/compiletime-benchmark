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
pub struct GetServiceLastAccessedDetailsWithEntitiesInput {
    /// <p>The ID of the request generated by the <code>GenerateServiceLastAccessedDetails</code> operation.</p>
    #[doc(hidden)]
    pub job_id: std::option::Option<std::string::String>,
    /// <p>The service namespace for an Amazon Web Services service. Provide the service namespace to learn when the IAM entity last attempted to access the specified service.</p>
    /// <p>To learn the service namespace for a service, see <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/reference_policies_actions-resources-contextkeys.html">Actions, resources, and condition keys for Amazon Web Services services</a> in the <i>IAM User Guide</i>. Choose the name of the service to view details for that service. In the first paragraph, find the service prefix. For example, <code>(service prefix: a4b)</code>. For more information about service namespaces, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">Amazon Web Services service namespaces</a> in the&nbsp;<i>Amazon Web Services General Reference</i>.</p>
    #[doc(hidden)]
    pub service_namespace: std::option::Option<std::string::String>,
    /// <p>Use this only when paginating results to indicate the maximum number of items you want in the response. If additional items exist beyond the maximum you specify, the <code>IsTruncated</code> response element is <code>true</code>.</p>
    /// <p>If you do not include this parameter, the number of items defaults to 100. Note that IAM might return fewer results, even when there are more results available. In that case, the <code>IsTruncated</code> response element returns <code>true</code>, and <code>Marker</code> contains a value to include in the subsequent call that tells the service where to continue from.</p>
    #[doc(hidden)]
    pub max_items: std::option::Option<i32>,
    /// <p>Use this parameter only when paginating results and only after you receive a response indicating that the results are truncated. Set it to the value of the <code>Marker</code> element in the response that you received to indicate where the next call should start.</p>
    #[doc(hidden)]
    pub marker: std::option::Option<std::string::String>,
}
impl GetServiceLastAccessedDetailsWithEntitiesInput {
    /// <p>The ID of the request generated by the <code>GenerateServiceLastAccessedDetails</code> operation.</p>
    pub fn job_id(&self) -> std::option::Option<&str> {
        self.job_id.as_deref()
    }
    /// <p>The service namespace for an Amazon Web Services service. Provide the service namespace to learn when the IAM entity last attempted to access the specified service.</p>
    /// <p>To learn the service namespace for a service, see <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/reference_policies_actions-resources-contextkeys.html">Actions, resources, and condition keys for Amazon Web Services services</a> in the <i>IAM User Guide</i>. Choose the name of the service to view details for that service. In the first paragraph, find the service prefix. For example, <code>(service prefix: a4b)</code>. For more information about service namespaces, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">Amazon Web Services service namespaces</a> in the&nbsp;<i>Amazon Web Services General Reference</i>.</p>
    pub fn service_namespace(&self) -> std::option::Option<&str> {
        self.service_namespace.as_deref()
    }
    /// <p>Use this only when paginating results to indicate the maximum number of items you want in the response. If additional items exist beyond the maximum you specify, the <code>IsTruncated</code> response element is <code>true</code>.</p>
    /// <p>If you do not include this parameter, the number of items defaults to 100. Note that IAM might return fewer results, even when there are more results available. In that case, the <code>IsTruncated</code> response element returns <code>true</code>, and <code>Marker</code> contains a value to include in the subsequent call that tells the service where to continue from.</p>
    pub fn max_items(&self) -> std::option::Option<i32> {
        self.max_items
    }
    /// <p>Use this parameter only when paginating results and only after you receive a response indicating that the results are truncated. Set it to the value of the <code>Marker</code> element in the response that you received to indicate where the next call should start.</p>
    pub fn marker(&self) -> std::option::Option<&str> {
        self.marker.as_deref()
    }
}
impl GetServiceLastAccessedDetailsWithEntitiesInput {
    /// Creates a new builder-style object to manufacture [`GetServiceLastAccessedDetailsWithEntitiesInput`](crate::operation::get_service_last_accessed_details_with_entities::GetServiceLastAccessedDetailsWithEntitiesInput).
    pub fn builder() -> crate::operation::get_service_last_accessed_details_with_entities::builders::GetServiceLastAccessedDetailsWithEntitiesInputBuilder{
        crate::operation::get_service_last_accessed_details_with_entities::builders::GetServiceLastAccessedDetailsWithEntitiesInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_service_last_accessed_details_with_entities::GetServiceLastAccessedDetailsWithEntitiesInput;
/// A builder for [`GetServiceLastAccessedDetailsWithEntitiesInput`](crate::operation::get_service_last_accessed_details_with_entities::GetServiceLastAccessedDetailsWithEntitiesInput).
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
pub struct GetServiceLastAccessedDetailsWithEntitiesInputBuilder {
    pub(crate) job_id: std::option::Option<std::string::String>,
    pub(crate) service_namespace: std::option::Option<std::string::String>,
    pub(crate) max_items: std::option::Option<i32>,
    pub(crate) marker: std::option::Option<std::string::String>,
}
impl GetServiceLastAccessedDetailsWithEntitiesInputBuilder {
    /// <p>The ID of the request generated by the <code>GenerateServiceLastAccessedDetails</code> operation.</p>
    pub fn job_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.job_id = Some(input.into());
        self
    }
    /// <p>The ID of the request generated by the <code>GenerateServiceLastAccessedDetails</code> operation.</p>
    pub fn set_job_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.job_id = input;
        self
    }
    /// <p>The service namespace for an Amazon Web Services service. Provide the service namespace to learn when the IAM entity last attempted to access the specified service.</p>
    /// <p>To learn the service namespace for a service, see <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/reference_policies_actions-resources-contextkeys.html">Actions, resources, and condition keys for Amazon Web Services services</a> in the <i>IAM User Guide</i>. Choose the name of the service to view details for that service. In the first paragraph, find the service prefix. For example, <code>(service prefix: a4b)</code>. For more information about service namespaces, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">Amazon Web Services service namespaces</a> in the&nbsp;<i>Amazon Web Services General Reference</i>.</p>
    pub fn service_namespace(mut self, input: impl Into<std::string::String>) -> Self {
        self.service_namespace = Some(input.into());
        self
    }
    /// <p>The service namespace for an Amazon Web Services service. Provide the service namespace to learn when the IAM entity last attempted to access the specified service.</p>
    /// <p>To learn the service namespace for a service, see <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/reference_policies_actions-resources-contextkeys.html">Actions, resources, and condition keys for Amazon Web Services services</a> in the <i>IAM User Guide</i>. Choose the name of the service to view details for that service. In the first paragraph, find the service prefix. For example, <code>(service prefix: a4b)</code>. For more information about service namespaces, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">Amazon Web Services service namespaces</a> in the&nbsp;<i>Amazon Web Services General Reference</i>.</p>
    pub fn set_service_namespace(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.service_namespace = input;
        self
    }
    /// <p>Use this only when paginating results to indicate the maximum number of items you want in the response. If additional items exist beyond the maximum you specify, the <code>IsTruncated</code> response element is <code>true</code>.</p>
    /// <p>If you do not include this parameter, the number of items defaults to 100. Note that IAM might return fewer results, even when there are more results available. In that case, the <code>IsTruncated</code> response element returns <code>true</code>, and <code>Marker</code> contains a value to include in the subsequent call that tells the service where to continue from.</p>
    pub fn max_items(mut self, input: i32) -> Self {
        self.max_items = Some(input);
        self
    }
    /// <p>Use this only when paginating results to indicate the maximum number of items you want in the response. If additional items exist beyond the maximum you specify, the <code>IsTruncated</code> response element is <code>true</code>.</p>
    /// <p>If you do not include this parameter, the number of items defaults to 100. Note that IAM might return fewer results, even when there are more results available. In that case, the <code>IsTruncated</code> response element returns <code>true</code>, and <code>Marker</code> contains a value to include in the subsequent call that tells the service where to continue from.</p>
    pub fn set_max_items(mut self, input: std::option::Option<i32>) -> Self {
        self.max_items = input;
        self
    }
    /// <p>Use this parameter only when paginating results and only after you receive a response indicating that the results are truncated. Set it to the value of the <code>Marker</code> element in the response that you received to indicate where the next call should start.</p>
    pub fn marker(mut self, input: impl Into<std::string::String>) -> Self {
        self.marker = Some(input.into());
        self
    }
    /// <p>Use this parameter only when paginating results and only after you receive a response indicating that the results are truncated. Set it to the value of the <code>Marker</code> element in the response that you received to indicate where the next call should start.</p>
    pub fn set_marker(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.marker = input;
        self
    }
    /// Consumes the builder and constructs a [`GetServiceLastAccessedDetailsWithEntitiesInput`](crate::operation::get_service_last_accessed_details_with_entities::GetServiceLastAccessedDetailsWithEntitiesInput).
    pub fn build(self) -> Result<crate::operation::get_service_last_accessed_details_with_entities::GetServiceLastAccessedDetailsWithEntitiesInput, aws_smithy_http::operation::error::BuildError>{
        Ok(
            crate::operation::get_service_last_accessed_details_with_entities::GetServiceLastAccessedDetailsWithEntitiesInput {
                job_id: self.job_id
                ,
                service_namespace: self.service_namespace
                ,
                max_items: self.max_items
                ,
                marker: self.marker
                ,
            }
        )
    }
}
