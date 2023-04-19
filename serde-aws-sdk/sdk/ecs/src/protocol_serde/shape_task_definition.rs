// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_task_definition<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::TaskDefinition>,
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
            let mut builder = crate::types::builders::TaskDefinitionBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "taskDefinitionArn" => {
                                builder = builder.set_task_definition_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "containerDefinitions" => {
                                builder = builder.set_container_definitions(
                                    crate::protocol_serde::shape_container_definitions::de_container_definitions(tokens)?
                                );
                            }
                            "family" => {
                                builder = builder.set_family(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "taskRoleArn" => {
                                builder = builder.set_task_role_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "executionRoleArn" => {
                                builder = builder.set_execution_role_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "networkMode" => {
                                builder = builder.set_network_mode(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped()
                                            .map(|u| crate::types::NetworkMode::from(u.as_ref()))
                                    })
                                    .transpose()?,
                                );
                            }
                            "revision" => {
                                builder = builder.set_revision(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "volumes" => {
                                builder = builder.set_volumes(
                                    crate::protocol_serde::shape_volume_list::de_volume_list(
                                        tokens,
                                    )?,
                                );
                            }
                            "status" => {
                                builder = builder.set_status(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::TaskDefinitionStatus::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "requiresAttributes" => {
                                builder = builder.set_requires_attributes(
                                    crate::protocol_serde::shape_requires_attributes::de_requires_attributes(tokens)?
                                );
                            }
                            "placementConstraints" => {
                                builder = builder.set_placement_constraints(
                                    crate::protocol_serde::shape_task_definition_placement_constraints::de_task_definition_placement_constraints(tokens)?
                                );
                            }
                            "compatibilities" => {
                                builder = builder.set_compatibilities(
                                    crate::protocol_serde::shape_compatibility_list::de_compatibility_list(tokens)?
                                );
                            }
                            "runtimePlatform" => {
                                builder = builder.set_runtime_platform(
                                    crate::protocol_serde::shape_runtime_platform::de_runtime_platform(tokens)?
                                );
                            }
                            "requiresCompatibilities" => {
                                builder = builder.set_requires_compatibilities(
                                    crate::protocol_serde::shape_compatibility_list::de_compatibility_list(tokens)?
                                );
                            }
                            "cpu" => {
                                builder = builder.set_cpu(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "memory" => {
                                builder = builder.set_memory(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "inferenceAccelerators" => {
                                builder = builder.set_inference_accelerators(
                                    crate::protocol_serde::shape_inference_accelerators::de_inference_accelerators(tokens)?
                                );
                            }
                            "pidMode" => {
                                builder = builder.set_pid_mode(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped()
                                            .map(|u| crate::types::PidMode::from(u.as_ref()))
                                    })
                                    .transpose()?,
                                );
                            }
                            "ipcMode" => {
                                builder = builder.set_ipc_mode(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped()
                                            .map(|u| crate::types::IpcMode::from(u.as_ref()))
                                    })
                                    .transpose()?,
                                );
                            }
                            "proxyConfiguration" => {
                                builder = builder.set_proxy_configuration(
                                    crate::protocol_serde::shape_proxy_configuration::de_proxy_configuration(tokens)?
                                );
                            }
                            "registeredAt" => {
                                builder = builder.set_registered_at(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                        tokens.next(),
                                        aws_smithy_types::date_time::Format::EpochSeconds,
                                    )?,
                                );
                            }
                            "deregisteredAt" => {
                                builder = builder.set_deregistered_at(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                        tokens.next(),
                                        aws_smithy_types::date_time::Format::EpochSeconds,
                                    )?,
                                );
                            }
                            "registeredBy" => {
                                builder = builder.set_registered_by(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "ephemeralStorage" => {
                                builder = builder.set_ephemeral_storage(
                                    crate::protocol_serde::shape_ephemeral_storage::de_ephemeral_storage(tokens)?
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