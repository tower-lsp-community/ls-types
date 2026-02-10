use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Default, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WorkDoneProgressBegin {
    /// Mandatory title of the progress operation. Used to briefly inform
    /// about the kind of operation being performed.
    /// Examples: "Indexing" or "Linking dependencies".
    pub title: String,

    /// Controls if a cancel button should show to allow the user to cancel the
    /// long running operation. Clients that don't support cancellation are allowed
    /// to ignore the setting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellable: Option<bool>,

    /// Optional, more detailed associated progress message. Contains
    /// complementary information to the `title`.
    ///
    /// Examples: "3/25 files", "project/src/module2", "`node_modules/some_dep`".
    /// If unset, the previous progress message (if any) is still valid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Optional progress percentage to display (value 100 is considered 100%).
    /// If not provided infinite progress is assumed and clients are allowed
    /// to ignore the `percentage` value in subsequent in report notifications.
    ///
    /// The value should be steadily rising. Clients are free to ignore values
    /// that are not following this rule. The value range is [0, 100]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<u32>,
}

#[derive(Debug, PartialEq, Eq, Default, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WorkDoneProgressReport {
    /// Controls if a cancel button should show to allow the user to cancel the
    /// long running operation. Clients that don't support cancellation are allowed
    /// to ignore the setting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellable: Option<bool>,

    /// Optional, more detailed associated progress message. Contains
    /// complementary information to the `title`.
    /// Examples: "3/25 files", "project/src/module2", "`node_modules/some_dep`".
    /// If unset, the previous progress message (if any) is still valid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Optional progress percentage to display (value 100 is considered 100%).
    /// If not provided infinite progress is assumed and clients are allowed
    /// to ignore the `percentage` value in subsequent in report notifications.
    ///
    /// The value should be steadily rising. Clients are free to ignore values
    /// that are not following this rule. The value range is [0, 100]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<u32>,
}

#[derive(Debug, PartialEq, Eq, Default, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WorkDoneProgressEnd {
    /// Optional, more detailed associated progress message. Contains
    /// complementary information to the `title`.
    /// Examples: "3/25 files", "project/src/module2", "`node_modules/some_dep`".
    /// If unset, the previous progress message (if any) is still valid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum WorkDoneProgress {
    Begin(WorkDoneProgressBegin),
    Report(WorkDoneProgressReport),
    End(WorkDoneProgressEnd),
}
