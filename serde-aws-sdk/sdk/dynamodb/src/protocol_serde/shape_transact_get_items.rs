// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_transact_get_items_input(
    input: &crate::operation::transact_get_items::TransactGetItemsInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_transact_get_items_input::ser_transact_get_items_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_transact_get_items_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::transact_get_items::TransactGetItemsOutput,
    crate::operation::transact_get_items::TransactGetItemsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::transact_get_items::TransactGetItemsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::transact_get_items::TransactGetItemsError::unhandled(generic),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServerError" => crate::operation::transact_get_items::TransactGetItemsError::InternalServerError({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServerErrorBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(response.body().as_ref(), output).map_err(crate::operation::transact_get_items::TransactGetItemsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidEndpointException" => crate::operation::transact_get_items::TransactGetItemsError::InvalidEndpointException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidEndpointExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::transact_get_items::TransactGetItemsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ProvisionedThroughputExceededException" => crate::operation::transact_get_items::TransactGetItemsError::ProvisionedThroughputExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ProvisionedThroughputExceededExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_provisioned_throughput_exceeded_exception::de_provisioned_throughput_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::transact_get_items::TransactGetItemsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "RequestLimitExceeded" => crate::operation::transact_get_items::TransactGetItemsError::RequestLimitExceeded({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::RequestLimitExceededBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_request_limit_exceeded::de_request_limit_exceeded_json_err(response.body().as_ref(), output).map_err(crate::operation::transact_get_items::TransactGetItemsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::transact_get_items::TransactGetItemsError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::transact_get_items::TransactGetItemsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TransactionCanceledException" => crate::operation::transact_get_items::TransactGetItemsError::TransactionCanceledException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::TransactionCanceledExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_transaction_canceled_exception::de_transaction_canceled_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::transact_get_items::TransactGetItemsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::transact_get_items::TransactGetItemsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_transact_get_items_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::transact_get_items::TransactGetItemsOutput,
    crate::operation::transact_get_items::TransactGetItemsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::transact_get_items::builders::TransactGetItemsOutputBuilder::default(
            );
        let _ = response;
        output = crate::protocol_serde::shape_transact_get_items::de_transact_get_items(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::operation::transact_get_items::TransactGetItemsError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_transact_get_items(
    value: &[u8],
    mut builder: crate::operation::transact_get_items::builders::TransactGetItemsOutputBuilder,
) -> Result<
    crate::operation::transact_get_items::builders::TransactGetItemsOutputBuilder,
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
                    "ConsumedCapacity" => {
                        builder = builder.set_consumed_capacity(
                            crate::protocol_serde::shape_consumed_capacity_multiple::de_consumed_capacity_multiple(tokens)?
                        );
                    }
                    "Responses" => {
                        builder = builder.set_responses(
                            crate::protocol_serde::shape_item_response_list::de_item_response_list(
                                tokens,
                            )?,
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