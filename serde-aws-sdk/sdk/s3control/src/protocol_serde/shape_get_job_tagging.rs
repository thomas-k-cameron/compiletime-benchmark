// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_job_tagging_headers(
    input: &crate::operation::get_job_tagging::GetJobTaggingInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_1) = &input.account_id {
        let formatted_2 = inner_1.as_str();
        if !formatted_2.is_empty() {
            let header_value = formatted_2;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "account_id",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("x-amz-account-id", header_value);
        }
    }
    Ok(builder)
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_job_tagging_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::get_job_tagging::GetJobTaggingOutput,
    crate::operation::get_job_tagging::GetJobTaggingError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::get_job_tagging::GetJobTaggingError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::get_job_tagging::GetJobTaggingError::unhandled(generic))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServiceException" => {
            crate::operation::get_job_tagging::GetJobTaggingError::InternalServiceException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InternalServiceExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_service_exception::de_internal_service_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::get_job_tagging::GetJobTaggingError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "NotFoundException" => {
            crate::operation::get_job_tagging::GetJobTaggingError::NotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::NotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::get_job_tagging::GetJobTaggingError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "TooManyRequestsException" => {
            crate::operation::get_job_tagging::GetJobTaggingError::TooManyRequestsException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::get_job_tagging::GetJobTaggingError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::get_job_tagging::GetJobTaggingError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_job_tagging_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::get_job_tagging::GetJobTaggingOutput,
    crate::operation::get_job_tagging::GetJobTaggingError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::get_job_tagging::builders::GetJobTaggingOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_job_tagging::de_get_job_tagging(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::operation::get_job_tagging::GetJobTaggingError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_get_job_tagging(
    inp: &[u8],
    mut builder: crate::operation::get_job_tagging::builders::GetJobTaggingOutputBuilder,
) -> Result<
    crate::operation::get_job_tagging::builders::GetJobTaggingOutputBuilder,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !start_el.matches("GetJobTaggingResult") {
        return Err(
                                aws_smithy_xml::decode::XmlDecodeError::custom(
                                    format!("encountered invalid XML root: expected GetJobTaggingResult but got {:?}. This is likely a bug in the SDK.", start_el)
                                )
                            );
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Tags") /* Tags com.amazonaws.s3control.synthetic#GetJobTaggingOutput$Tags */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_s3_tag_set::de_s3_tag_set(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
