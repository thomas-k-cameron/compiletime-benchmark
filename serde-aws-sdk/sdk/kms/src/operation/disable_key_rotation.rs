// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

impl DisableKeyRotationInput {
    /// Consumes the builder and constructs an Operation<[`DisableKeyRotation`](crate::operation::disable_key_rotation::DisableKeyRotation)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::disable_key_rotation::DisableKeyRotation,
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
                _input: &crate::operation::disable_key_rotation::DisableKeyRotationInput,
                output: &mut String,
            ) -> std::result::Result<(), aws_smithy_http::operation::error::BuildError>
            {
                write!(output, "/").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::disable_key_rotation::DisableKeyRotationInput,
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
                "TrentService.DisableKeyRotation",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from(
            crate::protocol_serde::shape_disable_key_rotation::ser_disable_key_rotation_input(
                &self,
            )?,
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
            crate::operation::disable_key_rotation::DisableKeyRotation::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "DisableKeyRotation",
            "kms",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
}
/// `ParseStrictResponse` impl for `DisableKeyRotation`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct DisableKeyRotation;
impl DisableKeyRotation {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DisableKeyRotation {
    type Output = std::result::Result<
        crate::operation::disable_key_rotation::DisableKeyRotationOutput,
        crate::operation::disable_key_rotation::DisableKeyRotationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::protocol_serde::shape_disable_key_rotation::de_disable_key_rotation_http_error(
                response,
            )
        } else {
            crate::protocol_serde::shape_disable_key_rotation::de_disable_key_rotation_http_response(
                response,
            )
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type DisableKeyRotationErrorKind = DisableKeyRotationError;
/// Error type for the `DisableKeyRotationError` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum DisableKeyRotationError {
    /// <p>The system timed out while trying to fulfill the request. You can retry the request.</p>
    DependencyTimeoutException(crate::types::error::DependencyTimeoutException),
    /// <p>The request was rejected because the specified KMS key is not enabled.</p>
    DisabledException(crate::types::error::DisabledException),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArnException(crate::types::error::InvalidArnException),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KmsInternalException(crate::types::error::KmsInternalException),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p>
    /// <p>This exceptions means one of the following:</p>
    /// <ul>
    /// <li> <p>The key state of the KMS key is not compatible with the operation. </p> <p>To find the key state, use the <code>DescribeKey</code> operation. For more information about which key states are compatible with each KMS operation, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key states of KMS keys</a> in the <i> <i>Key Management Service Developer Guide</i> </i>.</p> </li>
    /// <li> <p>For cryptographic operations on KMS keys in custom key stores, this exception represents a general failure with many possible causes. To identify the cause, see the error message that accompanies the exception.</p> </li>
    /// </ul>
    KmsInvalidStateException(crate::types::error::KmsInvalidStateException),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFoundException(crate::types::error::NotFoundException),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperationException(crate::types::error::UnsupportedOperationException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(aws_smithy_types::error::Unhandled),
}
impl aws_smithy_http::result::CreateUnhandledError for DisableKeyRotationError {
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
impl std::fmt::Display for DisableKeyRotationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DependencyTimeoutException(_inner) => _inner.fmt(f),
            Self::DisabledException(_inner) => _inner.fmt(f),
            Self::InvalidArnException(_inner) => _inner.fmt(f),
            Self::KmsInternalException(_inner) => _inner.fmt(f),
            Self::KmsInvalidStateException(_inner) => _inner.fmt(f),
            Self::NotFoundException(_inner) => _inner.fmt(f),
            Self::UnsupportedOperationException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for DisableKeyRotationError {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::DependencyTimeoutException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::DisabledException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidArnException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::KmsInternalException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::KmsInvalidStateException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::NotFoundException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::UnsupportedOperationException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl aws_http::request_id::RequestId
    for crate::operation::disable_key_rotation::DisableKeyRotationError
{
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for DisableKeyRotationError {
    fn code(&self) -> std::option::Option<&str> {
        aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> std::option::Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl DisableKeyRotationError {
    /// Creates the `DisableKeyRotationError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self::Unhandled(
            aws_smithy_types::error::Unhandled::builder()
                .source(err)
                .build(),
        )
    }

    /// Creates the `DisableKeyRotationError::Unhandled` variant from a `aws_smithy_types::error::ErrorMetadata`.
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
            Self::DependencyTimeoutException(e) => e.meta(),
            Self::DisabledException(e) => e.meta(),
            Self::InvalidArnException(e) => e.meta(),
            Self::KmsInternalException(e) => e.meta(),
            Self::KmsInvalidStateException(e) => e.meta(),
            Self::NotFoundException(e) => e.meta(),
            Self::UnsupportedOperationException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `DisableKeyRotationError::DependencyTimeoutException`.
    pub fn is_dependency_timeout_exception(&self) -> bool {
        matches!(self, Self::DependencyTimeoutException(_))
    }
    /// Returns `true` if the error kind is `DisableKeyRotationError::DisabledException`.
    pub fn is_disabled_exception(&self) -> bool {
        matches!(self, Self::DisabledException(_))
    }
    /// Returns `true` if the error kind is `DisableKeyRotationError::InvalidArnException`.
    pub fn is_invalid_arn_exception(&self) -> bool {
        matches!(self, Self::InvalidArnException(_))
    }
    /// Returns `true` if the error kind is `DisableKeyRotationError::KmsInternalException`.
    pub fn is_kms_internal_exception(&self) -> bool {
        matches!(self, Self::KmsInternalException(_))
    }
    /// Returns `true` if the error kind is `DisableKeyRotationError::KmsInvalidStateException`.
    pub fn is_kms_invalid_state_exception(&self) -> bool {
        matches!(self, Self::KmsInvalidStateException(_))
    }
    /// Returns `true` if the error kind is `DisableKeyRotationError::NotFoundException`.
    pub fn is_not_found_exception(&self) -> bool {
        matches!(self, Self::NotFoundException(_))
    }
    /// Returns `true` if the error kind is `DisableKeyRotationError::UnsupportedOperationException`.
    pub fn is_unsupported_operation_exception(&self) -> bool {
        matches!(self, Self::UnsupportedOperationException(_))
    }
}
impl std::error::Error for DisableKeyRotationError {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::DependencyTimeoutException(_inner) => Some(_inner),
            Self::DisabledException(_inner) => Some(_inner),
            Self::InvalidArnException(_inner) => Some(_inner),
            Self::KmsInternalException(_inner) => Some(_inner),
            Self::KmsInvalidStateException(_inner) => Some(_inner),
            Self::NotFoundException(_inner) => Some(_inner),
            Self::UnsupportedOperationException(_inner) => Some(_inner),
            Self::Unhandled(_inner) => Some(_inner),
        }
    }
}

pub use crate::operation::disable_key_rotation::_disable_key_rotation_output::DisableKeyRotationOutput;

pub use crate::operation::disable_key_rotation::_disable_key_rotation_input::DisableKeyRotationInput;

mod _disable_key_rotation_input;

mod _disable_key_rotation_output;

/// Builders
pub mod builders;
