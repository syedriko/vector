// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteOutboundCrossClusterSearchConnection`](crate::operation::delete_outbound_cross_cluster_search_connection::builders::DeleteOutboundCrossClusterSearchConnectionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cross_cluster_search_connection_id(impl Into<String>)`](crate::operation::delete_outbound_cross_cluster_search_connection::builders::DeleteOutboundCrossClusterSearchConnectionFluentBuilder::cross_cluster_search_connection_id) / [`set_cross_cluster_search_connection_id(Option<String>)`](crate::operation::delete_outbound_cross_cluster_search_connection::builders::DeleteOutboundCrossClusterSearchConnectionFluentBuilder::set_cross_cluster_search_connection_id):<br>required: **true**<br><p>The id of the outbound connection that you want to permanently delete.</p><br>
    /// - On success, responds with [`DeleteOutboundCrossClusterSearchConnectionOutput`](crate::operation::delete_outbound_cross_cluster_search_connection::DeleteOutboundCrossClusterSearchConnectionOutput) with field(s):
    ///   - [`cross_cluster_search_connection(Option<OutboundCrossClusterSearchConnection>)`](crate::operation::delete_outbound_cross_cluster_search_connection::DeleteOutboundCrossClusterSearchConnectionOutput::cross_cluster_search_connection): <p>Specifies the <code><code>OutboundCrossClusterSearchConnection</code></code> of deleted outbound connection. </p>
    /// - On failure, responds with [`SdkError<DeleteOutboundCrossClusterSearchConnectionError>`](crate::operation::delete_outbound_cross_cluster_search_connection::DeleteOutboundCrossClusterSearchConnectionError)
    pub fn delete_outbound_cross_cluster_search_connection(
        &self,
    ) -> crate::operation::delete_outbound_cross_cluster_search_connection::builders::DeleteOutboundCrossClusterSearchConnectionFluentBuilder {
        crate::operation::delete_outbound_cross_cluster_search_connection::builders::DeleteOutboundCrossClusterSearchConnectionFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
