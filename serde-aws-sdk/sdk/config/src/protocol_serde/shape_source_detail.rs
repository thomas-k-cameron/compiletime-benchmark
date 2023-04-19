// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_source_detail(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::SourceDetail,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.event_source {
        object.key("EventSource").string(var_1.as_str());
    }
    if let Some(var_2) = &input.message_type {
        object.key("MessageType").string(var_2.as_str());
    }
    if let Some(var_3) = &input.maximum_execution_frequency {
        object
            .key("MaximumExecutionFrequency")
            .string(var_3.as_str());
    }
    Ok(())
}

pub(crate) fn de_source_detail<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::types::SourceDetail>, aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<
        Item = Result<
            aws_smithy_json::deserialize::Token<'a>,
            aws_smithy_json::deserialize::error::DeserializeError,
        >,
    >,
{
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::SourceDetailBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "EventSource" => {
                                builder = builder.set_event_source(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped()
                                            .map(|u| crate::types::EventSource::from(u.as_ref()))
                                    })
                                    .transpose()?,
                                );
                            }
                            "MessageType" => {
                                builder = builder.set_message_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped()
                                            .map(|u| crate::types::MessageType::from(u.as_ref()))
                                    })
                                    .transpose()?,
                                );
                            }
                            "MaximumExecutionFrequency" => {
                                builder = builder.set_maximum_execution_frequency(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::MaximumExecutionFrequency::from(
                                                u.as_ref(),
                                            )
                                        })
                                    })
                                    .transpose()?,
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
            Ok(Some(builder.build()))
        }
        _ => Err(
            aws_smithy_json::deserialize::error::DeserializeError::custom(
                "expected start object or null",
            ),
        ),
    }
}
