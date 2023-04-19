// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_alias_routing_configuration<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::AliasRoutingConfiguration>,
    aws_smithy_json::deserialize::error::DeserializeError,
>
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
            let mut builder = crate::types::builders::AliasRoutingConfigurationBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "AdditionalVersionWeights" => {
                                builder = builder.set_additional_version_weights(
                                    crate::protocol_serde::shape_additional_version_weights::de_additional_version_weights(tokens)?
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

pub fn ser_alias_routing_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AliasRoutingConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.additional_version_weights {
        #[allow(unused_mut)]
        let mut object_2 = object.key("AdditionalVersionWeights").start_object();
        for (key_3, value_4) in var_1 {
            {
                object_2.key(key_3.as_str()).number(
                    #[allow(clippy::useless_conversion)]
                    aws_smithy_types::Number::Float((*value_4).into()),
                );
            }
        }
        object_2.finish();
    }
    Ok(())
}
