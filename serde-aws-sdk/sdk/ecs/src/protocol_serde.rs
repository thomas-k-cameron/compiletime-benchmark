// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_create_capacity_provider;

pub fn parse_http_error_metadata(
    response: &http::Response<bytes::Bytes>,
) -> Result<
    aws_smithy_types::error::metadata::Builder,
    aws_smithy_json::deserialize::error::DeserializeError,
> {
    crate::json_errors::parse_error_metadata(response.body(), response.headers())
}

pub(crate) mod shape_create_cluster;

pub(crate) mod shape_create_service;

pub(crate) mod shape_create_task_set;

pub(crate) mod shape_delete_account_setting;

pub(crate) mod shape_delete_attributes;

pub(crate) mod shape_delete_capacity_provider;

pub(crate) mod shape_delete_cluster;

pub(crate) mod shape_delete_service;

pub(crate) mod shape_delete_task_definitions;

pub(crate) mod shape_delete_task_set;

pub(crate) mod shape_deregister_container_instance;

pub(crate) mod shape_deregister_task_definition;

pub(crate) mod shape_describe_capacity_providers;

pub(crate) mod shape_describe_clusters;

pub(crate) mod shape_describe_container_instances;

pub(crate) mod shape_describe_services;

pub(crate) mod shape_describe_task_definition;

pub(crate) mod shape_describe_task_sets;

pub(crate) mod shape_describe_tasks;

pub(crate) mod shape_discover_poll_endpoint;

pub(crate) mod shape_execute_command;

pub(crate) mod shape_get_task_protection;

pub(crate) mod shape_list_account_settings;

pub(crate) mod shape_list_attributes;

pub(crate) mod shape_list_clusters;

pub(crate) mod shape_list_container_instances;

pub(crate) mod shape_list_services;

pub(crate) mod shape_list_services_by_namespace;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_list_task_definition_families;

pub(crate) mod shape_list_task_definitions;

pub(crate) mod shape_list_tasks;

pub(crate) mod shape_put_account_setting;

pub(crate) mod shape_put_account_setting_default;

pub(crate) mod shape_put_attributes;

pub(crate) mod shape_put_cluster_capacity_providers;

pub(crate) mod shape_register_container_instance;

pub(crate) mod shape_register_task_definition;

pub(crate) mod shape_run_task;

pub(crate) mod shape_start_task;

pub(crate) mod shape_stop_task;

pub(crate) mod shape_submit_attachment_state_changes;

pub(crate) mod shape_submit_container_state_change;

pub(crate) mod shape_submit_task_state_change;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_capacity_provider;

pub(crate) mod shape_update_cluster;

pub(crate) mod shape_update_cluster_settings;

pub(crate) mod shape_update_container_agent;

pub(crate) mod shape_update_container_instances_state;

pub(crate) mod shape_update_service;

pub(crate) mod shape_update_service_primary_task_set;

pub(crate) mod shape_update_task_protection;

pub(crate) mod shape_update_task_set;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_attribute_limit_exceeded_exception;

pub(crate) mod shape_blocked_exception;

pub(crate) mod shape_client_exception;

pub(crate) mod shape_cluster_contains_container_instances_exception;

pub(crate) mod shape_cluster_contains_services_exception;

pub(crate) mod shape_cluster_contains_tasks_exception;

pub(crate) mod shape_cluster_not_found_exception;

pub(crate) mod shape_create_capacity_provider_input;

pub(crate) mod shape_create_cluster_input;

pub(crate) mod shape_create_service_input;

pub(crate) mod shape_create_task_set_input;

pub(crate) mod shape_delete_account_setting_input;

pub(crate) mod shape_delete_attributes_input;

pub(crate) mod shape_delete_capacity_provider_input;

pub(crate) mod shape_delete_cluster_input;

pub(crate) mod shape_delete_service_input;

pub(crate) mod shape_delete_task_definitions_input;

pub(crate) mod shape_delete_task_set_input;

pub(crate) mod shape_deregister_container_instance_input;

pub(crate) mod shape_deregister_task_definition_input;

pub(crate) mod shape_describe_capacity_providers_input;

pub(crate) mod shape_describe_clusters_input;

pub(crate) mod shape_describe_container_instances_input;

pub(crate) mod shape_describe_services_input;

pub(crate) mod shape_describe_task_definition_input;

pub(crate) mod shape_describe_task_sets_input;

pub(crate) mod shape_describe_tasks_input;

pub(crate) mod shape_discover_poll_endpoint_input;

pub(crate) mod shape_execute_command_input;

pub(crate) mod shape_get_task_protection_input;

pub(crate) mod shape_invalid_parameter_exception;

pub(crate) mod shape_limit_exceeded_exception;

