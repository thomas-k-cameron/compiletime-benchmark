// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

impl AssociateVpcWithHostedZoneInput {
    /// Consumes the builder and constructs an Operation<[`AssociateVPCWithHostedZone`](crate::operation::associate_vpc_with_hosted_zone::AssociateVPCWithHostedZone)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        mut self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::associate_vpc_with_hosted_zone::AssociateVPCWithHostedZone,
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
                _input: &crate::operation::associate_vpc_with_hosted_zone::AssociateVpcWithHostedZoneInput,
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
                write!(
                    output,
                    "/2013-04-01/hostedzone/{HostedZoneId}/associatevpc",
                    HostedZoneId = hosted_zone_id
                )
                .expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::associate_vpc_with_hosted_zone::AssociateVpcWithHostedZoneInput,
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
                "application/xml",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from(
            crate::protocol_serde::shape_associate_vpc_with_hosted_zone::ser_associate_vpc_with_hosted_zone_op_input(&self)?
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
            crate::operation::associate_vpc_with_hosted_zone::AssociateVPCWithHostedZone::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "AssociateVPCWithHostedZone",
            "route53",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
}
/// `ParseStrictResponse` impl for `AssociateVPCWithHostedZone`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct AssociateVPCWithHostedZone;
impl AssociateVPCWithHostedZone {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AssociateVPCWithHostedZone {
    type Output = std::result::Result<
        crate::operation::associate_vpc_with_hosted_zone::AssociateVpcWithHostedZoneOutput,
        crate::operation::associate_vpc_with_hosted_zone::AssociateVPCWithHostedZoneError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::protocol_serde::shape_associate_vpc_with_hosted_zone::de_associate_vpc_with_hosted_zone_http_error(response)
        } else {
            crate::protocol_serde::shape_associate_vpc_with_hosted_zone::de_associate_vpc_with_hosted_zone_http_response(response)
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type AssociateVPCWithHostedZoneErrorKind = AssociateVPCWithHostedZoneError;
/// Error type for the `AssociateVPCWithHostedZoneError` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum AssociateVPCWithHostedZoneError {
    /// <p>The cause of this error depends on the operation that you're performing:</p>
    /// <ul>
    /// <li> <p> <b>Create a public hosted zone:</b> Two hosted zones that have the same name or that have a parent/child relationship (example.com and test.example.com) can't have any common name servers. You tried to create a hosted zone that has the same name as an existing hosted zone or that's the parent or child of an existing hosted zone, and you specified a delegation set that shares one or more name servers with the existing hosted zone. For more information, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_CreateReusableDelegationSet.html">CreateReusableDelegationSet</a>.</p> </li>
    /// <li> <p> <b>Create a private hosted zone:</b> A hosted zone with the specified name already exists and is already associated with the Amazon VPC that you specified.</p> </li>
    /// <li> <p> <b>Associate VPCs with a private hosted zone:</b> The VPC that you specified is already associated with another hosted zone that has the same name.</p> </li>
    /// </ul>
    ConflictingDomainExists(crate::types::error::ConflictingDomainExists),
    /// <p>The input is not valid.</p>
    InvalidInput(crate::types::error::InvalidInput),
    /// <p>The VPC ID that you specified either isn't a valid ID or the current account is not authorized to access this VPC.</p>
    InvalidVpcId(crate::types::error::InvalidVpcId),
    /// <p>This operation can't be completed because the current account has reached the limit on the resource you are trying to create. To request a higher limit, <a href="http://aws.amazon.com/route53-request">create a case</a> with the Amazon Web Services Support Center.</p>
    LimitsExceeded(crate::types::error::LimitsExceeded),
    /// <p>No hosted zone exists with the ID that you specified.</p>
    NoSuchHostedZone(crate::types::error::NoSuchHostedZone),
    /// <p>Associating the specified VPC with the specified hosted zone has not been authorized.</p>
    NotAuthorizedException(crate::types::error::NotAuthorizedException),
    /// <p>If Amazon Route 53 can't process a request before the next request arrives, it will reject subsequent requests for the same hosted zone and return an <code>HTTP 400 error</code> (<code>Bad request</code>). If Route 53 returns this error repeatedly for the same request, we recommend that you wait, in intervals of increasing duration, before you try the request again.</p>
    PriorRequestNotComplete(crate::types::error::PriorRequestNotComplete),
    /// <p>You're trying to associate a VPC with a public hosted zone. Amazon Route 53 doesn't support associating a VPC with a public hosted zone.</p>
    PublicZoneVpcAssociation(crate::types::error::PublicZoneVpcAssociation),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(aws_smithy_types::error::Unhandled),
}
impl aws_smithy_http::result::CreateUnhandledError for AssociateVPCWithHostedZoneError {
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
impl std::fmt::Display for AssociateVPCWithHostedZoneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConflictingDomainExists(_inner) => _inner.fmt(f),
            Self::InvalidInput(_inner) => _inner.fmt(f),
            Self::InvalidVpcId(_inner) => _inner.fmt(f),
            Self::LimitsExceeded(_inner) => _inner.fmt(f),
            Self::NoSuchHostedZone(_inner) => _inner.fmt(f),
            Self::NotAuthorizedException(_inner) => _inner.fmt(f),
            Self::PriorRequestNotComplete(_inner) => _inner.fmt(f),
            Self::PublicZoneVpcAssociation(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for AssociateVPCWithHostedZoneError {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::ConflictingDomainExists(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidInput(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidVpcId(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::LimitsExceeded(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::NoSuchHostedZone(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::NotAuthorizedException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::PriorRequestNotComplete(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::PublicZoneVpcAssociation(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl aws_http::request_id::RequestId
    for crate::operation::associate_vpc_with_hosted_zone::AssociateVPCWithHostedZoneError
{
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for AssociateVPCWithHostedZoneError {
    fn code(&self) -> std::option::Option<&str> {
        aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> std::option::Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl AssociateVPCWithHostedZoneError {
    /// Creates the `AssociateVPCWithHostedZoneError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self::Unhandled(
            aws_smithy_types::error::Unhandled::builder()
                .source(err)
                .build(),
        )
    }

    /// Creates the `AssociateVPCWithHostedZoneError::Unhandled` variant from a `aws_smithy_types::error::ErrorMetadata`.
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
            Self::ConflictingDomainExists(e) => e.meta(),
            Self::InvalidInput(e) => e.meta(),
            Self::InvalidVpcId(e) => e.meta(),
            Self::LimitsExceeded(e) => e.meta(),
            Self::NoSuchHostedZone(e) => e.meta(),
            Self::NotAuthorizedException(e) => e.meta(),
            Self::PriorRequestNotComplete(e) => e.meta(),
            Self::PublicZoneVpcAssociation(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `AssociateVPCWithHostedZoneError::ConflictingDomainExists`.
    pub fn is_conflicting_domain_exists(&self) -> bool {
        matches!(self, Self::ConflictingDomainExists(_))
    }
    /// Returns `true` if the error kind is `AssociateVPCWithHostedZoneError::InvalidInput`.
    pub fn is_invalid_input(&self) -> bool {
        matches!(self, Self::InvalidInput(_))
    }
    /// Returns `true` if the error kind is `AssociateVPCWithHostedZoneError::InvalidVpcId`.
    pub fn is_invalid_vpc_id(&self) -> bool {
        matches!(self, Self::InvalidVpcId(_))
    }
    /// Returns `true` if the error kind is `AssociateVPCWithHostedZoneError::LimitsExceeded`.
    pub fn is_limits_exceeded(&self) -> bool {
        matches!(self, Self::LimitsExceeded(_))
    }
    /// Returns `true` if the error kind is `AssociateVPCWithHostedZoneError::NoSuchHostedZone`.
    pub fn is_no_such_hosted_zone(&self) -> bool {
        matches!(self, Self::NoSuchHostedZone(_))
    }
    /// Returns `true` if the error kind is `AssociateVPCWithHostedZoneError::NotAuthorizedException`.
    pub fn is_not_authorized_exception(&self) -> bool {
        matches!(self, Self::NotAuthorizedException(_))
    }
    /// Returns `true` if the error kind is `AssociateVPCWithHostedZoneError::PriorRequestNotComplete`.
    pub fn is_prior_request_not_complete(&self) -> bool {
        matches!(self, Self::PriorRequestNotComplete(_))
    }
    /// Returns `true` if the error kind is `AssociateVPCWithHostedZoneError::PublicZoneVpcAssociation`.
    pub fn is_public_zone_vpc_association(&self) -> bool {
        matches!(self, Self::PublicZoneVpcAssociation(_))
    }
}
impl std::error::Error for AssociateVPCWithHostedZoneError {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ConflictingDomainExists(_inner) => Some(_inner),
            Self::InvalidInput(_inner) => Some(_inner),
            Self::InvalidVpcId(_inner) => Some(_inner),
            Self::LimitsExceeded(_inner) => Some(_inner),
            Self::NoSuchHostedZone(_inner) => Some(_inner),
            Self::NotAuthorizedException(_inner) => Some(_inner),
            Self::PriorRequestNotComplete(_inner) => Some(_inner),
            Self::PublicZoneVpcAssociation(_inner) => Some(_inner),
            Self::Unhandled(_inner) => Some(_inner),
        }
    }
}

pub use crate::operation::associate_vpc_with_hosted_zone::_associate_vpc_with_hosted_zone_output::AssociateVpcWithHostedZoneOutput;

pub use crate::operation::associate_vpc_with_hosted_zone::_associate_vpc_with_hosted_zone_input::AssociateVpcWithHostedZoneInput;

mod _associate_vpc_with_hosted_zone_input;

mod _associate_vpc_with_hosted_zone_output;

/// Builders
pub mod builders;