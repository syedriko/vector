// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn describe_delivery_stream_output_output_correct_errors(
    mut builder: crate::operation::describe_delivery_stream::builders::DescribeDeliveryStreamOutputBuilder,
) -> crate::operation::describe_delivery_stream::builders::DescribeDeliveryStreamOutputBuilder {
    if builder.delivery_stream_description.is_none() {
        builder.delivery_stream_description = {
            let builder = crate::types::builders::DeliveryStreamDescriptionBuilder::default();
            crate::serde_util::delivery_stream_description_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn list_delivery_streams_output_output_correct_errors(
    mut builder: crate::operation::list_delivery_streams::builders::ListDeliveryStreamsOutputBuilder,
) -> crate::operation::list_delivery_streams::builders::ListDeliveryStreamsOutputBuilder {
    if builder.delivery_stream_names.is_none() {
        builder.delivery_stream_names = Some(Default::default())
    }
    if builder.has_more_delivery_streams.is_none() {
        builder.has_more_delivery_streams = Some(Default::default())
    }
    builder
}

pub(crate) fn list_tags_for_delivery_stream_output_output_correct_errors(
    mut builder: crate::operation::list_tags_for_delivery_stream::builders::ListTagsForDeliveryStreamOutputBuilder,
) -> crate::operation::list_tags_for_delivery_stream::builders::ListTagsForDeliveryStreamOutputBuilder {
    if builder.tags.is_none() {
        builder.tags = Some(Default::default())
    }
    if builder.has_more_tags.is_none() {
        builder.has_more_tags = Some(Default::default())
    }
    builder
}

pub(crate) fn put_record_output_output_correct_errors(
    mut builder: crate::operation::put_record::builders::PutRecordOutputBuilder,
) -> crate::operation::put_record::builders::PutRecordOutputBuilder {
    if builder.record_id.is_none() {
        builder.record_id = Some(Default::default())
    }
    builder
}

pub(crate) fn put_record_batch_output_output_correct_errors(
    mut builder: crate::operation::put_record_batch::builders::PutRecordBatchOutputBuilder,
) -> crate::operation::put_record_batch::builders::PutRecordBatchOutputBuilder {
    if builder.failed_put_count.is_none() {
        builder.failed_put_count = Some(Default::default())
    }
    if builder.request_responses.is_none() {
        builder.request_responses = Some(Default::default())
    }
    builder
}

pub(crate) fn delivery_stream_description_correct_errors(
    mut builder: crate::types::builders::DeliveryStreamDescriptionBuilder,
) -> crate::types::builders::DeliveryStreamDescriptionBuilder {
    if builder.delivery_stream_name.is_none() {
        builder.delivery_stream_name = Some(Default::default())
    }
    if builder.delivery_stream_arn.is_none() {
        builder.delivery_stream_arn = Some(Default::default())
    }
    if builder.delivery_stream_status.is_none() {
        builder.delivery_stream_status = "no value was set".parse::<crate::types::DeliveryStreamStatus>().ok()
    }
    if builder.delivery_stream_type.is_none() {
        builder.delivery_stream_type = "no value was set".parse::<crate::types::DeliveryStreamType>().ok()
    }
    if builder.version_id.is_none() {
        builder.version_id = Some(Default::default())
    }
    if builder.destinations.is_none() {
        builder.destinations = Some(Default::default())
    }
    if builder.has_more_destinations.is_none() {
        builder.has_more_destinations = Some(Default::default())
    }
    builder
}

pub(crate) fn failure_description_correct_errors(
    mut builder: crate::types::builders::FailureDescriptionBuilder,
) -> crate::types::builders::FailureDescriptionBuilder {
    if builder.r#type.is_none() {
        builder.r#type = "no value was set".parse::<crate::types::DeliveryStreamFailureType>().ok()
    }
    if builder.details.is_none() {
        builder.details = Some(Default::default())
    }
    builder
}

pub(crate) fn tag_correct_errors(mut builder: crate::types::builders::TagBuilder) -> crate::types::builders::TagBuilder {
    if builder.key.is_none() {
        builder.key = Some(Default::default())
    }
    builder
}

pub(crate) fn destination_description_correct_errors(
    mut builder: crate::types::builders::DestinationDescriptionBuilder,
) -> crate::types::builders::DestinationDescriptionBuilder {
    if builder.destination_id.is_none() {
        builder.destination_id = Some(Default::default())
    }
    builder
}

pub(crate) fn authentication_configuration_correct_errors(
    mut builder: crate::types::builders::AuthenticationConfigurationBuilder,
) -> crate::types::builders::AuthenticationConfigurationBuilder {
    if builder.role_arn.is_none() {
        builder.role_arn = Some(Default::default())
    }
    if builder.connectivity.is_none() {
        builder.connectivity = "no value was set".parse::<crate::types::Connectivity>().ok()
    }
    builder
}

pub(crate) fn extended_s3_destination_description_correct_errors(
    mut builder: crate::types::builders::ExtendedS3DestinationDescriptionBuilder,
) -> crate::types::builders::ExtendedS3DestinationDescriptionBuilder {
    if builder.role_arn.is_none() {
        builder.role_arn = Some(Default::default())
    }
    if builder.bucket_arn.is_none() {
        builder.bucket_arn = Some(Default::default())
    }
    if builder.buffering_hints.is_none() {
        builder.buffering_hints = {
            let builder = crate::types::builders::BufferingHintsBuilder::default();
            Some(builder.build())
        }
    }
    if builder.compression_format.is_none() {
        builder.compression_format = "no value was set".parse::<crate::types::CompressionFormat>().ok()
    }
    if builder.encryption_configuration.is_none() {
        builder.encryption_configuration = {
            let builder = crate::types::builders::EncryptionConfigurationBuilder::default();
            Some(builder.build())
        }
    }
    builder
}

pub(crate) fn redshift_destination_description_correct_errors(
    mut builder: crate::types::builders::RedshiftDestinationDescriptionBuilder,
) -> crate::types::builders::RedshiftDestinationDescriptionBuilder {
    if builder.role_arn.is_none() {
        builder.role_arn = Some(Default::default())
    }
    if builder.cluster_jdbcurl.is_none() {
        builder.cluster_jdbcurl = Some(Default::default())
    }
    if builder.copy_command.is_none() {
        builder.copy_command = {
            let builder = crate::types::builders::CopyCommandBuilder::default();
            crate::serde_util::copy_command_correct_errors(builder).build().ok()
        }
    }
    if builder.username.is_none() {
        builder.username = Some(Default::default())
    }
    if builder.s3_destination_description.is_none() {
        builder.s3_destination_description = {
            let builder = crate::types::builders::S3DestinationDescriptionBuilder::default();
            crate::serde_util::s3_destination_description_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn s3_destination_description_correct_errors(
    mut builder: crate::types::builders::S3DestinationDescriptionBuilder,
) -> crate::types::builders::S3DestinationDescriptionBuilder {
    if builder.role_arn.is_none() {
        builder.role_arn = Some(Default::default())
    }
    if builder.bucket_arn.is_none() {
        builder.bucket_arn = Some(Default::default())
    }
    if builder.buffering_hints.is_none() {
        builder.buffering_hints = {
            let builder = crate::types::builders::BufferingHintsBuilder::default();
            Some(builder.build())
        }
    }
    if builder.compression_format.is_none() {
        builder.compression_format = "no value was set".parse::<crate::types::CompressionFormat>().ok()
    }
    if builder.encryption_configuration.is_none() {
        builder.encryption_configuration = {
            let builder = crate::types::builders::EncryptionConfigurationBuilder::default();
            Some(builder.build())
        }
    }
    builder
}

pub(crate) fn copy_command_correct_errors(mut builder: crate::types::builders::CopyCommandBuilder) -> crate::types::builders::CopyCommandBuilder {
    if builder.data_table_name.is_none() {
        builder.data_table_name = Some(Default::default())
    }
    builder
}

pub(crate) fn document_id_options_correct_errors(
    mut builder: crate::types::builders::DocumentIdOptionsBuilder,
) -> crate::types::builders::DocumentIdOptionsBuilder {
    if builder.default_document_id_format.is_none() {
        builder.default_document_id_format = "no value was set".parse::<crate::types::DefaultDocumentIdFormat>().ok()
    }
    builder
}

pub(crate) fn vpc_configuration_description_correct_errors(
    mut builder: crate::types::builders::VpcConfigurationDescriptionBuilder,
) -> crate::types::builders::VpcConfigurationDescriptionBuilder {
    if builder.subnet_ids.is_none() {
        builder.subnet_ids = Some(Default::default())
    }
    if builder.role_arn.is_none() {
        builder.role_arn = Some(Default::default())
    }
    if builder.security_group_ids.is_none() {
        builder.security_group_ids = Some(Default::default())
    }
    if builder.vpc_id.is_none() {
        builder.vpc_id = Some(Default::default())
    }
    builder
}

pub(crate) fn kms_encryption_config_correct_errors(
    mut builder: crate::types::builders::KmsEncryptionConfigBuilder,
) -> crate::types::builders::KmsEncryptionConfigBuilder {
    if builder.awskms_key_arn.is_none() {
        builder.awskms_key_arn = Some(Default::default())
    }
    builder
}

pub(crate) fn http_endpoint_common_attribute_correct_errors(
    mut builder: crate::types::builders::HttpEndpointCommonAttributeBuilder,
) -> crate::types::builders::HttpEndpointCommonAttributeBuilder {
    if builder.attribute_name.is_none() {
        builder.attribute_name = Some(Default::default())
    }
    if builder.attribute_value.is_none() {
        builder.attribute_value = Some(Default::default())
    }
    builder
}

pub(crate) fn processor_correct_errors(mut builder: crate::types::builders::ProcessorBuilder) -> crate::types::builders::ProcessorBuilder {
    if builder.r#type.is_none() {
        builder.r#type = "no value was set".parse::<crate::types::ProcessorType>().ok()
    }
    builder
}

pub(crate) fn processor_parameter_correct_errors(
    mut builder: crate::types::builders::ProcessorParameterBuilder,
) -> crate::types::builders::ProcessorParameterBuilder {
    if builder.parameter_name.is_none() {
        builder.parameter_name = "no value was set".parse::<crate::types::ProcessorParameterName>().ok()
    }
    if builder.parameter_value.is_none() {
        builder.parameter_value = Some(Default::default())
    }
    builder
}
