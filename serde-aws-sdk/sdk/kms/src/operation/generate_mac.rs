// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

impl GenerateMacInput {
    /// Consumes the builder and constructs an Operation<[`GenerateMac`](crate::operation::generate_mac::GenerateMac)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::generate_mac::GenerateMac,
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
                _input: &crate::operation::generate_mac::GenerateMacInput,
                output: &mut String,
            ) -> std::result::Result<(), aws_smithy_http::operation::error::BuildError>
            {
                write!(output, "/").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::generate_mac::GenerateMacInput,
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
                "TrentService.GenerateMac",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from(
            crate::protocol_serde::shape_generate_mac::ser_generate_mac_input(&self)?,
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
            crate::operation::generate_mac::GenerateMac::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "GenerateMac",
            "kms",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
}
/// `ParseStrictResponse` impl for `GenerateMac`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct GenerateMac;
impl GenerateMac {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GenerateMac {
    type Output = std::result::Result<
        crate::operation::generate_mac::GenerateMacOutput,
        crate::operation::generate_mac::GenerateMacError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::protocol_serde::shape_generate_mac::de_generate_mac_http_error(response)
        } else {
            crate::protocol_serde::shape_generate_mac::de_generate_mac_http_response(response)
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type GenerateMacErrorKind = GenerateMacError;
/// Error type for the `GenerateMacError` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum GenerateMacError {
    /// <p>The request was rejected because the specified KMS key is not enabled.</p>
    DisabledException(crate::types::error::DisabledException),
    /// <p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantTokenException(crate::types::error::InvalidGrantTokenException),
    /// <p>The request was rejected for one of the following reasons: </p>
    /// <ul>
    /// <li> <p>The <code>KeyUsage</code> value of the KMS key is incompatible with the API operation.</p> </li>
    /// <li> <p>The encryption algorithm or signing algorithm specified for the operation is incompatible with the type of key material in the KMS key <code>(KeySpec</code>).</p> </li>
    /// </ul>
    /// <p>For encrypting, decrypting, re-encrypting, and generating data keys, the <code>KeyUsage</code> must be <code>ENCRYPT_DECRYPT</code>. For signing and verifying messages, the <code>KeyUsage</code> must be <code>SIGN_VERIFY</code>. For generating and verifying message authentication codes (MACs), the <code>KeyUsage</code> must be <code>GENERATE_VERIFY_MAC</code>. To find the <code>KeyUsage</code> of a KMS key, use the <code>DescribeKey</code> operation.</p>
    /// <p>To find the encryption or signing algorithms supported for a particular KMS key, use the <code>DescribeKey</code> operation.</p>
    InvalidKeyUsageException(crate::types::error::InvalidKeyUsageException),
    /// <p>The request was rejected because the specified KMS key was not available. You can retry the request.</p>
    KeyUnavailableException(crate::types::error::KeyUnavailableException),
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
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(aws_smithy_types::error::Unhandled),
}
impl aws_smithy_http::result::CreateUnhandledError for GenerateMacError {
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
impl std::fmt::Display for GenerateMacError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DisabledException(_inner) => _inner.fmt(f),
            Self::InvalidGrantTokenException(_inner) => _inner.fmt(f),
            Self::InvalidKeyUsageException(_inner) => _inner.fmt(f),
            Self::KeyUnavailableException(_inner) => _inner.fmt(f),
            Self::KmsInternalException(_inner) => _inner.fmt(f),
            Self::KmsInvalidStateException(_inner) => _inner.fmt(f),
            Self::NotFoundException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for GenerateMacError {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::DisabledException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidGrantTokenException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidKeyUsageException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::KeyUnavailableException(_inner) => {
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
            Self::Unhandled(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl aws_http::request_id::RequestId for crate::operation::generate_mac::GenerateMacError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for GenerateMacError {
    fn code(&self) -> std::option::Option<&str> {
        aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> std::option::Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl GenerateMacError {
    /// Creates the `GenerateMacError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self::Unhandled(
            aws_smithy_types::error::Unhandled::builder()
                .source(err)
                .build(),
        )
    }

    /// Creates the `GenerateMacError::Unhandled` variant from a `aws_smithy_types::error::ErrorMetadata`.
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
            Self::DisabledException(e) => e.meta(),
            Self::InvalidGrantTokenException(e) => e.meta(),
            Self::InvalidKeyUsageException(e) => e.meta(),
            Self::KeyUnavailableException(e) => e.meta(),
            Self::KmsInternalException(e) => e.meta(),
            Self::KmsInvalidStateException(e) => e.meta(),
            Self::NotFoundException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `GenerateMacError::DisabledException`.
    pub fn is_disabled_exception(&self) -> bool {
        matches!(self, Self::DisabledException(_))
    }
    /// Returns `true` if the error kind is `GenerateMacError::InvalidGrantTokenException`.
    pub fn is_invalid_grant_token_exception(&self) -> bool {
        matches!(self, Self::InvalidGrantTokenException(_))
    }
    /// Returns `true` if the error kind is `GenerateMacError::InvalidKeyUsageException`.
    pub fn is_invalid_key_usage_exception(&self) -> bool {
        matches!(self, Self::InvalidKeyUsageException(_))
    }
    /// Returns `true` if the error kind is `GenerateMacError::KeyUnavailableException`.
    pub fn is_key_unavailable_exception(&self) -> bool {
        matches!(self, Self::KeyUnavailableException(_))
    }
    /// Returns `true` if the error kind is `GenerateMacError::KmsInternalException`.
    pub fn is_kms_internal_exception(&self) -> bool {
        matches!(self, Self::KmsInternalException(_))
    }
    /// Returns `true` if the error kind is `GenerateMacError::KmsInvalidStateException`.
    pub fn is_kms_invalid_state_exception(&self) -> bool {
        matches!(self, Self::KmsInvalidStateException(_))
    }
    /// Returns `true` if the error kind is `GenerateMacError::NotFoundException`.
    pub fn is_not_found_exception(&self) -> bool {
        matches!(self, Self::NotFoundException(_))
    }
}
impl std::error::Error for GenerateMacError {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::DisabledException(_inner) => Some(_inner),
            Self::InvalidGrantTokenException(_inner) => Some(_inner),
            Self::InvalidKeyUsageException(_inner) => Some(_inner),
            Self::KeyUnavailableException(_inner) => Some(_inner),
            Self::KmsInternalException(_inner) => Some(_inner),
            Self::KmsInvalidStateException(_inner) => Some(_inner),
            Self::NotFoundException(_inner) => Some(_inner),
            Self::Unhandled(_inner) => Some(_inner),
        }
    }
}

pub use crate::operation::generate_mac::_generate_mac_output::GenerateMacOutput;

pub use crate::operation::generate_mac::_generate_mac_input::GenerateMacInput;

mod _generate_mac_input;

mod _generate_mac_output;

/// Builders
pub mod builders;
