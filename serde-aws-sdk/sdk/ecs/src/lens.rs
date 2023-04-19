// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_list_account_settings_output_next_token(
    input: &crate::operation::list_account_settings::ListAccountSettingsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_attributes_output_next_token(
    input: &crate::operation::list_attributes::ListAttributesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_clusters_output_next_token(
    input: &crate::operation::list_clusters::ListClustersOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_container_instances_output_next_token(
    input: &crate::operation::list_container_instances::ListContainerInstancesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_services_output_next_token(
    input: &crate::operation::list_services::ListServicesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_services_by_namespace_output_next_token(
    input: &crate::operation::list_services_by_namespace::ListServicesByNamespaceOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_task_definition_families_output_next_token(
    input: &crate::operation::list_task_definition_families::ListTaskDefinitionFamiliesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_task_definitions_output_next_token(
    input: &crate::operation::list_task_definitions::ListTaskDefinitionsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_tasks_output_next_token(
    input: &crate::operation::list_tasks::ListTasksOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_account_settings_output_settings(
    input: crate::operation::list_account_settings::ListAccountSettingsOutput,
) -> std::option::Option<std::vec::Vec<crate::types::Setting>> {
    let input = match input.settings {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_attributes_output_attributes(
    input: crate::operation::list_attributes::ListAttributesOutput,
) -> std::option::Option<std::vec::Vec<crate::types::Attribute>> {
    let input = match input.attributes {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_clusters_output_cluster_arns(
    input: crate::operation::list_clusters::ListClustersOutput,
) -> std::option::Option<std::vec::Vec<std::string::String>> {
    let input = match input.cluster_arns {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_container_instances_output_container_instance_arns(
    input: crate::operation::list_container_instances::ListContainerInstancesOutput,
) -> std::option::Option<std::vec::Vec<std::string::String>> {
    let input = match input.container_instance_arns {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_services_output_service_arns(
    input: crate::operation::list_services::ListServicesOutput,
) -> std::option::Option<std::vec::Vec<std::string::String>> {
    let input = match input.service_arns {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_services_by_namespace_output_service_arns(
    input: crate::operation::list_services_by_namespace::ListServicesByNamespaceOutput,
) -> std::option::Option<std::vec::Vec<std::string::String>> {
    let input = match input.service_arns {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_task_definition_families_output_families(
    input: crate::operation::list_task_definition_families::ListTaskDefinitionFamiliesOutput,
) -> std::option::Option<std::vec::Vec<std::string::String>> {
    let input = match input.families {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_task_definitions_output_task_definition_arns(
    input: crate::operation::list_task_definitions::ListTaskDefinitionsOutput,
) -> std::option::Option<std::vec::Vec<std::string::String>> {
    let input = match input.task_definition_arns {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_tasks_output_task_arns(
    input: crate::operation::list_tasks::ListTasksOutput,
) -> std::option::Option<std::vec::Vec<std::string::String>> {
    let input = match input.task_arns {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}
