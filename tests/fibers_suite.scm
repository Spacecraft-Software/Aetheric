;;;; SPDX-License-Identifier: AGPL-3.0-or-later
;;;; Copyright (c) 2026 Mohamed Hammad & Spacecraft Software
;;;;
;;;; Fibers + channels smoke tests: select, rendezvous backpressure,
;;;; per-fiber guard isolation.

(use-modules (srfi srfi-64)
             (ice-9 exceptions))

;; Guarded so failures in one fiber don't crash the scheduler.
(define (guarded-fiber name thunk)
  (spawn-fiber
   (lambda ()
     (guard (exn (#t (test-assert (string-append "fiber " name " crashed")
                                  #f)))
       (thunk)))))

(test-begin "fibers-suite")

;; 1. Channel put/get round-trip.
(let ((ch (make-channel)))
  (spawn-fiber (lambda () (put-message ch 'hello)))
  (test-equal "channel round-trip"
    'hello (get-message ch)))

;; 2. choice-operation select.
(let ((ch-a (make-channel))
      (ch-b (make-channel)))
  (spawn-fiber (lambda () (put-message ch-a 'from-a)))
  (test-equal "choice-operation selects first ready channel"
    'from-a
    (perform-operation
     (choice-operation (wrap-operation (get-operation ch-a) (lambda (x) x))
                       (wrap-operation (get-operation ch-b) (lambda (x) x))))))

;; 3. Backpressure: unbuffered channel blocks sender until receiver arrives.
(let ((ch (make-channel))
      (done (make-channel)))
  (spawn-fiber
   (lambda ()
     (put-message ch 'block-me)
     (put-message done 'sender-finished)))
  (sleep 0.05)  ; give sender time to block
  (test-equal "receiver can still pull after sender blocked"
    'block-me (get-message ch))
  (test-equal "sender eventually finishes"
    'sender-finished (get-message done)))

;; 4. Isolation: crashing fiber does not kill others.
(let ((ch (make-channel)))
  (guarded-fiber "crash" (lambda () (error "deliberate crash")))
  (spawn-fiber (lambda () (put-message ch 'survivor)))
  (test-equal "survivor fiber still runs after neighbor crash"
    'survivor (get-message ch)))

(test-end "fibers-suite")
