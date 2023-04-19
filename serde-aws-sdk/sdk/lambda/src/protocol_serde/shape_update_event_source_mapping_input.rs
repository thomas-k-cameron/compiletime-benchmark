// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_event_source_mapping_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_event_source_mapping::UpdateEventSourceMappingInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.batch_size {
        object.key("BatchSize").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_1).into()),
        );
    }
    if let Some(var_2) = &input.bisect_batch_on_function_error {
        object.key("BisectBatchOnFunctionError").boolean(*var_2);
    }
    if let Some(var_3) = &input.destination_config {
        #[allow(unused_mut)]
        let mut object_4 = object.key("DestinationConfig").start_object();
        crate::protocol_serde::shape_destination_config::ser_destination_config(
            &mut object_4,
            var_3,
        )?;
        object_4.finish();
    }
    if let Some(var_5) = &input.document_db_event_source_config {
        #[allow(unused_mut)]
        let mut object_6 = object.key("DocumentDBEventSourceConfig").start_object();
        crate::protocol_serde::shape_document_db_event_source_config::ser_document_db_event_source_config(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.enabled {
        object.key("Enabled").boolean(*var_7);
    }
    if let Some(var_8) = &input.filter_criteria {
        #[allow(unused_mut)]
        let mut object_9 = object.key("FilterCriteria").start_object();
        crate::protocol_serde::shape_filter_criteria::ser_filter_criteria(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.function_name {
        object.key("FunctionName").string(var_10.as_str());
    }
    if let Some(var_11) = &input.function_response_types {
        let mut array_12 = object.key("FunctionResponseTypes").start_array();
        for item_13 in var_11 {
            {
                array_12.value().string(item_13.as_str());
            }
        }
        array_12.finish();
    }
    if let Some(var_14) = &input.maximum_batching_window_in_seconds {
        object.key("MaximumBatchingWindowInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_14).into()),
        );
    }
    if let Some(var_15) = &input.maximum_record_age_in_seconds {
        object.key("MaximumRecordAgeInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_15).into()),
        );
    }
    if let Some(var_16) = &input.maximum_retry_attempts {
        object.key("MaximumRetryAttempts").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_16).into()),
        );
    }
    if let Some(var_17) = &input.parallelization_factor {
        object.key("ParallelizationFactor").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_17).into()),
        );
    }
    if let Some(var_18) = &input.scaling_config {
        #[allow(unused_mut)]
        let mut object_19 = object.key("ScalingConfig").start_object();
        crate::protocol_serde::shape_scaling_config::ser_scaling_config(&mut object_19, var_18)?;
        object_19.finish();
    }
    if let Some(var_20) = &input.source_access_configurations {
        let mut array_21 = object.key("SourceAccessConfigurations").start_array();
        for item_22 in var_20 {
            {
                #[allow(unused_mut)]
                let mut object_23 = array_21.value().start_object();
                crate::protocol_serde::shape_source_access_configuration::ser_source_access_configuration(&mut object_23, item_22)?;
                object_23.finish();
            }
        }
        array_21.finish();
    }
    if let Some(var_24) = &input.tumbling_window_in_seconds {
        object.key("TumblingWindowInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_24).into()),
        );
    }
    Ok(())
}
