mod call_hierarchy;
mod code_action;
mod code_lens;
mod color;
mod completion;
mod document_diagnostic;
mod document_highlight;
mod document_link;
mod document_symbols;
mod file_operations;
mod folding_range;
mod formatting;
mod hover;
mod inlay_hint;
#[cfg(feature = "proposed")]
mod inline_completion;
mod inline_value;
mod linked_editing;
mod moniker;
mod notebook;
mod progress;
mod references;
mod rename;
mod selection_range;
mod semantic_tokens;
mod signature_help;
mod trace;
mod type_hierarchy;
mod window;
mod workspace_diagnostic;
mod workspace_folders;
mod workspace_symbols;

pub use call_hierarchy::*;
pub use code_action::*;
pub use code_lens::*;
pub use color::*;
pub use completion::*;
pub use document_diagnostic::*;
pub use document_highlight::*;
pub use document_link::*;
pub use document_symbols::*;
pub use file_operations::*;
pub use folding_range::*;
pub use formatting::*;
pub use hover::*;
pub use inlay_hint::*;
#[cfg(feature = "proposed")]
pub use inline_completion::*;
pub use inline_value::*;
pub use linked_editing::*;
pub use moniker::*;
pub use notebook::*;
pub use progress::*;
pub use references::*;
pub use rename::*;
pub use selection_range::*;
pub use semantic_tokens::*;
pub use signature_help::*;
pub use trace::*;
pub use type_hierarchy::*;
pub use window::*;
pub use workspace_diagnostic::*;
pub use workspace_folders::*;
pub use workspace_symbols::*;

use serde::{Deserialize, Serialize, de::Error};
use std::collections::HashMap;

use crate::Uri;
use crate::macros::lsp_enum;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct CancelParams {
    /// The request id to cancel.
    pub id: NumberOrString,
}

/// The LSP any type
///
/// @since 3.17.0
pub type LSPAny = serde_json::Value;

/// LSP object definition.
///
/// @since 3.17.0
pub type LSPObject = serde_json::Map<String, serde_json::Value>;

/// LSP arrays.
///
/// @since 3.17.0
pub type LSPArray = Vec<serde_json::Value>;

/// Represents a link between a source and a target location.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationLink {
    /// Span of the origin of this link.
    ///
    /// Used as the underlined span for mouse interaction. Defaults to the word range at
    /// the mouse position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_selection_range: Option<Range>,

    /// The target resource identifier of this link.
    pub target_uri: Uri,

    /// The full target range of this link.
    pub target_range: Range,

    /// The span of this link.
    pub target_selection_range: Range,
}

/// A type indicating how positions are encoded,
/// specifically what column offsets mean.
///
/// @since 3.17.0
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Deserialize, Serialize, Hash)]
pub struct PositionEncodingKind(std::borrow::Cow<'static, str>);

impl PositionEncodingKind {
    /// Character offsets count UTF-8 code units.
    pub const UTF8: Self = Self::new("utf-8");

    /// Character offsets count UTF-16 code units.
    ///
    /// This is the default and must always be supported
    /// by servers
    pub const UTF16: Self = Self::new("utf-16");

    /// Character offsets count UTF-32 code units.
    ///
    /// Implementation note: these are the same as Unicode code points,
    /// so this `PositionEncodingKind` may also be used for an
    /// encoding-agnostic representation of character offsets.
    pub const UTF32: Self = Self::new("utf-32");

    #[must_use]
    pub const fn new(tag: &'static str) -> Self {
        Self(std::borrow::Cow::Borrowed(tag))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for PositionEncodingKind {
    fn from(from: String) -> Self {
        Self(std::borrow::Cow::from(from))
    }
}

impl From<&'static str> for PositionEncodingKind {
    fn from(from: &'static str) -> Self {
        Self::new(from)
    }
}

/// Represents a diagnostic, such as a compiler error or warning.
/// Diagnostic objects are only valid in the scope of a resource.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Diagnostic {
    /// The range at which the message applies.
    pub range: Range,

    /// The diagnostic's severity. Can be omitted. If omitted it is up to the
    /// client to interpret diagnostics as error, warning, info or hint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<DiagnosticSeverity>,

    /// The diagnostic's code. Can be omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<NumberOrString>,

    /// An optional property to describe the error code.
    ///
    /// @since 3.16.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_description: Option<CodeDescription>,

    /// A human-readable string describing the source of this
    /// diagnostic, e.g. `typescript` or `super lint`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// The diagnostic's message.
    pub message: String,

    /// An array of related diagnostic information, e.g. when symbol-names within
    /// a scope collide all definitions can be marked via this property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_information: Option<Vec<DiagnosticRelatedInformation>>,

    /// Additional metadata about the diagnostic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<DiagnosticTag>>,

    /// A data entry field that is preserved between a `textDocument/publishDiagnostics`
    /// notification and `textDocument/codeAction` request.
    ///
    /// @since 3.16.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeDescription {
    pub href: Uri,
}

impl Diagnostic {
    #[must_use]
    pub fn new(
        range: Range,
        severity: Option<DiagnosticSeverity>,
        code: Option<NumberOrString>,
        source: Option<String>,
        message: String,
        related_information: Option<Vec<DiagnosticRelatedInformation>>,
        tags: Option<Vec<DiagnosticTag>>,
    ) -> Self {
        Self {
            range,
            severity,
            code,
            source,
            message,
            related_information,
            tags,
            ..Self::default()
        }
    }

    #[must_use]
    pub fn new_simple(range: Range, message: String) -> Self {
        Self::new(range, None, None, None, message, None, None)
    }

    #[must_use]
    pub fn new_with_code_number(
        range: Range,
        severity: DiagnosticSeverity,
        code_number: i32,
        source: Option<String>,
        message: String,
    ) -> Self {
        let code = Some(NumberOrString::Number(code_number));
        Self::new(range, Some(severity), code, source, message, None, None)
    }
}

/// The protocol currently supports the following diagnostic severities:
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DiagnosticSeverity(i32);

lsp_enum! {
    impl DiagnosticSeverity {
        /// Reports an error.
        const ERROR = 1;
        /// Reports a warning.
        const WARNING = 2;
        /// Reports an information.
        const INFORMATION = 3;
        /// Reports a hint.
        const HINT = 4;
    }
}

/// Represents a related message and source code location for a diagnostic. This
/// should be used to point to code locations that cause or related to a
/// diagnostics, e.g when duplicating a symbol in a scope.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct DiagnosticRelatedInformation {
    /// The location of this related diagnostic information.
    pub location: Location,

    /// The message of this related diagnostic information.
    pub message: String,
}

/// The diagnostic tags.
#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DiagnosticTag(i32);

lsp_enum! {
    impl DiagnosticTag {
        /// Unused or unnecessary code.
        /// Clients are allowed to render diagnostics with this tag faded out instead of having
        /// an error squiggle.
        const UNNECESSARY = 1;
        /// Deprecated or obsolete code.
        /// Clients are allowed to rendered diagnostics with this tag strike through.
        const DEPRECATED = 2;
    }
}

/// Represents a reference to a command. Provides a title which will be used to represent a command in the UI.
///
/// Commands are identified by a string identifier. The recommended way to handle commands is to implement
/// their execution on the server side if the client and server provides the corresponding capabilities.
/// Alternatively the tool extension code could handle the command.
/// The protocol currently doesn’t specify a set of well-known commands.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Command {
    /// Title of the command, like `save`.
    pub title: String,
    /// The identifier of the actual command handler.
    pub command: String,
    /// Arguments that the command handler should be
    /// invoked with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<serde_json::Value>>,
}

impl Command {
    #[must_use]
    pub const fn new(
        title: String,
        command: String,
        arguments: Option<Vec<serde_json::Value>>,
    ) -> Self {
        Self {
            title,
            command,
            arguments,
        }
    }
}

