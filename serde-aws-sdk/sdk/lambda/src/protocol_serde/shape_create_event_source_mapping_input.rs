// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_event_source_mapping_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_event_source_mapping::CreateEventSourceMappingInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.amazon_managed_kafka_event_source_config {
        #[allow(unused_mut)]
        let mut object_2 = object
            .key("AmazonManagedKafkaEventSourceConfig")
            .start_object();
        crate::protocol_serde::shape_amazon_managed_kafka_event_source_config::ser_amazon_managed_kafka_event_source_config(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.batch_size {
        object.key("BatchSize").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_3).into()),
        );
    }
    if let Some(var_4) = &input.bisect_batch_on_function_error {
        object.key("BisectBatchOnFunctionError").boolean(*var_4);
    }
    if let Some(var_5) = &input.destination_config {
        #[allow(unused_mut)]
        let mut object_6 = object.key("DestinationConfig").start_object();
        crate::protocol_serde::shape_destination_config::ser_destination_config(
            &mut object_6,
            var_5,
        )?;
        object_6.finish();
    }
    if let Some(var_7) = &input.document_db_event_source_config {
        #[allow(unused_mut)]
        let mut object_8 = object.key("DocumentDBEventSourceConfig").start_object();
        crate::protocol_serde::shape_document_db_event_source_config::ser_document_db_event_source_config(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.enabled {
        object.key("Enabled").boolean(*var_9);
    }
    if let Some(var_10) = &input.event_source_arn {
        object.key("EventSourceArn").string(var_10.as_str());
    }
    if let Some(var_11) = &input.filter_criteria {
        #[allow(unused_mut)]
        let mut object_12 = object.key("FilterCriteria").start_object();
        crate::protocol_serde::shape_filter_criteria::ser_filter_criteria(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.function_name {
        object.key("FunctionName").string(var_13.as_str());
    }
    if let Some(var_14) = &input.function_response_types {
        let mut array_15 = object.key("FunctionResponseTypes").start_array();
        for item_16 in var_14 {
            {
                array_15.value().string(item_16.as_str());
            }
        }
        array_15.finish();
    }
    if let Some(var_17) = &input.maximum_batching_window_in_seconds {
        object.key("MaximumBatchingWindowInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_17).into()),
        );
    }
    if let Some(var_18) = &input.maximum_record_age_in_seconds {
        object.key("MaximumRecordAgeInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_18).into()),
        );
    }
    if let Some(var_19) = &input.maximum_retry_attempts {
        object.key("MaximumRetryAttempts").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_19).into()),
        );
    }
    if let Some(var_20) = &input.parallelization_factor {
        object.key("ParallelizationFactor").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_20).into()),
        );
    }
    if let Some(var_21) = &input.queues {
        let mut array_22 = object.key("Queues").start_array();
        for item_23 in var_21 {
            {
                array_22.value().string(item_23.as_str());
            }
        }
        array_22.finish();
    }
    if let Some(var_24) = &input.scaling_config {
        #[allow(unused_mut)]
        let mut object_25 = object.key("ScalingConfig").start_object();
        crate::protocol_serde::shape_scaling_config::ser_scaling_config(&mut object_25, var_24)?;
        object_25.finish();
    }
    if let Some(var_26) = &input.self_managed_event_source {
        #[allow(unused_mut)]
        let mut object_27 = object.key("SelfManagedEventSource").start_object();
        crate::protocol_serde::shape_self_managed_event_source::ser_self_managed_event_source(
            &mut object_27,
            var_26,
        )?;
        object_27.finish();
    }
    if let Some(var_28) = &input.self_managed_kafka_event_source_config {
        #[allow(unused_mut)]
        let mut object_29 = object
            .key("SelfManagedKafkaEventSourceConfig")
            .start_object();
        crate::protocol_serde::shape_self_managed_kafka_event_source_config::ser_self_managed_kafka_event_source_config(&mut object_29, var_28)?;
        object_29.finish();
    }
    if let Some(var_30) = &input.source_access_configurations {
        let mut array_31 = object.key("SourceAccessConfigurations").start_array();
        for item_32 in var_30 {
            {
                #[allow(unused_mut)]
                let mut object_33 = array_31.value().start_object();
                crate::protocol_serde::shape_source_access_configuration::ser_source_access_configuration(&mut object_33, item_32)?;
                object_33.finish();
            }
        }
        array_31.finish();
    }
    if let Some(var_34) = &input.starting_position {
        object.key("StartingPosition").string(var_34.as_str());
    }
    if let Some(var_35) = &input.starting_position_timestamp {
        object
            .key("StartingPositionTimestamp")
            .date_time(var_35, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_36) = &input.topics {
        let mut array_37 = object.key("Topics").start_array();
        for item_38 in var_36 {
            {
                array_37.value().string(item_38.as_str());
            }
        }
        array_37.finish();
    }
    if let Some(var_39) = &input.tumbling_window_in_seconds {
        object.key("TumblingWindowInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_39).into()),
        );
    }
    Ok(())
}
