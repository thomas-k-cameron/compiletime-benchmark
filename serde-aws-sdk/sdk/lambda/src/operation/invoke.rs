// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

impl InvokeInput {
    /// Consumes the builder and constructs an Operation<[`Invoke`](crate::operation::invoke::Invoke)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::invoke::Invoke,
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
                _input: &crate::operation::invoke::InvokeInput,
                output: &mut String,
            ) -> std::result::Result<(), aws_smithy_http::operation::error::BuildError>
            {
                let input_1 = &_input.function_name;
                let input_1 = input_1.as_ref().ok_or_else(|| {
                    aws_smithy_http::operation::error::BuildError::missing_field(
                        "function_name",
                        "cannot be empty or unset",
                    )
                })?;
                let function_name = aws_smithy_http::label::fmt_string(
                    input_1,
                    aws_smithy_http::label::EncodingStrategy::Default,
                );
                if function_name.is_empty() {
                    return Err(
                        aws_smithy_http::operation::error::BuildError::missing_field(
                            "function_name",
                            "cannot be empty or unset",
                        ),
                    );
                }
                write!(
                    output,
                    "/2015-03-31/functions/{FunctionName}/invocations",
                    FunctionName = function_name
                )
                .expect("formatting should succeed");
                Ok(())
            }
            fn uri_query(
                _input: &crate::operation::invoke::InvokeInput,
                mut output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::error::BuildError> {
                let mut query = aws_smithy_http::query::Writer::new(&mut output);
                if let Some(inner_2) = &_input.qualifier {
                    {
                        query.push_kv("Qualifier", &aws_smithy_http::query::fmt_string(&inner_2));
                    }
                }
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::invoke::InvokeInput,
                builder: http::request::Builder,
            ) -> std::result::Result<
                http::request::Builder,
                aws_smithy_http::operation::error::BuildError,
            > {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                uri_query(input, &mut uri)?;
                let builder =
                    crate::protocol_serde::shape_invoke::ser_invoke_headers(input, builder)?;
                Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::CONTENT_TYPE,
                "application/octet-stream",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from(
            crate::protocol_serde::shape_invoke_input::ser_payload_http_payload(self.payload)?,
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
            crate::operation::invoke::Invoke::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "Invoke", "lambda",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
}
/// `ParseStrictResponse` impl for `Invoke`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct Invoke;
impl Invoke {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl aws_smithy_http::response::ParseStrictResponse for Invoke {
    type Output = std::result::Result<
        crate::operation::invoke::InvokeOutput,
        crate::operation::invoke::InvokeError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::protocol_serde::shape_invoke::de_invoke_http_error(response)
        } else {
            crate::protocol_serde::shape_invoke::de_invoke_http_response(response)
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type InvokeErrorKind = InvokeError;
/// Error type for the `InvokeError` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum InvokeError {
    /// <p>Need additional permissions to configure VPC settings.</p>
    Ec2AccessDeniedException(crate::types::error::Ec2AccessDeniedException),
    /// <p>Amazon EC2 throttled Lambda during Lambda function initialization using the execution role provided for the function.</p>
    Ec2ThrottledException(crate::types::error::Ec2ThrottledException),
    /// <p>Lambda received an unexpected Amazon EC2 client exception while setting up for the Lambda function.</p>
    Ec2UnexpectedException(crate::types::error::Ec2UnexpectedException),
    /// <p>An error occurred when reading from or writing to a connected file system.</p>
    EfsioException(crate::types::error::EfsioException),
    /// <p>The Lambda function couldn't make a network connection to the configured file system.</p>
    EfsMountConnectivityException(crate::types::error::EfsMountConnectivityException),
    /// <p>The Lambda function couldn't mount the configured file system due to a permission or configuration issue.</p>
    EfsMountFailureException(crate::types::error::EfsMountFailureException),
    /// <p>The Lambda function made a network connection to the configured file system, but the mount operation timed out.</p>
    EfsMountTimeoutException(crate::types::error::EfsMountTimeoutException),
    /// <p>Lambda couldn't create an elastic network interface in the VPC, specified as part of Lambda function configuration, because the limit for network interfaces has been reached. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/gettingstarted-limits.html">Lambda quotas</a>.</p>
    EniLimitReachedException(crate::types::error::EniLimitReachedException),
    /// <p>One of the parameters in the request is not valid.</p>
    InvalidParameterValueException(crate::types::error::InvalidParameterValueException),
    /// <p>The request body could not be parsed as JSON.</p>
    InvalidRequestContentException(crate::types::error::InvalidRequestContentException),
    /// <p>The runtime or runtime version specified is not supported.</p>
    InvalidRuntimeException(crate::types::error::InvalidRuntimeException),
    /// <p>The security group ID provided in the Lambda function VPC configuration is not valid.</p>
    InvalidSecurityGroupIdException(crate::types::error::InvalidSecurityGroupIdException),
    /// <p>The subnet ID provided in the Lambda function VPC configuration is not valid.</p>
    InvalidSubnetIdException(crate::types::error::InvalidSubnetIdException),
    /// <p>Lambda could not unzip the deployment package.</p>
    InvalidZipFileException(crate::types::error::InvalidZipFileException),
    /// <p>Lambda couldn't decrypt the environment variables because KMS access was denied. Check the Lambda function's KMS permissions.</p>
    KmsAccessDeniedException(crate::types::error::KmsAccessDeniedException),
    /// <p>Lambda couldn't decrypt the environment variables because the KMS key used is disabled. Check the Lambda function's KMS key settings.</p>
    KmsDisabledException(crate::types::error::KmsDisabledException),
    /// <p>Lambda couldn't decrypt the environment variables because the state of the KMS key used is not valid for Decrypt. Check the function's KMS key settings.</p>
    KmsInvalidStateException(crate::types::error::KmsInvalidStateException),
    /// <p>Lambda couldn't decrypt the environment variables because the KMS key was not found. Check the function's KMS key settings.</p>
    KmsNotFoundException(crate::types::error::KmsNotFoundException),
    /// <p>The request payload exceeded the <code>Invoke</code> request body JSON input quota. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/gettingstarted-limits.html">Lambda quotas</a>.</p>
    RequestTooLargeException(crate::types::error::RequestTooLargeException),
    /// <p>The resource already exists, or another operation is in progress.</p>
    ResourceConflictException(crate::types::error::ResourceConflictException),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
    /// <p>The function is inactive and its VPC connection is no longer available. Wait for the VPC connection to reestablish and try again.</p>
    ResourceNotReadyException(crate::types::error::ResourceNotReadyException),
    /// <p>The Lambda service encountered an internal error.</p>
    ServiceException(crate::types::error::ServiceException),
    /// <p>The <code>afterRestore()</code> <a href="https://docs.aws.amazon.com/lambda/latest/dg/snapstart-runtime-hooks.html">runtime hook</a> encountered an error. For more information, check the Amazon CloudWatch logs.</p>
    SnapStartException(crate::types::error::SnapStartException),
    /// <p>Lambda is initializing your function. You can invoke the function when the <a href="https://docs.aws.amazon.com/lambda/latest/dg/functions-states.html">function state</a> becomes <code>Active</code>.</p>
    SnapStartNotReadyException(crate::types::error::SnapStartNotReadyException),
    /// <p>Lambda couldn't restore the snapshot within the timeout limit.</p>
    SnapStartTimeoutException(crate::types::error::SnapStartTimeoutException),
    /// <p>Lambda couldn't set up VPC access for the Lambda function because one or more configured subnets has no available IP addresses.</p>
    SubnetIpAddressLimitReachedException(crate::types::error::SubnetIpAddressLimitReachedException),
    /// <p>The request throughput limit was exceeded. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/gettingstarted-limits.html#api-requests">Lambda quotas</a>.</p>
    TooManyRequestsException(crate::types::error::TooManyRequestsException),
    /// <p>The content type of the <code>Invoke</code> request body is not JSON.</p>
    UnsupportedMediaTypeException(crate::types::error::UnsupportedMediaTypeException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(aws_smithy_types::error::Unhandled),
}
impl aws_smithy_http::result::CreateUnhandledError for InvokeError {
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
impl std::fmt::Display for InvokeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ec2AccessDeniedException(_inner) => _inner.fmt(f),
            Self::Ec2ThrottledException(_inner) => _inner.fmt(f),
            Self::Ec2UnexpectedException(_inner) => _inner.fmt(f),
            Self::EfsioException(_inner) => _inner.fmt(f),
            Self::EfsMountConnectivityException(_inner) => _inner.fmt(f),
            Self::EfsMountFailureException(_inner) => _inner.fmt(f),
            Self::EfsMountTimeoutException(_inner) => _inner.fmt(f),
            Self::EniLimitReachedException(_inner) => _inner.fmt(f),
            Self::InvalidParameterValueException(_inner) => _inner.fmt(f),
            Self::InvalidRequestContentException(_inner) => _inner.fmt(f),
            Self::InvalidRuntimeException(_inner) => _inner.fmt(f),
            Self::InvalidSecurityGroupIdException(_inner) => _inner.fmt(f),
            Self::InvalidSubnetIdException(_inner) => _inner.fmt(f),
            Self::InvalidZipFileException(_inner) => _inner.fmt(f),
            Self::KmsAccessDeniedException(_inner) => _inner.fmt(f),
            Self::KmsDisabledException(_inner) => _inner.fmt(f),
            Self::KmsInvalidStateException(_inner) => _inner.fmt(f),
            Self::KmsNotFoundException(_inner) => _inner.fmt(f),
            Self::RequestTooLargeException(_inner) => _inner.fmt(f),
            Self::ResourceConflictException(_inner) => _inner.fmt(f),
            Self::ResourceNotFoundException(_inner) => _inner.fmt(f),
            Self::ResourceNotReadyException(_inner) => _inner.fmt(f),
            Self::ServiceException(_inner) => _inner.fmt(f),
            Self::SnapStartException(_inner) => _inner.fmt(f),
            Self::SnapStartNotReadyException(_inner) => _inner.fmt(f),
            Self::SnapStartTimeoutException(_inner) => _inner.fmt(f),
            Self::SubnetIpAddressLimitReachedException(_inner) => _inner.fmt(f),
            Self::TooManyRequestsException(_inner) => _inner.fmt(f),
            Self::UnsupportedMediaTypeException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for InvokeError {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::Ec2AccessDeniedException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Ec2ThrottledException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Ec2UnexpectedException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::EfsioException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::EfsMountConnectivityException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::EfsMountFailureException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::EfsMountTimeoutException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::EniLimitReachedException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidParameterValueException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidRequestContentException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidRuntimeException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidSecurityGroupIdException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidSubnetIdException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidZipFileException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::KmsAccessDeniedException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::KmsDisabledException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::KmsInvalidStateException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::KmsNotFoundException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::RequestTooLargeException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ResourceConflictException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ResourceNotFoundException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ResourceNotReadyException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ServiceException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::SnapStartException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::SnapStartNotReadyException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::SnapStartTimeoutException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::SubnetIpAddressLimitReachedException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::TooManyRequestsException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::UnsupportedMediaTypeException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl aws_http::request_id::RequestId for crate::operation::invoke::InvokeError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for InvokeError {
    fn code(&self) -> std::option::Option<&str> {
        aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> std::option::Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl InvokeError {
    /// Creates the `InvokeError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self::Unhandled(
            aws_smithy_types::error::Unhandled::builder()
                .source(err)
                .build(),
        )
    }

    /// Creates the `InvokeError::Unhandled` variant from a `aws_smithy_types::error::ErrorMetadata`.
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
            Self::Ec2AccessDeniedException(e) => e.meta(),
            Self::Ec2ThrottledException(e) => e.meta(),
            Self::Ec2UnexpectedException(e) => e.meta(),
            Self::EfsioException(e) => e.meta(),
            Self::EfsMountConnectivityException(e) => e.meta(),
            Self::EfsMountFailureException(e) => e.meta(),
            Self::EfsMountTimeoutException(e) => e.meta(),
            Self::EniLimitReachedException(e) => e.meta(),
            Self::InvalidParameterValueException(e) => e.meta(),
            Self::InvalidRequestContentException(e) => e.meta(),
            Self::InvalidRuntimeException(e) => e.meta(),
            Self::InvalidSecurityGroupIdException(e) => e.meta(),
            Self::InvalidSubnetIdException(e) => e.meta(),
            Self::InvalidZipFileException(e) => e.meta(),
            Self::KmsAccessDeniedException(e) => e.meta(),
            Self::KmsDisabledException(e) => e.meta(),
            Self::KmsInvalidStateException(e) => e.meta(),
            Self::KmsNotFoundException(e) => e.meta(),
            Self::RequestTooLargeException(e) => e.meta(),
            Self::ResourceConflictException(e) => e.meta(),
            Self::ResourceNotFoundException(e) => e.meta(),
            Self::ResourceNotReadyException(e) => e.meta(),
            Self::ServiceException(e) => e.meta(),
            Self::SnapStartException(e) => e.meta(),
            Self::SnapStartNotReadyException(e) => e.meta(),
            Self::SnapStartTimeoutException(e) => e.meta(),
            Self::SubnetIpAddressLimitReachedException(e) => e.meta(),
            Self::TooManyRequestsException(e) => e.meta(),
            Self::UnsupportedMediaTypeException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `InvokeError::Ec2AccessDeniedException`.
    pub fn is_ec2_access_denied_exception(&self) -> bool {
        matches!(self, Self::Ec2AccessDeniedException(_))
    }
    /// Returns `true` if the error kind is `InvokeError::Ec2ThrottledException`.
    pub fn is_ec2_throttled_exception(&self) -> bool {
        matches!(self, Self::Ec2ThrottledException(_))
    }
    /// Returns `true` if the error kind is `InvokeError::Ec2UnexpectedException`.
    pub fn is_ec2_unexpected_exception(&self) -> bool {
        matches!(self, Self::Ec2UnexpectedException(_))
    }
    /// Returns `true` if the error kind is `InvokeError::EfsioException`.
    pub fn is_efsio_exception(&self) -> bool {
        matches!(self, Self::EfsioException(_))
    }
    /// Returns `true` if the error kind is `InvokeError::EfsMountConnectivityException`.
    pub fn is_efs_mount_connectivity_exception(&self) -> bool {
        matches!(self, Self::EfsMountConnectivityException(_))
    }
    /// Returns `true` if the error kind is `InvokeError::EfsMountFailureException`.
    pub fn is_efs_mount_failure_exception(&self) -> bool {
        matches!(self, Self::EfsMountFailureException(_))
    }
    /// Returns `true` if the error kind is `InvokeError::EfsMountTimeoutException`.
    pub fn is_efs_mount_timeout_exception(&self) -> bool {
        matches!(self, Self::EfsMountTimeoutException(_))
    }
    /// Returns `true` if the error kind is `InvokeError::EniLimitReachedException`.
    pub fn is_eni_limit_reached_exception(&self) -> bool {
        matches!(self, Self::EniLimitReachedException(_))
    }
    /// Returns `true` if the error kind is `InvokeError::InvalidParameterValueException`.
    pub fn is_invalid_parameter_value_exception(&self) -> bool {
        matches!(self, Self::InvalidParameterValueException(_))
    }
    /// Returns `true` if the error kind is `InvokeError::InvalidRequestContentException`.
    pub fn is_invalid_request_content_exception(&self) -> bool {
        matches!(self, Self::InvalidRequestContentException(_))
    }
    /// Returns `true` if the error kind is `InvokeError::InvalidRuntimeException`.
    pub fn is_invalid_runtime_exception(&self) -> bool {
        matches!(self, Self::InvalidRuntimeException(_))
    }
    /// Returns `true` if the error kind is `InvokeError::InvalidSecurityGroupIdException`.
    pub fn is_invalid_security_group_id_exception(&self) -> bool {
        matches!(self, Self::InvalidSecurityGroupIdException(_))
    }
    /// Returns `true` if the error kind is `InvokeError::InvalidSubnetIdException`.
    pub fn is_invalid_subnet_id_exception(&self) -> bool {
        matches!(self, Self::InvalidSubnetIdException(_))
    }
    /// Returns `true` if the error kind is `InvokeError::InvalidZipFileException`.
    pub fn is_invalid_zip_file_exception(&self) -> bool {
        matches!(self, Self::InvalidZipFileException(_))
    }
    /// Returns `true` if the error kind is `InvokeError::KmsAccessDeniedException`.
    pub fn is_kms_access_denied_exception(&self) -> bool {
        matches!(self, Self::KmsAccessDeniedException(_))
    }
    /// Returns `true` if the error kind is `InvokeError::KmsDisabledException`.
    pub fn is_kms_disabled_exception(&self) -> bool {
        matches!(self, Self::KmsDisabledException(_))
    }
    /// Returns `true` if the error kind is `InvokeError::KmsInvalidStateException`.
    pub fn is_kms_invalid_state_exception(&self) -> bool {
        matches!(self, Self::KmsInvalidStateException(_))
    }
    /// Returns `true` if the error kind is `InvokeError::KmsNotFoundException`.
    pub fn is_kms_not_found_exception(&self) -> bool {
        matches!(self, Self::KmsNotFoundException(_))
    }
    /// Returns `true` if the error kind is `InvokeError::RequestTooLargeException`.
    pub fn is_request_too_large_exception(&self) -> bool {
        matches!(self, Self::RequestTooLargeException(_))
    }
    /// Returns `true` if the error kind is `InvokeError::ResourceConflictException`.
    pub fn is_resource_conflict_exception(&self) -> bool {
        matches!(self, Self::ResourceConflictException(_))
    }
    /// Returns `true` if the error kind is `InvokeError::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(self, Self::ResourceNotFoundException(_))
    }
    /// Returns `true` if the error kind is `InvokeError::ResourceNotReadyException`.
    pub fn is_resource_not_ready_exception(&self) -> bool {
        matches!(self, Self::ResourceNotReadyException(_))
    }
    /// Returns `true` if the error kind is `InvokeError::ServiceException`.
    pub fn is_service_exception(&self) -> bool {
        matches!(self, Self::ServiceException(_))
    }
    /// Returns `true` if the error kind is `InvokeError::SnapStartException`.
    pub fn is_snap_start_exception(&self) -> bool {
        matches!(self, Self::SnapStartException(_))
    }
    /// Returns `true` if the error kind is `InvokeError::SnapStartNotReadyException`.
    pub fn is_snap_start_not_ready_exception(&self) -> bool {
        matches!(self, Self::SnapStartNotReadyException(_))
    }
    /// Returns `true` if the error kind is `InvokeError::SnapStartTimeoutException`.
    pub fn is_snap_start_timeout_exception(&self) -> bool {
        matches!(self, Self::SnapStartTimeoutException(_))
    }
    /// Returns `true` if the error kind is `InvokeError::SubnetIpAddressLimitReachedException`.
    pub fn is_subnet_ip_address_limit_reached_exception(&self) -> bool {
        matches!(self, Self::SubnetIpAddressLimitReachedException(_))
    }
    /// Returns `true` if the error kind is `InvokeError::TooManyRequestsException`.
    pub fn is_too_many_requests_exception(&self) -> bool {
        matches!(self, Self::TooManyRequestsException(_))
    }
    /// Returns `true` if the error kind is `InvokeError::UnsupportedMediaTypeException`.
    pub fn is_unsupported_media_type_exception(&self) -> bool {
        matches!(self, Self::UnsupportedMediaTypeException(_))
    }
}
impl std::error::Error for InvokeError {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Ec2AccessDeniedException(_inner) => Some(_inner),
            Self::Ec2ThrottledException(_inner) => Some(_inner),
            Self::Ec2UnexpectedException(_inner) => Some(_inner),
            Self::EfsioException(_inner) => Some(_inner),
            Self::EfsMountConnectivityException(_inner) => Some(_inner),
            Self::EfsMountFailureException(_inner) => Some(_inner),
            Self::EfsMountTimeoutException(_inner) => Some(_inner),
            Self::EniLimitReachedException(_inner) => Some(_inner),
            Self::InvalidParameterValueException(_inner) => Some(_inner),
            Self::InvalidRequestContentException(_inner) => Some(_inner),
            Self::InvalidRuntimeException(_inner) => Some(_inner),
            Self::InvalidSecurityGroupIdException(_inner) => Some(_inner),
            Self::InvalidSubnetIdException(_inner) => Some(_inner),
            Self::InvalidZipFileException(_inner) => Some(_inner),
            Self::KmsAccessDeniedException(_inner) => Some(_inner),
            Self::KmsDisabledException(_inner) => Some(_inner),
            Self::KmsInvalidStateException(_inner) => Some(_inner),
            Self::KmsNotFoundException(_inner) => Some(_inner),
            Self::RequestTooLargeException(_inner) => Some(_inner),
            Self::ResourceConflictException(_inner) => Some(_inner),
            Self::ResourceNotFoundException(_inner) => Some(_inner),
            Self::ResourceNotReadyException(_inner) => Some(_inner),
            Self::ServiceException(_inner) => Some(_inner),
            Self::SnapStartException(_inner) => Some(_inner),
            Self::SnapStartNotReadyException(_inner) => Some(_inner),
            Self::SnapStartTimeoutException(_inner) => Some(_inner),
            Self::SubnetIpAddressLimitReachedException(_inner) => Some(_inner),
            Self::TooManyRequestsException(_inner) => Some(_inner),
            Self::UnsupportedMediaTypeException(_inner) => Some(_inner),
            Self::Unhandled(_inner) => Some(_inner),
        }
    }
}

pub use crate::operation::invoke::_invoke_output::InvokeOutput;

pub use crate::operation::invoke::_invoke_input::InvokeInput;

mod _invoke_input;

mod _invoke_output;

/// Builders
pub mod builders;