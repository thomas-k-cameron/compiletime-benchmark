// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

impl CreateVirtualMfaDeviceInput {
    /// Consumes the builder and constructs an Operation<[`CreateVirtualMFADevice`](crate::operation::create_virtual_mfa_device::CreateVirtualMFADevice)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::create_virtual_mfa_device::CreateVirtualMFADevice,
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
                _input: &crate::operation::create_virtual_mfa_device::CreateVirtualMfaDeviceInput,
                output: &mut String,
            ) -> std::result::Result<(), aws_smithy_http::operation::error::BuildError>
            {
                write!(output, "/").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::create_virtual_mfa_device::CreateVirtualMfaDeviceInput,
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
                "application/x-www-form-urlencoded",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from(
            crate::protocol_serde::shape_create_virtual_mfa_device_input::ser_create_virtual_mfa_device_input_input(&self)?
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
            crate::operation::create_virtual_mfa_device::CreateVirtualMFADevice::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "CreateVirtualMFADevice",
            "iam",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
}
/// `ParseStrictResponse` impl for `CreateVirtualMFADevice`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct CreateVirtualMFADevice;
impl CreateVirtualMFADevice {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateVirtualMFADevice {
    type Output = std::result::Result<
        crate::operation::create_virtual_mfa_device::CreateVirtualMfaDeviceOutput,
        crate::operation::create_virtual_mfa_device::CreateVirtualMFADeviceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::protocol_serde::shape_create_virtual_mfa_device::de_create_virtual_mfa_device_http_error(response)
        } else {
            crate::protocol_serde::shape_create_virtual_mfa_device::de_create_virtual_mfa_device_http_response(response)
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type CreateVirtualMFADeviceErrorKind = CreateVirtualMFADeviceError;
/// Error type for the `CreateVirtualMFADeviceError` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum CreateVirtualMFADeviceError {
    /// <p>The request was rejected because multiple requests to change this object were submitted simultaneously. Wait a few minutes and submit your request again.</p>
    ConcurrentModificationException(crate::types::error::ConcurrentModificationException),
    /// <p>The request was rejected because it attempted to create a resource that already exists.</p>
    EntityAlreadyExistsException(crate::types::error::EntityAlreadyExistsException),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInputException(crate::types::error::InvalidInputException),
    /// <p>The request was rejected because it attempted to create resources beyond the current Amazon Web Services account limits. The error message describes the limit exceeded.</p>
    LimitExceededException(crate::types::error::LimitExceededException),
    /// <p>The request processing has failed because of an unknown error, exception or failure.</p>
    ServiceFailureException(crate::types::error::ServiceFailureException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(aws_smithy_types::error::Unhandled),
}
impl aws_smithy_http::result::CreateUnhandledError for CreateVirtualMFADeviceError {
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
impl std::fmt::Display for CreateVirtualMFADeviceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConcurrentModificationException(_inner) => _inner.fmt(f),
            Self::EntityAlreadyExistsException(_inner) => _inner.fmt(f),
            Self::InvalidInputException(_inner) => _inner.fmt(f),
            Self::LimitExceededException(_inner) => _inner.fmt(f),
            Self::ServiceFailureException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for CreateVirtualMFADeviceError {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::ConcurrentModificationException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::EntityAlreadyExistsException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidInputException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::LimitExceededException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ServiceFailureException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl aws_http::request_id::RequestId
    for crate::operation::create_virtual_mfa_device::CreateVirtualMFADeviceError
{
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for CreateVirtualMFADeviceError {
    fn code(&self) -> std::option::Option<&str> {
        aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> std::option::Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl CreateVirtualMFADeviceError {
    /// Creates the `CreateVirtualMFADeviceError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self::Unhandled(
            aws_smithy_types::error::Unhandled::builder()
                .source(err)
                .build(),
        )
    }

    /// Creates the `CreateVirtualMFADeviceError::Unhandled` variant from a `aws_smithy_types::error::ErrorMetadata`.
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
            Self::ConcurrentModificationException(e) => e.meta(),
            Self::EntityAlreadyExistsException(e) => e.meta(),
            Self::InvalidInputException(e) => e.meta(),
            Self::LimitExceededException(e) => e.meta(),
            Self::ServiceFailureException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `CreateVirtualMFADeviceError::ConcurrentModificationException`.
    pub fn is_concurrent_modification_exception(&self) -> bool {
        matches!(self, Self::ConcurrentModificationException(_))
    }
    /// Returns `true` if the error kind is `CreateVirtualMFADeviceError::EntityAlreadyExistsException`.
    pub fn is_entity_already_exists_exception(&self) -> bool {
        matches!(self, Self::EntityAlreadyExistsException(_))
    }
    /// Returns `true` if the error kind is `CreateVirtualMFADeviceError::InvalidInputException`.
    pub fn is_invalid_input_exception(&self) -> bool {
        matches!(self, Self::InvalidInputException(_))
    }
    /// Returns `true` if the error kind is `CreateVirtualMFADeviceError::LimitExceededException`.
    pub fn is_limit_exceeded_exception(&self) -> bool {
        matches!(self, Self::LimitExceededException(_))
    }
    /// Returns `true` if the error kind is `CreateVirtualMFADeviceError::ServiceFailureException`.
    pub fn is_service_failure_exception(&self) -> bool {
        matches!(self, Self::ServiceFailureException(_))
    }
}
impl std::error::Error for CreateVirtualMFADeviceError {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ConcurrentModificationException(_inner) => Some(_inner),
            Self::EntityAlreadyExistsException(_inner) => Some(_inner),
            Self::InvalidInputException(_inner) => Some(_inner),
            Self::LimitExceededException(_inner) => Some(_inner),
            Self::ServiceFailureException(_inner) => Some(_inner),
            Self::Unhandled(_inner) => Some(_inner),
        }
    }
}

pub use crate::operation::create_virtual_mfa_device::_create_virtual_mfa_device_output::CreateVirtualMfaDeviceOutput;

pub use crate::operation::create_virtual_mfa_device::_create_virtual_mfa_device_input::CreateVirtualMfaDeviceInput;

mod _create_virtual_mfa_device_input;

mod _create_virtual_mfa_device_output;

/// Builders
pub mod builders;
