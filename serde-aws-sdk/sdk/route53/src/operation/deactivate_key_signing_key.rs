// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

impl DeactivateKeySigningKeyInput {
    /// Consumes the builder and constructs an Operation<[`DeactivateKeySigningKey`](crate::operation::deactivate_key_signing_key::DeactivateKeySigningKey)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        mut self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::deactivate_key_signing_key::DeactivateKeySigningKey,
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
        crate::route53_resource_id_preprocessor::trim_resource_id(&mut self.hosted_zone_id);
        let mut request = {
            fn uri_base(
                _input: &crate::operation::deactivate_key_signing_key::DeactivateKeySigningKeyInput,
                output: &mut String,
            ) -> std::result::Result<(), aws_smithy_http::operation::error::BuildError>
            {
                let input_1 = &_input.hosted_zone_id;
                let input_1 = input_1.as_ref().ok_or_else(|| {
                    aws_smithy_http::operation::error::BuildError::missing_field(
                        "hosted_zone_id",
                        "cannot be empty or unset",
                    )
                })?;
                let hosted_zone_id = aws_smithy_http::label::fmt_string(
                    input_1,
                    aws_smithy_http::label::EncodingStrategy::Default,
                );
                if hosted_zone_id.is_empty() {
                    return Err(
                        aws_smithy_http::operation::error::BuildError::missing_field(
                            "hosted_zone_id",
                            "cannot be empty or unset",
                        ),
                    );
                }
                let input_2 = &_input.name;
                let input_2 = input_2.as_ref().ok_or_else(|| {
                    aws_smithy_http::operation::error::BuildError::missing_field(
                        "name",
                        "cannot be empty or unset",
                    )
                })?;
                let name = aws_smithy_http::label::fmt_string(
                    input_2,
                    aws_smithy_http::label::EncodingStrategy::Default,
                );
                if name.is_empty() {
                    return Err(
                        aws_smithy_http::operation::error::BuildError::missing_field(
                            "name",
                            "cannot be empty or unset",
                        ),
                    );
                }
                write!(
                    output,
                    "/2013-04-01/keysigningkey/{HostedZoneId}/{Name}/deactivate",
                    HostedZoneId = hosted_zone_id,
                    Name = name
                )
                .expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::deactivate_key_signing_key::DeactivateKeySigningKeyInput,
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
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from("");
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
            crate::operation::deactivate_key_signing_key::DeactivateKeySigningKey::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "DeactivateKeySigningKey",
            "route53",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
}
/// `ParseStrictResponse` impl for `DeactivateKeySigningKey`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct DeactivateKeySigningKey;
impl DeactivateKeySigningKey {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeactivateKeySigningKey {
    type Output = std::result::Result<
        crate::operation::deactivate_key_signing_key::DeactivateKeySigningKeyOutput,
        crate::operation::deactivate_key_signing_key::DeactivateKeySigningKeyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::protocol_serde::shape_deactivate_key_signing_key::de_deactivate_key_signing_key_http_error(response)
        } else {
            crate::protocol_serde::shape_deactivate_key_signing_key::de_deactivate_key_signing_key_http_response(response)
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type DeactivateKeySigningKeyErrorKind = DeactivateKeySigningKeyError;
/// Error type for the `DeactivateKeySigningKeyError` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum DeactivateKeySigningKeyError {
    /// <p>Another user submitted a request to create, update, or delete the object at the same time that you did. Retry the request. </p>
    ConcurrentModification(crate::types::error::ConcurrentModification),
    /// <p>The input is not valid.</p>
    InvalidInput(crate::types::error::InvalidInput),
    /// <p>The key-signing key (KSK) status isn't valid or another KSK has the status <code>INTERNAL_FAILURE</code>.</p>
    InvalidKeySigningKeyStatus(crate::types::error::InvalidKeySigningKeyStatus),
    /// <p>Your hosted zone status isn't valid for this operation. In the hosted zone, change the status to enable <code>DNSSEC</code> or disable <code>DNSSEC</code>.</p>
    InvalidSigningStatus(crate::types::error::InvalidSigningStatus),
    /// <p>The key-signing key (KSK) is specified in a parent DS record.</p>
    KeySigningKeyInParentDsRecord(crate::types::error::KeySigningKeyInParentDsRecord),
    /// <p>The key-signing key (KSK) that you specified can't be deactivated because it's the only KSK for a currently-enabled DNSSEC. Disable DNSSEC signing, or add or enable another KSK.</p>
    KeySigningKeyInUse(crate::types::error::KeySigningKeyInUse),
    /// <p>The specified key-signing key (KSK) doesn't exist.</p>
    NoSuchKeySigningKey(crate::types::error::NoSuchKeySigningKey),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(aws_smithy_types::error::Unhandled),
}
impl aws_smithy_http::result::CreateUnhandledError for DeactivateKeySigningKeyError {
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
impl std::fmt::Display for DeactivateKeySigningKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConcurrentModification(_inner) => _inner.fmt(f),
            Self::InvalidInput(_inner) => _inner.fmt(f),
            Self::InvalidKeySigningKeyStatus(_inner) => _inner.fmt(f),
            Self::InvalidSigningStatus(_inner) => _inner.fmt(f),
            Self::KeySigningKeyInParentDsRecord(_inner) => _inner.fmt(f),
            Self::KeySigningKeyInUse(_inner) => _inner.fmt(f),
            Self::NoSuchKeySigningKey(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for DeactivateKeySigningKeyError {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::ConcurrentModification(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidInput(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidKeySigningKeyStatus(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidSigningStatus(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::KeySigningKeyInParentDsRecord(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::KeySigningKeyInUse(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::NoSuchKeySigningKey(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl aws_http::request_id::RequestId
    for crate::operation::deactivate_key_signing_key::DeactivateKeySigningKeyError
{
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for DeactivateKeySigningKeyError {
    fn code(&self) -> std::option::Option<&str> {
        aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> std::option::Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl DeactivateKeySigningKeyError {
    /// Creates the `DeactivateKeySigningKeyError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self::Unhandled(
            aws_smithy_types::error::Unhandled::builder()
                .source(err)
                .build(),
        )
    }

    /// Creates the `DeactivateKeySigningKeyError::Unhandled` variant from a `aws_smithy_types::error::ErrorMetadata`.
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
            Self::ConcurrentModification(e) => e.meta(),
            Self::InvalidInput(e) => e.meta(),
            Self::InvalidKeySigningKeyStatus(e) => e.meta(),
            Self::InvalidSigningStatus(e) => e.meta(),
            Self::KeySigningKeyInParentDsRecord(e) => e.meta(),
            Self::KeySigningKeyInUse(e) => e.meta(),
            Self::NoSuchKeySigningKey(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `DeactivateKeySigningKeyError::ConcurrentModification`.
    pub fn is_concurrent_modification(&self) -> bool {
        matches!(self, Self::ConcurrentModification(_))
    }
    /// Returns `true` if the error kind is `DeactivateKeySigningKeyError::InvalidInput`.
    pub fn is_invalid_input(&self) -> bool {
        matches!(self, Self::InvalidInput(_))
    }
    /// Returns `true` if the error kind is `DeactivateKeySigningKeyError::InvalidKeySigningKeyStatus`.
    pub fn is_invalid_key_signing_key_status(&self) -> bool {
        matches!(self, Self::InvalidKeySigningKeyStatus(_))
    }
    /// Returns `true` if the error kind is `DeactivateKeySigningKeyError::InvalidSigningStatus`.
    pub fn is_invalid_signing_status(&self) -> bool {
        matches!(self, Self::InvalidSigningStatus(_))
    }
    /// Returns `true` if the error kind is `DeactivateKeySigningKeyError::KeySigningKeyInParentDsRecord`.
    pub fn is_key_signing_key_in_parent_ds_record(&self) -> bool {
        matches!(self, Self::KeySigningKeyInParentDsRecord(_))
    }
    /// Returns `true` if the error kind is `DeactivateKeySigningKeyError::KeySigningKeyInUse`.
    pub fn is_key_signing_key_in_use(&self) -> bool {
        matches!(self, Self::KeySigningKeyInUse(_))
    }
    /// Returns `true` if the error kind is `DeactivateKeySigningKeyError::NoSuchKeySigningKey`.
    pub fn is_no_such_key_signing_key(&self) -> bool {
        matches!(self, Self::NoSuchKeySigningKey(_))
    }
}
impl std::error::Error for DeactivateKeySigningKeyError {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ConcurrentModification(_inner) => Some(_inner),
            Self::InvalidInput(_inner) => Some(_inner),
            Self::InvalidKeySigningKeyStatus(_inner) => Some(_inner),
            Self::InvalidSigningStatus(_inner) => Some(_inner),
            Self::KeySigningKeyInParentDsRecord(_inner) => Some(_inner),
            Self::KeySigningKeyInUse(_inner) => Some(_inner),
            Self::NoSuchKeySigningKey(_inner) => Some(_inner),
            Self::Unhandled(_inner) => Some(_inner),
        }
    }
}

pub use crate::operation::deactivate_key_signing_key::_deactivate_key_signing_key_output::DeactivateKeySigningKeyOutput;

pub use crate::operation::deactivate_key_signing_key::_deactivate_key_signing_key_input::DeactivateKeySigningKeyInput;

mod _deactivate_key_signing_key_input;

mod _deactivate_key_signing_key_output;

/// Builders
pub mod builders;
