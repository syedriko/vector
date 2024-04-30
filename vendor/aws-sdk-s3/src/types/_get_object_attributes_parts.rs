// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A collection of parts associated with a multipart upload.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetObjectAttributesParts {
    /// <p>The total number of parts.</p>
    pub total_parts_count: ::std::option::Option<i32>,
    /// <p>The marker for the current part.</p>
    pub part_number_marker: ::std::option::Option<::std::string::String>,
    /// <p>When a list is truncated, this element specifies the last part in the list, as well as the value to use for the <code>PartNumberMarker</code> request parameter in a subsequent request.</p>
    pub next_part_number_marker: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of parts allowed in the response.</p>
    pub max_parts: ::std::option::Option<i32>,
    /// <p>Indicates whether the returned list of parts is truncated. A value of <code>true</code> indicates that the list was truncated. A list can be truncated if the number of parts exceeds the limit returned in the <code>MaxParts</code> element.</p>
    pub is_truncated: ::std::option::Option<bool>,
    /// <p>A container for elements related to a particular part. A response can contain zero or more <code>Parts</code> elements.</p> <note>
    /// <ul>
    /// <li> <p> <b>General purpose buckets</b> - For <code>GetObjectAttributes</code>, if a additional checksum (including <code>x-amz-checksum-crc32</code>, <code>x-amz-checksum-crc32c</code>, <code>x-amz-checksum-sha1</code>, or <code>x-amz-checksum-sha256</code>) isn't applied to the object specified in the request, the response doesn't return <code>Part</code>.</p> </li>
    /// <li> <p> <b>Directory buckets</b> - For <code>GetObjectAttributes</code>, no matter whether a additional checksum is applied to the object specified in the request, the response returns <code>Part</code>.</p> </li>
    /// </ul>
    /// </note>
    pub parts: ::std::option::Option<::std::vec::Vec<crate::types::ObjectPart>>,
}
impl GetObjectAttributesParts {
    /// <p>The total number of parts.</p>
    pub fn total_parts_count(&self) -> ::std::option::Option<i32> {
        self.total_parts_count
    }
    /// <p>The marker for the current part.</p>
    pub fn part_number_marker(&self) -> ::std::option::Option<&str> {
        self.part_number_marker.as_deref()
    }
    /// <p>When a list is truncated, this element specifies the last part in the list, as well as the value to use for the <code>PartNumberMarker</code> request parameter in a subsequent request.</p>
    pub fn next_part_number_marker(&self) -> ::std::option::Option<&str> {
        self.next_part_number_marker.as_deref()
    }
    /// <p>The maximum number of parts allowed in the response.</p>
    pub fn max_parts(&self) -> ::std::option::Option<i32> {
        self.max_parts
    }
    /// <p>Indicates whether the returned list of parts is truncated. A value of <code>true</code> indicates that the list was truncated. A list can be truncated if the number of parts exceeds the limit returned in the <code>MaxParts</code> element.</p>
    pub fn is_truncated(&self) -> ::std::option::Option<bool> {
        self.is_truncated
    }
    /// <p>A container for elements related to a particular part. A response can contain zero or more <code>Parts</code> elements.</p> <note>
    /// <ul>
    /// <li> <p> <b>General purpose buckets</b> - For <code>GetObjectAttributes</code>, if a additional checksum (including <code>x-amz-checksum-crc32</code>, <code>x-amz-checksum-crc32c</code>, <code>x-amz-checksum-sha1</code>, or <code>x-amz-checksum-sha256</code>) isn't applied to the object specified in the request, the response doesn't return <code>Part</code>.</p> </li>
    /// <li> <p> <b>Directory buckets</b> - For <code>GetObjectAttributes</code>, no matter whether a additional checksum is applied to the object specified in the request, the response returns <code>Part</code>.</p> </li>
    /// </ul>
    /// </note>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.parts.is_none()`.
    pub fn parts(&self) -> &[crate::types::ObjectPart] {
        self.parts.as_deref().unwrap_or_default()
    }
}
impl GetObjectAttributesParts {
    /// Creates a new builder-style object to manufacture [`GetObjectAttributesParts`](crate::types::GetObjectAttributesParts).
    pub fn builder() -> crate::types::builders::GetObjectAttributesPartsBuilder {
        crate::types::builders::GetObjectAttributesPartsBuilder::default()
    }
}