/// A textual edit applicable to a text document.
///
/// If n `TextEdit`s are applied to a text document all text edits describe changes to the initial document version.
/// Execution wise text edits should applied from the bottom to the top of the text document. Overlapping text edits
/// are not supported.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextEdit {
    /// The range of the text document to be manipulated. To insert
    /// text into a document create a range where start === end.
    pub range: Range,
    /// The string to be inserted. For delete operations use an
    /// empty string.
    pub new_text: String,
}

impl TextEdit {
    #[must_use]
    pub const fn new(range: Range, new_text: String) -> Self {
        Self { range, new_text }
    }
}

/// An identifier referring to a change annotation managed by a workspace
/// edit.
///
/// @since 3.16.0
pub type ChangeAnnotationIdentifier = String;

/// A special text edit with an additional change annotation.
///
/// @since 3.16.0
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnotatedTextEdit {
    #[serde(flatten)]
    pub text_edit: TextEdit,

    /// The actual annotation
    pub annotation_id: ChangeAnnotationIdentifier,
}

/// Describes textual changes on a single text document.
///
/// The text document is referred to as a
/// `OptionalVersionedTextDocumentIdentifier` to allow clients to check the text
/// document version before an edit is applied. A `TextDocumentEdit` describes
/// all changes on a version Si and after they are applied move the document
/// to version Si+1. So the creator of a `TextDocumentEdit` doesn't need to
/// sort the array or do any kind of ordering. However the edits must be non
/// overlapping.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentEdit {
    /// The text document to change.
    pub text_document: OptionalVersionedTextDocumentIdentifier,

    /// The edits to be applied.
    ///
    /// @since 3.16.0 - support for `AnnotatedTextEdit`. This is guarded by the
    /// client capability `workspace.workspaceEdit.changeAnnotationSupport`
    pub edits: Vec<OneOf<TextEdit, AnnotatedTextEdit>>,
}

/// Additional information that describes document changes.
///
/// @since 3.16.0
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAnnotation {
    /// A human-readable string describing the actual change. The string
    /// is rendered prominent in the user interface.
    pub label: String,

    /// A flag which indicates that user confirmation is needed
    /// before applying the change.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_confirmation: Option<bool>,

    /// A human-readable string which is rendered less prominent in
    /// the user interface.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAnnotationWorkspaceEditClientCapabilities {
    /// Whether the client groups edits with equal labels into tree nodes,
    /// for instance all edits labelled with "Changes in Strings" would
    /// be a tree node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups_on_label: Option<bool>,
}

/// Options to create a file.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFileOptions {
    /// Overwrite existing file. Overwrite wins over `ignoreIfExists`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite: Option<bool>,
    /// Ignore if exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_if_exists: Option<bool>,
}

/// Create file operation
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFile {
    /// The resource to create.
    pub uri: Uri,
    /// Additional options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<CreateFileOptions>,

    /// An optional annotation identifier describing the operation.
    ///
    /// @since 3.16.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_id: Option<ChangeAnnotationIdentifier>,
}

/// Rename file options
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameFileOptions {
    /// Overwrite target if existing. Overwrite wins over `ignoreIfExists`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite: Option<bool>,
    /// Ignores if target exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_if_exists: Option<bool>,
}

/// Rename file operation
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameFile {
    /// The old (existing) location.
    pub old_uri: Uri,
    /// The new location.
    pub new_uri: Uri,
    /// Rename options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<RenameFileOptions>,

    /// An optional annotation identifier describing the operation.
    ///
    /// @since 3.16.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_id: Option<ChangeAnnotationIdentifier>,
}

/// Delete file options
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteFileOptions {
    /// Delete the content recursively if a folder is denoted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,
    /// Ignore the operation if the file doesn't exist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_if_not_exists: Option<bool>,

    /// An optional annotation identifier describing the operation.
    ///
    /// @since 3.16.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_id: Option<ChangeAnnotationIdentifier>,
}

/// Delete file operation
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteFile {
    /// The file to delete.
    pub uri: Uri,
    /// Delete options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<DeleteFileOptions>,
}

/// A workspace edit represents changes to many resources managed in the workspace.
///
/// The edit should either provide `changes` or `documentChanges`.
/// If the client can handle versioned document edits and if `documentChanges` are present,
/// the latter are preferred over `changes`.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceEdit {
    /// Holds changes to existing resources.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub changes: Option<HashMap<Uri, Vec<TextEdit>>>, //    changes?: { [uri: string]: TextEdit[]; };

    /// Depending on the client capability `workspace.workspaceEdit.resourceOperations` document changes
    /// are either an array of `TextDocumentEdit`s to express changes to n different text documents
    /// where each text document edit addresses a specific version of a text document. Or it can contain
    /// above `TextDocumentEdit`s mixed with create, rename and delete file / folder operations.
    ///
    /// Whether a client supports versioned document edits is expressed via
    /// `workspace.workspaceEdit.documentChanges` client capability.
    ///
    /// If a client neither supports `documentChanges` nor `workspace.workspaceEdit.resourceOperations` then
    /// only plain `TextEdit`s using the `changes` property are supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_changes: Option<DocumentChanges>,

    /// A map of change annotations that can be referenced in
    /// `AnnotatedTextEdit`s or create, rename and delete file / folder
    /// operations.
    ///
    /// Whether clients honor this property depends on the client capability
    /// `workspace.changeAnnotationSupport`.
    ///
    /// @since 3.16.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_annotations: Option<HashMap<ChangeAnnotationIdentifier, ChangeAnnotation>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DocumentChanges {
    Edits(Vec<TextDocumentEdit>),
    Operations(Vec<DocumentChangeOperation>),
}

// TODO: Once https://github.com/serde-rs/serde/issues/912 is solved
// we can remove ResourceOp and switch to the following implementation
// of DocumentChangeOperation:
//
// #[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
// #[serde(tag = "kind", rename_all="lowercase" )]
// pub enum DocumentChangeOperation {
//     Create(CreateFile),
//     Rename(RenameFile),
//     Delete(DeleteFile),
//
//     #[serde(other)]
//     Edit(TextDocumentEdit),
// }

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged, rename_all = "lowercase")]
pub enum DocumentChangeOperation {
    Op(ResourceOp),
    Edit(TextDocumentEdit),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum ResourceOp {
    Create(CreateFile),
    Rename(RenameFile),
    Delete(DeleteFile),
}

pub type DidChangeConfigurationClientCapabilities = DynamicRegistrationClientCapabilities;

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigurationParams {
    pub items: Vec<ConfigurationItem>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigurationItem {
    /// The scope to get the configuration section for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_uri: Option<Uri>,

    ///The configuration section asked for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,
}

impl WorkspaceEdit {
    #[must_use]
    pub fn new(changes: HashMap<Uri, Vec<TextEdit>>) -> Self {
        Self {
            changes: Some(changes),
            document_changes: None,
            ..Default::default()
        }
    }
}

/// Text documents are identified using a URI. On the protocol level, URIs are passed as strings.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TextDocumentIdentifier {
    // !!!!!! Note:
    // In the spec VersionedTextDocumentIdentifier extends TextDocumentIdentifier
    // This modelled by "mixing-in" TextDocumentIdentifier in VersionedTextDocumentIdentifier,
    // so any changes to this type must be effected in the sub-type as well.
    /// The text document's URI.
    pub uri: Uri,
}

impl TextDocumentIdentifier {
    #[must_use]
    pub const fn new(uri: Uri) -> Self {
        Self { uri }
    }
}

/// An item to transfer a text document from the client to the server.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentItem {
    /// The text document's URI.
    pub uri: Uri,

    /// The text document's language identifier.
    pub language_id: String,

    /// The version number of this document (it will strictly increase after each
    /// change, including undo/redo).
    pub version: i32,

    /// The content of the opened text document.
    pub text: String,
}

