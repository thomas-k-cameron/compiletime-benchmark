// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

impl PutMultiRegionAccessPointPolicyInput {
    /// Consumes the builder and constructs an Operation<[`PutMultiRegionAccessPointPolicy`](crate::operation::put_multi_region_access_point_policy::PutMultiRegionAccessPointPolicy)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        mut self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::put_multi_region_access_point_policy::PutMultiRegionAccessPointPolicy,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::operation::error::BuildError,
    > {
        let params_result = crate::endpoint::Params::builder()
            .set_region(_config.region.as_ref().map(|r| r.as_ref().to_owned()))
            .set_use_fips(_config.use_fips)
            .set_use_dual_stack(_config.use_dual_stack)
            .set_endpoint(_config.endpoint_url.clone())
            .set_use_arn_region(_config.use_arn_region)
            .set_requires_account_id(Some(true))
            .set_account_id(self.account_id.clone())
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
        if self.client_token.is_none() {
            self.client_token = Some(_config.make_token.make_idempotency_token());
        }
        let mut request = {
            fn uri_base(
                _input: &crate::operation::put_multi_region_access_point_policy::PutMultiRegionAccessPointPolicyInput,
                output: &mut String,
            ) -> std::result::Result<(), aws_smithy_http::operation::error::BuildError>
            {
                write!(output, "/v20180820/async-requests/mrap/put-policy")
                    .expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::put_multi_region_access_point_policy::PutMultiRegionAccessPointPolicyInput,
                builder: http::request::Builder,
            ) -> std::result::Result<
                http::request::Builder,
                aws_smithy_http::operation::error::BuildError,
            > {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                let builder = crate::protocol_serde::shape_put_multi_region_access_point_policy::ser_put_multi_region_access_point_policy_headers(input, builder)?;
                Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::CONTENT_TYPE,
                "application/xml",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from(
            crate::protocol_serde::shape_put_multi_region_access_point_policy::ser_put_multi_region_access_point_policy_op_input(&self)?
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
        request = request.augment(|mut req, _| {
            let data = req
                .body()
                .bytes()
                .expect("checksum can only be computed for non-streaming operations");
            let checksum = <md5::Md5 as md5::Digest>::digest(data);
            req.headers_mut().insert(
                http::header::HeaderName::from_static("content-md5"),
                aws_smithy_types::base64::encode(&checksum[..])
                    .parse()
                    .expect("checksum is valid header value"),
            );
            Result::<_, aws_smithy_http::operation::error::BuildError>::Ok(req)
        })?;
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
        let op = aws_smithy_http::operation::Operation::new(request, crate::operation::put_multi_region_access_point_policy::PutMultiRegionAccessPointPolicy::new())
                            .with_metadata(aws_smithy_http::operation::Metadata::new("PutMultiRegionAccessPointPolicy", "s3control"));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
}
/// `ParseStrictResponse` impl for `PutMultiRegionAccessPointPolicy`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct PutMultiRegionAccessPointPolicy;
impl PutMultiRegionAccessPointPolicy {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutMultiRegionAccessPointPolicy {
    type Output = std::result::Result<crate::operation::put_multi_region_access_point_policy::PutMultiRegionAccessPointPolicyOutput, crate::operation::put_multi_region_access_point_policy::PutMultiRegionAccessPointPolicyError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::protocol_serde::shape_put_multi_region_access_point_policy::de_put_multi_region_access_point_policy_http_error(response)
        } else {
            crate::protocol_serde::shape_put_multi_region_access_point_policy::de_put_multi_region_access_point_policy_http_response(response)
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type PutMultiRegionAccessPointPolicyErrorKind = PutMultiRegionAccessPointPolicyError;
/// Error type for the `PutMultiRegionAccessPointPolicyError` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum PutMultiRegionAccessPointPolicyError {
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(aws_smithy_types::error::Unhandled),
}
impl aws_smithy_http::result::CreateUnhandledError for PutMultiRegionAccessPointPolicyError {
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
impl std::fmt::Display for PutMultiRegionAccessPointPolicyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata
    for PutMultiRegionAccessPointPolicyError
{
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::Unhandled(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl aws_http::request_id::RequestId
    for crate::operation::put_multi_region_access_point_policy::PutMultiRegionAccessPointPolicyError
{
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for PutMultiRegionAccessPointPolicyError {
    fn code(&self) -> std::option::Option<&str> {
        aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> std::option::Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl PutMultiRegionAccessPointPolicyError {
    /// Creates the `PutMultiRegionAccessPointPolicyError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self::Unhandled(
            aws_smithy_types::error::Unhandled::builder()
                .source(err)
                .build(),
        )
    }

    /// Creates the `PutMultiRegionAccessPointPolicyError::Unhandled` variant from a `aws_smithy_types::error::ErrorMetadata`.
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
impl std::error::Error for PutMultiRegionAccessPointPolicyError {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Unhandled(_inner) => Some(_inner),
        }
    }
}

pub use crate::operation::put_multi_region_access_point_policy::_put_multi_region_access_point_policy_output::PutMultiRegionAccessPointPolicyOutput;

pub use crate::operation::put_multi_region_access_point_policy::_put_multi_region_access_point_policy_input::PutMultiRegionAccessPointPolicyInput;

mod _put_multi_region_access_point_policy_input;

mod _put_multi_region_access_point_policy_output;

/// Builders
pub mod builders;
