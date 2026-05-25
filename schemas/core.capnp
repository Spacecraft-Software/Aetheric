# SPDX-License-Identifier: AGPL-3.0-or-later
# Copyright (c) 2026 Mohamed Hammad & Spacecraft Software
# Constellation Core Schema v0.1 — ordinals are permanent, never reuse.
@0xf9f709274fc27fb6;

struct WindowDimensions { widthMm @0 :Float32; heightMm @1 :Float32; }
struct BufferHandle { bufferId @0 :UInt32; path @1 :Text; }
struct SnapshotHandle { snapshotId @0 :UInt32; bufferId @1 :UInt32; version @2 :UInt64; }
struct InsertPayload { bufferId @0 :UInt32; byteOffset @1 :UInt64; content @2 :Text; }
struct DeletePayload { bufferId @0 :UInt32; byteOffset @1 :UInt64; length  @2 :UInt64; }

struct DiagnosticAnnotation {
  bufferId  @0 :UInt32; startByte @1 :UInt64; endByte @2 :UInt64;
  severity  @3 :DiagnosticSeverity; message @4 :Text; source @5 :Text; code @6 :Text;
}
enum DiagnosticSeverity { error @0; warning @1; information @2; hint @3; }

struct ViewportCommand { bufferId @0 :UInt32; startLine @1 :UInt64; lineCount @2 :UInt32; }
struct RmsError { code @0 :UInt32; message @1 :Text; cmdId @2 :UInt64; }

struct EditorEvent {
  union {
    keyPress @0 :Text;            windowResize @1 :WindowDimensions;
    clientShutdown @2 :Void;      bufferOpened @3 :BufferHandle;
    snapshotReady @4 :SnapshotHandle; snapshotReleased @5 :UInt32;
    bufferClosed @6 :UInt32;      error @7 :RmsError;
  }
}

struct CoreCommand {
  id @0 :UInt64;
  union {
    openBuffer @1 :Text;          insertText @2 :InsertPayload;
    deleteText @3 :DeletePayload; requestSnapshot @4 :UInt32;
    releaseSnapshot @5 :UInt32;   closeBuffer @6 :UInt32;
    setDiagnostics @7 :List(DiagnosticAnnotation); setViewport @8 :ViewportCommand;
    toggleOverlay @9 :Void;       shutdown @10 :Void;
  }
}