impl TextDocumentItem {
    #[must_use]
    pub const fn new(uri: Uri, language_id: String, version: i32, text: String) -> Self {
        Self {
            uri,
            language_id,
            version,
            text,
        }
    }
}

/// An identifier to denote a specific version of a text document. This information usually flows from the client to the server.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct VersionedTextDocumentIdentifier {
    // This field was "mixed-in" from TextDocumentIdentifier
    /// The text document's URI.
    pub uri: Uri,

    /// The version number of this document.
    ///
    /// The version number of a document will increase after each change,
    /// including undo/redo. The number doesn't need to be consecutive.
    pub version: i32,
}

impl VersionedTextDocumentIdentifier {
    #[must_use]
    pub const fn new(uri: Uri, version: i32) -> Self {
        Self { uri, version }
    }
}

/// An identifier which optionally denotes a specific version of a text document. This information usually flows from the server to the client
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct OptionalVersionedTextDocumentIdentifier {
    // This field was "mixed-in" from TextDocumentIdentifier
    /// The text document's URI.
    pub uri: Uri,

    /// The version number of this document. If an optional versioned text document
    /// identifier is sent from the server to the client and the file is not
    /// open in the editor (the server has not received an open notification
    /// before) the server can send `null` to indicate that the version is
    /// known and the content on disk is the master (as specified with document
    /// content ownership).
    ///
    /// The version number of a document will increase after each change,
    /// including undo/redo. The number doesn't need to be consecutive.
    pub version: Option<i32>,
}

impl OptionalVersionedTextDocumentIdentifier {
    #[must_use]
    pub const fn new(uri: Uri, version: i32) -> Self {
        Self {
            uri,
            version: Some(version),
        }
    }
}

/// A parameter literal used in requests to pass a text document and a position inside that document.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentPositionParams {
    // !!!!!! Note:
    // In the spec ReferenceParams extends TextDocumentPositionParams
    // This modelled by "mixing-in" TextDocumentPositionParams in ReferenceParams,
    // so any changes to this type must be effected in sub-type as well.
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    /// The position inside the text document.
    pub position: Position,
}

impl TextDocumentPositionParams {
    #[must_use]
    pub const fn new(text_document: TextDocumentIdentifier, position: Position) -> Self {
        Self {
            text_document,
            position,
        }
    }
}

/// A document filter denotes a document through properties like language, schema or pattern.
///
/// Examples are a filter that applies to TypeScript files on disk or a filter the applies to JSON
/// files with name package.json:
///
///
/// `{ language: 'typescript', scheme: 'file' }`
/// `{ language: 'json', pattern: '**/package.json' }`
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct DocumentFilter {
    /// A language id, like `typescript`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// A Uri [scheme](#Uri.scheme), like `file` or `untitled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,

    /// A glob pattern, like `*.{ts,js}`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
}

/// A document selector is the combination of one or many document filters.
pub type DocumentSelector = Vec<DocumentFilter>;

