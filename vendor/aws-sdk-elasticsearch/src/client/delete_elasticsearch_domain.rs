// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteElasticsearchDomain`](crate::operation::delete_elasticsearch_domain::builders::DeleteElasticsearchDomainFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain_name(impl Into<String>)`](crate::operation::delete_elasticsearch_domain::builders::DeleteElasticsearchDomainFluentBuilder::domain_name) / [`set_domain_name(Option<String>)`](crate::operation::delete_elasticsearch_domain::builders::DeleteElasticsearchDomainFluentBuilder::set_domain_name):<br>required: **true**<br><p>The name of the Elasticsearch domain that you want to permanently delete.</p><br>
    /// - On success, responds with [`DeleteElasticsearchDomainOutput`](crate::operation::delete_elasticsearch_domain::DeleteElasticsearchDomainOutput) with field(s):
    ///   - [`domain_status(Option<ElasticsearchDomainStatus>)`](crate::operation::delete_elasticsearch_domain::DeleteElasticsearchDomainOutput::domain_status): <p>The status of the Elasticsearch domain being deleted.</p>
    /// - On failure, responds with [`SdkError<DeleteElasticsearchDomainError>`](crate::operation::delete_elasticsearch_domain::DeleteElasticsearchDomainError)
    pub fn delete_elasticsearch_domain(&self) -> crate::operation::delete_elasticsearch_domain::builders::DeleteElasticsearchDomainFluentBuilder {
        crate::operation::delete_elasticsearch_domain::builders::DeleteElasticsearchDomainFluentBuilder::new(self.handle.clone())
    }
}
