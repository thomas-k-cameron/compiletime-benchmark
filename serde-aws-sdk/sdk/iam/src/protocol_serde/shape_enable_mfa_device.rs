// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_enable_mfa_device_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::enable_mfa_device::EnableMfaDeviceOutput,
    crate::operation::enable_mfa_device::EnableMFADeviceError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::enable_mfa_device::EnableMFADeviceError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::enable_mfa_device::EnableMFADeviceError::unhandled(generic),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "EntityAlreadyExists" => crate::operation::enable_mfa_device::EnableMFADeviceError::EntityAlreadyExistsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::EntityAlreadyExistsExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_entity_already_exists_exception::de_entity_already_exists_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::enable_mfa_device::EnableMFADeviceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "EntityTemporarilyUnmodifiable" => crate::operation::enable_mfa_device::EnableMFADeviceError::EntityTemporarilyUnmodifiableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::EntityTemporarilyUnmodifiableExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_entity_temporarily_unmodifiable_exception::de_entity_temporarily_unmodifiable_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::enable_mfa_device::EnableMFADeviceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidAuthenticationCode" => crate::operation::enable_mfa_device::EnableMFADeviceError::InvalidAuthenticationCodeException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidAuthenticationCodeExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_authentication_code_exception::de_invalid_authentication_code_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::enable_mfa_device::EnableMFADeviceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "LimitExceeded" => crate::operation::enable_mfa_device::EnableMFADeviceError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::LimitExceededExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::enable_mfa_device::EnableMFADeviceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchEntity" => crate::operation::enable_mfa_device::EnableMFADeviceError::NoSuchEntityException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NoSuchEntityExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_entity_exception::de_no_such_entity_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::enable_mfa_device::EnableMFADeviceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceFailure" => crate::operation::enable_mfa_device::EnableMFADeviceError::ServiceFailureException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServiceFailureExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_failure_exception::de_service_failure_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::enable_mfa_device::EnableMFADeviceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::enable_mfa_device::EnableMFADeviceError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_enable_mfa_device_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::enable_mfa_device::EnableMfaDeviceOutput,
    crate::operation::enable_mfa_device::EnableMFADeviceError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::enable_mfa_device::builders::EnableMfaDeviceOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