// ========================= Actual Protocol =========================

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeParams {
    /// The process Id of the parent process that started
    /// the server. Is null if the process has not been started by another process.
    /// If the parent process is not alive then the server should exit (see exit notification) its process.
    pub process_id: Option<u32>,

    /// The rootPath of the workspace. Is null
    /// if no folder is open.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[deprecated(note = "Use `root_uri` instead when possible")]
    pub root_path: Option<String>,

    /// The rootUri of the workspace. Is null if no
    /// folder is open. If both `rootPath` and `rootUri` are set
    /// `rootUri` wins.
    #[serde(default)]
    #[deprecated(note = "Use `workspace_folders` instead when possible")]
    pub root_uri: Option<Uri>,

    /// User provided initialization options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initialization_options: Option<serde_json::Value>,

    /// The capabilities provided by the client (editor or tool)
    pub capabilities: ClientCapabilities,

    /// The initial trace setting. If omitted trace is disabled (`off`).
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<TraceValue>,

    /// The workspace folders configured in the client when the server starts.
    /// This property is only available if the client supports workspace folders.
    /// It can be `null` if the client supports workspace folders but none are
    /// configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_folders: Option<Vec<WorkspaceFolder>>,

    /// Information about the client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_info: Option<ClientInfo>,

    /// The locale the client is currently showing the user interface
    /// in. This must not necessarily be the locale of the operating
    /// system.
    ///
    /// Uses IETF language tags as the value's syntax
    /// (See <https://en.wikipedia.org/wiki/IETF_language_tag>)
    ///
    /// @since 3.16.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,

    /// The LSP server may report about initialization progress to the client
    /// by using the following work done token if it was passed by the client.
    #[serde(flatten)]
    pub work_done_progress_params: WorkDoneProgressParams,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ClientInfo {
    /// The name of the client as defined by the client.
    pub name: String,
    /// The client's version as defined by the client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct InitializedParams {}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct GenericRegistrationOptions {
    #[serde(flatten)]
    pub text_document_registration_options: TextDocumentRegistrationOptions,

    #[serde(flatten)]
    pub options: GenericOptions,

    #[serde(flatten)]
    pub static_registration_options: StaticRegistrationOptions,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct GenericOptions {
    #[serde(flatten)]
    pub work_done_progress_options: WorkDoneProgressOptions,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct GenericParams {
    #[serde(flatten)]
    pub text_document_position_params: TextDocumentPositionParams,

    #[serde(flatten)]
    pub work_done_progress_params: WorkDoneProgressParams,

    #[serde(flatten)]
    pub partial_result_params: PartialResultParams,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamicRegistrationClientCapabilities {
    /// This capability supports dynamic registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_registration: Option<bool>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GotoCapability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_registration: Option<bool>,

    /// The client supports additional metadata in the form of definition links.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_support: Option<bool>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceEditClientCapabilities {
    /// The client supports versioned document changes in `WorkspaceEdit`s
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_changes: Option<bool>,

    /// The resource operations the client supports. Clients should at least
    /// support `create`, `rename` and `delete` files and folders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_operations: Option<Vec<ResourceOperationKind>>,

    /// The failure handling strategy of a client if applying the workspace edit fails.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_handling: Option<FailureHandlingKind>,

    /// Whether the client normalizes line endings to the client specific
    /// setting.
    /// If set to `true` the client will normalize line ending characters
    /// in a workspace edit to the client specific new line character(s).
    ///
    /// @since 3.16.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalizes_line_endings: Option<bool>,

    /// Whether the client in general supports change annotations on text edits,
    /// create file, rename file and delete file changes.
    ///
    /// @since 3.16.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_annotation_support: Option<ChangeAnnotationWorkspaceEditClientCapabilities>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ResourceOperationKind {
    Create,
    Rename,
    Delete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FailureHandlingKind {
    Abort,
    Transactional,
    TextOnlyTransactional,
    Undo,
}

/// Specific capabilities for the `SymbolKind` in the `workspace/symbol` request.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolKindCapability {
    /// The symbol kind values the client supports. When this
    /// property exists the client also guarantees that it will
    /// handle values outside its set gracefully and falls back
    /// to a default value when unknown.
    ///
    /// If this property is not present the client only supports
    /// the symbol kinds from `File` to `Array` as defined in
    /// the initial version of the protocol.
    pub value_set: Option<Vec<SymbolKind>>,
}

/// Workspace specific client capabilities.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceClientCapabilities {
    /// The client supports applying batch edits to the workspace by supporting
    /// the request `workspace/applyEdit`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_edit: Option<bool>,

    /// Capabilities specific to `WorkspaceEdit`s
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_edit: Option<WorkspaceEditClientCapabilities>,

    /// Capabilities specific to the `workspace/didChangeConfiguration` notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub did_change_configuration: Option<DidChangeConfigurationClientCapabilities>,

    /// Capabilities specific to the `workspace/didChangeWatchedFiles` notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub did_change_watched_files: Option<DidChangeWatchedFilesClientCapabilities>,

    /// Capabilities specific to the `workspace/symbol` request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<WorkspaceSymbolClientCapabilities>,

    /// Capabilities specific to the `workspace/executeCommand` request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute_command: Option<ExecuteCommandClientCapabilities>,

    /// The client has support for workspace folders.
    ///
    /// @since 3.6.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_folders: Option<bool>,

    /// The client supports `workspace/configuration` requests.
    ///
    /// @since 3.6.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<bool>,

    /// Capabilities specific to the semantic token requests scoped to the workspace.
    ///
    /// @since 3.16.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_tokens: Option<SemanticTokensWorkspaceClientCapabilities>,

    /// Capabilities specific to the code lens requests scoped to the workspace.
    ///
    /// @since 3.16.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_lens: Option<CodeLensWorkspaceClientCapabilities>,

    /// The client has support for file requests/notifications.
    ///
    /// @since 3.16.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_operations: Option<WorkspaceFileOperationsClientCapabilities>,

    /// Client workspace capabilities specific to inline values.
    ///
    /// @since 3.17.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_value: Option<InlineValueWorkspaceClientCapabilities>,

    /// Client workspace capabilities specific to inlay hints.
    ///
    /// @since 3.17.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inlay_hint: Option<InlayHintWorkspaceClientCapabilities>,

    /// Client workspace capabilities specific to diagnostics.
    /// since 3.17.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<DiagnosticWorkspaceClientCapabilities>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentSyncClientCapabilities {
    /// Whether text document synchronization supports dynamic registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_registration: Option<bool>,

    /// The client supports sending will save notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub will_save: Option<bool>,

    /// The client supports sending a will save request and
    /// waits for a response providing text edits which will
    /// be applied to the document before it is saved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub will_save_wait_until: Option<bool>,

    /// The client supports did save notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub did_save: Option<bool>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishDiagnosticsClientCapabilities {
    /// Whether the clients accepts diagnostics with related information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_information: Option<bool>,

    /// Client supports the tag property to provide meta data about a diagnostic.
    /// Clients supporting tags have to handle unknown tags gracefully.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "TagSupport::deserialize_compat"
    )]
    pub tag_support: Option<TagSupport<DiagnosticTag>>,

    /// Whether the client interprets the version property of the
    /// `textDocument/publishDiagnostics` notification's parameter.
    ///
    /// @since 3.15.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_support: Option<bool>,

    /// Client supports a codeDescription property
    ///
    /// @since 3.16.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_description_support: Option<bool>,

    /// Whether code action supports the `data` property which is
    /// preserved between a `textDocument/publishDiagnostics` and
    /// `textDocument/codeAction` request.
    ///
    /// @since 3.16.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_support: Option<bool>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TagSupport<T> {
    /// The tags supported by the client.
    pub value_set: Vec<T>,
}

impl<T> TagSupport<T> {
    /// Support for deserializing a boolean tag Support, in case it's present.
    ///
    /// This is currently the case for vscode 1.41.1
    fn deserialize_compat<'de, S>(serializer: S) -> Result<Option<Self>, S::Error>
    where
        S: serde::Deserializer<'de>,
        T: serde::Deserialize<'de>,
    {
        Ok(
            match Option::<serde_json::Value>::deserialize(serializer)
                .map_err(serde::de::Error::custom)?
            {
                Some(serde_json::Value::Bool(false)) | None => None,
                Some(serde_json::Value::Bool(true)) => Some(Self { value_set: vec![] }),
                Some(other) => Some(Self::deserialize(other).map_err(serde::de::Error::custom)?),
            },
        )
    }
}

/// Text document specific client capabilities.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synchronization: Option<TextDocumentSyncClientCapabilities>,
    /// Capabilities specific to the `textDocument/completion`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion: Option<CompletionClientCapabilities>,

    /// Capabilities specific to the `textDocument/hover`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover: Option<HoverClientCapabilities>,

    /// Capabilities specific to the `textDocument/signatureHelp`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_help: Option<SignatureHelpClientCapabilities>,

    /// Capabilities specific to the `textDocument/references`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<ReferenceClientCapabilities>,

    /// Capabilities specific to the `textDocument/documentHighlight`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_highlight: Option<DocumentHighlightClientCapabilities>,

    /// Capabilities specific to the `textDocument/documentSymbol`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_symbol: Option<DocumentSymbolClientCapabilities>,
    /// Capabilities specific to the `textDocument/formatting`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatting: Option<DocumentFormattingClientCapabilities>,

    /// Capabilities specific to the `textDocument/rangeFormatting`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_formatting: Option<DocumentRangeFormattingClientCapabilities>,

    /// Capabilities specific to the `textDocument/onTypeFormatting`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_type_formatting: Option<DocumentOnTypeFormattingClientCapabilities>,

    /// Capabilities specific to the `textDocument/declaration`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declaration: Option<GotoCapability>,

    /// Capabilities specific to the `textDocument/definition`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<GotoCapability>,

    /// Capabilities specific to the `textDocument/typeDefinition`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_definition: Option<GotoCapability>,

    /// Capabilities specific to the `textDocument/implementation`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implementation: Option<GotoCapability>,

    /// Capabilities specific to the `textDocument/codeAction`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_action: Option<CodeActionClientCapabilities>,

    /// Capabilities specific to the `textDocument/codeLens`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_lens: Option<CodeLensClientCapabilities>,

    /// Capabilities specific to the `textDocument/documentLink`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_link: Option<DocumentLinkClientCapabilities>,

    /// Capabilities specific to the `textDocument/documentColor` and the
    /// `textDocument/colorPresentation` request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_provider: Option<DocumentColorClientCapabilities>,

    /// Capabilities specific to the `textDocument/rename`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rename: Option<RenameClientCapabilities>,

    /// Capabilities specific to `textDocument/publishDiagnostics`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_diagnostics: Option<PublishDiagnosticsClientCapabilities>,

    /// Capabilities specific to `textDocument/foldingRange` requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folding_range: Option<FoldingRangeClientCapabilities>,

    /// Capabilities specific to the `textDocument/selectionRange` request.
    ///
    /// @since 3.15.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_range: Option<SelectionRangeClientCapabilities>,

    /// Capabilities specific to `textDocument/linkedEditingRange` requests.
    ///
    /// @since 3.16.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_editing_range: Option<LinkedEditingRangeClientCapabilities>,

    /// Capabilities specific to the various call hierarchy requests.
    ///
    /// @since 3.16.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_hierarchy: Option<CallHierarchyClientCapabilities>,

    /// Capabilities specific to the `textDocument/semanticTokens/*` requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_tokens: Option<SemanticTokensClientCapabilities>,

    /// Capabilities specific to the `textDocument/moniker` request.
    ///
    /// @since 3.16.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moniker: Option<MonikerClientCapabilities>,

    /// Capabilities specific to the various type hierarchy requests.
    ///
    /// @since 3.17.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_hierarchy: Option<TypeHierarchyClientCapabilities>,

    /// Capabilities specific to the `textDocument/inlineValue` request.
    ///
    /// @since 3.17.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_value: Option<InlineValueClientCapabilities>,

    /// Capabilities specific to the `textDocument/inlayHint` request.
    ///
    /// @since 3.17.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inlay_hint: Option<InlayHintClientCapabilities>,

    /// Capabilities specific to the diagnostic pull model.
    ///
    /// @since 3.17.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostic: Option<DiagnosticClientCapabilities>,

    /// Capabilities specific to the `textDocument/inlineCompletion` request.
    ///
    /// @since 3.18.0
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg(feature = "proposed")]
    pub inline_completion: Option<InlineCompletionClientCapabilities>,
}

