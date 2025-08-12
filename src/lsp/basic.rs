//! Corresponds to the [Basic JSON Structure] section of the specification.
//!
//! [Basic JSON Structure]: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#basicJsonStructures

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    Uri,
    lsp::{DocumentChanges, NumberOrString, OneOf, WorkspaceFolder},
    macros::lsp_enum,
};

// URI
// See module `ls_types::uri`.

// Regular Expression

/// Client capabilities specific to regular expressions.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegularExpressionsClientCapabilities {
    /// The engine's name.
    pub engine: String,

    /// The engine's version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

// Text Documents

// Position

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

// Range

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

// Text Document Item

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

// Text Document Identifier

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

// Versionned Text Document Identifier

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

// Text Document Position Params

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

// Patterns

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

// Document Filter

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

// String Value

#[cfg(feature = "proposed")]
/// A string value used as a snippet is a template which allows to insert text
/// and to control the editor cursor when insertion happens.
///
/// A snippet can define tab stops and placeholders with `$1`, `$2`
/// and `${3:foo}`. `$0` defines the final tab stop, it defaults to
/// the end of the snippet. Variables are defined with `$name` and
/// `${name:default value}`.
///
/// @since 3.18.0
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind")]
pub enum StringValue {
    Snippet(String),
}

// Text Edit

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

#[cfg(feature = "proposed")]
/// An interactive text edit.
///
/// @since 3.18.0
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnippetTextEdit {
    /// The range of the text document to be manipulated.
    pub range: Range,
    /// The snippet to be inserted.
    pub snippet: StringValue,
    /// The actual identifier of the snippet edit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_id: Option<ChangeAnnotationIdentifier>,
}

// Text Edit Array

// Text Document Edit

/// Describes textual changes on a single text document.
///
/// The text document is referred to as an `OptionalVersionedTextDocumentIdentifier`
/// to allow clients to check the text document version before an edit is
/// applied. A `TextDocumentEdit` describes all changes on a version Si and
/// after they are applied move the document to version Si+1. So, the creator
/// of a `TextDocumentEdit` doesn't need to sort the array or do any kind of
/// ordering. However, the edits must be non overlapping.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentEdit {
    /// The text document to change.
    pub text_document: OptionalVersionedTextDocumentIdentifier,

    #[cfg(not(feature = "proposed"))]
    /// The edits to be applied.
    ///
    /// @since 3.16.0 - support for `AnnotatedTextEdit`. This is guarded by the
    /// client capability `workspace.workspaceEdit.changeAnnotationSupport`
    pub edits: Vec<OneOf<TextEdit, AnnotatedTextEdit>>,

    #[cfg(feature = "proposed")]
    /// The edits to be applied.
    ///
    /// @since 3.16.0 - support for `AnnotatedTextEdit`. This is guarded by the
    /// client capability `workspace.workspaceEdit.changeAnnotationSupport`
    ///
    /// @since 3.18.0 - support for `SnippetTextEdit`. This is guarded by the
    /// client capability `workspace.workspaceEdit.snippetEditSupport`
    // TODO: refactor to enum
    pub edits: Vec<OneOf<TextEdit, OneOf<AnnotatedTextEdit, SnippetTextEdit>>>,
}

// Location

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

// Location Link

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

// Diagnostic

/// Represents a diagnostic, such as a compiler error or warning.
/// Diagnostic objects are only valid in the scope of a resource.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
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

    #[cfg(not(feature = "proposed"))]
    /// The diagnostic's message.
    pub message: String,

    #[cfg(feature = "proposed")]
    /// The diagnostic's message.
    ///
    /// @since 3.18.0 - support for `MarkupContent`. This is guarded by the client
    /// capability `textDocument.diagnostic.markupMessageSupport`.
    pub message: OneOf<String, MarkupContent>,

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

