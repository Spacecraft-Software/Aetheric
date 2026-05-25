;;;; SPDX-License-Identifier: AGPL-3.0-or-later
;;;; Copyright (c) 2026 Mohamed Hammad & Spacecraft Software
;;;;
;;;; (aetheric core) — immutable state + pure apply-event transition.
;;;;
;;;; The core path uses no set!; mutation lives only at the edges
;;;; (SS bridge, logging, terminal/GPU output).

(define-module (aetheric core)
  #:use-module (ice-9 match)
  #:use-module (srfi srfi-1)
  #:use-module (srfi srfi-9)
  #:export (initial-state apply-event))

;; ---------------------------------------------------------------------------
;; Records — immutable, functional updaters only.
;; ---------------------------------------------------------------------------

(define-record-type <buffer>
  (make-buffer id major-mode minor-modes locals)
  buffer?
  (id          buffer-id)
  (major-mode  buffer-major-mode)
  (minor-modes buffer-minor-modes)
  (locals      buffer-locals))

(define (buffer-with-mode buf mode)
  (make-buffer (buffer-id buf) mode (buffer-minor-modes buf) (buffer-locals buf)))

(define-record-type <editor-state>
  (make-editor-state buffers keymaps diagnostics theme)
  editor-state?
  (buffers     editor-state-buffers)
  (keymaps     editor-state-keymaps)
  (diagnostics editor-state-diagnostics)
  (theme       editor-state-theme))

;; ---------------------------------------------------------------------------
;; Initial state
;; ---------------------------------------------------------------------------

(define (initial-state)
  (make-editor-state '() '() '() 'void))

;; ---------------------------------------------------------------------------
;; Pure transition: (apply-event state event) → state'
;; ---------------------------------------------------------------------------

(define (apply-event state event)
  (match event
    ;; RMS events
    (('rms   . ('key-press k))      (dispatch-key state k))
    (('rms   . ('window-resize d))  (resize-viewport state d))
    (('rms   . ('buffer-opened h))  (register-buffer state h))
    (('rms   . ('snapshot-ready s)) (register-snapshot state s))
    (('rms   . ('buffer-closed id)) (unregister-buffer state id))
    (('rms   . ('error e))          (log-rms-error state e))

    ;; LSP events
    (('lsp   diags)                 (set-diagnostics state diags))

    ;; AI events
    (('ai    . ('token id t))       (append-ai-token state id t))
    (('ai    . ('done id))          (finalize-ai-stream state id))

    ;; Timer / idle
    (('timer . _)                   (run-idle-timers state))

    ;; Unknown → unchanged
    (_                              state)))

;; ---------------------------------------------------------------------------
;; Transition helpers (all pure, return new state)
;; ---------------------------------------------------------------------------

(define (dispatch-key state key)
  ;; TODO: Astrolabe keymap lookup (AE-P2-028)
  state)

(define (resize-viewport state dims)
  ;; TODO: update viewport dimensions (AE-P2-013)
  state)

(define (register-buffer state handle)
  (make-editor-state
   (cons handle (editor-state-buffers state))
   (editor-state-keymaps state)
   (editor-state-diagnostics state)
   (editor-state-theme state)))

(define (register-snapshot state snapshot)
  ;; TODO: track snapshot handles
  state)

(define (unregister-buffer state id)
  (make-editor-state
   (filter (lambda (b) (not (= (assoc-ref b 'id) id)))
           (editor-state-buffers state))
   (editor-state-keymaps state)
   (editor-state-diagnostics state)
   (editor-state-theme state)))

(define (log-rms-error state err)
  ;; TODO: Lumen logging
  state)

(define (set-diagnostics state diags)
  (make-editor-state
   (editor-state-buffers state)
   (editor-state-keymaps state)
   diags
   (editor-state-theme state)))

(define (append-ai-token state id token)
  ;; TODO: ai-mode buffer append (AE-AI-005)
  state)

(define (finalize-ai-stream state id)
  ;; TODO: mark stream done
  state)

(define (run-idle-timers state)
  ;; TODO: run idle hooks
  state)
