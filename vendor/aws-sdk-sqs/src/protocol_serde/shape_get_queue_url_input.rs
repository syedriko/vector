// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_queue_url_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::get_queue_url::GetQueueUrlInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.queue_name {
        object.key("QueueName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.queue_owner_aws_account_id {
        object.key("QueueOwnerAWSAccountId").string(var_2.as_str());
    }
    Ok(())
}
