;;; pjmai-test.el --- ERT tests for pjmai.el -*- lexical-binding: t; -*-

;;; Commentary:

;; Tests for the pjmai Emacs integration package.
;; Run with: emacs -batch -l ert -l pjmai.el -l pjmai-test.el -f ert-run-tests-batch-and-exit

;;; Code:

(require 'ert)
(require 'pjmai)

;;; --- Test helpers ---

(defvar pjmai-test--original-program nil
  "Saved value of `pjmai-program' for test teardown.")

(defun pjmai-test--mock-program (script)
  "Create a temporary mock script returning SCRIPT output.
Returns the path to the mock script."
  (let ((tmp (make-temp-file "pjmai-mock-" nil ".sh")))
    (with-temp-file tmp
      (insert "#!/bin/sh\n" script "\n"))
    (set-file-modes tmp #o755)
    tmp))

(defmacro pjmai-test--with-mock (script &rest body)
  "Execute BODY with `pjmai-program' set to a mock that runs SCRIPT."
  (declare (indent 1))
  `(let* ((mock (pjmai-test--mock-program ,script))
          (pjmai-program mock))
     (unwind-protect
         (progn ,@body)
       (delete-file mock))))

;;; --- pjmai--call tests ---

(ert-deftest pjmai-test-call-success ()
  "pjmai--call returns trimmed stdout on success."
  (pjmai-test--with-mock "echo '  hello world  '"
    (should (equal (pjmai--call) "hello world"))))

(ert-deftest pjmai-test-call-with-args ()
  "pjmai--call passes arguments to the program."
  (pjmai-test--with-mock "echo \"$1 $2\""
    (should (equal (pjmai--call "foo" "bar") "foo bar"))))

(ert-deftest pjmai-test-call-failure ()
  "pjmai--call signals an error on exit code > 3."
  (pjmai-test--with-mock "echo 'bad stuff' >&2; exit 4"
    (should-error (pjmai--call) :type 'error)))

(ert-deftest pjmai-test-call-empty-output ()
  "pjmai--call returns empty string for empty output."
  (pjmai-test--with-mock "true"
    (should (equal (pjmai--call) ""))))

;;; --- pjmai--call-json tests ---

(ert-deftest pjmai-test-call-json-parses ()
  "pjmai--call-json parses JSON output into plist."
  (pjmai-test--with-mock "echo '{\"name\":\"foo\",\"path\":\"/tmp/foo\"}'"
    ;; --json is passed as first arg; mock ignores it
    (let ((result (pjmai--call-json)))
      (should (equal (plist-get result :name) "foo"))
      (should (equal (plist-get result :path) "/tmp/foo")))))

(ert-deftest pjmai-test-call-json-with-array ()
  "pjmai--call-json handles JSON with arrays."
  (pjmai-test--with-mock "echo '{\"items\":[\"a\",\"b\",\"c\"]}'"
    (let ((result (pjmai--call-json)))
      (should (equal (plist-get result :items) '("a" "b" "c"))))))

;;; --- pjmai--project-names tests ---

(ert-deftest pjmai-test-project-names ()
  "pjmai--project-names splits newline-separated output."
  (pjmai-test--with-mock "echo 'alpha\nbeta\ngamma'"
    (should (equal (pjmai--project-names) '("alpha" "beta" "gamma")))))

(ert-deftest pjmai-test-project-names-empty ()
  "pjmai--project-names returns nil for empty output."
  (pjmai-test--with-mock "true"
    (should (null (pjmai--project-names)))))

;;; --- pjmai-resolve-path tests ---

(ert-deftest pjmai-test-resolve-path-valid ()
  "pjmai-resolve-path returns directory path with trailing slash."
  (pjmai-test--with-mock
      (format "echo '{\"name\":\"test\",\"path\":\"%s\",\"type\":\"directory\"}'" (temporary-file-directory))
    (let ((result (pjmai-resolve-path "test")))
      (should (string-suffix-p "/" result))
      (should (file-directory-p result)))))

(ert-deftest pjmai-test-resolve-path-invalid ()
  "pjmai-resolve-path signals error for non-existent path."
  (pjmai-test--with-mock "echo '{\"name\":\"bad\",\"path\":\"/nonexistent/pjmai-fake-12345\",\"type\":\"directory\"}'"
    (should-error (pjmai-resolve-path "bad") :type 'error)))

;;; --- Shell buffer name tests ---

(ert-deftest pjmai-test-shell-buffer-name-default ()
  "pjmai--shell-buffer-name uses default format."
  (let ((pjmai-shell-buffer-format "*pjmai:%s*"))
    (should (equal (pjmai--shell-buffer-name "myproj") "*pjmai:myproj*"))))

(ert-deftest pjmai-test-shell-buffer-name-custom ()
  "pjmai--shell-buffer-name respects custom format."
  (let ((pjmai-shell-buffer-format "*proj/%s*"))
    (should (equal (pjmai--shell-buffer-name "foo") "*proj/foo*"))))

;;; --- Display command tests ---

(ert-deftest pjmai-test-display-creates-buffer ()
  "pjmai--display creates an output buffer with content."
  (pjmai-test--with-mock "echo 'line1\nline2'"
    (let ((buf (pjmai--display "*pjmai-test-display*")))
      (unwind-protect
          (with-current-buffer buf
            (should (string-match-p "line1" (buffer-string)))
            (should (string-match-p "line2" (buffer-string)))
            (should (eq major-mode 'special-mode)))
        (kill-buffer buf)))))

(ert-deftest pjmai-test-display-error ()
  "pjmai--display signals error on exit code > 3."
  (pjmai-test--with-mock "echo 'error'; exit 4"
    (should-error (pjmai--display "*pjmai-test-err*") :type 'error)
    (when (get-buffer "*pjmai-test-err*")
      (kill-buffer "*pjmai-test-err*"))))

;;; --- Keymap structure tests ---

(ert-deftest pjmai-test-keymap-core-bindings ()
  "pjmai-command-map has expected core bindings."
  (should (eq (lookup-key pjmai-command-map (kbd "h")) #'pjmai-help))
  (should (eq (lookup-key pjmai-command-map (kbd "c")) #'pjmai-change))
  (should (eq (lookup-key pjmai-command-map (kbd "s")) #'pjmai-show))
  (should (eq (lookup-key pjmai-command-map (kbd "l")) #'pjmai-list))
  (should (eq (lookup-key pjmai-command-map (kbd "L")) #'pjmai-list-long))
  (should (eq (lookup-key pjmai-command-map (kbd "q")) #'pjmai-query))
  (should (eq (lookup-key pjmai-command-map (kbd "t")) #'pjmai-context))
  (should (eq (lookup-key pjmai-command-map (kbd "x")) #'pjmai-exports))
  (should (eq (lookup-key pjmai-command-map (kbd "H")) #'pjmai-history))
  (should (eq (lookup-key pjmai-command-map (kbd "d")) #'pjmai-dired)))

(ert-deftest pjmai-test-keymap-state-bindings ()
  "pjmai-command-map has state-changing command bindings."
  (should (eq (lookup-key pjmai-command-map (kbd "a")) #'pjmai-add))
  (should (eq (lookup-key pjmai-command-map (kbd "e")) #'pjmai-edit))
  (should (eq (lookup-key pjmai-command-map (kbd "r")) #'pjmai-remove))
  (should (eq (lookup-key pjmai-command-map (kbd "R")) #'pjmai-rename))
  (should (eq (lookup-key pjmai-command-map (kbd "p")) #'pjmai-push))
  (should (eq (lookup-key pjmai-command-map (kbd "o")) #'pjmai-pop)))

(ert-deftest pjmai-test-keymap-group-submap ()
  "pjmai-command-map has group submap under 'g'."
  (let ((gmap (lookup-key pjmai-command-map (kbd "g"))))
    (should (keymapp gmap))
    (should (eq (lookup-key gmap (kbd "l")) #'pjmai-group-list))
    (should (eq (lookup-key gmap (kbd "s")) #'pjmai-group-show))
    (should (eq (lookup-key gmap (kbd "p")) #'pjmai-group-prompt))))

(ert-deftest pjmai-test-keymap-stack-submap ()
  "pjmai-command-map has stack submap under 'k'."
  (let ((kmap (lookup-key pjmai-command-map (kbd "k"))))
    (should (keymapp kmap))
    (should (commandp (lookup-key kmap (kbd "s"))))
    (should (commandp (lookup-key kmap (kbd "c"))))))

;;; --- Global mode tests ---

(ert-deftest pjmai-test-global-mode-enable-disable ()
  "pjmai-global-mode installs and removes prefix binding."
  (unwind-protect
      (progn
        (pjmai-global-mode 1)
        (should pjmai-global-mode)
        (should (eq (lookup-key pjmai-mode-map (kbd pjmai-key-prefix))
                    pjmai-command-map))
        (pjmai-global-mode -1)
        (should-not pjmai-global-mode)
        (should-not (lookup-key pjmai-mode-map (kbd pjmai-key-prefix))))
    (pjmai-global-mode -1)))

;;; --- Customization tests ---

(ert-deftest pjmai-test-default-program ()
  "Default program resolves to pjmai-rs."
  (should (string-match-p "pjmai-rs" (default-value 'pjmai-program))))

(ert-deftest pjmai-test-default-buffer-format ()
  "Default shell buffer format matches expected pattern."
  (should (equal (default-value 'pjmai-shell-buffer-format) "*pjmai:%s*")))

(ert-deftest pjmai-test-default-key-prefix ()
  "Default key prefix is C-c p."
  (should (equal (default-value 'pjmai-key-prefix) "C-c p")))

;;; --- Query command tests ---

(ert-deftest pjmai-test-call-status ()
  "pjmai--call-status returns the raw exit code."
  (pjmai-test--with-mock "exit 2"
    (should (= (pjmai--call-status) 2))))

(ert-deftest pjmai-test-query-found ()
  "pjmai-query reports success for existing project."
  (pjmai-test--with-mock "exit 0"
    (should (string-match-p "exists" (pjmai-query "test-proj")))))

(ert-deftest pjmai-test-query-not-found ()
  "pjmai-query reports not-found for missing project."
  (pjmai-test--with-mock "exit 1"
    (should (string-match-p "not found" (pjmai-query "nonexistent")))))

(provide 'pjmai-test)

;;; pjmai-test.el ends here