impl Diagnostic {
    #[must_use]
    pub const fn new(
        range: Range,
        severity: Option<DiagnosticSeverity>,
        code: Option<NumberOrString>,
        source: Option<String>,
        #[cfg(not(feature = "proposed"))] message: String,
        #[cfg(feature = "proposed")] message: OneOf<String, MarkupContent>,
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
            code_description: None,
            data: None,
        }
    }

    #[must_use]
    pub const fn new_simple(range: Range, message: String) -> Self {
        #[cfg(not(feature = "proposed"))]
        {
            Self::new(range, None, None, None, message, None, None)
        }
        #[cfg(feature = "proposed")]
        {
            Self::new(range, None, None, None, OneOf::Left(message), None, None)
        }
    }

    #[must_use]
    pub const fn new_with_code_number(
        range: Range,
        severity: DiagnosticSeverity,
        code_number: i32,
        source: Option<String>,
        message: String,
    ) -> Self {
        let code = Some(NumberOrString::Number(code_number));
        #[cfg(not(feature = "proposed"))]
        {
            Self::new(range, Some(severity), code, source, message, None, None)
        }
        #[cfg(feature = "proposed")]
        {
            Self::new(
                range,
                Some(severity),
                code,
                source,
                OneOf::Left(message),
                None,
                None,
            )
        }
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
        /// Reports information.
        const INFORMATION = 3;
        /// Reports a hint.
        const HINT = 4;
    }
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
        /// Clients are allowed to render diagnostics with this tag strike through.
        const DEPRECATED = 2;
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

/// Structure to capture a description for an error code.
///
/// @since 3.16.0
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeDescription {
    /// A URI to open with more information about the diagnostic error.
    pub href: Uri,
}

// Command

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

// MarkupContent

/// Describes the content type that a client supports in various
/// result literals like `Hover`, `ParameterInfo` or `CompletionItem`.
///
/// Please note that `MarkupKinds` must not start with a `$`. These kinds
/// are reserved for internal usage.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MarkupKind {
    /// Plain text is supported as a content format.
    PlainText,
    /// Markdown is supported as a content format.
    Markdown,
}

/// A `MarkupContent` literal represents a string value whose content can be
/// represented in different formats.
///
/// Currently `plaintext` and `markdown` are supported formats. A
/// `MarkupContent` is usually used in documentation properties of result
/// literals like `CompletionItem` or `SignatureInformation`. If the format
/// is `markdown` the content should follow the [GitHub Flavored Markdown Specification](https://github.github.com/gfm/).
///
/// Here is an example how such a string can be constructed using JavaScript / TypeScript:
///
/// ```typescript
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
/// Please *Note* that clients might sanitize the returned markdown. A client
/// could decide to remove HTML from the markdown to avoid script execution.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct MarkupContent {
    /// The type of the Markup.
    pub kind: MarkupKind,
    /// The content itself
    pub value: String,
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

// File Resource changes

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

    /// An optional annotation identifier describing the operation.
    ///
    /// @since 3.16.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_id: Option<ChangeAnnotationIdentifier>,
}

// WorkspaceEdit

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

    /// Whether the client supports `WorkspaceEditMetadata` in `WorkspaceEdit`s.
    ///
    /// @since 3.18.0
    /// @proposed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_support: Option<bool>,

    /// Whether the client supports snippets as text edits.
    ///
    /// @since 3.18.0
    /// @proposed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippet_edit_support: Option<bool>,
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

/// The kind of resource operations supported by the client.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ResourceOperationKind {
    /// Supports creating new files and folders.
    Create,
    /// Supports renaming existing files and folders.
    Rename,
    /// Supports deleting existing files and folders.
    Delete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FailureHandlingKind {
    /// Applying the workspace change is simply aborted if one of the changes
    /// provided fails. All operations executed before the failing operation
    /// stay executed.
    Abort,
    /// All operations are executed transactionally. That means they either all
    /// succeed or no changes at all are applied to the workspace.
    Transactional,
    /// If the workspace edit contains only textual file changes they are
    /// executed transactionally. If resource changes (create, rename or delete
    /// file) are part of the change the failure handling strategy is abort.
    TextOnlyTransactional,
    /// The client tries to undo the operations already executed. But there is
    /// no guarantee that this is succeeding.
    Undo,
}

// Work Done Progress
// Client Initiated Progress
// Server Initiated Progress
// Partial Result Progress
// Partial Result Params
//
// See module `ls_types::lsp::progress`.

// Trace Value

/// A `TraceValue` represents the level of verbosity with which the server systematically
/// reports its execution trace using `LogTrace` notifications.
///
/// The initial trace value is set by the client at initialization and can be modified
/// later using the `SetTrace` notification.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TraceValue {
    /// The server should not send any `$/logTrace` notification
    #[default]
    Off,
    /// The server should not add the 'verbose' field in the `LogTraceParams`
    Messages,
    Verbose,
}