pub(crate) mod shape_list_account_settings_input;

pub(crate) mod shape_list_attributes_input;

pub(crate) mod shape_list_clusters_input;

pub(crate) mod shape_list_container_instances_input;

pub(crate) mod shape_list_services_by_namespace_input;

pub(crate) mod shape_list_services_input;

pub(crate) mod shape_list_tags_for_resource_input;

pub(crate) mod shape_list_task_definition_families_input;

pub(crate) mod shape_list_task_definitions_input;

pub(crate) mod shape_list_tasks_input;

pub(crate) mod shape_missing_version_exception;

pub(crate) mod shape_namespace_not_found_exception;

pub(crate) mod shape_no_update_available_exception;

pub(crate) mod shape_platform_task_definition_incompatibility_exception;

pub(crate) mod shape_platform_unknown_exception;

pub(crate) mod shape_put_account_setting_default_input;

pub(crate) mod shape_put_account_setting_input;

pub(crate) mod shape_put_attributes_input;

pub(crate) mod shape_put_cluster_capacity_providers_input;

pub(crate) mod shape_register_container_instance_input;

pub(crate) mod shape_register_task_definition_input;

pub(crate) mod shape_resource_in_use_exception;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_run_task_input;

pub(crate) mod shape_server_exception;

pub(crate) mod shape_service_not_active_exception;

pub(crate) mod shape_service_not_found_exception;

pub(crate) mod shape_start_task_input;

pub(crate) mod shape_stop_task_input;

pub(crate) mod shape_submit_attachment_state_changes_input;

pub(crate) mod shape_submit_container_state_change_input;

pub(crate) mod shape_submit_task_state_change_input;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_target_not_connected_exception;

pub(crate) mod shape_target_not_found_exception;

pub(crate) mod shape_task_set_not_found_exception;

pub(crate) mod shape_unsupported_feature_exception;

pub(crate) mod shape_untag_resource_input;

pub(crate) mod shape_update_capacity_provider_input;

pub(crate) mod shape_update_cluster_input;

pub(crate) mod shape_update_cluster_settings_input;

pub(crate) mod shape_update_container_agent_input;

pub(crate) mod shape_update_container_instances_state_input;

pub(crate) mod shape_update_in_progress_exception;

pub(crate) mod shape_update_service_input;

pub(crate) mod shape_update_service_primary_task_set_input;

pub(crate) mod shape_update_task_protection_input;

pub(crate) mod shape_update_task_set_input;

pub(crate) mod shape_attachment_state_change;

pub(crate) mod shape_attribute;

pub(crate) mod shape_attributes;

pub(crate) mod shape_auto_scaling_group_provider;

pub(crate) mod shape_auto_scaling_group_provider_update;

pub(crate) mod shape_capacity_provider;

pub(crate) mod shape_capacity_provider_strategy_item;

pub(crate) mod shape_capacity_providers;

pub(crate) mod shape_cluster;

pub(crate) mod shape_cluster_configuration;

pub(crate) mod shape_cluster_service_connect_defaults_request;

pub(crate) mod shape_cluster_setting;

pub(crate) mod shape_clusters;

pub(crate) mod shape_container_definition;

pub(crate) mod shape_container_instance;

pub(crate) mod shape_container_instances;

pub(crate) mod shape_container_state_change;

pub(crate) mod shape_deployment_configuration;

pub(crate) mod shape_deployment_controller;

pub(crate) mod shape_ephemeral_storage;

pub(crate) mod shape_failures;

pub(crate) mod shape_inference_accelerator;

pub(crate) mod shape_load_balancer;

pub(crate) mod shape_managed_agent_state_change;

pub(crate) mod shape_network_binding;

pub(crate) mod shape_network_configuration;

pub(crate) mod shape_placement_constraint;

pub(crate) mod shape_placement_strategy;

pub(crate) mod shape_platform_device;

pub(crate) mod shape_protected_tasks;

pub(crate) mod shape_proxy_configuration;

pub(crate) mod shape_resource;

pub(crate) mod shape_runtime_platform;

pub(crate) mod shape_scale;

pub(crate) mod shape_service;

pub(crate) mod shape_service_connect_configuration;

pub(crate) mod shape_service_registry;

pub(crate) mod shape_services;

pub(crate) mod shape_session;

pub(crate) mod shape_setting;

pub(crate) mod shape_settings;

pub(crate) mod shape_string_list;

pub(crate) mod shape_tag;

pub(crate) mod shape_tags;

pub(crate) mod shape_task;

pub(crate) mod shape_task_definition;

pub(crate) mod shape_task_definition_list;

pub(crate) mod shape_task_definition_placement_constraint;

pub(crate) mod shape_task_override;

pub(crate) mod shape_task_set;

pub(crate) mod shape_task_sets;

