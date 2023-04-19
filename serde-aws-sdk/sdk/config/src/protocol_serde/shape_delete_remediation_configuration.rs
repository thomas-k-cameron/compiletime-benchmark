// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_remediation_configuration_input(
    input: &crate::operation::delete_remediation_configuration::DeleteRemediationConfigurationInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_delete_remediation_configuration_input::ser_delete_remediation_configuration_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_remediation_configuration_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_remediation_configuration::DeleteRemediationConfigurationOutput,
    crate::operation::delete_remediation_configuration::DeleteRemediationConfigurationError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::delete_remediation_configuration::DeleteRemediationConfigurationError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::delete_remediation_configuration::DeleteRemediationConfigurationError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InsufficientPermissionsException" => crate::operation::delete_remediation_configuration::DeleteRemediationConfigurationError::InsufficientPermissionsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InsufficientPermissionsExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_insufficient_permissions_exception::de_insufficient_permissions_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_remediation_configuration::DeleteRemediationConfigurationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterValueException" => crate::operation::delete_remediation_configuration::DeleteRemediationConfigurationError::InvalidParameterValueException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterValueExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_remediation_configuration::DeleteRemediationConfigurationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchRemediationConfigurationException" => crate::operation::delete_remediation_configuration::DeleteRemediationConfigurationError::NoSuchRemediationConfigurationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NoSuchRemediationConfigurationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_remediation_configuration_exception::de_no_such_remediation_configuration_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_remediation_configuration::DeleteRemediationConfigurationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "RemediationInProgressException" => crate::operation::delete_remediation_configuration::DeleteRemediationConfigurationError::RemediationInProgressException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::RemediationInProgressExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_remediation_in_progress_exception::de_remediation_in_progress_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_remediation_configuration::DeleteRemediationConfigurationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::delete_remediation_configuration::DeleteRemediationConfigurationError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_remediation_configuration_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_remediation_configuration::DeleteRemediationConfigurationOutput,
    crate::operation::delete_remediation_configuration::DeleteRemediationConfigurationError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_remediation_configuration::builders::DeleteRemediationConfigurationOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
