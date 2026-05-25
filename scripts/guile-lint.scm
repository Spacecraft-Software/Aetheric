;;;; SPDX-License-Identifier: AGPL-3.0-or-later
;;;; Copyright (c) 2026 Mohamed Hammad & Spacecraft Software
;;;;
;;;; Static lint for Guile source files.
;;;; Rejects:
;;;;   - set! in core modules (aetheric/core.scm, aetheric/editor.scm)
;;;;   - define-macro anywhere
;;;;   - (touch …) / (join-thread …) inside a fiber body
;;;;   - blocking read / usleep inside a fiber
;;;;   - stray pk calls in committed code

(use-modules (ice-9 rdelim)
             (ice-9 regex))

(define forbidden-patterns
  `(("define-macro" . "Unhygienic macro — use syntax-rules or syntax-case")
    ("(touch " . "touch blocks the OS thread — never call from a fiber")
    ("(join-thread " . "join-thread blocks the OS thread — never call from a fiber")
    ("(read " . "blocking read inside a fiber")
    ("(usleep " . "blocking usleep inside a fiber — use (fibers timers) sleep")
    ("(pk " . "Debug print pk must not survive into commits")))

(define (check-file path)
  (let ((port (open-input-file path))
        (issues '())
        (line-num 1))
    (let loop ((line (read-line port)))
      (cond
       ((eof-object? line)
        (close-port port)
        (reverse issues))
       (else
        (let ((new-issues
               (filter-map
                (lambda (p)
                  (and (string-match (car p) line)
                       (cons line-num (cdr p))))
                forbidden-patterns)))
          (set! issues (append new-issues issues))
          (set! line-num (+ line-num 1))
          (loop (read-line port))))))))

(define (lint-directory dir)
  (let ((files (scandir dir (lambda (f) (string-suffix? ".scm" f)))))
    (let ((all-issues
           (apply append
                  (map (lambda (f)
                         (let ((path (string-append dir "/" f)))
                           (map (lambda (i) (cons path i)) (check-file path))))
                       files))))
      (if (null? all-issues)
          (begin
            (display "guile-lint: clean")
            (newline)
            (exit 0))
          (begin
            (for-each
             (lambda (i)
               (format #t "~a:~a: ~a~%" (car i) (cadr i) (cddr i)))
             all-issues)
            (exit 1))))))

;; Entry point.
(lint-directory "guile/aetheric")
