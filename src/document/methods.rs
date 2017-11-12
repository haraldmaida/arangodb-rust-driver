
use std::fmt::Debug;
use std::iter::FromIterator;
use std::marker::PhantomData;

use serde::de::DeserializeOwned;
use serde::ser::Serialize;

use api::method::{Method, Operation, Parameters, Prepare, RpcReturnType};
use arango::protocol::{FIELD_CODE, HEADER_IF_MATCH, HEADER_IF_NON_MATCH,
    PARAM_IGNORE_REVISIONS, PARAM_KEEP_NULL, PARAM_MERGE_OBJECTS,
    PARAM_RETURN_NEW, PARAM_RETURN_OLD, PARAM_WAIT_FOR_SYNC, PATH_API_DOCUMENT};
use super::types::*;

#[derive(Clone, Debug, PartialEq)]
pub struct GetDocument<T> {
    id: DocumentId,
    if_match: Option<String>,
    if_non_match: Option<String>,
    content: PhantomData<T>,
}

impl<T> GetDocument<T> {
    pub fn new(id: DocumentId) -> Self {
        GetDocument {
            id,
            if_match: None,
            if_non_match: None,
            content: PhantomData,
        }
    }

    pub fn with_if_match<Im>(mut self, if_match: Im) -> Self
        where Im: Into<Option<String>>
    {
        self.if_match = if_match.into();
        self
    }

    pub fn with_if_non_match<Inm>(mut self, if_non_match: Inm) -> Self
        where Inm: Into<Option<String>>
    {
        self.if_non_match = if_non_match.into();
        self
    }

    pub fn id(&self) -> &DocumentId {
        &self.id
    }

    pub fn if_match(&self) -> Option<&String> {
        self.if_match.as_ref()
    }

    pub fn if_non_match(&self) -> Option<&String> {
        self.if_non_match.as_ref()
    }
}

