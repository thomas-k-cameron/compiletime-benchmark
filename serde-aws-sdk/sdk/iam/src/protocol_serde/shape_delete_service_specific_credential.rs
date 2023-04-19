// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_service_specific_credential_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_service_specific_credential::DeleteServiceSpecificCredentialOutput,
    crate::operation::delete_service_specific_credential::DeleteServiceSpecificCredentialError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::delete_service_specific_credential::DeleteServiceSpecificCredentialError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::delete_service_specific_credential::DeleteServiceSpecificCredentialError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "NoSuchEntity" => crate::operation::delete_service_specific_credential::DeleteServiceSpecificCredentialError::NoSuchEntityException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NoSuchEntityExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_entity_exception::de_no_such_entity_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::delete_service_specific_credential::DeleteServiceSpecificCredentialError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::delete_service_specific_credential::DeleteServiceSpecificCredentialError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_service_specific_credential_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_service_specific_credential::DeleteServiceSpecificCredentialOutput,
    crate::operation::delete_service_specific_credential::DeleteServiceSpecificCredentialError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_service_specific_credential::builders::DeleteServiceSpecificCredentialOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