/// Where `ClientCapabilities` are currently empty:
#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientCapabilities {
    /// Workspace specific client capabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<WorkspaceClientCapabilities>,

    /// Text document specific client capabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_document: Option<TextDocumentClientCapabilities>,

    /// Capabilities specific to the notebook document support.
    ///
    /// @since 3.17.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_document: Option<NotebookDocumentClientCapabilities>,

    /// Window specific client capabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<WindowClientCapabilities>,

    /// General client capabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general: Option<GeneralClientCapabilities>,

    /// Unofficial UT8-offsets extension.
    ///
    /// See <https://clangd.llvm.org/extensions.html#utf-8-offsets>.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg(feature = "proposed")]
    pub offset_encoding: Option<Vec<String>>,

    /// Experimental client capabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<serde_json::Value>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralClientCapabilities {
    /// Client capabilities specific to regular expressions.
    ///
    /// @since 3.16.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regular_expressions: Option<RegularExpressionsClientCapabilities>,

    /// Client capabilities specific to the client's markdown parser.
    ///
    /// @since 3.16.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<MarkdownClientCapabilities>,

    /// Client capability that signals how the client handles stale requests (e.g. a request for
    /// which the client will not process the response anymore since the information is outdated).
    ///
    /// @since 3.17.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stale_request_support: Option<StaleRequestSupportClientCapabilities>,

    /// The position encodings supported by the client. Client and server
    /// have to agree on the same position encoding to ensure that offsets
    /// (e.g. character position in a line) are interpreted the same on both
    /// side.
    ///
    /// To keep the protocol backwards compatible the following applies: if
    /// the value `utf-16` is missing from the array of position encodings
    /// servers can assume that the client supports UTF-16. UTF-16 is
    /// therefore a mandatory encoding.
    ///
    /// If omitted it defaults to `["utf-16"]`.
    ///
    /// Implementation considerations: since the conversion from one encoding
    /// into another requires the content of the file / line the conversion
    /// is best done where the file is read which is usually on the server
    /// side.
    ///
    /// @since 3.17.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_encodings: Option<Vec<PositionEncodingKind>>,
}

/// Client capability that signals how the client
/// handles stale requests (e.g. a request
/// for which the client will not process the response
/// anymore since the information is outdated).
///
/// @since 3.17.0
#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StaleRequestSupportClientCapabilities {
    /// The client will actively cancel the request.
    pub cancel: bool,

    /// The list of requests for which the client
    /// will retry the request if it receives a
    /// response with error code `ContentModified`
    pub retry_on_content_modified: Vec<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegularExpressionsClientCapabilities {
    /// The engine's name.
    pub engine: String,

    /// The engine's version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkdownClientCapabilities {
    /// The name of the parser.
    pub parser: String,

    /// The version of the parser.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// A list of HTML tags that the client allows / supports in
    /// Markdown.
    ///
    /// @since 3.17.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_tags: Option<Vec<String>>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeResult {
    /// The capabilities the language server provides.
    pub capabilities: ServerCapabilities,

    /// Information about the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_info: Option<ServerInfo>,

    /// Unofficial UT8-offsets extension.
    ///
    /// See <https://clangd.llvm.org/extensions.html#utf-8-offsets>.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg(feature = "proposed")]
    pub offset_encoding: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ServerInfo {
    /// The name of the server as defined by the server.
    pub name: String,
    /// The servers's version as defined by the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct InitializeError {
    /// Indicates whether the client execute the following retry logic:
    ///
    /// - (1) show the message provided by the `ResponseError` to the user
    /// - (2) user selects retry or cancel
    /// - (3) if user selected retry the initialize method is sent again.
    pub retry: bool,
}

// The server can signal the following capabilities:

/// Defines how the host (editor) should sync document changes to the language server.
#[derive(Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TextDocumentSyncKind(i32);

lsp_enum! {
    impl TextDocumentSyncKind {
        /// Documents should not be synced at all.
        const NONE = 0;
        /// Documents are synced by always sending the full content of the document.
        const FULL = 1;
        /// Documents are synced by sending the full content on open. After that only
        /// incremental updates to the document are sent.
        const INCREMENTAL = 2;
    }
}

pub type ExecuteCommandClientCapabilities = DynamicRegistrationClientCapabilities;

/// Execute command options.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ExecuteCommandOptions {
    /// The commands to be executed on the server
    pub commands: Vec<String>,

    #[serde(flatten)]
    pub work_done_progress_options: WorkDoneProgressOptions,
}

/// Save options.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveOptions {
    /// The client is supposed to include the content on save.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_text: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TextDocumentSyncSaveOptions {
    Supported(bool),
    SaveOptions(SaveOptions),
}

impl From<SaveOptions> for TextDocumentSyncSaveOptions {
    fn from(from: SaveOptions) -> Self {
        Self::SaveOptions(from)
    }
}

