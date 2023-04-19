// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_bucket_metrics_configuration_headers(
    input: &crate::operation::put_bucket_metrics_configuration::PutBucketMetricsConfigurationInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_1) = &input.expected_bucket_owner {
        let formatted_2 = inner_1.as_str();
        if !formatted_2.is_empty() {
            let header_value = formatted_2;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "expected_bucket_owner",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("x-amz-expected-bucket-owner", header_value);
        }
    }
    Ok(builder)
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_bucket_metrics_configuration_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::put_bucket_metrics_configuration::PutBucketMetricsConfigurationOutput,
    crate::operation::put_bucket_metrics_configuration::PutBucketMetricsConfigurationError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::put_bucket_metrics_configuration::PutBucketMetricsConfigurationError::unhandled)?;
    generic_builder =
        crate::s3_request_id::apply_extended_request_id(generic_builder, response.headers());
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::operation::put_bucket_metrics_configuration::PutBucketMetricsConfigurationError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_bucket_metrics_configuration_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::put_bucket_metrics_configuration::PutBucketMetricsConfigurationOutput,
    crate::operation::put_bucket_metrics_configuration::PutBucketMetricsConfigurationError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::put_bucket_metrics_configuration::builders::PutBucketMetricsConfigurationOutputBuilder::default();
        let _ = response;
        output._set_extended_request_id(
            crate::s3_request_id::RequestIdExt::extended_request_id(response).map(str::to_string),
        );
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}