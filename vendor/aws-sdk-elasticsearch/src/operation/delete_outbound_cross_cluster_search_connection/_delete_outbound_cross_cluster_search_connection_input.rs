// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Container for the parameters to the <code><code>DeleteOutboundCrossClusterSearchConnection</code></code> operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteOutboundCrossClusterSearchConnectionInput {
    /// <p>The id of the outbound connection that you want to permanently delete.</p>
    pub cross_cluster_search_connection_id: ::std::option::Option<::std::string::String>,
}
impl DeleteOutboundCrossClusterSearchConnectionInput {
    /// <p>The id of the outbound connection that you want to permanently delete.</p>
    pub fn cross_cluster_search_connection_id(&self) -> ::std::option::Option<&str> {
        self.cross_cluster_search_connection_id.as_deref()
    }
}
impl DeleteOutboundCrossClusterSearchConnectionInput {
    /// Creates a new builder-style object to manufacture [`DeleteOutboundCrossClusterSearchConnectionInput`](crate::operation::delete_outbound_cross_cluster_search_connection::DeleteOutboundCrossClusterSearchConnectionInput).
    pub fn builder(
    ) -> crate::operation::delete_outbound_cross_cluster_search_connection::builders::DeleteOutboundCrossClusterSearchConnectionInputBuilder {
        crate::operation::delete_outbound_cross_cluster_search_connection::builders::DeleteOutboundCrossClusterSearchConnectionInputBuilder::default()
    }
}

/// A builder for [`DeleteOutboundCrossClusterSearchConnectionInput`](crate::operation::delete_outbound_cross_cluster_search_connection::DeleteOutboundCrossClusterSearchConnectionInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteOutboundCrossClusterSearchConnectionInputBuilder {
    pub(crate) cross_cluster_search_connection_id: ::std::option::Option<::std::string::String>,
}
impl DeleteOutboundCrossClusterSearchConnectionInputBuilder {
    /// <p>The id of the outbound connection that you want to permanently delete.</p>
    /// This field is required.
    pub fn cross_cluster_search_connection_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.cross_cluster_search_connection_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The id of the outbound connection that you want to permanently delete.</p>
    pub fn set_cross_cluster_search_connection_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.cross_cluster_search_connection_id = input;
        self
    }
    /// <p>The id of the outbound connection that you want to permanently delete.</p>
    pub fn get_cross_cluster_search_connection_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.cross_cluster_search_connection_id
    }
    /// Consumes the builder and constructs a [`DeleteOutboundCrossClusterSearchConnectionInput`](crate::operation::delete_outbound_cross_cluster_search_connection::DeleteOutboundCrossClusterSearchConnectionInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_outbound_cross_cluster_search_connection::DeleteOutboundCrossClusterSearchConnectionInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::delete_outbound_cross_cluster_search_connection::DeleteOutboundCrossClusterSearchConnectionInput {
                cross_cluster_search_connection_id: self.cross_cluster_search_connection_id,
            },
        )
    }
}
