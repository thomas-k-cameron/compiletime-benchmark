// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the output of a <code>BatchGetItem</code> operation.</p>
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct BatchGetItemOutput {
    /// <p>A map of table name to a list of items. Each object in <code>Responses</code> consists of a table name, along with a map of attribute data consisting of the data type and attribute value.</p>
    #[doc(hidden)]
    pub responses: std::option::Option<
        std::collections::HashMap<
            std::string::String,
            std::vec::Vec<
                std::collections::HashMap<std::string::String, crate::types::AttributeValue>,
            >,
        >,
    >,
    /// <p>A map of tables and their respective keys that were not processed with the current response. The <code>UnprocessedKeys</code> value is in the same form as <code>RequestItems</code>, so the value can be provided directly to a subsequent <code>BatchGetItem</code> operation. For more information, see <code>RequestItems</code> in the Request Parameters section.</p>
    /// <p>Each element consists of:</p>
    /// <ul>
    /// <li> <p> <code>Keys</code> - An array of primary key attribute values that define specific items in the table.</p> </li>
    /// <li> <p> <code>ProjectionExpression</code> - One or more attributes to be retrieved from the table or index. By default, all attributes are returned. If a requested attribute is not found, it does not appear in the result.</p> </li>
    /// <li> <p> <code>ConsistentRead</code> - The consistency of a read operation. If set to <code>true</code>, then a strongly consistent read is used; otherwise, an eventually consistent read is used.</p> </li>
    /// </ul>
    /// <p>If there are no unprocessed keys remaining, the response contains an empty <code>UnprocessedKeys</code> map.</p>
    #[doc(hidden)]
    pub unprocessed_keys: std::option::Option<
        std::collections::HashMap<std::string::String, crate::types::KeysAndAttributes>,
    >,
    /// <p>The read capacity units consumed by the entire <code>BatchGetItem</code> operation.</p>
    /// <p>Each element consists of:</p>
    /// <ul>
    /// <li> <p> <code>TableName</code> - The table that consumed the provisioned throughput.</p> </li>
    /// <li> <p> <code>CapacityUnits</code> - The total number of capacity units consumed.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub consumed_capacity: std::option::Option<std::vec::Vec<crate::types::ConsumedCapacity>>,
    _request_id: Option<String>,
}
impl BatchGetItemOutput {
    /// <p>A map of table name to a list of items. Each object in <code>Responses</code> consists of a table name, along with a map of attribute data consisting of the data type and attribute value.</p>
    pub fn responses(
        &self,
    ) -> std::option::Option<
        &std::collections::HashMap<
            std::string::String,
            std::vec::Vec<
                std::collections::HashMap<std::string::String, crate::types::AttributeValue>,
            >,
        >,
    > {
        self.responses.as_ref()
    }
    /// <p>A map of tables and their respective keys that were not processed with the current response. The <code>UnprocessedKeys</code> value is in the same form as <code>RequestItems</code>, so the value can be provided directly to a subsequent <code>BatchGetItem</code> operation. For more information, see <code>RequestItems</code> in the Request Parameters section.</p>
    /// <p>Each element consists of:</p>
    /// <ul>
    /// <li> <p> <code>Keys</code> - An array of primary key attribute values that define specific items in the table.</p> </li>
    /// <li> <p> <code>ProjectionExpression</code> - One or more attributes to be retrieved from the table or index. By default, all attributes are returned. If a requested attribute is not found, it does not appear in the result.</p> </li>
    /// <li> <p> <code>ConsistentRead</code> - The consistency of a read operation. If set to <code>true</code>, then a strongly consistent read is used; otherwise, an eventually consistent read is used.</p> </li>
    /// </ul>
    /// <p>If there are no unprocessed keys remaining, the response contains an empty <code>UnprocessedKeys</code> map.</p>
    pub fn unprocessed_keys(
        &self,
    ) -> std::option::Option<
        &std::collections::HashMap<std::string::String, crate::types::KeysAndAttributes>,
    > {
        self.unprocessed_keys.as_ref()
    }
    /// <p>The read capacity units consumed by the entire <code>BatchGetItem</code> operation.</p>
    /// <p>Each element consists of:</p>
    /// <ul>
    /// <li> <p> <code>TableName</code> - The table that consumed the provisioned throughput.</p> </li>
    /// <li> <p> <code>CapacityUnits</code> - The total number of capacity units consumed.</p> </li>
    /// </ul>
    pub fn consumed_capacity(&self) -> std::option::Option<&[crate::types::ConsumedCapacity]> {
        self.consumed_capacity.as_deref()
    }
}
impl aws_http::request_id::RequestId for BatchGetItemOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl BatchGetItemOutput {
    /// Creates a new builder-style object to manufacture [`BatchGetItemOutput`](crate::operation::batch_get_item::BatchGetItemOutput).
    pub fn builder() -> crate::operation::batch_get_item::builders::BatchGetItemOutputBuilder {
        crate::operation::batch_get_item::builders::BatchGetItemOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::batch_get_item::BatchGetItemOutput;
/// A builder for [`BatchGetItemOutput`](crate::operation::batch_get_item::BatchGetItemOutput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct BatchGetItemOutputBuilder {
    pub(crate) responses: std::option::Option<
        std::collections::HashMap<
            std::string::String,
            std::vec::Vec<
                std::collections::HashMap<std::string::String, crate::types::AttributeValue>,
            >,
        >,
    >,
    pub(crate) unprocessed_keys: std::option::Option<
        std::collections::HashMap<std::string::String, crate::types::KeysAndAttributes>,
    >,
    pub(crate) consumed_capacity:
        std::option::Option<std::vec::Vec<crate::types::ConsumedCapacity>>,
    _request_id: Option<String>,
}
impl BatchGetItemOutputBuilder {
    /// Adds a key-value pair to `responses`.
    ///
    /// To override the contents of this collection use [`set_responses`](Self::set_responses).
    ///
    /// <p>A map of table name to a list of items. Each object in <code>Responses</code> consists of a table name, along with a map of attribute data consisting of the data type and attribute value.</p>
    pub fn responses(
        mut self,
        k: impl Into<std::string::String>,
        v: std::vec::Vec<
            std::collections::HashMap<std::string::String, crate::types::AttributeValue>,
        >,
    ) -> Self {
        let mut hash_map = self.responses.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.responses = Some(hash_map);
        self
    }
    /// <p>A map of table name to a list of items. Each object in <code>Responses</code> consists of a table name, along with a map of attribute data consisting of the data type and attribute value.</p>
    pub fn set_responses(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<
                std::string::String,
                std::vec::Vec<
                    std::collections::HashMap<std::string::String, crate::types::AttributeValue>,
                >,
            >,
        >,
    ) -> Self {
        self.responses = input;
        self
    }
    /// Adds a key-value pair to `unprocessed_keys`.
    ///
    /// To override the contents of this collection use [`set_unprocessed_keys`](Self::set_unprocessed_keys).
    ///
    /// <p>A map of tables and their respective keys that were not processed with the current response. The <code>UnprocessedKeys</code> value is in the same form as <code>RequestItems</code>, so the value can be provided directly to a subsequent <code>BatchGetItem</code> operation. For more information, see <code>RequestItems</code> in the Request Parameters section.</p>
    /// <p>Each element consists of:</p>
    /// <ul>
    /// <li> <p> <code>Keys</code> - An array of primary key attribute values that define specific items in the table.</p> </li>
    /// <li> <p> <code>ProjectionExpression</code> - One or more attributes to be retrieved from the table or index. By default, all attributes are returned. If a requested attribute is not found, it does not appear in the result.</p> </li>
    /// <li> <p> <code>ConsistentRead</code> - The consistency of a read operation. If set to <code>true</code>, then a strongly consistent read is used; otherwise, an eventually consistent read is used.</p> </li>
    /// </ul>
    /// <p>If there are no unprocessed keys remaining, the response contains an empty <code>UnprocessedKeys</code> map.</p>
    pub fn unprocessed_keys(
        mut self,
        k: impl Into<std::string::String>,
        v: crate::types::KeysAndAttributes,
    ) -> Self {
        let mut hash_map = self.unprocessed_keys.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.unprocessed_keys = Some(hash_map);
        self
    }
    /// <p>A map of tables and their respective keys that were not processed with the current response. The <code>UnprocessedKeys</code> value is in the same form as <code>RequestItems</code>, so the value can be provided directly to a subsequent <code>BatchGetItem</code> operation. For more information, see <code>RequestItems</code> in the Request Parameters section.</p>
    /// <p>Each element consists of:</p>
    /// <ul>
    /// <li> <p> <code>Keys</code> - An array of primary key attribute values that define specific items in the table.</p> </li>
    /// <li> <p> <code>ProjectionExpression</code> - One or more attributes to be retrieved from the table or index. By default, all attributes are returned. If a requested attribute is not found, it does not appear in the result.</p> </li>
    /// <li> <p> <code>ConsistentRead</code> - The consistency of a read operation. If set to <code>true</code>, then a strongly consistent read is used; otherwise, an eventually consistent read is used.</p> </li>
    /// </ul>
    /// <p>If there are no unprocessed keys remaining, the response contains an empty <code>UnprocessedKeys</code> map.</p>
    pub fn set_unprocessed_keys(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, crate::types::KeysAndAttributes>,
        >,
    ) -> Self {
        self.unprocessed_keys = input;
        self
    }
    /// Appends an item to `consumed_capacity`.
    ///
    /// To override the contents of this collection use [`set_consumed_capacity`](Self::set_consumed_capacity).
    ///
    /// <p>The read capacity units consumed by the entire <code>BatchGetItem</code> operation.</p>
    /// <p>Each element consists of:</p>
    /// <ul>
    /// <li> <p> <code>TableName</code> - The table that consumed the provisioned throughput.</p> </li>
    /// <li> <p> <code>CapacityUnits</code> - The total number of capacity units consumed.</p> </li>
    /// </ul>
    pub fn consumed_capacity(mut self, input: crate::types::ConsumedCapacity) -> Self {
        let mut v = self.consumed_capacity.unwrap_or_default();
        v.push(input);
        self.consumed_capacity = Some(v);
        self
    }
    /// <p>The read capacity units consumed by the entire <code>BatchGetItem</code> operation.</p>
    /// <p>Each element consists of:</p>
    /// <ul>
    /// <li> <p> <code>TableName</code> - The table that consumed the provisioned throughput.</p> </li>
    /// <li> <p> <code>CapacityUnits</code> - The total number of capacity units consumed.</p> </li>
    /// </ul>
    pub fn set_consumed_capacity(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ConsumedCapacity>>,
    ) -> Self {
        self.consumed_capacity = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`BatchGetItemOutput`](crate::operation::batch_get_item::BatchGetItemOutput).
    pub fn build(self) -> crate::operation::batch_get_item::BatchGetItemOutput {
        crate::operation::batch_get_item::BatchGetItemOutput {
            responses: self.responses,
            unprocessed_keys: self.unprocessed_keys,
            consumed_capacity: self.consumed_capacity,
            _request_id: self._request_id,
        }
    }
}
