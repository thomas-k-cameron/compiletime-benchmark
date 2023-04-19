// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_tag_open_id_connect_provider_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::tag_open_id_connect_provider::TagOpenIdConnectProviderOutput,
    crate::operation::tag_open_id_connect_provider::TagOpenIDConnectProviderError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::tag_open_id_connect_provider::TagOpenIDConnectProviderError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::tag_open_id_connect_provider::TagOpenIDConnectProviderError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ConcurrentModification" => crate::operation::tag_open_id_connect_provider::TagOpenIDConnectProviderError::ConcurrentModificationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ConcurrentModificationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_concurrent_modification_exception::de_concurrent_modification_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::tag_open_id_connect_provider::TagOpenIDConnectProviderError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidInput" => crate::operation::tag_open_id_connect_provider::TagOpenIDConnectProviderError::InvalidInputException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidInputExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_input_exception::de_invalid_input_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::tag_open_id_connect_provider::TagOpenIDConnectProviderError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "LimitExceeded" => crate::operation::tag_open_id_connect_provider::TagOpenIDConnectProviderError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::LimitExceededExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::tag_open_id_connect_provider::TagOpenIDConnectProviderError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchEntity" => crate::operation::tag_open_id_connect_provider::TagOpenIDConnectProviderError::NoSuchEntityException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NoSuchEntityExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_entity_exception::de_no_such_entity_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::tag_open_id_connect_provider::TagOpenIDConnectProviderError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceFailure" => crate::operation::tag_open_id_connect_provider::TagOpenIDConnectProviderError::ServiceFailureException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServiceFailureExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_failure_exception::de_service_failure_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::tag_open_id_connect_provider::TagOpenIDConnectProviderError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::tag_open_id_connect_provider::TagOpenIDConnectProviderError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_tag_open_id_connect_provider_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::tag_open_id_connect_provider::TagOpenIdConnectProviderOutput,
    crate::operation::tag_open_id_connect_provider::TagOpenIDConnectProviderError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::tag_open_id_connect_provider::builders::TagOpenIdConnectProviderOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
