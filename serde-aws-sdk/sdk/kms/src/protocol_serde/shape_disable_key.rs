// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_disable_key_input(
    input: &crate::operation::disable_key::DisableKeyInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_disable_key_input::ser_disable_key_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_disable_key_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::disable_key::DisableKeyOutput,
    crate::operation::disable_key::DisableKeyError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::disable_key::DisableKeyError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::disable_key::DisableKeyError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "DependencyTimeoutException" => {
            crate::operation::disable_key::DisableKeyError::DependencyTimeoutException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::DependencyTimeoutExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::disable_key::DisableKeyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidArnException" => {
            crate::operation::disable_key::DisableKeyError::InvalidArnException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InvalidArnExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_arn_exception::de_invalid_arn_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::disable_key::DisableKeyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "KMSInternalException" => {
            crate::operation::disable_key::DisableKeyError::KmsInternalException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::KmsInternalExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_kms_internal_exception::de_kms_internal_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::disable_key::DisableKeyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "KMSInvalidStateException" => {
            crate::operation::disable_key::DisableKeyError::KmsInvalidStateException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::KmsInvalidStateExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::disable_key::DisableKeyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "NotFoundException" => crate::operation::disable_key::DisableKeyError::NotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::NotFoundExceptionBuilder::default();
                let _ = response;
                output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::disable_key::DisableKeyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::disable_key::DisableKeyError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_disable_key_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::disable_key::DisableKeyOutput,
    crate::operation::disable_key::DisableKeyError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::disable_key::builders::DisableKeyOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
