// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_container_definition(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ContainerDefinition,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.image {
        object.key("image").string(var_2.as_str());
    }
    if let Some(var_3) = &input.repository_credentials {
        #[allow(unused_mut)]
        let mut object_4 = object.key("repositoryCredentials").start_object();
        crate::protocol_serde::shape_repository_credentials::ser_repository_credentials(
            &mut object_4,
            var_3,
        )?;
        object_4.finish();
    }
    if input.cpu != 0 {
        object.key("cpu").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.cpu).into()),
        );
    }
    if let Some(var_5) = &input.memory {
        object.key("memory").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_5).into()),
        );
    }
    if let Some(var_6) = &input.memory_reservation {
        object.key("memoryReservation").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_6).into()),
        );
    }
    if let Some(var_7) = &input.links {
        let mut array_8 = object.key("links").start_array();
        for item_9 in var_7 {
            {
                array_8.value().string(item_9.as_str());
            }
        }
        array_8.finish();
    }
    if let Some(var_10) = &input.port_mappings {
        let mut array_11 = object.key("portMappings").start_array();
        for item_12 in var_10 {
            {
                #[allow(unused_mut)]
                let mut object_13 = array_11.value().start_object();
                crate::protocol_serde::shape_port_mapping::ser_port_mapping(
                    &mut object_13,
                    item_12,
                )?;
                object_13.finish();
            }
        }
        array_11.finish();
    }
    if let Some(var_14) = &input.essential {
        object.key("essential").boolean(*var_14);
    }
    if let Some(var_15) = &input.entry_point {
        let mut array_16 = object.key("entryPoint").start_array();
        for item_17 in var_15 {
            {
                array_16.value().string(item_17.as_str());
            }
        }
        array_16.finish();
    }
    if let Some(var_18) = &input.command {
        let mut array_19 = object.key("command").start_array();
        for item_20 in var_18 {
            {
                array_19.value().string(item_20.as_str());
            }
        }
        array_19.finish();
    }
    if let Some(var_21) = &input.environment {
        let mut array_22 = object.key("environment").start_array();
        for item_23 in var_21 {
            {
                #[allow(unused_mut)]
                let mut object_24 = array_22.value().start_object();
                crate::protocol_serde::shape_key_value_pair::ser_key_value_pair(
                    &mut object_24,
                    item_23,
                )?;
                object_24.finish();
            }
        }
        array_22.finish();
    }
    if let Some(var_25) = &input.environment_files {
        let mut array_26 = object.key("environmentFiles").start_array();
        for item_27 in var_25 {
            {
                #[allow(unused_mut)]
                let mut object_28 = array_26.value().start_object();
                crate::protocol_serde::shape_environment_file::ser_environment_file(
                    &mut object_28,
                    item_27,
                )?;
                object_28.finish();
            }
        }
        array_26.finish();
    }
    if let Some(var_29) = &input.mount_points {
        let mut array_30 = object.key("mountPoints").start_array();
        for item_31 in var_29 {
            {
                #[allow(unused_mut)]
                let mut object_32 = array_30.value().start_object();
                crate::protocol_serde::shape_mount_point::ser_mount_point(&mut object_32, item_31)?;
                object_32.finish();
            }
        }
        array_30.finish();
    }
    if let Some(var_33) = &input.volumes_from {
        let mut array_34 = object.key("volumesFrom").start_array();
        for item_35 in var_33 {
            {
                #[allow(unused_mut)]
                let mut object_36 = array_34.value().start_object();
                crate::protocol_serde::shape_volume_from::ser_volume_from(&mut object_36, item_35)?;
                object_36.finish();
            }
        }
        array_34.finish();
    }
    if let Some(var_37) = &input.linux_parameters {
        #[allow(unused_mut)]
        let mut object_38 = object.key("linuxParameters").start_object();
        crate::protocol_serde::shape_linux_parameters::ser_linux_parameters(
            &mut object_38,
            var_37,
        )?;
        object_38.finish();
    }
    if let Some(var_39) = &input.secrets {
        let mut array_40 = object.key("secrets").start_array();
        for item_41 in var_39 {
            {
                #[allow(unused_mut)]
                let mut object_42 = array_40.value().start_object();
                crate::protocol_serde::shape_secret::ser_secret(&mut object_42, item_41)?;
                object_42.finish();
            }
        }
        array_40.finish();
    }
    if let Some(var_43) = &input.depends_on {
        let mut array_44 = object.key("dependsOn").start_array();
        for item_45 in var_43 {
            {
                #[allow(unused_mut)]
                let mut object_46 = array_44.value().start_object();
                crate::protocol_serde::shape_container_dependency::ser_container_dependency(
                    &mut object_46,
                    item_45,
                )?;
                object_46.finish();
            }
        }
        array_44.finish();
    }
    if let Some(var_47) = &input.start_timeout {
        object.key("startTimeout").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_47).into()),
        );
    }
    if let Some(var_48) = &input.stop_timeout {
        object.key("stopTimeout").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_48).into()),
        );
    }
    if let Some(var_49) = &input.hostname {
        object.key("hostname").string(var_49.as_str());
    }
    if let Some(var_50) = &input.user {
        object.key("user").string(var_50.as_str());
    }
    if let Some(var_51) = &input.working_directory {
        object.key("workingDirectory").string(var_51.as_str());
    }
    if let Some(var_52) = &input.disable_networking {
        object.key("disableNetworking").boolean(*var_52);
    }
    if let Some(var_53) = &input.privileged {
        object.key("privileged").boolean(*var_53);
    }
    if let Some(var_54) = &input.readonly_root_filesystem {
        object.key("readonlyRootFilesystem").boolean(*var_54);
    }
    if let Some(var_55) = &input.dns_servers {
        let mut array_56 = object.key("dnsServers").start_array();
        for item_57 in var_55 {
            {
                array_56.value().string(item_57.as_str());
            }
        }
        array_56.finish();
    }
    if let Some(var_58) = &input.dns_search_domains {
        let mut array_59 = object.key("dnsSearchDomains").start_array();
        for item_60 in var_58 {
            {
                array_59.value().string(item_60.as_str());
            }
        }
        array_59.finish();
    }
    if let Some(var_61) = &input.extra_hosts {
        let mut array_62 = object.key("extraHosts").start_array();
        for item_63 in var_61 {
            {
                #[allow(unused_mut)]
                let mut object_64 = array_62.value().start_object();
                crate::protocol_serde::shape_host_entry::ser_host_entry(&mut object_64, item_63)?;
                object_64.finish();
            }
        }
        array_62.finish();
    }
    if let Some(var_65) = &input.docker_security_options {
        let mut array_66 = object.key("dockerSecurityOptions").start_array();
        for item_67 in var_65 {
            {
                array_66.value().string(item_67.as_str());
            }
        }
        array_66.finish();
    }
    if let Some(var_68) = &input.interactive {
        object.key("interactive").boolean(*var_68);
    }
    if let Some(var_69) = &input.pseudo_terminal {
        object.key("pseudoTerminal").boolean(*var_69);
    }
    if let Some(var_70) = &input.docker_labels {
        #[allow(unused_mut)]
        let mut object_71 = object.key("dockerLabels").start_object();
        for (key_72, value_73) in var_70 {
            {
                object_71.key(key_72.as_str()).string(value_73.as_str());
            }
        }
        object_71.finish();
    }
    if let Some(var_74) = &input.ulimits {
        let mut array_75 = object.key("ulimits").start_array();
        for item_76 in var_74 {
            {
                #[allow(unused_mut)]
                let mut object_77 = array_75.value().start_object();
                crate::protocol_serde::shape_ulimit::ser_ulimit(&mut object_77, item_76)?;
                object_77.finish();
            }
        }
        array_75.finish();
    }
    if let Some(var_78) = &input.log_configuration {
        #[allow(unused_mut)]
        let mut object_79 = object.key("logConfiguration").start_object();
        crate::protocol_serde::shape_log_configuration::ser_log_configuration(
            &mut object_79,
            var_78,
        )?;
        object_79.finish();
    }
    if let Some(var_80) = &input.health_check {
        #[allow(unused_mut)]
        let mut object_81 = object.key("healthCheck").start_object();
        crate::protocol_serde::shape_health_check::ser_health_check(&mut object_81, var_80)?;
        object_81.finish();
    }
    if let Some(var_82) = &input.system_controls {
        let mut array_83 = object.key("systemControls").start_array();
        for item_84 in var_82 {
            {
                #[allow(unused_mut)]
                let mut object_85 = array_83.value().start_object();
                crate::protocol_serde::shape_system_control::ser_system_control(
                    &mut object_85,
                    item_84,
                )?;
                object_85.finish();
            }
        }
        array_83.finish();
    }
    if let Some(var_86) = &input.resource_requirements {
        let mut array_87 = object.key("resourceRequirements").start_array();
        for item_88 in var_86 {
            {
                #[allow(unused_mut)]
                let mut object_89 = array_87.value().start_object();
                crate::protocol_serde::shape_resource_requirement::ser_resource_requirement(
                    &mut object_89,
                    item_88,
                )?;
                object_89.finish();
            }
        }
        array_87.finish();
    }
    if let Some(var_90) = &input.firelens_configuration {
        #[allow(unused_mut)]
        let mut object_91 = object.key("firelensConfiguration").start_object();
        crate::protocol_serde::shape_firelens_configuration::ser_firelens_configuration(
            &mut object_91,
            var_90,
        )?;
        object_91.finish();
    }
    Ok(())
}