/// A builder for [`GetObjectAttributesParts`](crate::types::GetObjectAttributesParts).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetObjectAttributesPartsBuilder {
    pub(crate) total_parts_count: ::std::option::Option<i32>,
    pub(crate) part_number_marker: ::std::option::Option<::std::string::String>,
    pub(crate) next_part_number_marker: ::std::option::Option<::std::string::String>,
    pub(crate) max_parts: ::std::option::Option<i32>,
    pub(crate) is_truncated: ::std::option::Option<bool>,
    pub(crate) parts: ::std::option::Option<::std::vec::Vec<crate::types::ObjectPart>>,
}
impl GetObjectAttributesPartsBuilder {
    /// <p>The total number of parts.</p>
    pub fn total_parts_count(mut self, input: i32) -> Self {
        self.total_parts_count = ::std::option::Option::Some(input);
        self
    }
    /// <p>The total number of parts.</p>
    pub fn set_total_parts_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.total_parts_count = input;
        self
    }
    /// <p>The total number of parts.</p>
    pub fn get_total_parts_count(&self) -> &::std::option::Option<i32> {
        &self.total_parts_count
    }
    /// <p>The marker for the current part.</p>
    pub fn part_number_marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.part_number_marker = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The marker for the current part.</p>
    pub fn set_part_number_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.part_number_marker = input;
        self
    }
    /// <p>The marker for the current part.</p>
    pub fn get_part_number_marker(&self) -> &::std::option::Option<::std::string::String> {
        &self.part_number_marker
    }
    /// <p>When a list is truncated, this element specifies the last part in the list, as well as the value to use for the <code>PartNumberMarker</code> request parameter in a subsequent request.</p>
    pub fn next_part_number_marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_part_number_marker = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>When a list is truncated, this element specifies the last part in the list, as well as the value to use for the <code>PartNumberMarker</code> request parameter in a subsequent request.</p>
    pub fn set_next_part_number_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_part_number_marker = input;
        self
    }
    /// <p>When a list is truncated, this element specifies the last part in the list, as well as the value to use for the <code>PartNumberMarker</code> request parameter in a subsequent request.</p>
    pub fn get_next_part_number_marker(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_part_number_marker
    }
    /// <p>The maximum number of parts allowed in the response.</p>
    pub fn max_parts(mut self, input: i32) -> Self {
        self.max_parts = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of parts allowed in the response.</p>
    pub fn set_max_parts(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_parts = input;
        self
    }
    /// <p>The maximum number of parts allowed in the response.</p>
    pub fn get_max_parts(&self) -> &::std::option::Option<i32> {
        &self.max_parts
    }
    /// <p>Indicates whether the returned list of parts is truncated. A value of <code>true</code> indicates that the list was truncated. A list can be truncated if the number of parts exceeds the limit returned in the <code>MaxParts</code> element.</p>
    pub fn is_truncated(mut self, input: bool) -> Self {
        self.is_truncated = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether the returned list of parts is truncated. A value of <code>true</code> indicates that the list was truncated. A list can be truncated if the number of parts exceeds the limit returned in the <code>MaxParts</code> element.</p>
    pub fn set_is_truncated(mut self, input: ::std::option::Option<bool>) -> Self {
        self.is_truncated = input;
        self
    }
    /// <p>Indicates whether the returned list of parts is truncated. A value of <code>true</code> indicates that the list was truncated. A list can be truncated if the number of parts exceeds the limit returned in the <code>MaxParts</code> element.</p>
    pub fn get_is_truncated(&self) -> &::std::option::Option<bool> {
        &self.is_truncated
    }
    /// Appends an item to `parts`.
    ///
    /// To override the contents of this collection use [`set_parts`](Self::set_parts).
    ///
    /// <p>A container for elements related to a particular part. A response can contain zero or more <code>Parts</code> elements.</p> <note>
    /// <ul>
    /// <li> <p> <b>General purpose buckets</b> - For <code>GetObjectAttributes</code>, if a additional checksum (including <code>x-amz-checksum-crc32</code>, <code>x-amz-checksum-crc32c</code>, <code>x-amz-checksum-sha1</code>, or <code>x-amz-checksum-sha256</code>) isn't applied to the object specified in the request, the response doesn't return <code>Part</code>.</p> </li>
    /// <li> <p> <b>Directory buckets</b> - For <code>GetObjectAttributes</code>, no matter whether a additional checksum is applied to the object specified in the request, the response returns <code>Part</code>.</p> </li>
    /// </ul>
    /// </note>
    pub fn parts(mut self, input: crate::types::ObjectPart) -> Self {
        let mut v = self.parts.unwrap_or_default();
        v.push(input);
        self.parts = ::std::option::Option::Some(v);
        self
    }
    /// <p>A container for elements related to a particular part. A response can contain zero or more <code>Parts</code> elements.</p> <note>
    /// <ul>
    /// <li> <p> <b>General purpose buckets</b> - For <code>GetObjectAttributes</code>, if a additional checksum (including <code>x-amz-checksum-crc32</code>, <code>x-amz-checksum-crc32c</code>, <code>x-amz-checksum-sha1</code>, or <code>x-amz-checksum-sha256</code>) isn't applied to the object specified in the request, the response doesn't return <code>Part</code>.</p> </li>
    /// <li> <p> <b>Directory buckets</b> - For <code>GetObjectAttributes</code>, no matter whether a additional checksum is applied to the object specified in the request, the response returns <code>Part</code>.</p> </li>
    /// </ul>
    /// </note>
    pub fn set_parts(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ObjectPart>>) -> Self {
        self.parts = input;
        self
    }
    /// <p>A container for elements related to a particular part. A response can contain zero or more <code>Parts</code> elements.</p> <note>
    /// <ul>
    /// <li> <p> <b>General purpose buckets</b> - For <code>GetObjectAttributes</code>, if a additional checksum (including <code>x-amz-checksum-crc32</code>, <code>x-amz-checksum-crc32c</code>, <code>x-amz-checksum-sha1</code>, or <code>x-amz-checksum-sha256</code>) isn't applied to the object specified in the request, the response doesn't return <code>Part</code>.</p> </li>
    /// <li> <p> <b>Directory buckets</b> - For <code>GetObjectAttributes</code>, no matter whether a additional checksum is applied to the object specified in the request, the response returns <code>Part</code>.</p> </li>
    /// </ul>
    /// </note>
    pub fn get_parts(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ObjectPart>> {
        &self.parts
    }
    /// Consumes the builder and constructs a [`GetObjectAttributesParts`](crate::types::GetObjectAttributesParts).
    pub fn build(self) -> crate::types::GetObjectAttributesParts {
        crate::types::GetObjectAttributesParts {
            total_parts_count: self.total_parts_count,
            part_number_marker: self.part_number_marker,
            next_part_number_marker: self.next_part_number_marker,
            max_parts: self.max_parts,
            is_truncated: self.is_truncated,
            parts: self.parts,
        }
    }
}
