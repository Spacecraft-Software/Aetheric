// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

use crate::core_capnp::core_command;

/// Owned representation of a command sent from Majestic to RMS.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreCommand {
    /// Open a file buffer.
    OpenBuffer(String),
    /// Insert text at a byte offset.
    InsertText {
        buffer_id: u32,
        byte_offset: u64,
        content: String,
    },
    /// Delete text at a byte offset.
    DeleteText {
        buffer_id: u32,
        byte_offset: u64,
        length: u64,
    },
    /// Request a snapshot of a buffer.
    RequestSnapshot(u32),
    /// Release a previously acquired snapshot.
    ReleaseSnapshot(u32),
    /// Close a buffer.
    CloseBuffer(u32),
    /// Set diagnostics for a buffer.
    SetDiagnostics(Vec<DiagnosticAnnotation>),
    /// Update the viewport.
    SetViewport {
        buffer_id: u32,
        start_line: u64,
        line_count: u32,
    },
    /// Toggle the debug overlay.
    ToggleOverlay,
    /// Shut down the microkernel.
    Shutdown,
}

/// A single diagnostic annotation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticAnnotation {
    pub buffer_id: u32,
    pub start_byte: u64,
    pub end_byte: u64,
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub source: String,
    pub code: String,
}

/// Severity level of a diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Information,
    Hint,
}

/// Orchestration command (Majestic → RMS, heavy ops).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrchCommand {
    /// Request Celestial export.
    CelestialExport {
        buffer_id: u32,
        format: ExportFormat,
        output_path: String,
    },
    /// Query Celestial agenda.
    CelestialAgendaQuery {
        range_start_iso: String,
        range_end_iso: String,
        tags: Vec<String>,
    },
    /// Request Nexus status.
    NexusStatusRequest(u32),
    /// Request Nexus commit log.
    NexusCommitLogRequest(u32),
    /// Stage a hunk.
    NexusStageHunk { path: String, hunk_index: u32 },
    /// Unstage a hunk.
    NexusUnstageHunk { path: String, hunk_index: u32 },
    /// Commit with message.
    NexusCommit(String),
    /// Start AI stream.
    AiStreamRequest {
        request_id: u64,
        prompt: String,
        context_buf: u32,
    },
    /// Cancel AI stream.
    AiStreamCancel(u64),
}

/// Export format for Celestial.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Pdf,
    Html,
    Markdown,
    Latex,
}

/// Orchestration event (RMS → Majestic, result of heavy ops).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrchEvent {
    /// Export completed.
    ExportDone { path: String, size_bytes: u64 },
    /// Agenda query result.
    AgendaResult(Vec<AgendaItem>),
    /// Nexus status result.
    NexusStatusResult(Vec<NexusStatusEntry>),
    /// Nexus commit log result.
    NexusCommitLogResult(Vec<NexusCommit>),
    /// Nexus diff result.
    NexusDiff(Vec<NexusHunk>),
    /// AI stream chunk.
    AiStreamChunk {
        request_id: u64,
        content: String,
        done: bool,
    },
    /// Orchestration error.
    OrchError {
        code: u32,
        message: String,
        cmd_id: u64,
    },
}

/// A single agenda item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgendaItem {
    pub heading: String,
    pub state: String,
    pub deadline: String,
    pub scheduled: String,
    pub tags: Vec<String>,
    pub priority: String,
    pub buffer_id: u32,
    pub byte_offset: u64,
}

/// A single Nexus status entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NexusStatusEntry {
    pub path: String,
    pub index_state: GitState,
    pub wd_state: GitState,
}

/// Git file state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitState {
    Unmodified,
    Modified,
    Added,
    Deleted,
    Renamed,
    Copied,
    Untracked,
    Ignored,
}

/// A single commit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NexusCommit {
    pub hash: String,
    pub short_hash: String,
    pub author: String,
    pub author_email: String,
    pub timestamp_z: String,
    pub message: String,
}

/// A diff hunk.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NexusHunk {
    pub old_start: u32,
    pub old_count: u32,
    pub new_start: u32,
    pub new_count: u32,
    pub header: String,
    pub lines: Vec<String>,
}

impl CoreCommand {
    /// Serialize into a Cap'n Proto builder.
    ///
    /// # Errors
    ///
    /// Returns an error if the message exceeds Cap'n Proto size limits.
    pub fn to_builder<'a>(
        &self,
        mut builder: core_command::Builder<'a>,
    ) -> Result<(), crate::IpcError> {
        match self {
            CoreCommand::OpenBuffer(path) => {
                builder.set_id(0);
                builder.set_open_buffer(path);
            }
            CoreCommand::InsertText {
                buffer_id,
                byte_offset,
                content,
            } => {
                builder.set_id(0);
                let mut p = builder.init_insert_text();
                p.set_buffer_id(*buffer_id);
                p.set_byte_offset(*byte_offset);
                p.set_content(content);
            }
            CoreCommand::DeleteText {
                buffer_id,
                byte_offset,
                length,
            } => {
                builder.set_id(0);
                let mut p = builder.init_delete_text();
                p.set_buffer_id(*buffer_id);
                p.set_byte_offset(*byte_offset);
                p.set_length(*length);
            }
            CoreCommand::RequestSnapshot(id) => {
                builder.set_id(0);
                builder.set_request_snapshot(*id);
            }
            CoreCommand::ReleaseSnapshot(id) => {
                builder.set_id(0);
                builder.set_release_snapshot(*id);
            }
            CoreCommand::CloseBuffer(id) => {
                builder.set_id(0);
                builder.set_close_buffer(*id);
            }
            CoreCommand::SetDiagnostics(diags) => {
                builder.set_id(0);
                let mut list = builder.init_set_diagnostics(diags.len() as u32);
                for (i, d) in diags.iter().enumerate() {
                    let mut item = list.reborrow().get(i as u32);
                    item.set_buffer_id(d.buffer_id);
                    item.set_start_byte(d.start_byte);
                    item.set_end_byte(d.end_byte);
                    item.set_severity(match d.severity {
                        DiagnosticSeverity::Error => crate::core_capnp::DiagnosticSeverity::Error,
                        DiagnosticSeverity::Warning => {
                            crate::core_capnp::DiagnosticSeverity::Warning
                        }
                        DiagnosticSeverity::Information => {
                            crate::core_capnp::DiagnosticSeverity::Information
                        }
                        DiagnosticSeverity::Hint => crate::core_capnp::DiagnosticSeverity::Hint,
                    });
                    item.set_message(&d.message);
                    item.set_source(&d.source);
                    item.set_code(&d.code);
                }
            }
            CoreCommand::SetViewport {
                buffer_id,
                start_line,
                line_count,
            } => {
                builder.set_id(0);
                let mut p = builder.init_set_viewport();
                p.set_buffer_id(*buffer_id);
                p.set_start_line(*start_line);
                p.set_line_count(*line_count);
            }
            CoreCommand::ToggleOverlay => {
                builder.set_id(0);
                builder.set_toggle_overlay(());
            }
            CoreCommand::Shutdown => {
                builder.set_id(0);
                builder.set_shutdown(());
            }
        }
        Ok(())
    }
}
