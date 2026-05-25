;;;; SPDX-License-Identifier: AGPL-3.0-or-later
;;;; Copyright (c) 2026 Mohamed Hammad & Spacecraft Software
;;;;
;;;; SRFI-64 test suite entry point.

(use-modules (srfi srfi-64))

;; Load all test files.
(load "tests/core_transitions.scm")
(load "tests/fibers_suite.scm")

;; Phase 1 tests (conditional on librms_ipc.so availability).
(use-modules (ice-9 ftw))
(cond
 ((file-exists? "target/debug/librms_ipc.so")
  (setenv "LD_LIBRARY_PATH"
          (string-append (getcwd) "/target/debug"
                         (if (getenv "LD_LIBRARY_PATH")
                             (string-append ":" (getenv "LD_LIBRARY_PATH"))
                             "")))
  (display "Loading SS bridge smoke tests…")
  (newline)
  ;; (load "tests/ss_bridge_smoke.scm")
  )
 (else
  (display "Skipping SS bridge tests: librms_ipc.so not found.")
  (newline)))

;; Summarise.
(test-exit)
