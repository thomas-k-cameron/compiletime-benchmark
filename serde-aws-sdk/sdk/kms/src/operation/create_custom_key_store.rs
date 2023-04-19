// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

impl CreateCustomKeyStoreInput {
    /// Consumes the builder and constructs an Operation<[`CreateCustomKeyStore`](crate::operation::create_custom_key_store::CreateCustomKeyStore)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::create_custom_key_store::CreateCustomKeyStore,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::operation::error::BuildError,
    > {
        let params_result = crate::endpoint::Params::builder()
            .set_region(_config.region.as_ref().map(|r| r.as_ref().to_owned()))
            .set_use_dual_stack(_config.use_dual_stack)
            .set_use_fips(_config.use_fips)
            .set_endpoint(_config.endpoint_url.clone())
            .build()
            .map_err(|err| {
                aws_smithy_http::endpoint::ResolveEndpointError::from_source(
                    "could not construct endpoint parameters",
                    err,
                )
            });
        let (endpoint_result, params) = match params_result {
            Ok(params) => (
                _config.endpoint_resolver.resolve_endpoint(&params),
                Some(params),
            ),
            Err(e) => (Err(e), None),
        };
        let mut request = {
            fn uri_base(
                _input: &crate::operation::create_custom_key_store::CreateCustomKeyStoreInput,
                output: &mut String,
            ) -> std::result::Result<(), aws_smithy_http::operation::error::BuildError>
            {
                write!(output, "/").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::create_custom_key_store::CreateCustomKeyStoreInput,
                builder: http::request::Builder,
            ) -> std::result::Result<
                http::request::Builder,
                aws_smithy_http::operation::error::BuildError,
            > {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::CONTENT_TYPE,
                "application/x-amz-json-1.1",
            );
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::HeaderName::from_static("x-amz-target"),
                "TrentService.CreateCustomKeyStore",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from(
            crate::protocol_serde::shape_create_custom_key_store::ser_create_custom_key_store_input(&self)?
        );
        if let Some(content_length) = body.content_length() {
            request = aws_smithy_http::header::set_request_header_if_absent(
                request,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request.properties_mut().insert(endpoint_result);
        if let Some(params) = params {
            request.properties_mut().insert(params);
        }
        request
            .properties_mut()
            .insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let mut user_agent = aws_http::user_agent::AwsUserAgent::new_from_environment(
            aws_types::os_shim_internal::Env::real(),
            crate::meta::API_METADATA.clone(),
        );
        if let Some(app_name) = _config.app_name() {
            user_agent = user_agent.with_app_name(app_name.clone());
        }
        request.properties_mut().insert(user_agent);
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        if let Some(region) = &_config.region {
            request
                .properties_mut()
                .insert(aws_types::region::SigningRegion::from(region.clone()));
        }
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_http::auth::set_credentials_cache(
            &mut request.properties_mut(),
            _config.credentials_cache.clone(),
        );
        let op = aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::create_custom_key_store::CreateCustomKeyStore::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "CreateCustomKeyStore",
            "kms",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
}
/// `ParseStrictResponse` impl for `CreateCustomKeyStore`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct CreateCustomKeyStore;
impl CreateCustomKeyStore {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateCustomKeyStore {
    type Output = std::result::Result<
        crate::operation::create_custom_key_store::CreateCustomKeyStoreOutput,
        crate::operation::create_custom_key_store::CreateCustomKeyStoreError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::protocol_serde::shape_create_custom_key_store::de_create_custom_key_store_http_error(response)
        } else {
            crate::protocol_serde::shape_create_custom_key_store::de_create_custom_key_store_http_response(response)
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type CreateCustomKeyStoreErrorKind = CreateCustomKeyStoreError;
/// Error type for the `CreateCustomKeyStoreError` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum CreateCustomKeyStoreError {
    /// <p>The request was rejected because the specified CloudHSM cluster is already associated with an CloudHSM key store in the account, or it shares a backup history with an CloudHSM key store in the account. Each CloudHSM key store in the account must be associated with a different CloudHSM cluster.</p>
    /// <p>CloudHSM clusters that share a backup history have the same cluster certificate. To view the cluster certificate of an CloudHSM cluster, use the <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_DescribeClusters.html">DescribeClusters</a> operation.</p>
    CloudHsmClusterInUseException(crate::types::error::CloudHsmClusterInUseException),
    /// <p>The request was rejected because the associated CloudHSM cluster did not meet the configuration requirements for an CloudHSM key store.</p>
    /// <ul>
    /// <li> <p>The CloudHSM cluster must be configured with private subnets in at least two different Availability Zones in the Region.</p> </li>
    /// <li> <p>The <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/configure-sg.html">security group for the cluster</a> (cloudhsm-cluster-<i>
    /// <cluster-id></cluster-id></i>-sg) must include inbound rules and outbound rules that allow TCP traffic on ports 2223-2225. The <b>Source</b> in the inbound rules and the <b>Destination</b> in the outbound rules must match the security group ID. These rules are set by default when you create the CloudHSM cluster. Do not delete or change them. To get information about a particular security group, use the <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeSecurityGroups.html">DescribeSecurityGroups</a> operation.</p> </li>
    /// <li> <p>The CloudHSM cluster must contain at least as many HSMs as the operation requires. To add HSMs, use the CloudHSM <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_CreateHsm.html">CreateHsm</a> operation.</p> <p>For the <code>CreateCustomKeyStore</code>, <code>UpdateCustomKeyStore</code>, and <code>CreateKey</code> operations, the CloudHSM cluster must have at least two active HSMs, each in a different Availability Zone. For the <code>ConnectCustomKeyStore</code> operation, the CloudHSM must contain at least one active HSM.</p> </li>
    /// </ul>
    /// <p>For information about the requirements for an CloudHSM cluster that is associated with an CloudHSM key store, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keystore.html#before-keystore">Assemble the Prerequisites</a> in the <i>Key Management Service Developer Guide</i>. For information about creating a private subnet for an CloudHSM cluster, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/create-subnets.html">Create a Private Subnet</a> in the <i>CloudHSM User Guide</i>. For information about cluster security groups, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/configure-sg.html">Configure a Default Security Group</a> in the <i> <i>CloudHSM User Guide</i> </i>. </p>
    CloudHsmClusterInvalidConfigurationException(
        crate::types::error::CloudHsmClusterInvalidConfigurationException,
    ),
    /// <p>The request was rejected because the CloudHSM cluster associated with the CloudHSM key store is not active. Initialize and activate the cluster and try the command again. For detailed instructions, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/getting-started.html">Getting Started</a> in the <i>CloudHSM User Guide</i>.</p>
    CloudHsmClusterNotActiveException(crate::types::error::CloudHsmClusterNotActiveException),
    /// <p>The request was rejected because KMS cannot find the CloudHSM cluster with the specified cluster ID. Retry the request with a different cluster ID.</p>
    CloudHsmClusterNotFoundException(crate::types::error::CloudHsmClusterNotFoundException),
    /// <p>The request was rejected because the specified custom key store name is already assigned to another custom key store in the account. Try again with a custom key store name that is unique in the account.</p>
    CustomKeyStoreNameInUseException(crate::types::error::CustomKeyStoreNameInUseException),
    /// <p>The request was rejected because the trust anchor certificate in the request to create an CloudHSM key store is not the trust anchor certificate for the specified CloudHSM cluster.</p>
    /// <p>When you <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/initialize-cluster.html#sign-csr">initialize the CloudHSM cluster</a>, you create the trust anchor certificate and save it in the <code>customerCA.crt</code> file.</p>
    IncorrectTrustAnchorException(crate::types::error::IncorrectTrustAnchorException),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KmsInternalException(crate::types::error::KmsInternalException),
    /// <p>The request was rejected because a quota was exceeded. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/limits.html">Quotas</a> in the <i>Key Management Service Developer Guide</i>.</p>
    LimitExceededException(crate::types::error::LimitExceededException),
    /// <p>The request was rejected because the proxy credentials failed to authenticate to the specified external key store proxy. The specified external key store proxy rejected a status request from KMS due to invalid credentials. This can indicate an error in the credentials or in the identification of the external key store proxy.</p>
    XksProxyIncorrectAuthenticationCredentialException(
        crate::types::error::XksProxyIncorrectAuthenticationCredentialException,
    ),
    /// <p>The request was rejected because the Amazon VPC endpoint service configuration does not fulfill the requirements for an external key store proxy. For details, see the exception message.</p>
    XksProxyInvalidConfigurationException(
        crate::types::error::XksProxyInvalidConfigurationException,
    ),
    /// <p></p>
    /// <p>KMS cannot interpret the response it received from the external key store proxy. The problem might be a poorly constructed response, but it could also be a transient network issue. If you see this error repeatedly, report it to the proxy vendor.</p>
    XksProxyInvalidResponseException(crate::types::error::XksProxyInvalidResponseException),
    /// <p>The request was rejected because the concatenation of the <code>XksProxyUriEndpoint</code> is already associated with an external key store in the Amazon Web Services account and Region. Each external key store in an account and Region must use a unique external key store proxy address.</p>
    XksProxyUriEndpointInUseException(crate::types::error::XksProxyUriEndpointInUseException),
    /// <p>The request was rejected because the concatenation of the <code>XksProxyUriEndpoint</code> and <code>XksProxyUriPath</code> is already associated with an external key store in the Amazon Web Services account and Region. Each external key store in an account and Region must use a unique external key store proxy API address.</p>
    XksProxyUriInUseException(crate::types::error::XksProxyUriInUseException),
    /// <p>KMS was unable to reach the specified <code>XksProxyUriPath</code>. The path must be reachable before you create the external key store or update its settings.</p>
    /// <p>This exception is also thrown when the external key store proxy response to a <code>GetHealthStatus</code> request indicates that all external key manager instances are unavailable.</p>
    XksProxyUriUnreachableException(crate::types::error::XksProxyUriUnreachableException),
    /// <p>The request was rejected because the specified Amazon VPC endpoint service is already associated with an external key store in the Amazon Web Services account and Region. Each external key store in an Amazon Web Services account and Region must use a different Amazon VPC endpoint service.</p>
    XksProxyVpcEndpointServiceInUseException(
        crate::types::error::XksProxyVpcEndpointServiceInUseException,
    ),
    /// <p>The request was rejected because the Amazon VPC endpoint service configuration does not fulfill the requirements for an external key store proxy. For details, see the exception message and <a href="kms/latest/developerguide/vpc-connectivity.html#xks-vpc-requirements">review the requirements</a> for Amazon VPC endpoint service connectivity for an external key store.</p>
    XksProxyVpcEndpointServiceInvalidConfigurationException(
        crate::types::error::XksProxyVpcEndpointServiceInvalidConfigurationException,
    ),
    /// <p>The request was rejected because KMS could not find the specified VPC endpoint service. Use <code>DescribeCustomKeyStores</code> to verify the VPC endpoint service name for the external key store. Also, confirm that the <code>Allow principals</code> list for the VPC endpoint service includes the KMS service principal for the Region, such as <code>cks.kms.us-east-1.amazonaws.com</code>.</p>
    XksProxyVpcEndpointServiceNotFoundException(
        crate::types::error::XksProxyVpcEndpointServiceNotFoundException,
    ),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(aws_smithy_types::error::Unhandled),
}
impl aws_smithy_http::result::CreateUnhandledError for CreateCustomKeyStoreError {
    fn create_unhandled_error(
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
        meta: std::option::Option<aws_smithy_types::error::ErrorMetadata>,
    ) -> Self {
        Self::Unhandled({
            let mut builder = aws_smithy_types::error::Unhandled::builder().source(source);
            builder.set_meta(meta);
            builder.build()
        })
    }
}
impl std::fmt::Display for CreateCustomKeyStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CloudHsmClusterInUseException(_inner) => _inner.fmt(f),
            Self::CloudHsmClusterInvalidConfigurationException(_inner) => _inner.fmt(f),
            Self::CloudHsmClusterNotActiveException(_inner) => _inner.fmt(f),
            Self::CloudHsmClusterNotFoundException(_inner) => _inner.fmt(f),
            Self::CustomKeyStoreNameInUseException(_inner) => _inner.fmt(f),
            Self::IncorrectTrustAnchorException(_inner) => _inner.fmt(f),
            Self::KmsInternalException(_inner) => _inner.fmt(f),
            Self::LimitExceededException(_inner) => _inner.fmt(f),
            Self::XksProxyIncorrectAuthenticationCredentialException(_inner) => _inner.fmt(f),
            Self::XksProxyInvalidConfigurationException(_inner) => _inner.fmt(f),
            Self::XksProxyInvalidResponseException(_inner) => _inner.fmt(f),
            Self::XksProxyUriEndpointInUseException(_inner) => _inner.fmt(f),
            Self::XksProxyUriInUseException(_inner) => _inner.fmt(f),
            Self::XksProxyUriUnreachableException(_inner) => _inner.fmt(f),
            Self::XksProxyVpcEndpointServiceInUseException(_inner) => _inner.fmt(f),
            Self::XksProxyVpcEndpointServiceInvalidConfigurationException(_inner) => _inner.fmt(f),
            Self::XksProxyVpcEndpointServiceNotFoundException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for CreateCustomKeyStoreError {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::CloudHsmClusterInUseException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::CloudHsmClusterInvalidConfigurationException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::CloudHsmClusterNotActiveException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::CloudHsmClusterNotFoundException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::CustomKeyStoreNameInUseException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::IncorrectTrustAnchorException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::KmsInternalException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::LimitExceededException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::XksProxyIncorrectAuthenticationCredentialException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::XksProxyInvalidConfigurationException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::XksProxyInvalidResponseException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::XksProxyUriEndpointInUseException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::XksProxyUriInUseException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::XksProxyUriUnreachableException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::XksProxyVpcEndpointServiceInUseException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::XksProxyVpcEndpointServiceInvalidConfigurationException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::XksProxyVpcEndpointServiceNotFoundException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl aws_http::request_id::RequestId
    for crate::operation::create_custom_key_store::CreateCustomKeyStoreError
{
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for CreateCustomKeyStoreError {
    fn code(&self) -> std::option::Option<&str> {
        aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> std::option::Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl CreateCustomKeyStoreError {
    /// Creates the `CreateCustomKeyStoreError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self::Unhandled(
            aws_smithy_types::error::Unhandled::builder()
                .source(err)
                .build(),
        )
    }

    /// Creates the `CreateCustomKeyStoreError::Unhandled` variant from a `aws_smithy_types::error::ErrorMetadata`.
    pub fn generic(err: aws_smithy_types::error::ErrorMetadata) -> Self {
        Self::Unhandled(
            aws_smithy_types::error::Unhandled::builder()
                .source(err.clone())
                .meta(err)
                .build(),
        )
    }
    ///
    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    ///
    pub fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        use aws_smithy_types::error::metadata::ProvideErrorMetadata;
        match self {
            Self::CloudHsmClusterInUseException(e) => e.meta(),
            Self::CloudHsmClusterInvalidConfigurationException(e) => e.meta(),
            Self::CloudHsmClusterNotActiveException(e) => e.meta(),
            Self::CloudHsmClusterNotFoundException(e) => e.meta(),
            Self::CustomKeyStoreNameInUseException(e) => e.meta(),
            Self::IncorrectTrustAnchorException(e) => e.meta(),
            Self::KmsInternalException(e) => e.meta(),
            Self::LimitExceededException(e) => e.meta(),
            Self::XksProxyIncorrectAuthenticationCredentialException(e) => e.meta(),
            Self::XksProxyInvalidConfigurationException(e) => e.meta(),
            Self::XksProxyInvalidResponseException(e) => e.meta(),
            Self::XksProxyUriEndpointInUseException(e) => e.meta(),
            Self::XksProxyUriInUseException(e) => e.meta(),
            Self::XksProxyUriUnreachableException(e) => e.meta(),
            Self::XksProxyVpcEndpointServiceInUseException(e) => e.meta(),
            Self::XksProxyVpcEndpointServiceInvalidConfigurationException(e) => e.meta(),
            Self::XksProxyVpcEndpointServiceNotFoundException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `CreateCustomKeyStoreError::CloudHsmClusterInUseException`.
    pub fn is_cloud_hsm_cluster_in_use_exception(&self) -> bool {
        matches!(self, Self::CloudHsmClusterInUseException(_))
    }
    /// Returns `true` if the error kind is `CreateCustomKeyStoreError::CloudHsmClusterInvalidConfigurationException`.
    pub fn is_cloud_hsm_cluster_invalid_configuration_exception(&self) -> bool {
        matches!(self, Self::CloudHsmClusterInvalidConfigurationException(_))
    }
    /// Returns `true` if the error kind is `CreateCustomKeyStoreError::CloudHsmClusterNotActiveException`.
    pub fn is_cloud_hsm_cluster_not_active_exception(&self) -> bool {
        matches!(self, Self::CloudHsmClusterNotActiveException(_))
    }
    /// Returns `true` if the error kind is `CreateCustomKeyStoreError::CloudHsmClusterNotFoundException`.
    pub fn is_cloud_hsm_cluster_not_found_exception(&self) -> bool {
        matches!(self, Self::CloudHsmClusterNotFoundException(_))
    }
    /// Returns `true` if the error kind is `CreateCustomKeyStoreError::CustomKeyStoreNameInUseException`.
    pub fn is_custom_key_store_name_in_use_exception(&self) -> bool {
        matches!(self, Self::CustomKeyStoreNameInUseException(_))
    }
    /// Returns `true` if the error kind is `CreateCustomKeyStoreError::IncorrectTrustAnchorException`.
    pub fn is_incorrect_trust_anchor_exception(&self) -> bool {
        matches!(self, Self::IncorrectTrustAnchorException(_))
    }
    /// Returns `true` if the error kind is `CreateCustomKeyStoreError::KmsInternalException`.
    pub fn is_kms_internal_exception(&self) -> bool {
        matches!(self, Self::KmsInternalException(_))
    }
    /// Returns `true` if the error kind is `CreateCustomKeyStoreError::LimitExceededException`.
    pub fn is_limit_exceeded_exception(&self) -> bool {
        matches!(self, Self::LimitExceededException(_))
    }
    /// Returns `true` if the error kind is `CreateCustomKeyStoreError::XksProxyIncorrectAuthenticationCredentialException`.
    pub fn is_xks_proxy_incorrect_authentication_credential_exception(&self) -> bool {
        matches!(
            self,
            Self::XksProxyIncorrectAuthenticationCredentialException(_)
        )
    }
    /// Returns `true` if the error kind is `CreateCustomKeyStoreError::XksProxyInvalidConfigurationException`.
    pub fn is_xks_proxy_invalid_configuration_exception(&self) -> bool {
        matches!(self, Self::XksProxyInvalidConfigurationException(_))
    }
    /// Returns `true` if the error kind is `CreateCustomKeyStoreError::XksProxyInvalidResponseException`.
    pub fn is_xks_proxy_invalid_response_exception(&self) -> bool {
        matches!(self, Self::XksProxyInvalidResponseException(_))
    }
    /// Returns `true` if the error kind is `CreateCustomKeyStoreError::XksProxyUriEndpointInUseException`.
    pub fn is_xks_proxy_uri_endpoint_in_use_exception(&self) -> bool {
        matches!(self, Self::XksProxyUriEndpointInUseException(_))
    }
    /// Returns `true` if the error kind is `CreateCustomKeyStoreError::XksProxyUriInUseException`.
    pub fn is_xks_proxy_uri_in_use_exception(&self) -> bool {
        matches!(self, Self::XksProxyUriInUseException(_))
    }
    /// Returns `true` if the error kind is `CreateCustomKeyStoreError::XksProxyUriUnreachableException`.
    pub fn is_xks_proxy_uri_unreachable_exception(&self) -> bool {
        matches!(self, Self::XksProxyUriUnreachableException(_))
    }
    /// Returns `true` if the error kind is `CreateCustomKeyStoreError::XksProxyVpcEndpointServiceInUseException`.
    pub fn is_xks_proxy_vpc_endpoint_service_in_use_exception(&self) -> bool {
        matches!(self, Self::XksProxyVpcEndpointServiceInUseException(_))
    }
    /// Returns `true` if the error kind is `CreateCustomKeyStoreError::XksProxyVpcEndpointServiceInvalidConfigurationException`.
    pub fn is_xks_proxy_vpc_endpoint_service_invalid_configuration_exception(&self) -> bool {
        matches!(
            self,
            Self::XksProxyVpcEndpointServiceInvalidConfigurationException(_)
        )
    }
    /// Returns `true` if the error kind is `CreateCustomKeyStoreError::XksProxyVpcEndpointServiceNotFoundException`.
    pub fn is_xks_proxy_vpc_endpoint_service_not_found_exception(&self) -> bool {
        matches!(self, Self::XksProxyVpcEndpointServiceNotFoundException(_))
    }
}
impl std::error::Error for CreateCustomKeyStoreError {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::CloudHsmClusterInUseException(_inner) => Some(_inner),
            Self::CloudHsmClusterInvalidConfigurationException(_inner) => Some(_inner),
            Self::CloudHsmClusterNotActiveException(_inner) => Some(_inner),
            Self::CloudHsmClusterNotFoundException(_inner) => Some(_inner),
            Self::CustomKeyStoreNameInUseException(_inner) => Some(_inner),
            Self::IncorrectTrustAnchorException(_inner) => Some(_inner),
            Self::KmsInternalException(_inner) => Some(_inner),
            Self::LimitExceededException(_inner) => Some(_inner),
            Self::XksProxyIncorrectAuthenticationCredentialException(_inner) => Some(_inner),
            Self::XksProxyInvalidConfigurationException(_inner) => Some(_inner),
            Self::XksProxyInvalidResponseException(_inner) => Some(_inner),
            Self::XksProxyUriEndpointInUseException(_inner) => Some(_inner),
            Self::XksProxyUriInUseException(_inner) => Some(_inner),
            Self::XksProxyUriUnreachableException(_inner) => Some(_inner),
            Self::XksProxyVpcEndpointServiceInUseException(_inner) => Some(_inner),
            Self::XksProxyVpcEndpointServiceInvalidConfigurationException(_inner) => Some(_inner),
            Self::XksProxyVpcEndpointServiceNotFoundException(_inner) => Some(_inner),
            Self::Unhandled(_inner) => Some(_inner),
        }
    }
}

pub use crate::operation::create_custom_key_store::_create_custom_key_store_output::CreateCustomKeyStoreOutput;

pub use crate::operation::create_custom_key_store::_create_custom_key_store_input::CreateCustomKeyStoreInput;

mod _create_custom_key_store_input;

mod _create_custom_key_store_output;

/// Builders
pub mod builders;
