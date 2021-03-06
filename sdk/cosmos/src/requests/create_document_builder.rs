use crate::prelude::*;
use crate::responses::CreateDocumentResponse;
use crate::ResourceType;
use azure_core::errors::{extract_status_headers_and_body, AzureError, UnexpectedHTTPResult};
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use serde::Serialize;
use std::convert::TryFrom;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct CreateDocumentBuilder<'a, 'b, C, D, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    collection_client: &'a dyn CollectionClient<C, D>,
    p_partition_keys: PhantomData<PartitionKeysSet>,
    partition_keys: Option<&'b PartitionKeys>,
    is_upsert: bool,
    indexing_directive: IndexingDirective,
    if_match_condition: Option<IfMatchCondition<'b>>,
    if_modified_since: Option<&'b DateTime<Utc>>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel<'b>>,
    allow_tentative_writes: bool,
}

impl<'a, 'b, C, D> CreateDocumentBuilder<'a, 'b, C, D, No>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    pub(crate) fn new(
        collection_client: &'a dyn CollectionClient<C, D>,
    ) -> CreateDocumentBuilder<'a, 'b, C, D, No> {
        CreateDocumentBuilder {
            collection_client,
            p_partition_keys: PhantomData {},
            partition_keys: None,
            is_upsert: false,
            indexing_directive: IndexingDirective::Default,
            if_match_condition: None,
            if_modified_since: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            allow_tentative_writes: false,
        }
    }
}

impl<'a, 'b, C, D, PartitionKeysSet> CollectionClientRequired<'a, C, D>
    for CreateDocumentBuilder<'a, 'b, C, D, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn collection_client(&self) -> &'a dyn CollectionClient<C, D> {
        self.collection_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, 'b, C, D> PartitionKeysRequired<'b> for CreateDocumentBuilder<'a, 'b, C, D, Yes>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn partition_keys(&self) -> &'b PartitionKeys {
        self.partition_keys.unwrap()
    }
}

impl<'a, 'b, C, D, PartitionKeysSet> IsUpsertOption
    for CreateDocumentBuilder<'a, 'b, C, D, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn is_upsert(&self) -> bool {
        self.is_upsert
    }
}

impl<'a, 'b, C, D, PartitionKeysSet> IndexingDirectiveOption
    for CreateDocumentBuilder<'a, 'b, C, D, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn indexing_directive(&self) -> IndexingDirective {
        self.indexing_directive
    }
}

impl<'a, 'b, C, D, PartitionKeysSet> IfMatchConditionOption<'b>
    for CreateDocumentBuilder<'a, 'b, C, D, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn if_match_condition(&self) -> Option<IfMatchCondition<'b>> {
        self.if_match_condition
    }
}

impl<'a, 'b, C, D, PartitionKeysSet> IfModifiedSinceOption<'b>
    for CreateDocumentBuilder<'a, 'b, C, D, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn if_modified_since(&self) -> Option<&'b DateTime<Utc>> {
        self.if_modified_since
    }
}

impl<'a, 'b, C, D, PartitionKeysSet> UserAgentOption<'b>
    for CreateDocumentBuilder<'a, 'b, C, D, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, C, D, PartitionKeysSet> ActivityIdOption<'b>
    for CreateDocumentBuilder<'a, 'b, C, D, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, C, D, PartitionKeysSet> ConsistencyLevelOption<'b>
    for CreateDocumentBuilder<'a, 'b, C, D, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'b>> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b, C, D, PartitionKeysSet> AllowTentativeWritesOption
    for CreateDocumentBuilder<'a, 'b, C, D, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn allow_tentative_writes(&self) -> bool {
        self.allow_tentative_writes
    }
}

impl<'a, 'b, C, D> PartitionKeysSupport<'b> for CreateDocumentBuilder<'a, 'b, C, D, No>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = CreateDocumentBuilder<'a, 'b, C, D, Yes>;

    #[inline]
    fn with_partition_keys(self, partition_keys: &'b PartitionKeys) -> Self::O {
        CreateDocumentBuilder {
            collection_client: self.collection_client,
            p_partition_keys: PhantomData {},
            partition_keys: Some(partition_keys),
            is_upsert: self.is_upsert,
            indexing_directive: self.indexing_directive,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
        }
    }
}

impl<'a, 'b, C, D, PartitionKeysSet> IsUpsertSupport
    for CreateDocumentBuilder<'a, 'b, C, D, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = CreateDocumentBuilder<'a, 'b, C, D, PartitionKeysSet>;

    #[inline]
    fn with_is_upsert(self, is_upsert: bool) -> Self::O {
        CreateDocumentBuilder {
            collection_client: self.collection_client,
            p_partition_keys: PhantomData {},
            partition_keys: self.partition_keys,
            is_upsert,
            indexing_directive: self.indexing_directive,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
        }
    }
}

impl<'a, 'b, C, D, PartitionKeysSet> IndexingDirectiveSupport
    for CreateDocumentBuilder<'a, 'b, C, D, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = CreateDocumentBuilder<'a, 'b, C, D, PartitionKeysSet>;

    #[inline]
    fn with_indexing_directive(self, indexing_directive: IndexingDirective) -> Self::O {
        CreateDocumentBuilder {
            collection_client: self.collection_client,
            p_partition_keys: PhantomData {},
            partition_keys: self.partition_keys,
            is_upsert: self.is_upsert,
            indexing_directive,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
        }
    }
}