impl From<bool> for TextDocumentSyncSaveOptions {
    fn from(from: bool) -> Self {
        Self::Supported(from)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentSyncOptions {
    /// Open and close notifications are sent to the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_close: Option<bool>,

    /// Change notifications are sent to the server. See TextDocumentSyncKind.None, TextDocumentSyncKind.Full
    /// and TextDocumentSyncKind.Incremental.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change: Option<TextDocumentSyncKind>,

    /// Will save notifications are sent to the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub will_save: Option<bool>,

    /// Will save wait until requests are sent to the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub will_save_wait_until: Option<bool>,

    /// Save notifications are sent to the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save: Option<TextDocumentSyncSaveOptions>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OneOf<A, B> {
    Left(A),
    Right(B),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TextDocumentSyncCapability {
    Kind(TextDocumentSyncKind),
    Options(TextDocumentSyncOptions),
}

impl From<TextDocumentSyncOptions> for TextDocumentSyncCapability {
    fn from(from: TextDocumentSyncOptions) -> Self {
        Self::Options(from)
    }
}

impl From<TextDocumentSyncKind> for TextDocumentSyncCapability {
    fn from(from: TextDocumentSyncKind) -> Self {
        Self::Kind(from)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ImplementationProviderCapability {
    Simple(bool),
    Options(StaticTextDocumentRegistrationOptions),
}

impl From<StaticTextDocumentRegistrationOptions> for ImplementationProviderCapability {
    fn from(from: StaticTextDocumentRegistrationOptions) -> Self {
        Self::Options(from)
    }
}

impl From<bool> for ImplementationProviderCapability {
    fn from(from: bool) -> Self {
        Self::Simple(from)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TypeDefinitionProviderCapability {
    Simple(bool),
    Options(StaticTextDocumentRegistrationOptions),
}

impl From<StaticTextDocumentRegistrationOptions> for TypeDefinitionProviderCapability {
    fn from(from: StaticTextDocumentRegistrationOptions) -> Self {
        Self::Options(from)
    }
}

impl From<bool> for TypeDefinitionProviderCapability {
    fn from(from: bool) -> Self {
        Self::Simple(from)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerCapabilities {
    /// The position encoding the server picked from the encodings offered
    /// by the client via the client capability `general.positionEncodings`.
    ///
    /// If the client didn't provide any position encodings the only valid
    /// value that a server can return is `utf-16`.
    ///
    /// If omitted it defaults to `utf-16`.
    ///
    /// @since 3.17.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_encoding: Option<PositionEncodingKind>,

    /// Defines how text documents are synced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_document_sync: Option<TextDocumentSyncCapability>,

    /// Defines how notebook documents are synced.
    ///
    /// @since 3.17.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_document_sync:
        Option<OneOf<NotebookDocumentSyncOptions, NotebookDocumentSyncRegistrationOptions>>,

    /// Capabilities specific to `textDocument/selectionRange` requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_range_provider: Option<SelectionRangeProviderCapability>,

    /// The server provides hover support.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_provider: Option<HoverProviderCapability>,

    /// The server provides completion support.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_provider: Option<CompletionOptions>,

    /// The server provides signature help support.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_help_provider: Option<SignatureHelpOptions>,

    /// The server provides goto definition support.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition_provider: Option<OneOf<bool, DefinitionOptions>>,

    /// The server provides goto type definition support.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_definition_provider: Option<TypeDefinitionProviderCapability>,

    /// The server provides goto implementation support.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implementation_provider: Option<ImplementationProviderCapability>,

    /// The server provides find references support.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references_provider: Option<OneOf<bool, ReferenceOptions>>,

    /// The server provides document highlight support.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_highlight_provider: Option<OneOf<bool, DocumentHighlightOptions>>,

    /// The server provides document symbol support.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_symbol_provider: Option<OneOf<bool, DocumentSymbolOptions>>,

    /// The server provides workspace symbol support.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_symbol_provider: Option<OneOf<bool, WorkspaceSymbolOptions>>,

    /// The server provides code actions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_action_provider: Option<CodeActionProviderCapability>,

    /// The server provides code lens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_lens_provider: Option<CodeLensOptions>,

    /// The server provides document formatting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_formatting_provider: Option<OneOf<bool, DocumentFormattingOptions>>,

    /// The server provides document range formatting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_range_formatting_provider: Option<OneOf<bool, DocumentRangeFormattingOptions>>,

    /// The server provides document formatting on typing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_on_type_formatting_provider: Option<DocumentOnTypeFormattingOptions>,

    /// The server provides rename support.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rename_provider: Option<OneOf<bool, RenameOptions>>,

    /// The server provides document link support.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_link_provider: Option<DocumentLinkOptions>,

    /// The server provides color provider support.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_provider: Option<ColorProviderCapability>,

    /// The server provides folding provider support.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folding_range_provider: Option<FoldingRangeProviderCapability>,

    /// The server provides go to declaration support.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declaration_provider: Option<DeclarationCapability>,

    /// The server provides execute command support.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute_command_provider: Option<ExecuteCommandOptions>,

    /// Workspace specific server capabilities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<WorkspaceServerCapabilities>,

    /// Call hierarchy provider capabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_hierarchy_provider: Option<CallHierarchyServerCapability>,

    /// Semantic tokens server capabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_tokens_provider: Option<SemanticTokensServerCapabilities>,

    /// Whether server provides moniker support.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moniker_provider: Option<OneOf<bool, MonikerServerCapabilities>>,

    /// The server provides linked editing range support.
    ///
    /// @since 3.16.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_editing_range_provider: Option<LinkedEditingRangeServerCapabilities>,

    /// The server provides inline values.
    ///
    /// @since 3.17.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_value_provider: Option<OneOf<bool, InlineValueServerCapabilities>>,

    /// The server provides inlay hints.
    ///
    /// @since 3.17.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inlay_hint_provider: Option<OneOf<bool, InlayHintServerCapabilities>>,

    /// The server has support for pull model diagnostics.
    ///
    /// @since 3.17.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostic_provider: Option<DiagnosticServerCapabilities>,

    /// The server provides inline completions.
    ///
    /// @since 3.18.0
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg(feature = "proposed")]
    pub inline_completion_provider: Option<OneOf<bool, InlineCompletionOptions>>,

    /// Experimental server capabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<serde_json::Value>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceServerCapabilities {
    /// The server supports workspace folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_folders: Option<WorkspaceFoldersServerCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_operations: Option<WorkspaceFileOperationsServerCapabilities>,
}

/// General parameters to to register for a capability.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Registration {
    /// The id used to register the request. The id can be used to deregister
    /// the request again.
    pub id: String,

    /// The method / capability to register for.
    pub method: String,

    /// Options necessary for the registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register_options: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RegistrationParams {
    pub registrations: Vec<Registration>,
}

/// Since most of the registration options require to specify a document selector there is a base
/// interface that can be used.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentRegistrationOptions {
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DeclarationCapability {
    Simple(bool),
    RegistrationOptions(DeclarationRegistrationOptions),
    Options(DeclarationOptions),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeclarationRegistrationOptions {
    #[serde(flatten)]
    pub declaration_options: DeclarationOptions,

    #[serde(flatten)]
    pub text_document_registration_options: TextDocumentRegistrationOptions,

    #[serde(flatten)]
    pub static_registration_options: StaticRegistrationOptions,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeclarationOptions {
    #[serde(flatten)]
    pub work_done_progress_options: WorkDoneProgressOptions,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StaticRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentFormattingOptions {
    #[serde(flatten)]
    pub work_done_progress_options: WorkDoneProgressOptions,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentRangeFormattingOptions {
    #[serde(flatten)]
    pub work_done_progress_options: WorkDoneProgressOptions,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DefinitionOptions {
    #[serde(flatten)]
    pub work_done_progress_options: WorkDoneProgressOptions,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSymbolOptions {
    /// A human-readable string that is shown when multiple outlines trees are
    /// shown for the same document.
    ///
    /// @since 3.16.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    #[serde(flatten)]
    pub work_done_progress_options: WorkDoneProgressOptions,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSymbolRegistrationOptions {
    #[serde(flatten)]
    text_document_registration_options: TextDocumentRegistrationOptions,

    #[serde(flatten)]
    document_symbol_options: DocumentSymbolOptions,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceOptions {
    #[serde(flatten)]
    pub work_done_progress_options: WorkDoneProgressOptions,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentHighlightOptions {
    #[serde(flatten)]
    pub work_done_progress_options: WorkDoneProgressOptions,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceSymbolOptions {
    #[serde(flatten)]
    pub work_done_progress_options: WorkDoneProgressOptions,

    /// The server provides support to resolve additional
    /// information for a workspace symbol.
    ///
    /// @since 3.17.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve_provider: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StaticTextDocumentRegistrationOptions {
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// General parameters to unregister a capability.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Unregistration {
    /// The id used to unregister the request or notification. Usually an id
    /// provided during the register request.
    pub id: String,

    /// The method / capability to unregister for.
    pub method: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UnregistrationParams {
    pub unregisterations: Vec<Unregistration>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct DidChangeConfigurationParams {
    /// The actual changed settings
    pub settings: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DidOpenTextDocumentParams {
    /// The document that was opened.
    pub text_document: TextDocumentItem,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeTextDocumentParams {
    /// The document that did change. The version number points
    /// to the version after all provided content changes have
    /// been applied.
    pub text_document: VersionedTextDocumentIdentifier,
    /// The actual content changes.
    pub content_changes: Vec<TextDocumentContentChangeEvent>,
}

/// An event describing a change to a text document. If range and rangeLength are omitted
/// the new text is considered to be the full content of the document.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentContentChangeEvent {
    /// The range of the document that changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<Range>,

    /// The length of the range that got replaced.
    ///
    /// Deprecated: Use range instead
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_length: Option<u32>,

    /// The new text of the document.
    pub text: String,
}

/// Describe options to be used when registering for text document change events.
///
/// Extends `TextDocumentRegistrationOptions`
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentChangeRegistrationOptions {
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    /// How documents are synced to the server. See TextDocumentSyncKind.Full
    /// and TextDocumentSyncKind.Incremental.
    pub sync_kind: TextDocumentSyncKind,
}

/// The parameters send in a will save text document notification.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WillSaveTextDocumentParams {
    /// The document that will be saved.
    pub text_document: TextDocumentIdentifier,

    /// The [`TextDocumentSaveReason`].
    pub reason: TextDocumentSaveReason,
}

/// Represents reasons why a text document is saved.
#[derive(Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TextDocumentSaveReason(i32);

lsp_enum! {
    impl TextDocumentSaveReason {
        /// Manually triggered, e.g. by the user pressing save, by starting debugging,
        /// or by an API call.
        const MANUAL = 1;
        /// Automatic after a delay.
        const AFTER_DELAY = 2;
        /// When the editor lost focus.
        const FOCUS_OUT = 3;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DidCloseTextDocumentParams {
    /// The document that was closed.
    pub text_document: TextDocumentIdentifier,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DidSaveTextDocumentParams {
    /// The document that was saved.
    pub text_document: TextDocumentIdentifier,

    /// Optional the content when saved. Depends on the includeText value
    /// when the save notification was requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentSaveRegistrationOptions {
    /// The client is supposed to include the content on save.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_text: Option<bool>,

    #[serde(flatten)]
    pub text_document_registration_options: TextDocumentRegistrationOptions,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeWatchedFilesClientCapabilities {
    /// Did change watched files notification supports dynamic registration.
    /// Please note that the current protocol doesn't support static
    /// configuration for file changes from the server side.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_registration: Option<bool>,

    /// Whether the client has support for relative patterns
    /// or not.
    ///
    /// @since 3.17.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_pattern_support: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct DidChangeWatchedFilesParams {
    /// The actual file events.
    pub changes: Vec<FileEvent>,
}

/// The file event type.
#[derive(Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Hash)]
#[serde(transparent)]
pub struct FileChangeType(i32);

lsp_enum! {
    impl FileChangeType {
        /// The file got created.
        const CREATED = 1;
        /// The file got changed.
        const CHANGED = 2;
        /// The file got deleted.
        const DELETED = 3;
    }
}

/// An event describing a file change.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct FileEvent {
    /// The file's URI.
    pub uri: Uri,

    /// The change type.
    #[serde(rename = "type")]
    pub typ: FileChangeType,
}

impl FileEvent {
    #[must_use]
    pub const fn new(uri: Uri, typ: FileChangeType) -> Self {
        Self { uri, typ }
    }
}

/// Describe options to be used when registered for text document change events.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct DidChangeWatchedFilesRegistrationOptions {
    /// The watchers to register.
    pub watchers: Vec<FileSystemWatcher>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileSystemWatcher {
    /// The glob pattern to watch. See {@link `GlobPattern` glob pattern}
    /// for more detail.
    ///
    /// @since 3.17.0 support for relative patterns.
    pub glob_pattern: GlobPattern,

    /// The kind of events of interest. If omitted it defaults to WatchKind.Create |
    /// WatchKind.Change | WatchKind.Delete which is 7.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<WatchKind>,
}

/// The glob pattern. Either a string pattern or a relative pattern.
///
/// @since 3.17.0
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(untagged)]
pub enum GlobPattern {
    String(Pattern),
    Relative(RelativePattern),
}

impl From<Pattern> for GlobPattern {
    #[inline]
    fn from(from: Pattern) -> Self {
        Self::String(from)
    }
}

impl From<RelativePattern> for GlobPattern {
    #[inline]
    fn from(from: RelativePattern) -> Self {
        Self::Relative(from)
    }
}

/// A relative pattern is a helper to construct glob patterns that are matched
/// relatively to a base URI. The common value for a `baseUri` is a workspace
/// folder root, but it can be another absolute URI as well.
///
/// @since 3.17.0
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RelativePattern {
    /// A workspace folder or a base URI to which this pattern will be matched
    /// against relatively.
    pub base_uri: OneOf<WorkspaceFolder, Uri>,

    /// The actual glob pattern.
    pub pattern: Pattern,
}

/// The glob pattern to watch relative to the base path. Glob patterns can have
/// the following syntax:
/// - `*` to match one or more characters in a path segment
/// - `?` to match on one character in a path segment
/// - `**` to match any number of path segments, including none
/// - `{}` to group conditions (e.g. `**​/*.{ts,js}` matches all TypeScript
///   and JavaScript files)
/// - `[]` to declare a range of characters to match in a path segment
///   (e.g., `example.[0-9]` to match on `example.0`, `example.1`, …)
/// - `[!...]` to negate a range of characters to match in a path segment
///   (e.g., `example.[!0-9]` to match on `example.a`, `example.b`,
///   but not `example.0`)
///
/// @since 3.17.0
pub type Pattern = String;

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct WatchKind: u8 {
        /// Interested in create events.
        const Create = 1;
        /// Interested in change events
        const Change = 2;
        /// Interested in delete events
        const Delete = 4;
    }
}

impl<'de> serde::Deserialize<'de> for WatchKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let i = u8::deserialize(deserializer)?;
        Self::from_bits(i).ok_or_else(|| {
            D::Error::invalid_value(
                serde::de::Unexpected::Unsigned(u64::from(i)),
                &"Unknown flag",
            )
        })
    }
}

impl serde::Serialize for WatchKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(self.bits())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct PublishDiagnosticsParams {
    /// The URI for which diagnostic information is reported.
    pub uri: Uri,

    /// An array of diagnostic information items.
    pub diagnostics: Vec<Diagnostic>,

    /// Optional the version number of the document the diagnostics are published for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

impl PublishDiagnosticsParams {
    #[must_use]
    pub const fn new(uri: Uri, diagnostics: Vec<Diagnostic>, version: Option<i32>) -> Self {
        Self {
            uri,
            diagnostics,
            version,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Documentation {
    String(String),
    MarkupContent(MarkupContent),
}

/// `MarkedString` can be used to render human readable text. It is either a
/// markdown string or a code-block that provides a language and a code snippet.
/// The language identifier is semantically equal to the optional language
/// identifier in fenced code blocks in GitHub issues.
///
/// The pair of a language and a value is an equivalent to markdown:
///
/// ```LANGUAGE
/// VALUE
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MarkedString {
    String(String),
    LanguageString(LanguageString),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct LanguageString {
    pub language: String,
    pub value: String,
}

impl MarkedString {
    #[must_use]
    pub const fn from_markdown(markdown: String) -> Self {
        Self::String(markdown)
    }

    #[must_use]
    pub const fn from_language_code(language: String, code_block: String) -> Self {
        Self::LanguageString(LanguageString {
            language,
            value: code_block,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GotoDefinitionParams {
    #[serde(flatten)]
    pub text_document_position_params: TextDocumentPositionParams,

    #[serde(flatten)]
    pub work_done_progress_params: WorkDoneProgressParams,

    #[serde(flatten)]
    pub partial_result_params: PartialResultParams,
}

/// `GotoDefinition` response can be single location, or multiple Locations or a link.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum GotoDefinitionResponse {
    Scalar(Location),
    Array(Vec<Location>),
    Link(Vec<LocationLink>),
}

impl From<Location> for GotoDefinitionResponse {
    fn from(location: Location) -> Self {
        Self::Scalar(location)
    }
}

impl From<Vec<Location>> for GotoDefinitionResponse {
    fn from(locations: Vec<Location>) -> Self {
        Self::Array(locations)
    }
}

impl From<Vec<LocationLink>> for GotoDefinitionResponse {
    fn from(locations: Vec<LocationLink>) -> Self {
        Self::Link(locations)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ExecuteCommandParams {
    /// The identifier of the actual command handler.
    pub command: String,
    /// Arguments that the command should be invoked with.
    #[serde(default)]
    pub arguments: Vec<serde_json::Value>,

    #[serde(flatten)]
    pub work_done_progress_params: WorkDoneProgressParams,
}

/// Execute command registration options.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ExecuteCommandRegistrationOptions {
    /// The commands to be executed on the server
    pub commands: Vec<String>,

    #[serde(flatten)]
    pub execute_command_options: ExecuteCommandOptions,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplyWorkspaceEditParams {
    /// An optional label of the workspace edit. This label is
    /// presented in the user interface for example on an undo
    /// stack to undo the workspace edit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// The edits to apply.
    pub edit: WorkspaceEdit,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplyWorkspaceEditResponse {
    /// Indicates whether the edit was applied or not.
    pub applied: bool,

    /// An optional textual description for why the edit was not applied.
    /// This may be used may be used by the server for diagnostic
    /// logging or to provide a suitable error for a request that
    /// triggered the edit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,

    /// Depending on the client's failure handling strategy `failedChange` might
    /// contain the index of the change that failed. This property is only available
    /// if the client signals a `failureHandlingStrategy` in its client capabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_change: Option<u32>,
}

/// Describes the content type that a client supports in various
/// result literals like `Hover`, `ParameterInfo` or `CompletionItem`.
///
/// Please note that `MarkupKinds` must not start with a `$`. This kinds
/// are reserved for internal usage.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MarkupKind {
    /// Plain text is supported as a content format
    PlainText,
    /// Markdown is supported as a content format
    Markdown,
}

/// A `MarkupContent` literal represents a string value which content can be represented in different formats.
///
/// Currently `plaintext` and `markdown` are supported formats. A `MarkupContent` is usually used in
/// documentation properties of result literals like `CompletionItem` or `SignatureInformation`.
/// If the format is `markdown` the content should follow the [GitHub Flavored Markdown Specification](https://github.github.com/gfm/).
///
/// Here is an example how such a string can be constructed using JavaScript / TypeScript:
///
/// ```ignore
/// let markdown: MarkupContent = {
///     kind: MarkupKind::Markdown,
///     value: [
///         "# Header",
///         "Some text",
///         "```typescript",
///         "someCode();",
///         "```"
///     ]
///     .join("\n"),
/// };
/// ```
///
/// Please *Note* that clients might sanitize the return markdown. A client could decide to
/// remove HTML from the markdown to avoid script execution.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct MarkupContent {
    pub kind: MarkupKind,
    pub value: String,
}

/// A parameter literal used to pass a partial result token.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PartialResultParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_result_token: Option<ProgressToken>,
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tests::test_serialization;

    #[test]
    fn one_of() {
        test_serialization(&OneOf::<bool, ()>::Left(true), r"true");
        test_serialization(&OneOf::<String, ()>::Left("abcd".into()), r#""abcd""#);
        test_serialization(
            &OneOf::<String, WorkDoneProgressOptions>::Right(WorkDoneProgressOptions {
                work_done_progress: Some(false),
            }),
            r#"{"workDoneProgress":false}"#,
        );
    }

    #[test]
    fn number_or_string() {
        test_serialization(&NumberOrString::Number(123), r"123");

        test_serialization(&NumberOrString::String("abcd".into()), r#""abcd""#);
    }

    #[test]
    fn marked_string() {
        test_serialization(&MarkedString::from_markdown("xxx".into()), r#""xxx""#);

        test_serialization(
            &MarkedString::from_language_code("lang".into(), "code".into()),
            r#"{"language":"lang","value":"code"}"#,
        );
    }

    #[test]
    fn language_string() {
        test_serialization(
            &LanguageString {
                language: "LL".into(),
                value: "VV".into(),
            },
            r#"{"language":"LL","value":"VV"}"#,
        );
    }

    #[test]
    fn workspace_edit() {
        test_serialization(
            &WorkspaceEdit {
                changes: Some(vec![].into_iter().collect()),
                document_changes: None,
                ..Default::default()
            },
            r#"{"changes":{}}"#,
        );

        test_serialization(
            &WorkspaceEdit {
                changes: None,
                document_changes: None,
                ..Default::default()
            },
            r"{}",
        );

        test_serialization(
            &WorkspaceEdit {
                changes: Some(
                    vec![("file://test".parse().unwrap(), vec![])]
                        .into_iter()
                        .collect(),
                ),
                document_changes: None,
                ..Default::default()
            },
            r#"{"changes":{"file://test":[]}}"#,
        );
    }

    #[test]
    fn root_uri_can_be_missing() {
        serde_json::from_str::<InitializeParams>(r#"{ "capabilities": {} }"#).unwrap();
    }

    #[test]
    fn test_watch_kind() {
        test_serialization(&WatchKind::Create, "1");
        test_serialization(&(WatchKind::Create | WatchKind::Change), "3");
        test_serialization(
            &(WatchKind::Create | WatchKind::Change | WatchKind::Delete),
            "7",
        );
    }

    #[test]
    fn test_resource_operation_kind() {
        test_serialization(
            &vec![
                ResourceOperationKind::Create,
                ResourceOperationKind::Rename,
                ResourceOperationKind::Delete,
            ],
            r#"["create","rename","delete"]"#,
        );
    }
}

// A number of messages include an id/token that is either a number or a string.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
#[serde(untagged)]
pub enum NumberOrString {
    Number(i32),
    String(String),
}

/// Position in a text document expressed as zero-based line and character offset.
/// A position is between two characters like an `insert` cursor in a editor.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize,
)]
pub struct Position {
    /// Line position in a document (zero-based).
    pub line: u32,
    /// Character offset on a line in a document (zero-based). The meaning of this
    /// offset is determined by the negotiated `PositionEncodingKind`.
    ///
    /// If the character value is greater than the line length it defaults back
    /// to the line length.
    pub character: u32,
}

impl Position {
    #[must_use]
    pub const fn new(line: u32, character: u32) -> Self {
        Self { line, character }
    }
}

/// A range in a text document expressed as (zero-based) start and end positions.
/// A range is comparable to a selection in an editor. Therefore the end position is exclusive.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct Range {
    /// The range's start position.
    pub start: Position,
    /// The range's end position.
    pub end: Position,
}

impl Range {
    #[must_use]
    pub const fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}

/// Represents a location inside a resource, such as a line inside a text file.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct Location {
    pub uri: Uri,
    pub range: Range,
}

impl Location {
    #[must_use]
    pub const fn new(uri: Uri, range: Range) -> Self {
        Self { uri, range }
    }
}

/// Represents programming constructs like variables, classes, interfaces etc.
///
/// that appear in a document. Document symbols can be hierarchical and they have two ranges:
/// one that encloses its definition and one that points to its most interesting range,
/// e.g. the range of an identifier.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSymbol {
    /// The name of this symbol.
    pub name: String,
    /// More detail for this symbol, e.g the signature of a function. If not provided the
    /// name is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// The kind of this symbol.
    pub kind: SymbolKind,
    /// Tags for this completion item.
    ///
    /// @since 3.15.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<SymbolTag>>,
    /// Indicates if this symbol is deprecated.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[deprecated(note = "Use tags instead")]
    pub deprecated: Option<bool>,
    /// The range enclosing this symbol not including leading/trailing whitespace but everything else
    /// like comments. This information is typically used to determine if the the clients cursor is
    /// inside the symbol to reveal in the symbol in the UI.
    pub range: Range,
    /// The range that should be selected and revealed when this symbol is being picked, e.g the name of a function.
    /// Must be contained by the the `range`.
    pub selection_range: Range,
    /// Children of this symbol, e.g. properties of a class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<DocumentSymbol>>,
}

/// A symbol kind.
#[derive(Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SymbolKind(i32);

lsp_enum! {
    impl SymbolKind {
        const FILE = 1;
        const MODULE = 2;
        const NAMESPACE = 3;
        const PACKAGE = 4;
        const CLASS = 5;
        const METHOD = 6;
        const PROPERTY = 7;
        const FIELD = 8;
        const CONSTRUCTOR = 9;
        const ENUM = 10;
        const INTERFACE = 11;
        const FUNCTION = 12;
        const VARIABLE = 13;
        const CONSTANT = 14;
        const STRING = 15;
        const NUMBER = 16;
        const BOOLEAN = 17;
        const ARRAY = 18;
        const OBJECT = 19;
        const KEY = 20;
        const NULL = 21;
        const ENUM_MEMBER = 22;
        const STRUCT = 23;
        const EVENT = 24;
        const OPERATOR = 25;
        const TYPE_PARAMETER = 26;
    }
}

/// Symbol tags are extra annotations that tweak the rendering of a symbol.
///
/// @since 3.16.0
#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SymbolTag(i32);

lsp_enum! {
    impl SymbolTag {
        /// Render a symbol as obsolete, usually using a strike-out.
        const DEPRECATED = 1;
    }
}
