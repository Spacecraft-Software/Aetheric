;;;; SPDX-License-Identifier: AGPL-3.0-or-later
;;;; Copyright (c) 2026 Mohamed Hammad & Spacecraft Software
;;;;
;;;; Pure apply-event tests — no editor, no GPU, no fibers needed.

(use-modules (srfi srfi-64)
             (ice-9 match))

;; Minimal functional core for testing.
(define-record-type <editor-state>
  (make-editor-state buffers keymaps diagnostics)
  editor-state?
  (buffers     editor-state-buffers)
  (keymaps     editor-state-keymaps)
  (diagnostics editor-state-diagnostics))

(define (initial-state)
  (make-editor-state '() '() '()))

(define (apply-event state event)
  (match event
    (('rms . ('key-press k))
     (make-editor-state (editor-state-buffers state)
                        (editor-state-keymaps state)
                        (editor-state-diagnostics state)))
    (('rms . ('buffer-opened h))
     (make-editor-state (cons h (editor-state-buffers state))
                        (editor-state-keymaps state)
                        (editor-state-diagnostics state)))
    (('lsp . diags)
     (make-editor-state (editor-state-buffers state)
                        (editor-state-keymaps state)
                        diags))
    (_ state)))

(test-begin "core-transitions")

(let ((s0 (initial-state)))
  (test-equal "initial state has empty buffers"
    '() (editor-state-buffers s0))

  (let ((s1 (apply-event s0 '(rms . (buffer-opened . ((id . 1) (path . "/tmp/x")))))))
    (test-equal "buffer-opened adds one buffer"
      1 (length (editor-state-buffers s1))))

  (let ((s2 (apply-event s0 '(lsp . ((severity . error) (message . "unused"))))))
    (test-equal "lsp diagnostics are stored"
      1 (length (editor-state-diagnostics s2))))

  (let ((s3 (apply-event s0 '(unknown . whatever))))
    (test-equal "unknown event leaves state unchanged"
      s0 s3)))

(test-end "core-transitions")
