// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_resource_policy_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::put_resource_policy::PutResourcePolicyInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.policy_name {
        object.key("policyName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.policy_document {
        object.key("policyDocument").string(var_2.as_str());
    }
    Ok(())
}
