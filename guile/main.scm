;;;; SPDX-License-Identifier: AGPL-3.0-or-later
;;;; Copyright (c) 2026 Mohamed Hammad & Spacecraft Software
;;;;
;;;; guile/main.scm — run-fibers single entry point.
;;;;
;;;; Spawns all fibers inside run-fibers.  The interactive loop is a CSP
;;;; select over RMS / LSP / AI / frame-tick channels, folding each event
;;;; through apply-event in tail position.

(use-modules (fibers)
             (fibers channels)
             (fibers operations)
             (fibers timers)
             (ice-9 match)
             (aetheric core))

;; ---------------------------------------------------------------------------
;; Parameters — fiber-local context
;; ---------------------------------------------------------------------------

(define current-conn   (make-parameter #f))
(define current-buffer (make-parameter #f))
(define current-keymap (make-parameter #f))
(define current-theme  (make-parameter #f))

;; ---------------------------------------------------------------------------
;; Event loop — CSP select, never blocks on a single source.
;; ---------------------------------------------------------------------------

(define (event-loop conn rms-ch lsp-ch ai-ch)
  (let loop ((state (initial-state)))
    (let ((event
           (perform-operation
            (choice-operation
             (wrap-operation (get-operation rms-ch) (lambda (e) (cons 'rms e)))
             (wrap-operation (get-operation lsp-ch) (lambda (m) (cons 'lsp m)))
             (wrap-operation (get-operation ai-ch)  (lambda (t) (cons 'ai  t)))
             (wrap-operation (sleep-operation 0.016) (lambda _ (cons 'timer #f)))))))
      (loop (apply-event state event)))))

;; ---------------------------------------------------------------------------
;; Entry point
;; ---------------------------------------------------------------------------

(define (main args)
  (let ((socket-path (if (null? args) "/tmp/aetheric.sock" (car args))))
    (display (string-append "Aetheric Majestic — " socket-path "\n"))
    (run-fibers
     (lambda ()
       (let ((rms-ch (make-channel))
             (lsp-ch (make-channel))
             (ai-ch  (make-channel)))
         ;; TODO: spawn-rms-reader (AE-P1-066)
         ;; TODO: spawn-lsp-readers (AE-P3-004)
         ;; TODO: spawn-ai-reader   (AE-AI-005)
         (event-loop #f rms-ch lsp-ch ai-ch))))))

;; If run as script, invoke main.
(if (equal? (current-filename) (program-arguments))
    (main (cdr (program-arguments))))