impl<'a, 'b, C, D, PartitionKeysSet> IfMatchConditionSupport<'b>
    for CreateDocumentBuilder<'a, 'b, C, D, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = CreateDocumentBuilder<'a, 'b, C, D, PartitionKeysSet>;

    #[inline]
    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'b>) -> Self::O {
        CreateDocumentBuilder {
            collection_client: self.collection_client,
            p_partition_keys: PhantomData {},
            partition_keys: self.partition_keys,
            is_upsert: self.is_upsert,
            indexing_directive: self.indexing_directive,
            if_match_condition: Some(if_match_condition),
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
        }
    }
}

impl<'a, 'b, C, D, PartitionKeysSet> IfModifiedSinceSupport<'b>
    for CreateDocumentBuilder<'a, 'b, C, D, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = CreateDocumentBuilder<'a, 'b, C, D, PartitionKeysSet>;

    #[inline]
    fn with_if_modified_since(self, if_modified_since: &'b DateTime<Utc>) -> Self::O {
        CreateDocumentBuilder {
            collection_client: self.collection_client,
            p_partition_keys: PhantomData {},
            partition_keys: self.partition_keys,
            is_upsert: self.is_upsert,
            indexing_directive: self.indexing_directive,
            if_match_condition: self.if_match_condition,
            if_modified_since: Some(if_modified_since),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
        }
    }
}

impl<'a, 'b, C, D, PartitionKeysSet> UserAgentSupport<'b>
    for CreateDocumentBuilder<'a, 'b, C, D, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = CreateDocumentBuilder<'a, 'b, C, D, PartitionKeysSet>;

    #[inline]
    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        CreateDocumentBuilder {
            collection_client: self.collection_client,
            p_partition_keys: PhantomData {},
            partition_keys: self.partition_keys,
            is_upsert: self.is_upsert,
            indexing_directive: self.indexing_directive,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
        }
    }
}

impl<'a, 'b, C, D, PartitionKeysSet> ActivityIdSupport<'b>
    for CreateDocumentBuilder<'a, 'b, C, D, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = CreateDocumentBuilder<'a, 'b, C, D, PartitionKeysSet>;

    #[inline]
    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        CreateDocumentBuilder {
            collection_client: self.collection_client,
            p_partition_keys: PhantomData {},
            partition_keys: self.partition_keys,
            is_upsert: self.is_upsert,
            indexing_directive: self.indexing_directive,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
        }
    }
}

impl<'a, 'b, C, D, PartitionKeysSet> ConsistencyLevelSupport<'b>
    for CreateDocumentBuilder<'a, 'b, C, D, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = CreateDocumentBuilder<'a, 'b, C, D, PartitionKeysSet>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'b>) -> Self::O {
        CreateDocumentBuilder {
            collection_client: self.collection_client,
            p_partition_keys: PhantomData {},
            partition_keys: self.partition_keys,
            is_upsert: self.is_upsert,
            indexing_directive: self.indexing_directive,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
            allow_tentative_writes: self.allow_tentative_writes,
        }
    }
}

impl<'a, 'b, C, D, PartitionKeysSet> AllowTentativeWritesSupport
    for CreateDocumentBuilder<'a, 'b, C, D, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = CreateDocumentBuilder<'a, 'b, C, D, PartitionKeysSet>;

    #[inline]
    fn with_allow_tentative_writes(self, allow_tentative_writes: bool) -> Self::O {
        CreateDocumentBuilder {
            collection_client: self.collection_client,
            p_partition_keys: PhantomData {},
            partition_keys: self.partition_keys,
            is_upsert: self.is_upsert,
            indexing_directive: self.indexing_directive,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes,
        }
    }
}

// methods callable regardless
impl<'a, 'b, C, D, PartitionKeysSet> CreateDocumentBuilder<'a, 'b, C, D, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, C, D> CreateDocumentBuilder<'a, 'b, C, D, Yes>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    pub async fn execute_with_document<T>(
        &self,
        document: &T,
    ) -> Result<CreateDocumentResponse, AzureError>
    where
        T: Serialize,
    {
        let mut req = self.collection_client.cosmos_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/docs",
                self.collection_client.database_client().database_name(),
                self.collection_client.collection_name()
            ),
            hyper::Method::POST,
            ResourceType::Documents,
        );

        // add trait headers
        req = IfMatchConditionOption::add_header(self, req);
        req = IfModifiedSinceOption::add_header(self, req);
        req = UserAgentOption::add_header(self, req);
        req = ActivityIdOption::add_header(self, req);
        req = ConsistencyLevelOption::add_header(self, req);
        req = PartitionKeysRequired::add_header(self, req);
        req = IsUpsertOption::add_header(self, req);
        req = IndexingDirectiveOption::add_header(self, req);
        req = AllowTentativeWritesOption::add_header(self, req);

        let serialized = serde_json::to_string(document)?;
        let req = req.body(hyper::Body::from(serialized))?;

        let (status_code, headers, whole_body) =
            extract_status_headers_and_body(self.collection_client.hyper_client().request(req))
                .await?;

        debug!("status_core == {:?}", status_code);
        debug!("headers == {:?}", headers);
        debug!("whole body == {:#?}", whole_body);

        // expect CREATED is IsUpsert is off. Otherwise either
        // CREATED or OK means success.
        if !self.is_upsert() && status_code != StatusCode::CREATED {
            return Err(UnexpectedHTTPResult::new(
                StatusCode::CREATED,
                status_code,
                std::str::from_utf8(&whole_body)?,
            )
            .into());
        } else if status_code != StatusCode::CREATED && status_code != StatusCode::OK {
            return Err(UnexpectedHTTPResult::new_multiple(
                vec![StatusCode::CREATED, StatusCode::OK],
                status_code,
                std::str::from_utf8(&whole_body)?,
            )
            .into());
        }

        CreateDocumentResponse::try_from((status_code, &headers, &whole_body as &[u8]))
    }
}