pub(crate) mod shape_tasks;

pub(crate) mod shape_version_info;

pub(crate) mod shape_volume;

pub(crate) mod shape_attachments;

pub(crate) mod shape_aws_vpc_configuration;

pub(crate) mod shape_capacity_provider_strategy;

pub(crate) mod shape_cluster_service_connect_defaults;

pub(crate) mod shape_cluster_settings;

pub(crate) mod shape_compatibility_list;

pub(crate) mod shape_container_definitions;

pub(crate) mod shape_container_dependency;

pub(crate) mod shape_container_instance_health_status;

pub(crate) mod shape_container_override;

pub(crate) mod shape_containers;

pub(crate) mod shape_deployment_alarms;

pub(crate) mod shape_deployment_circuit_breaker;

pub(crate) mod shape_deployments;

pub(crate) mod shape_docker_volume_configuration;

pub(crate) mod shape_efs_volume_configuration;

pub(crate) mod shape_environment_file;

pub(crate) mod shape_execute_command_configuration;

pub(crate) mod shape_f_sx_windows_file_server_volume_configuration;

pub(crate) mod shape_failure;

pub(crate) mod shape_firelens_configuration;

pub(crate) mod shape_health_check;

pub(crate) mod shape_host_entry;

pub(crate) mod shape_host_volume_properties;

pub(crate) mod shape_inference_accelerator_override;

pub(crate) mod shape_inference_accelerators;

pub(crate) mod shape_key_value_pair;

pub(crate) mod shape_linux_parameters;

pub(crate) mod shape_load_balancers;

pub(crate) mod shape_log_configuration;

pub(crate) mod shape_managed_scaling;

pub(crate) mod shape_mount_point;

pub(crate) mod shape_placement_constraints;

pub(crate) mod shape_placement_strategies;

pub(crate) mod shape_port_mapping;

pub(crate) mod shape_protected_task;

pub(crate) mod shape_repository_credentials;

pub(crate) mod shape_requires_attributes;

pub(crate) mod shape_resource_requirement;

pub(crate) mod shape_resources;

pub(crate) mod shape_secret;

pub(crate) mod shape_service_connect_service;

pub(crate) mod shape_service_events;

pub(crate) mod shape_service_registries;

pub(crate) mod shape_statistics;

pub(crate) mod shape_system_control;

pub(crate) mod shape_task_definition_placement_constraints;

pub(crate) mod shape_ulimit;

pub(crate) mod shape_volume_from;

pub(crate) mod shape_volume_list;

pub(crate) mod shape_attachment;

pub(crate) mod shape_container;

pub(crate) mod shape_container_overrides;

pub(crate) mod shape_deployment;

pub(crate) mod shape_device;

pub(crate) mod shape_efs_authorization_config;

pub(crate) mod shape_execute_command_log_configuration;

pub(crate) mod shape_f_sx_windows_file_server_authorization_config;

pub(crate) mod shape_inference_accelerator_overrides;

pub(crate) mod shape_instance_health_check_result_list;

pub(crate) mod shape_kernel_capabilities;

pub(crate) mod shape_proxy_configuration_properties;

pub(crate) mod shape_service_connect_client_alias;

pub(crate) mod shape_service_event;

pub(crate) mod shape_tmpfs;

pub(crate) mod shape_attachment_details;

pub(crate) mod shape_container_dependencies;

pub(crate) mod shape_docker_labels_map;

pub(crate) mod shape_environment_files;

pub(crate) mod shape_environment_variables;

pub(crate) mod shape_gpu_ids;

pub(crate) mod shape_host_entry_list;

pub(crate) mod shape_instance_health_check_result;

pub(crate) mod shape_managed_agents;

pub(crate) mod shape_mount_point_list;

pub(crate) mod shape_network_bindings;

pub(crate) mod shape_network_interfaces;

pub(crate) mod shape_port_mapping_list;

pub(crate) mod shape_resource_requirements;

pub(crate) mod shape_secret_list;

pub(crate) mod shape_service_connect_service_resource_list;

pub(crate) mod shape_system_controls;

pub(crate) mod shape_ulimit_list;

pub(crate) mod shape_volume_from_list;

pub(crate) mod shape_devices_list;

pub(crate) mod shape_firelens_configuration_options_map;

pub(crate) mod shape_log_configuration_options_map;

pub(crate) mod shape_managed_agent;

pub(crate) mod shape_network_interface;

pub(crate) mod shape_service_connect_service_list;

pub(crate) mod shape_service_connect_service_resource;

pub(crate) mod shape_string_map;

pub(crate) mod shape_tmpfs_list;

pub(crate) mod shape_device_cgroup_permissions;

pub(crate) mod shape_service_connect_client_alias_list;