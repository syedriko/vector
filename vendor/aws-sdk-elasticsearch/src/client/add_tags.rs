// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AddTags`](crate::operation::add_tags::builders::AddTagsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::operation::add_tags::builders::AddTagsFluentBuilder::arn) / [`set_arn(Option<String>)`](crate::operation::add_tags::builders::AddTagsFluentBuilder::set_arn):<br>required: **true**<br><p> Specify the <code>ARN</code> for which you want to add the tags.</p><br>
    ///   - [`tag_list(Tag)`](crate::operation::add_tags::builders::AddTagsFluentBuilder::tag_list) / [`set_tag_list(Option<Vec::<Tag>>)`](crate::operation::add_tags::builders::AddTagsFluentBuilder::set_tag_list):<br>required: **true**<br><p> List of <code>Tag</code> that need to be added for the Elasticsearch domain. </p><br>
    /// - On success, responds with [`AddTagsOutput`](crate::operation::add_tags::AddTagsOutput)
    /// - On failure, responds with [`SdkError<AddTagsError>`](crate::operation::add_tags::AddTagsError)
    pub fn add_tags(&self) -> crate::operation::add_tags::builders::AddTagsFluentBuilder {
        crate::operation::add_tags::builders::AddTagsFluentBuilder::new(self.handle.clone())
    }
}
