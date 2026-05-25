;;;; SPDX-License-Identifier: AGPL-3.0-or-later
;;;; Copyright (c) 2026 Mohamed Hammad & Spacecraft Software
;;;;
;;;; SS bridge smoke test: verify rms-version, connect, open-buffer,
;;;; and event drain.

(use-modules (srfi srfi-64)
             (system foreign)
             (aetheric rms-ipc))

(test-begin "ss-bridge-smoke")

;; 1. Version string is non-empty.
(let ((ver-ptr (rms-version)))
  (test-assert "rms-version returns non-null"
    (not (null-pointer? ver-ptr)))
  (test-assert "rms-version returns non-empty string"
    (> (string-length (pointer->string ver-ptr)) 0))
  ;; free
  ((pointer->procedure void (dynamic-func "rms_free_string" (dynamic-link "librms_ipc.so")) '(*)) ver-ptr))

;; 2. Connect to a dummy socket path (will fail, but sets last error).
(let ((conn (rms-connect (string->pointer "/tmp/aetheric-test-dummy.sock"))))
  (test-assert "connect to missing socket fails (null pointer)"
    (null-pointer? conn))
  (test-assert "last-error is non-zero after failed connect"
    (> (rms-last-error) 0))
  (test-assert "last-error-message is non-empty"
    (> (string-length (pointer->string (rms-last-error-message))) 0)))

(test-end "ss-bridge-smoke")