impl<T> Method for GetDocument<T>
    where T: DeserializeOwned
{
    type Result = Document<T>;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl<T> Prepare for GetDocument<T> {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> String {
        String::from(PATH_API_DOCUMENT) + "/" + &self.id.to_string()
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn header(&self) -> Parameters {
        let mut header = Parameters::new();
        if let Some(ref if_match) = self.if_match {
            header.insert(HEADER_IF_MATCH, if_match.to_owned());
        }
        if let Some(ref if_non_match) = self.if_non_match {
            header.insert(HEADER_IF_NON_MATCH, if_non_match.to_owned());
        }
        header
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GetDocumentHeader {
    id: DocumentId,
    if_match: Option<String>,
    if_non_match: Option<String>,
}

impl GetDocumentHeader {
    pub fn new(id: DocumentId) -> Self {
        GetDocumentHeader {
            id,
            if_match: None,
            if_non_match: None,
        }
    }

    pub fn with_if_match<Im>(mut self, if_match: Im) -> Self
        where Im: Into<Option<String>>
    {
        self.if_match = if_match.into();
        self
    }

    pub fn with_if_non_match<Inm>(mut self, if_non_match: Inm) -> Self
        where Inm: Into<Option<String>>
    {
        self.if_non_match = if_non_match.into();
        self
    }

    pub fn id(&self) -> &DocumentId {
        &self.id
    }

    pub fn if_match(&self) -> Option<&String> {
        self.if_match.as_ref()
    }

    pub fn if_non_match(&self) -> Option<&String> {
        self.if_non_match.as_ref()
    }
}

impl Method for GetDocumentHeader {
    type Result = ();
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for GetDocumentHeader {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::ReadHeader
    }

    fn path(&self) -> String {
        String::from(PATH_API_DOCUMENT) + "/" + &self.id.to_string()
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn header(&self) -> Parameters {
        let mut header = Parameters::new();
        if let Some(ref if_match) = self.if_match {
            header.insert(HEADER_IF_MATCH, if_match.to_owned());
        }
        if let Some(ref if_non_match) = self.if_non_match {
            header.insert(HEADER_IF_NON_MATCH, if_non_match.to_owned());
        }
        header
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct InsertDocument<T> {
    collection_name: String,
    document: NewDocument<T>,
    force_wait_for_sync: Option<bool>,
}

impl<T> InsertDocument<T> {
    pub fn new<N>(collection_name: N, document: NewDocument<T>) -> Self
        where N: Into<String>
    {
        InsertDocument {
            collection_name: collection_name.into(),
            document,
            force_wait_for_sync: None,
        }
    }

    pub fn collection_name(&self) -> &str {
        &self.collection_name
    }

    pub fn document(&self) -> &NewDocument<T> {
        &self.document
    }

    pub fn with_force_wait_for_sync<Wfs>(mut self, force_wait_for_sync: Wfs) -> Self
        where Wfs: Into<Option<bool>>
    {
        self.force_wait_for_sync = force_wait_for_sync.into();
        self
    }

    pub fn is_force_wait_for_sync(&self) -> Option<bool> {
        self.force_wait_for_sync
    }
}

impl<T> Method for InsertDocument<T>
    where T: DeserializeOwned + Debug
{
    type Result = DocumentHeader;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl<T> Prepare for InsertDocument<T>
    where T: Serialize + Debug
{
    type Content = NewDocument<T>;

    fn operation(&self) -> Operation {
        Operation::Create
    }

    fn path(&self) -> String {
        String::from(PATH_API_DOCUMENT) + "/" + &self.collection_name
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();
        params.insert(PARAM_RETURN_NEW, false);
        if let Some(force_wait_for_sync) = self.force_wait_for_sync {
            params.insert(PARAM_WAIT_FOR_SYNC, force_wait_for_sync);
        }
        params
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.document)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct InsertDocumentReturnNew<T> {
    collection_name: String,
    document: NewDocument<T>,
    force_wait_for_sync: Option<bool>,
}

impl<T> InsertDocumentReturnNew<T> {
    pub fn new<N>(collection_name: N, document: NewDocument<T>) -> Self
        where N: Into<String>
    {
        InsertDocumentReturnNew {
            collection_name: collection_name.into(),
            document,
            force_wait_for_sync: None,
        }
    }

    pub fn collection_name(&self) -> &str {
        &self.collection_name
    }

    pub fn document(&self) -> &NewDocument<T> {
        &self.document
    }

    pub fn with_force_wait_for_sync<Wfs>(mut self, force_wait_for_sync: Wfs) -> Self
        where Wfs: Into<Option<bool>>
    {
        self.force_wait_for_sync = force_wait_for_sync.into();
        self
    }

    pub fn is_force_wait_for_sync(&self) -> Option<bool> {
        self.force_wait_for_sync
    }
}

impl<T> Method for InsertDocumentReturnNew<T>
    where T: DeserializeOwned + Debug
{
    type Result = Document<T>;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl<T> Prepare for InsertDocumentReturnNew<T>
    where T: Serialize + Debug
{
    type Content = NewDocument<T>;

    fn operation(&self) -> Operation {
        Operation::Create
    }

    fn path(&self) -> String {
        String::from(PATH_API_DOCUMENT) + "/" + &self.collection_name
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();
        params.insert(PARAM_RETURN_NEW, true);
        if let Some(force_wait_for_sync) = self.force_wait_for_sync {
            params.insert(PARAM_WAIT_FOR_SYNC, force_wait_for_sync);
        }
        params
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.document)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct InsertDocuments<T> {
    collection_name: String,
    documents: Vec<NewDocument<T>>,
    force_wait_for_sync: Option<bool>,
}

impl<T> InsertDocuments<T> {
    pub fn new<N, Docs>(collection_name: N, documents: Docs) -> Self
        where N: Into<String>, Docs: IntoIterator<Item=NewDocument<T>>
    {
        InsertDocuments {
            collection_name: collection_name.into(),
            documents: Vec::from_iter(documents.into_iter()),
            force_wait_for_sync: None,
        }
    }

    pub fn collection_name(&self) -> &str {
        &self.collection_name
    }

    pub fn documents(&self) -> &[NewDocument<T>] {
        &self.documents
    }

    pub fn with_force_wait_for_sync<Wfs>(mut self, force_wait_for_sync: Wfs) -> Self
        where Wfs: Into<Option<bool>>
    {
        self.force_wait_for_sync = force_wait_for_sync.into();
        self
    }

    pub fn is_force_wait_for_sync(&self) -> Option<bool> {
        self.force_wait_for_sync
    }
}

impl<T> Method for InsertDocuments<T>
    where T: DeserializeOwned + Debug
{
    type Result = Vec<DocumentHeader>;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl<T> Prepare for InsertDocuments<T>
    where T: Serialize + Debug
{
    type Content = Vec<NewDocument<T>>;

    fn operation(&self) -> Operation {
        Operation::Create
    }

    fn path(&self) -> String {
        String::from(PATH_API_DOCUMENT) + "/" + &self.collection_name
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();
        params.insert(PARAM_RETURN_NEW, false);
        if let Some(force_wait_for_sync) = self.force_wait_for_sync {
            params.insert(PARAM_WAIT_FOR_SYNC, force_wait_for_sync);
        }
        params
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.documents)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct InsertDocumentsReturnNew<T> {
    collection_name: String,
    documents: Vec<NewDocument<T>>,
    force_wait_for_sync: Option<bool>,
}

impl<T> InsertDocumentsReturnNew<T> {
    pub fn new<N, Docs>(collection_name: N, documents: Docs) -> Self
        where N: Into<String>, Docs: IntoIterator<Item=NewDocument<T>>
    {
        InsertDocumentsReturnNew {
            collection_name: collection_name.into(),
            documents: Vec::from_iter(documents.into_iter()),
            force_wait_for_sync: None,
        }
    }

    pub fn collection_name(&self) -> &str {
        &self.collection_name
    }

    pub fn documents(&self) -> &[NewDocument<T>] {
        &self.documents
    }

    pub fn with_force_wait_for_sync<Wfs>(mut self, force_wait_for_sync: Wfs) -> Self
        where Wfs: Into<Option<bool>>
    {
        self.force_wait_for_sync = force_wait_for_sync.into();
        self
    }

    pub fn is_force_wait_for_sync(&self) -> Option<bool> {
        self.force_wait_for_sync
    }
}

impl<T> Method for InsertDocumentsReturnNew<T>
    where T: DeserializeOwned + Debug
{
    type Result = Vec<Document<T>>;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl<T> Prepare for InsertDocumentsReturnNew<T>
    where T: Serialize + Debug
{
    type Content = Vec<NewDocument<T>>;

    fn operation(&self) -> Operation {
        Operation::Create
    }

    fn path(&self) -> String {
        String::from(PATH_API_DOCUMENT) + "/" + &self.collection_name
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();
        params.insert(PARAM_RETURN_NEW, true);
        if let Some(force_wait_for_sync) = self.force_wait_for_sync {
            params.insert(PARAM_WAIT_FOR_SYNC, force_wait_for_sync);
        }
        params
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.documents)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReplaceDocument<Old, New> {
    document_id: DocumentId,
    new_document: DocumentUpdate<New>,
    old_document: PhantomData<Old>,
    force_wait_for_sync: Option<bool>,
    ignore_revisions: Option<bool>,
    if_match: Option<String>,
    return_old: Option<bool>,
    return_new: Option<bool>,
}

impl<Old, New> ReplaceDocument<Old, New> {
    pub fn new(document_id: DocumentId, new_document: DocumentUpdate<New>) -> Self {
        ReplaceDocument {
            document_id,
            new_document,
            old_document: PhantomData,
            force_wait_for_sync: None,
            ignore_revisions: None,
            if_match: None,
            return_old: None,
            return_new: None,
        }
    }

    pub fn with_force_wait_for_sync<W>(mut self, force_wait_for_sync: W) -> Self
        where W: Into<Option<bool>>
    {
        self.force_wait_for_sync = force_wait_for_sync.into();
        self
    }

    pub fn with_ignore_revisions<R>(mut self, ignore_revisions: R) -> Self
        where R: Into<Option<bool>>
    {
        self.ignore_revisions = ignore_revisions.into();
        self
    }

    pub fn with_if_match<M>(mut self, if_match: M) -> Self
        where M: Into<Option<String>>
    {
        self.if_match = if_match.into();
        self
    }

    pub fn with_return_old<O>(mut self, return_old: O) -> Self
        where O: Into<Option<bool>>
    {
        self.return_old = return_old.into();
        self
    }

    pub fn with_return_new<N>(mut self, return_new: N) -> Self
        where N: Into<Option<bool>>
    {
        self.return_new = return_new.into();
        self
    }

    pub fn force_wait_for_sync(&self) -> Option<bool> {
        self.force_wait_for_sync
    }

    pub fn ignore_revisions(&self) -> Option<bool> {
        self.ignore_revisions
    }

    pub fn if_match(&self) -> Option<&String> {
        self.if_match.as_ref()
    }

    pub fn return_old(&self) -> Option<bool> {
        self.return_old
    }

    pub fn return_new(&self) -> Option<bool> {
        self.return_new
    }
}

impl<Old, New> Method for ReplaceDocument<Old, New>
    where Old: DeserializeOwned, New: DeserializeOwned
{
    type Result = UpdatedDocument<Old, New>;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl<Old, New> Prepare for ReplaceDocument<Old, New>
    where New: Serialize + Debug
{
    type Content = DocumentUpdate<New>;

    fn operation(&self) -> Operation {
        Operation::Replace
    }

    fn path(&self) -> String {
        String::from(PATH_API_DOCUMENT) + "/" + &self.document_id.to_string()
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();
        if let Some(force_wait_for_sync) = self.force_wait_for_sync {
            params.insert(PARAM_WAIT_FOR_SYNC, force_wait_for_sync);
        }
        if let Some(ignore_revisions) = self.ignore_revisions {
            params.insert(PARAM_IGNORE_REVISIONS, ignore_revisions);
        }
        if let Some(return_old) = self.return_old {
            params.insert(PARAM_RETURN_OLD, return_old);
        }
        if let Some(return_new) = self.return_new {
            params.insert(PARAM_RETURN_NEW, return_new);
        }
        params
    }

    fn header(&self) -> Parameters {
        let mut header = Parameters::new();
        if let Some(ref if_match) = self.if_match {
            header.insert(HEADER_IF_MATCH, if_match.to_owned());
        }
        header
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.new_document)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct UpdateDocument<Upd, Old, New> {
    document_id: DocumentId,
    update: DocumentUpdate<Upd>,
    old_content: PhantomData<Old>,
    new_content: PhantomData<New>,
    force_wait_for_sync: Option<bool>,
    ignore_revisions: Option<bool>,
    if_match: Option<String>,
    keep_none: Option<bool>,
    merge_objects: Option<bool>,
    return_old: Option<bool>,
    return_new: Option<bool>,
}

impl<Upd, Old, New> UpdateDocument<Upd, Old, New> {
    pub fn new(document_id: DocumentId, update: DocumentUpdate<Upd>) -> Self {
        UpdateDocument {
            document_id,
            update,
            old_content: PhantomData,
            new_content: PhantomData,
            force_wait_for_sync: None,
            ignore_revisions: None,
            if_match: None,
            keep_none: None,
            merge_objects: None,
            return_old: None,
            return_new: None,
        }
    }

    pub fn with_force_wait_for_sync<W>(mut self, force_wait_for_sync: W) -> Self
        where W: Into<Option<bool>>
    {
        self.force_wait_for_sync = force_wait_for_sync.into();
        self
    }

    pub fn with_ignore_revisions<R>(mut self, ignore_revisions: R) -> Self
        where R: Into<Option<bool>>
    {
        self.ignore_revisions = ignore_revisions.into();
        self
    }

    pub fn with_if_match<M>(mut self, if_match: M) -> Self
        where M: Into<Option<String>>
    {
        self.if_match = if_match.into();
        self
    }

    pub fn with_keep_none<K>(mut self, keep_none: K) -> Self
        where K: Into<Option<bool>>
    {
        self.keep_none = keep_none.into();
        self
    }

    pub fn with_merge_objects<M>(mut self, merge_objects: M) -> Self
        where M: Into<Option<bool>>
    {
        self.merge_objects = merge_objects.into();
        self
    }

    pub fn with_return_old<O>(mut self, return_old: O) -> Self
        where O: Into<Option<bool>>
    {
        self.return_old = return_old.into();
        self
    }

    pub fn with_return_new<N>(mut self, return_new: N) -> Self
        where N: Into<Option<bool>>
    {
        self.return_new = return_new.into();
        self
    }

    pub fn force_wait_for_sync(&self) -> Option<bool> {
        self.force_wait_for_sync
    }

    pub fn ignore_revisions(&self) -> Option<bool> {
        self.ignore_revisions
    }

    pub fn if_match(&self) -> Option<&String> {
        self.if_match.as_ref()
    }

    pub fn keep_none(&self) -> Option<bool> {
        self.keep_none
    }

    pub fn merge_objects(&self) -> Option<bool> {
        self.merge_objects
    }

    pub fn return_old(&self) -> Option<bool> {
        self.return_old
    }

    pub fn return_new(&self) -> Option<bool> {
        self.return_new
    }
}

impl<Upd, Old, New> Method for UpdateDocument<Upd, Old, New>
    where Old: DeserializeOwned, New: DeserializeOwned
{
    type Result = UpdatedDocument<Old, New>;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl<Upd, Old, New> Prepare for UpdateDocument<Upd, Old, New>
    where Upd: Serialize + Debug
{
    type Content = DocumentUpdate<Upd>;

    fn operation(&self) -> Operation {
        Operation::Modify
    }

    fn path(&self) -> String {
        String::from(PATH_API_DOCUMENT) + "/" + &self.document_id.to_string()
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();
        if let Some(force_wait_for_sync) = self.force_wait_for_sync {
            params.insert(PARAM_WAIT_FOR_SYNC, force_wait_for_sync);
        }
        if let Some(ignore_revisions) = self.ignore_revisions {
            params.insert(PARAM_IGNORE_REVISIONS, ignore_revisions);
        }
        if let Some(keep_none) = self.keep_none {
            params.insert(PARAM_KEEP_NULL, keep_none);
        }
        if let Some(merge_objects) = self.merge_objects {
            params.insert(PARAM_MERGE_OBJECTS, merge_objects);
        }
        if let Some(return_old) = self.return_old {
            params.insert(PARAM_RETURN_OLD, return_old);
        }
        if let Some(return_new) = self.return_new {
            params.insert(PARAM_RETURN_NEW, return_new);
        }
        params
    }

    fn header(&self) -> Parameters {
        let mut header = Parameters::new();
        if let Some(ref if_match) = self.if_match {
            header.insert(HEADER_IF_MATCH, if_match.to_owned());
        }
        header
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.update)
    }
}
