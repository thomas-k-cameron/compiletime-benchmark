// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_primary_region_input(
    input: &crate::operation::update_primary_region::UpdatePrimaryRegionInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_update_primary_region_input::ser_update_primary_region_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_primary_region_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::update_primary_region::UpdatePrimaryRegionOutput,
    crate::operation::update_primary_region::UpdatePrimaryRegionError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::update_primary_region::UpdatePrimaryRegionError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::update_primary_region::UpdatePrimaryRegionError::unhandled(
                    generic,
                ),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "DisabledException" => crate::operation::update_primary_region::UpdatePrimaryRegionError::DisabledException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::DisabledExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_disabled_exception::de_disabled_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_primary_region::UpdatePrimaryRegionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidArnException" => crate::operation::update_primary_region::UpdatePrimaryRegionError::InvalidArnException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidArnExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_arn_exception::de_invalid_arn_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_primary_region::UpdatePrimaryRegionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "KMSInternalException" => crate::operation::update_primary_region::UpdatePrimaryRegionError::KmsInternalException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::KmsInternalExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_kms_internal_exception::de_kms_internal_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_primary_region::UpdatePrimaryRegionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "KMSInvalidStateException" => crate::operation::update_primary_region::UpdatePrimaryRegionError::KmsInvalidStateException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::KmsInvalidStateExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_primary_region::UpdatePrimaryRegionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NotFoundException" => crate::operation::update_primary_region::UpdatePrimaryRegionError::NotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_primary_region::UpdatePrimaryRegionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnsupportedOperationException" => crate::operation::update_primary_region::UpdatePrimaryRegionError::UnsupportedOperationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UnsupportedOperationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_primary_region::UpdatePrimaryRegionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::update_primary_region::UpdatePrimaryRegionError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_primary_region_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::update_primary_region::UpdatePrimaryRegionOutput,
    crate::operation::update_primary_region::UpdatePrimaryRegionError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::update_primary_region::builders::UpdatePrimaryRegionOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
