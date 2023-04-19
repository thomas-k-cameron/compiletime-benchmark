// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_resource_input(
    input: &crate::operation::get_resource::GetResourceInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_get_resource_input::ser_get_resource_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_resource_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::get_resource::GetResourceOutput,
    crate::operation::get_resource::GetResourceError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::get_resource::GetResourceError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::get_resource::GetResourceError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AlreadyExistsException" => {
            crate::operation::get_resource::GetResourceError::AlreadyExistsException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::AlreadyExistsExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_already_exists_exception::de_already_exists_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_resource::GetResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "GeneralServiceException" => {
            crate::operation::get_resource::GetResourceError::GeneralServiceException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::GeneralServiceExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_general_service_exception::de_general_service_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_resource::GetResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "HandlerFailureException" => {
            crate::operation::get_resource::GetResourceError::HandlerFailureException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::HandlerFailureExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_handler_failure_exception::de_handler_failure_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_resource::GetResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "HandlerInternalFailureException" => {
            crate::operation::get_resource::GetResourceError::HandlerInternalFailureException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::HandlerInternalFailureExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_handler_internal_failure_exception::de_handler_internal_failure_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_resource::GetResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidCredentialsException" => {
            crate::operation::get_resource::GetResourceError::InvalidCredentialsException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InvalidCredentialsExceptionBuilder::default(
                        );
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_credentials_exception::de_invalid_credentials_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_resource::GetResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidRequestException" => {
            crate::operation::get_resource::GetResourceError::InvalidRequestException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InvalidRequestExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_resource::GetResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "NetworkFailureException" => {
            crate::operation::get_resource::GetResourceError::NetworkFailureException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::NetworkFailureExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_network_failure_exception::de_network_failure_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_resource::GetResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "NotStabilizedException" => {
            crate::operation::get_resource::GetResourceError::NotStabilizedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::NotStabilizedExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_not_stabilized_exception::de_not_stabilized_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_resource::GetResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "NotUpdatableException" => {
            crate::operation::get_resource::GetResourceError::NotUpdatableException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::NotUpdatableExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_not_updatable_exception::de_not_updatable_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_resource::GetResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "PrivateTypeException" => {
            crate::operation::get_resource::GetResourceError::PrivateTypeException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::PrivateTypeExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_private_type_exception::de_private_type_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_resource::GetResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ResourceConflictException" => {
            crate::operation::get_resource::GetResourceError::ResourceConflictException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ResourceConflictExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_conflict_exception::de_resource_conflict_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_resource::GetResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ResourceNotFoundException" => {
            crate::operation::get_resource::GetResourceError::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_resource::GetResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ServiceInternalErrorException" => {
            crate::operation::get_resource::GetResourceError::ServiceInternalErrorException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServiceInternalErrorExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_internal_error_exception::de_service_internal_error_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_resource::GetResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ServiceLimitExceededException" => {
            crate::operation::get_resource::GetResourceError::ServiceLimitExceededException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServiceLimitExceededExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_limit_exceeded_exception::de_service_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_resource::GetResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ThrottlingException" => {
            crate::operation::get_resource::GetResourceError::ThrottlingException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ThrottlingExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_resource::GetResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "TypeNotFoundException" => {
            crate::operation::get_resource::GetResourceError::TypeNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::TypeNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_type_not_found_exception::de_type_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_resource::GetResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "UnsupportedActionException" => {
            crate::operation::get_resource::GetResourceError::UnsupportedActionException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::UnsupportedActionExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unsupported_action_exception::de_unsupported_action_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_resource::GetResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::get_resource::GetResourceError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_resource_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::get_resource::GetResourceOutput,
    crate::operation::get_resource::GetResourceError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::get_resource::builders::GetResourceOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_resource::de_get_resource(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::operation::get_resource::GetResourceError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_get_resource(
    value: &[u8],
    mut builder: crate::operation::get_resource::builders::GetResourceOutputBuilder,
) -> Result<
    crate::operation::get_resource::builders::GetResourceOutputBuilder,
    aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "TypeName" => {
                        builder = builder.set_type_name(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "ResourceDescription" => {
                        builder = builder.set_resource_description(
                            crate::protocol_serde::shape_resource_description::de_resource_description(tokens)?
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            other => {
                return Err(
                    aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                        "expected object key or end object, found: {:?}",
                        other
                    )),
                )
            }
        }
    }
    if tokens.next().is_some() {
        return Err(
            aws_smithy_json::deserialize::error::DeserializeError::custom(
                "found more JSON tokens after completing parsing",
            ),
        );
    }
    Ok(builder)
}