// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_complete_multipart_upload_headers(
    input: &crate::operation::complete_multipart_upload::CompleteMultipartUploadInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_1) = &input.archive_size {
        let formatted_2 = inner_1.as_str();
        if !formatted_2.is_empty() {
            let header_value = formatted_2;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "archive_size",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("x-amz-archive-size", header_value);
        }
    }
    if let Some(inner_3) = &input.checksum {
        let formatted_4 = inner_3.as_str();
        if !formatted_4.is_empty() {
            let header_value = formatted_4;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "checksum",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("x-amz-sha256-tree-hash", header_value);
        }
    }
    Ok(builder)
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_complete_multipart_upload_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::complete_multipart_upload::CompleteMultipartUploadOutput,
    crate::operation::complete_multipart_upload::CompleteMultipartUploadError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::complete_multipart_upload::CompleteMultipartUploadError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(
            crate::operation::complete_multipart_upload::CompleteMultipartUploadError::unhandled(
                generic,
            ),
        ),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidParameterValueException" => crate::operation::complete_multipart_upload::CompleteMultipartUploadError::InvalidParameterValueException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterValueExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::complete_multipart_upload::CompleteMultipartUploadError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "MissingParameterValueException" => crate::operation::complete_multipart_upload::CompleteMultipartUploadError::MissingParameterValueException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::MissingParameterValueExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_missing_parameter_value_exception::de_missing_parameter_value_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::complete_multipart_upload::CompleteMultipartUploadError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::complete_multipart_upload::CompleteMultipartUploadError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::complete_multipart_upload::CompleteMultipartUploadError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceUnavailableException" => crate::operation::complete_multipart_upload::CompleteMultipartUploadError::ServiceUnavailableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServiceUnavailableExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_unavailable_exception::de_service_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::complete_multipart_upload::CompleteMultipartUploadError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::complete_multipart_upload::CompleteMultipartUploadError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_complete_multipart_upload_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::complete_multipart_upload::CompleteMultipartUploadOutput,
    crate::operation::complete_multipart_upload::CompleteMultipartUploadError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::complete_multipart_upload::builders::CompleteMultipartUploadOutputBuilder::default();
        let _ = response;
        output = output.set_archive_id(
            crate::protocol_serde::shape_complete_multipart_upload_output::de_archive_id_header(response.headers())
                                    .map_err(|_|crate::operation::complete_multipart_upload::CompleteMultipartUploadError::unhandled("Failed to parse archiveId from header `x-amz-archive-id"))?
        );
        output = output.set_checksum(
            crate::protocol_serde::shape_complete_multipart_upload_output::de_checksum_header(response.headers())
                                    .map_err(|_|crate::operation::complete_multipart_upload::CompleteMultipartUploadError::unhandled("Failed to parse checksum from header `x-amz-sha256-tree-hash"))?
        );
        output = output.set_location(
            crate::protocol_serde::shape_complete_multipart_upload_output::de_location_header(response.headers())
                                    .map_err(|_|crate::operation::complete_multipart_upload::CompleteMultipartUploadError::unhandled("Failed to parse location from header `Location"))?
        );
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}