pub(crate) fn de_container_definition<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::ContainerDefinition>,
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
            let mut builder = crate::types::builders::ContainerDefinitionBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "name" => {
                                builder = builder.set_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "image" => {
                                builder = builder.set_image(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "repositoryCredentials" => {
                                builder = builder.set_repository_credentials(
                                    crate::protocol_serde::shape_repository_credentials::de_repository_credentials(tokens)?
                                );
                            }
                            "cpu" => {
                                builder = builder.set_cpu(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "memory" => {
                                builder = builder.set_memory(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "memoryReservation" => {
                                builder = builder.set_memory_reservation(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "links" => {
                                builder = builder.set_links(
                                    crate::protocol_serde::shape_string_list::de_string_list(
                                        tokens,
                                    )?,
                                );
                            }
                            "portMappings" => {
                                builder = builder.set_port_mappings(
                                    crate::protocol_serde::shape_port_mapping_list::de_port_mapping_list(tokens)?
                                );
                            }
                            "essential" => {
                                builder = builder.set_essential(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(
                                        tokens.next(),
                                    )?,
                                );
                            }
                            "entryPoint" => {
                                builder = builder.set_entry_point(
                                    crate::protocol_serde::shape_string_list::de_string_list(
                                        tokens,
                                    )?,
                                );
                            }
                            "command" => {
                                builder = builder.set_command(
                                    crate::protocol_serde::shape_string_list::de_string_list(
                                        tokens,
                                    )?,
                                );
                            }
                            "environment" => {
                                builder = builder.set_environment(
                                    crate::protocol_serde::shape_environment_variables::de_environment_variables(tokens)?
                                );
                            }
                            "environmentFiles" => {
                                builder = builder.set_environment_files(
                                    crate::protocol_serde::shape_environment_files::de_environment_files(tokens)?
                                );
                            }
                            "mountPoints" => {
                                builder = builder.set_mount_points(
                                    crate::protocol_serde::shape_mount_point_list::de_mount_point_list(tokens)?
                                );
                            }
                            "volumesFrom" => {
                                builder = builder.set_volumes_from(
                                    crate::protocol_serde::shape_volume_from_list::de_volume_from_list(tokens)?
                                );
                            }
                            "linuxParameters" => {
                                builder = builder.set_linux_parameters(
                                    crate::protocol_serde::shape_linux_parameters::de_linux_parameters(tokens)?
                                );
                            }
                            "secrets" => {
                                builder = builder.set_secrets(
                                    crate::protocol_serde::shape_secret_list::de_secret_list(
                                        tokens,
                                    )?,
                                );
                            }
                            "dependsOn" => {
                                builder = builder.set_depends_on(
                                    crate::protocol_serde::shape_container_dependencies::de_container_dependencies(tokens)?
                                );
                            }
                            "startTimeout" => {
                                builder = builder.set_start_timeout(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "stopTimeout" => {
                                builder = builder.set_stop_timeout(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "hostname" => {
                                builder = builder.set_hostname(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "user" => {
                                builder = builder.set_user(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "workingDirectory" => {
                                builder = builder.set_working_directory(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "disableNetworking" => {
                                builder = builder.set_disable_networking(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(
                                        tokens.next(),
                                    )?,
                                );
                            }
                            "privileged" => {
                                builder = builder.set_privileged(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(
                                        tokens.next(),
                                    )?,
                                );
                            }
                            "readonlyRootFilesystem" => {
                                builder = builder.set_readonly_root_filesystem(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(
                                        tokens.next(),
                                    )?,
                                );
                            }
                            "dnsServers" => {
                                builder = builder.set_dns_servers(
                                    crate::protocol_serde::shape_string_list::de_string_list(
                                        tokens,
                                    )?,
                                );
                            }
                            "dnsSearchDomains" => {
                                builder = builder.set_dns_search_domains(
                                    crate::protocol_serde::shape_string_list::de_string_list(
                                        tokens,
                                    )?,
                                );
                            }
                            "extraHosts" => {
                                builder = builder.set_extra_hosts(
                                    crate::protocol_serde::shape_host_entry_list::de_host_entry_list(tokens)?
                                );
                            }
                            "dockerSecurityOptions" => {
                                builder = builder.set_docker_security_options(
                                    crate::protocol_serde::shape_string_list::de_string_list(
                                        tokens,
                                    )?,
                                );
                            }
                            "interactive" => {
                                builder = builder.set_interactive(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(
                                        tokens.next(),
                                    )?,
                                );
                            }
                            "pseudoTerminal" => {
                                builder = builder.set_pseudo_terminal(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(
                                        tokens.next(),
                                    )?,
                                );
                            }
                            "dockerLabels" => {
                                builder = builder.set_docker_labels(
                                    crate::protocol_serde::shape_docker_labels_map::de_docker_labels_map(tokens)?
                                );
                            }
                            "ulimits" => {
                                builder = builder.set_ulimits(
                                    crate::protocol_serde::shape_ulimit_list::de_ulimit_list(
                                        tokens,
                                    )?,
                                );
                            }
                            "logConfiguration" => {
                                builder = builder.set_log_configuration(
                                    crate::protocol_serde::shape_log_configuration::de_log_configuration(tokens)?
                                );
                            }
                            "healthCheck" => {
                                builder = builder.set_health_check(
                                    crate::protocol_serde::shape_health_check::de_health_check(
                                        tokens,
                                    )?,
                                );
                            }
                            "systemControls" => {
                                builder = builder.set_system_controls(
                                    crate::protocol_serde::shape_system_controls::de_system_controls(tokens)?
                                );
                            }
                            "resourceRequirements" => {
                                builder = builder.set_resource_requirements(
                                    crate::protocol_serde::shape_resource_requirements::de_resource_requirements(tokens)?
                                );
                            }
                            "firelensConfiguration" => {
                                builder = builder.set_firelens_configuration(
                                    crate::protocol_serde::shape_firelens_configuration::de_firelens_configuration(tokens)?
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
