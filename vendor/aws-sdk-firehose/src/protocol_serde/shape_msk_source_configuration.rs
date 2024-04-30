// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_msk_source_configuration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::MskSourceConfiguration,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("MSKClusterARN").string(input.msk_cluster_arn.as_str());
    }
    {
        object.key("TopicName").string(input.topic_name.as_str());
    }
    if let Some(var_1) = &input.authentication_configuration {
        #[allow(unused_mut)]
        let mut object_2 = object.key("AuthenticationConfiguration").start_object();
        crate::protocol_serde::shape_authentication_configuration::ser_authentication_configuration(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}
