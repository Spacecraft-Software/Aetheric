;;;; SPDX-License-Identifier: AGPL-3.0-or-later
;;;; Copyright (c) 2026 Mohamed Hammad & Spacecraft Software
;;;;
;;;; (aetheric rms-ipc) — SingleScrew bridge wrappers.
;;;;
;;;; The critical rule: rms_recv() blocks, so Majestic must NEVER call it
;;;; from a fiber.  We expose rms_fd() (pollable notification fd) and
;;;; rms_try_recv() (non-blocking).  A dedicated reader fiber turns the C
;;;; bridge into a clean CSP source.

(define-module (aetheric rms-ipc)
  #:use-module (system foreign)
  #:use-module (ice-9 match)
  #:export (rms-connect
            rms-disconnect
            rms-fd
            rms-version
            rms-last-error
            rms-last-error-message
            rms-try-recv
            rms-send-open-buffer
            rms-send-insert-text
            rms-send-delete-text
            rms-send-request-snapshot
            rms-send-release-snapshot
            rms-send-close-buffer
            rms-send-shutdown
            spawn-rms-reader
            drain-events))

;; ---------------------------------------------------------------------------
;; FFI bindings to librms_ipc.so
;; ---------------------------------------------------------------------------

(define lib
  (dynamic-link "librms_ipc.so"))

(define rms-version
  (pointer->procedure '* (dynamic-func "rms_version" lib) '()))

(define rms-last-error
  (pointer->procedure int (dynamic-func "rms_last_error" lib) '()))

(define rms-last-error-message
  (pointer->procedure '* (dynamic-func "rms_last_error_message" lib) '()))

(define rms-connect
  (pointer->procedure '* (dynamic-func "rms_connect" lib) '(*)))

(define rms-disconnect
  (pointer->procedure void (dynamic-func "rms_disconnect" lib) '(*)))

(define rms-fd
  (pointer->procedure int (dynamic-func "rms_fd" lib) '(*)))

(define rms-try-recv
  (pointer->procedure '* (dynamic-func "rms_try_recv" lib) '(*)))

(define rms-send
  (pointer->procedure int (dynamic-func "rms_send" lib) '(* *)))

(define rms-msg-kind
  (pointer->procedure int (dynamic-func "rms_msg_kind" lib) '(*)))

(define rms-msg-key-press-str
  (pointer->procedure '* (dynamic-func "rms_msg_key_press_str" lib) '(*)))

(define rms-msg-free
  (pointer->procedure void (dynamic-func "rms_msg_free" lib) '(*)))

(define rms-cmd-open-buffer
  (pointer->procedure '* (dynamic-func "rms_cmd_open_buffer" lib) '(*)))

(define rms-cmd-insert-text
  (pointer->procedure '* (dynamic-func "rms_cmd_insert_text" lib) '(* * *)))

(define rms-cmd-delete-text
  (pointer->procedure '* (dynamic-func "rms_cmd_delete_text" lib) '(* * *)))

(define rms-cmd-request-snapshot
  (pointer->procedure '* (dynamic-func "rms_cmd_request_snapshot" lib) '(*)))

(define rms-cmd-release-snapshot
  (pointer->procedure '* (dynamic-func "rms_cmd_release_snapshot" lib) '(*)))

(define rms-cmd-close-buffer
  (pointer->procedure '* (dynamic-func "rms_cmd_close_buffer" lib) '(*)))

(define rms-cmd-shutdown
  (pointer->procedure '* (dynamic-func "rms_cmd_shutdown" lib) '()))

(define rms-cmd-free
  (pointer->procedure void (dynamic-func "rms_cmd_free" lib) '(*)))

;; ---------------------------------------------------------------------------
;; Helpers
;; ---------------------------------------------------------------------------

(define (string->c-string s)
  (string->pointer s))

(define (c-string->string ptr)
  (if (null-pointer? ptr)
      #f
      (pointer->string ptr)))

(define (rms-send-open-buffer conn path)
  (let ((cmd (rms-cmd-open-buffer (string->c-string path))))
    (rms-send conn cmd)
    (rms-cmd-free cmd)))

(define (rms-send-insert-text conn buffer-id byte-offset content)
  (let ((cmd (rms-cmd-insert-text
               buffer-id
               byte-offset
               (string->c-string content))))
    (rms-send conn cmd)
    (rms-cmd-free cmd)))

(define (rms-send-delete-text conn buffer-id byte-offset len)
  (let ((cmd (rms-cmd-delete-text buffer-id byte-offset len)))
    (rms-send conn cmd)
    (rms-cmd-free cmd)))

(define (rms-send-request-snapshot conn buffer-id)
  (let ((cmd (rms-cmd-request-snapshot buffer-id)))
    (rms-send conn cmd)
    (rms-cmd-free cmd)))

(define (rms-send-release-snapshot conn snapshot-id)
  (let ((cmd (rms-cmd-release-snapshot snapshot-id)))
    (rms-send conn cmd)
    (rms-cmd-free cmd)))

(define (rms-send-close-buffer conn buffer-id)
  (let ((cmd (rms-cmd-close-buffer buffer-id)))
    (rms-send conn cmd)
    (rms-cmd-free cmd)))

(define (rms-send-shutdown conn)
  (let ((cmd (rms-cmd-shutdown)))
    (rms-send conn cmd)
    (rms-cmd-free cmd)))

;; ---------------------------------------------------------------------------
;; Fiber-aware reader
;; ---------------------------------------------------------------------------

;; Spawn a dedicated reader fiber that waits on the notification fd
;; (suspendable port) and drains non-blocking events onto a channel.
(define (spawn-rms-reader conn rms-ch)
  (spawn-fiber
   (lambda ()
     (let ((port (fdopen (rms-fd conn) "r")))
       (let loop ()
         (get-u8 port)              ; fiber-aware wait for readability
         (drain-events conn rms-ch)
         (loop))))))

;; Non-blocking drain: rms_try_recv until empty.
(define (drain-events conn ch)
  (let loop ()
    (let ((msg (rms-try-recv conn)))
      (when (not (null-pointer? msg))
        (let ((kind (rms-msg-kind msg)))
          (put-message ch (parse-msg kind msg))
          (rms-msg-free msg)
          (loop))))))

(define (parse-msg kind msg)
  (cond
   ((= kind 0)                      ; keyPress
    (cons 'key-press (c-string->string (rms-msg-key-press-str msg))))
   ;; TODO: handle remaining event kinds (windowResize, bufferOpened, …)
   (else
    (cons 'unknown kind))))
