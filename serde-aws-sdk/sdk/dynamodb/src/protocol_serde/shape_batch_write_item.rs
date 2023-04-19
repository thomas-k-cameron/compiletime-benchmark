// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_batch_write_item_input(
    input: &crate::operation::batch_write_item::BatchWriteItemInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_batch_write_item_input::ser_batch_write_item_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_batch_write_item_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::batch_write_item::BatchWriteItemOutput,
    crate::operation::batch_write_item::BatchWriteItemError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::batch_write_item::BatchWriteItemError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::batch_write_item::BatchWriteItemError::unhandled(generic))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServerError" => crate::operation::batch_write_item::BatchWriteItemError::InternalServerError({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServerErrorBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(response.body().as_ref(), output).map_err(crate::operation::batch_write_item::BatchWriteItemError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidEndpointException" => crate::operation::batch_write_item::BatchWriteItemError::InvalidEndpointException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidEndpointExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::batch_write_item::BatchWriteItemError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ItemCollectionSizeLimitExceededException" => crate::operation::batch_write_item::BatchWriteItemError::ItemCollectionSizeLimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ItemCollectionSizeLimitExceededExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_item_collection_size_limit_exceeded_exception::de_item_collection_size_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::batch_write_item::BatchWriteItemError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ProvisionedThroughputExceededException" => crate::operation::batch_write_item::BatchWriteItemError::ProvisionedThroughputExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ProvisionedThroughputExceededExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_provisioned_throughput_exceeded_exception::de_provisioned_throughput_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::batch_write_item::BatchWriteItemError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "RequestLimitExceeded" => crate::operation::batch_write_item::BatchWriteItemError::RequestLimitExceeded({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::RequestLimitExceededBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_request_limit_exceeded::de_request_limit_exceeded_json_err(response.body().as_ref(), output).map_err(crate::operation::batch_write_item::BatchWriteItemError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::batch_write_item::BatchWriteItemError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::batch_write_item::BatchWriteItemError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::batch_write_item::BatchWriteItemError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_batch_write_item_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::batch_write_item::BatchWriteItemOutput,
    crate::operation::batch_write_item::BatchWriteItemError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::batch_write_item::builders::BatchWriteItemOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_batch_write_item::de_batch_write_item(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::operation::batch_write_item::BatchWriteItemError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_batch_write_item(
    value: &[u8],
    mut builder: crate::operation::batch_write_item::builders::BatchWriteItemOutputBuilder,
) -> Result<
    crate::operation::batch_write_item::builders::BatchWriteItemOutputBuilder,
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
                    "UnprocessedItems" => {
                        builder = builder.set_unprocessed_items(
                            crate::protocol_serde::shape_batch_write_item_request_map::de_batch_write_item_request_map(tokens)?
                        );
                    }
                    "ItemCollectionMetrics" => {
                        builder = builder.set_item_collection_metrics(
                            crate::protocol_serde::shape_item_collection_metrics_per_table::de_item_collection_metrics_per_table(tokens)?
                        );
                    }
                    "ConsumedCapacity" => {
                        builder = builder.set_consumed_capacity(
                            crate::protocol_serde::shape_consumed_capacity_multiple::de_consumed_capacity_multiple(tokens)?
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
