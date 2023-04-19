// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

impl UploadPartCopyInput {
    /// Consumes the builder and constructs an Operation<[`UploadPartCopy`](crate::operation::upload_part_copy::UploadPartCopy)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::upload_part_copy::UploadPartCopy,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::operation::error::BuildError,
    > {
        let params_result = crate::endpoint::Params::builder()
            .set_region(_config.region.as_ref().map(|r| r.as_ref().to_owned()))
            .set_use_fips(_config.use_fips)
            .set_use_dual_stack(_config.use_dual_stack)
            .set_endpoint(_config.endpoint_url.clone())
            .set_force_path_style(_config.force_path_style)
            .set_use_arn_region(_config.use_arn_region)
            .set_disable_multi_region_access_points(_config.disable_multi_region_access_points)
            .set_accelerate(_config.accelerate)
            .set_bucket(self.bucket.clone())
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
                _input: &crate::operation::upload_part_copy::UploadPartCopyInput,
                output: &mut String,
            ) -> std::result::Result<(), aws_smithy_http::operation::error::BuildError>
            {
                let input_1 = &_input.key;
                let input_1 = input_1.as_ref().ok_or_else(|| {
                    aws_smithy_http::operation::error::BuildError::missing_field(
                        "key",
                        "cannot be empty or unset",
                    )
                })?;
                let key = aws_smithy_http::label::fmt_string(
                    input_1,
                    aws_smithy_http::label::EncodingStrategy::Greedy,
                );
                if key.is_empty() {
                    return Err(
                        aws_smithy_http::operation::error::BuildError::missing_field(
                            "key",
                            "cannot be empty or unset",
                        ),
                    );
                }
                write!(output, "/{Key}", Key = key).expect("formatting should succeed");
                Ok(())
            }
            fn uri_query(
                _input: &crate::operation::upload_part_copy::UploadPartCopyInput,
                mut output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::error::BuildError> {
                let mut query = aws_smithy_http::query::Writer::new(&mut output);
                query.push_kv("x-id", "UploadPartCopy");
                let inner_2 = &_input.part_number;
                let inner_2 = inner_2.as_ref().ok_or_else(|| {
                    aws_smithy_http::operation::error::BuildError::missing_field(
                        "part_number",
                        "cannot be empty or unset",
                    )
                })?;
                query.push_kv(
                    "partNumber",
                    aws_smithy_types::primitive::Encoder::from(*inner_2).encode(),
                );
                let inner_3 = &_input.upload_id;
                let inner_3 = inner_3.as_ref().ok_or_else(|| {
                    aws_smithy_http::operation::error::BuildError::missing_field(
                        "upload_id",
                        "cannot be empty or unset",
                    )
                })?;
                if inner_3.is_empty() {
                    return Err(
                        aws_smithy_http::operation::error::BuildError::missing_field(
                            "upload_id",
                            "cannot be empty or unset",
                        ),
                    );
                }
                query.push_kv("uploadId", &aws_smithy_http::query::fmt_string(&inner_3));
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::upload_part_copy::UploadPartCopyInput,
                builder: http::request::Builder,
            ) -> std::result::Result<
                http::request::Builder,
                aws_smithy_http::operation::error::BuildError,
            > {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                uri_query(input, &mut uri)?;
                let builder =
                    crate::protocol_serde::shape_upload_part_copy::ser_upload_part_copy_headers(
                        input, builder,
                    )?;
                Ok(builder.method("PUT").uri(uri))
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
        signing_config.signing_options.content_sha256_header = true;
        signing_config.signing_options.double_uri_encode = false;
        signing_config.signing_options.normalize_uri_path = false;
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
            crate::operation::upload_part_copy::UploadPartCopy::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "UploadPartCopy",
            "s3",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
}
/// `ParseStrictResponse` impl for `UploadPartCopy`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct UploadPartCopy;
impl UploadPartCopy {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UploadPartCopy {
    type Output = std::result::Result<
        crate::operation::upload_part_copy::UploadPartCopyOutput,
        crate::operation::upload_part_copy::UploadPartCopyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        tracing::debug!(extended_request_id = ?crate::s3_request_id::RequestIdExt::extended_request_id(response));
        tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::protocol_serde::shape_upload_part_copy::de_upload_part_copy_http_error(response)
        } else {
            crate::protocol_serde::shape_upload_part_copy::de_upload_part_copy_http_response(
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
pub type UploadPartCopyErrorKind = UploadPartCopyError;
/// Error type for the `UploadPartCopyError` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum UploadPartCopyError {
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(aws_smithy_types::error::Unhandled),
}
impl aws_smithy_http::result::CreateUnhandledError for UploadPartCopyError {
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
impl std::fmt::Display for UploadPartCopyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for UploadPartCopyError {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::Unhandled(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl crate::s3_request_id::RequestIdExt
    for crate::operation::upload_part_copy::UploadPartCopyError
{
    fn extended_request_id(&self) -> Option<&str> {
        self.meta().extended_request_id()
    }
}
impl aws_http::request_id::RequestId for crate::operation::upload_part_copy::UploadPartCopyError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for UploadPartCopyError {
    fn code(&self) -> std::option::Option<&str> {
        aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> std::option::Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl UploadPartCopyError {
    /// Creates the `UploadPartCopyError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self::Unhandled(
            aws_smithy_types::error::Unhandled::builder()
                .source(err)
                .build(),
        )
    }

    /// Creates the `UploadPartCopyError::Unhandled` variant from a `aws_smithy_types::error::ErrorMetadata`.
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
            Self::Unhandled(e) => e.meta(),
        }
    }
}
impl std::error::Error for UploadPartCopyError {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Unhandled(_inner) => Some(_inner),
        }
    }
}

pub use crate::operation::upload_part_copy::_upload_part_copy_output::UploadPartCopyOutput;

pub use crate::operation::upload_part_copy::_upload_part_copy_input::UploadPartCopyInput;

mod _upload_part_copy_input;

mod _upload_part_copy_output;

/// Builders
pub mod builders;
