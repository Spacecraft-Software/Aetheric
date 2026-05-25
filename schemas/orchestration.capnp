# SPDX-License-Identifier: AGPL-3.0-or-later
# Copyright (c) 2026 Mohamed Hammad & Spacecraft Software
@0xcab62eb19bc487ce;
using Core = import "core.capnp";

struct CelestialExportRequest { bufferId @0 :UInt32; format @1 :ExportFormat; outputPath @2 :Text; }
enum ExportFormat { pdf @0; html @1; markdown @2; latex @3; }
struct CelestialAgendaQuery { rangeStartIso @0 :Text; rangeEndIso @1 :Text; tags @2 :List(Text); }
struct CelestialAgendaItem {
  heading @0 :Text; state @1 :Text; deadline @2 :Text; scheduled @3 :Text;
  tags @4 :List(Text); priority @5 :Text; bufferId @6 :UInt32; byteOffset @7 :UInt64;
}
struct NexusStatusEntry { path @0 :Text; indexState @1 :GitState; wdState @2 :GitState; }
enum GitState { unmodified @0; modified @1; added @2; deleted @3; renamed @4; copied @5; untracked @6; ignored @7; }
struct NexusCommit { hash @0 :Text; shortHash @1 :Text; author @2 :Text; authorEmail @3 :Text; timestampZ @4 :Text; message @5 :Text; }
struct NexusHunkRange { startLine @0 :UInt32; lineCount @1 :UInt32; }
struct NexusHunk { oldRange @0 :NexusHunkRange; newRange @1 :NexusHunkRange; header @2 :Text; lines @3 :List(Text); }
struct NexusStageHunk { path @0 :Text; hunkIndex @1 :UInt32; }
struct AiStreamRequest { requestId @0 :UInt64; prompt @1 :Text; contextBuf @2 :UInt32; }
struct AiStreamChunk   { requestId @0 :UInt64; content @1 :Text; done @2 :Bool; }

struct OrchCommand {
  id @0 :UInt64;
  union {
    celestialExport @1 :CelestialExportRequest; celestialAgendaQuery @2 :CelestialAgendaQuery;
    nexusStatusRequest @3 :UInt32; nexusCommitLogRequest @4 :UInt32;
    nexusStageHunk @5 :NexusStageHunk; nexusUnstageHunk @6 :NexusStageHunk;
    nexusCommit @7 :Text; aiStreamRequest @8 :AiStreamRequest; aiStreamCancel @9 :UInt64;
  }
}
struct OrchEvent {
  union {
    exportDone @0 :ExportDoneResult; agendaResult @1 :List(CelestialAgendaItem);
    nexusStatusResult @2 :List(NexusStatusEntry); nexusCommitLogResult @3 :List(NexusCommit);
    nexusDiff @4 :List(NexusHunk); aiStreamChunk @5 :AiStreamChunk; orchError @6 :Core.RmsError;
  }
}
struct ExportDoneResult { path @0 :Text; sizeBytes @1 :UInt64; }
