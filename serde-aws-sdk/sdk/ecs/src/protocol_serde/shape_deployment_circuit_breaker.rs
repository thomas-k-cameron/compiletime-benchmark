// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_deployment_circuit_breaker(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::DeploymentCircuitBreaker,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    {
        object.key("enable").boolean(input.enable);
    }
    {
        object.key("rollback").boolean(input.rollback);
    }
    Ok(())
}

pub(crate) fn de_deployment_circuit_breaker<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::DeploymentCircuitBreaker>,
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
            let mut builder = crate::types::builders::DeploymentCircuitBreakerBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "enable" => {
                                builder = builder.set_enable(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(
                                        tokens.next(),
                                    )?,
                                );
                            }
                            "rollback" => {
                                builder = builder.set_rollback(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(
                                        tokens.next(),
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
            Ok(Some(builder.build()))
        }
        _ => Err(
            aws_smithy_json::deserialize::error::DeserializeError::custom(
                "expected start object or null",
            ),
        ),
    }
}