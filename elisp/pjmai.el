;;; pjmai.el --- Emacs integration for pjmai-rs project manager -*- lexical-binding: t; -*-

;; Copyright (C) 2026 Mike Wright
;; Author: Mike Wright
;; Version: 0.1.0
;; Package-Requires: ((emacs "27.1"))
;; Keywords: tools, convenience, projects
;; URL: https://github.com/sw-cli-tools/pjmai-rs

;;; Commentary:

;; Emacs-native interface to the pjmai-rs CLI project manager.
;;
;; Instead of relying on shell aliases (which cause stale
;; `default-directory' in shell-mode), this package calls the pjmai
;; binary directly and manages per-project shell buffers with correct
;; directory state from the start.
;;
;; Quick start:
;;   (require 'pjmai)
;;   (pjmai-global-mode 1)
;;
;; Then use C-c p as the prefix:
;;   C-c p c  change project (open/switch shell)
;;   C-c p l  list projects
;;   C-c p s  show current project
;;   C-c p h  help / aliases
;;   C-c p q  query project
;;   C-c p t  context
;;   C-c p H  history
;;   C-c p x  exports
;;   C-c p a  add project
;;   C-c p e  edit project
;;   C-c p p  push project
;;   C-c p o  pop project
;;   C-c p g  group submap (l/s/p)
;;   C-c p k  stack submap (s/c)

;;; Code:

(require 'json)

;;; --- Customization ---

(defgroup pjmai nil
  "Emacs integration for pjmai-rs project manager."
  :group 'tools
  :prefix "pjmai-")

(defcustom pjmai-program
  (or (executable-find "pjmai-rs")
      (let ((local-bin (expand-file-name "~/.local/bin/pjmai-rs")))
        (when (file-executable-p local-bin) local-bin))
      "pjmai-rs")
  "Path to the pjmai-rs executable."
  :type 'string
  :group 'pjmai)

(defcustom pjmai-shell-function #'shell
  "Function to create a new shell buffer.
Called with one argument: the buffer name.
Override to use `vterm' or `eshell' instead."
  :type 'function
  :group 'pjmai)

(defcustom pjmai-key-prefix "C-c p"
  "Prefix key for pjmai commands.
Takes effect when `pjmai-global-mode' is enabled."
  :type 'string
  :group 'pjmai)

(defcustom pjmai-shell-buffer-format "*pjmai:%s*"
  "Format string for per-project shell buffer names.
%s is replaced with the project nickname."
  :type 'string
  :group 'pjmai)

;;; --- Core CLI interface ---

(defun pjmai--call (&rest args)
  "Run pjmai with ARGS and return trimmed stdout.
Signal an error if the command exits with an error code.
Exit codes 0-3 are treated as success (the CLI uses 2 for cd,
3 for source-file signaling to the shell wrapper)."
  (with-temp-buffer
    (let ((status (apply #'process-file pjmai-program nil (current-buffer) nil args)))
      (let ((output (string-trim (buffer-string))))
        (if (<= status 3)
            output
          (error "pjmai failed (exit %s): %s" status output))))))

(defun pjmai--call-json (&rest args)
  "Run pjmai with --json and ARGS, return parsed JSON as plist."
  (let ((raw (apply #'pjmai--call "--json" args)))
    (json-parse-string raw :object-type 'plist :array-type 'list)))

(defun pjmai--display (buffer-name &rest args)
  "Run pjmai with ARGS, display output in BUFFER-NAME.
Returns the output buffer."
  (let ((buf (get-buffer-create buffer-name)))
    (with-current-buffer buf
      (let ((inhibit-read-only t))
        (erase-buffer)
        (let ((status (apply #'process-file pjmai-program nil buf nil args)))
          (goto-char (point-min))
          (special-mode)
          (unless (<= status 3)
            (error "pjmai command failed; see %s" buffer-name)))))
    (pop-to-buffer buf)
    buf))

;;; --- Project name completion ---

(defun pjmai--project-names ()
  "Return a list of project nicknames from the CLI."
  (let ((raw (pjmai--call "complete" "projects")))
    (if (string-empty-p raw)
        nil
      (split-string raw "\n" t))))

(defun pjmai--read-project (prompt)
  "Read a project nickname with completion using PROMPT."
  (completing-read prompt (pjmai--project-names) nil nil))

;;; --- Project resolution ---

(defun pjmai-resolve-path (nickname)
  "Resolve NICKNAME to an absolute directory path via the CLI.
Uses `change -p' which returns the project path (exit 2 = cd).
Returns the path as a string with trailing slash."
  (let* ((info (pjmai--call-json "change" "-p" nickname))
         (path (plist-get info :path)))
    (unless (and path (file-directory-p path))
      (error "Resolved path is not a directory: %s" path))
    (file-name-as-directory (expand-file-name path))))

;;; --- Shell buffer management ---

(defun pjmai--shell-buffer-name (nickname)
  "Return the buffer name for project NICKNAME."
  (format pjmai-shell-buffer-format nickname))

(defun pjmai-shell (nickname)
  "Create or switch to a shell buffer for project NICKNAME.
If the buffer already exists and is live, switch to it and
re-sync `default-directory'.  Otherwise create a new shell
at the project root."
  (interactive (list (pjmai--read-project "Project: ")))
  (let* ((dir (pjmai-resolve-path nickname))
         (bufname (pjmai--shell-buffer-name nickname))
         (buf (get-buffer bufname)))
    (if (buffer-live-p buf)
        ;; Re-sync existing shell
        (progn
          (pop-to-buffer buf)
          (with-current-buffer buf
            (setq-local default-directory dir)
            (goto-char (point-max))
            (insert (format "cd %s" (shell-quote-argument dir)))
            (comint-send-input)))
      ;; Create new shell at project dir
      (let ((default-directory dir))
        (funcall pjmai-shell-function bufname)
        (with-current-buffer bufname
          (setq-local default-directory dir)
          (rename-buffer bufname t))
        (pop-to-buffer bufname)))))

;;; --- Read-only display commands ---

(defun pjmai-help ()
  "Display pjmai help / alias list."
  (interactive)
  (pjmai--display "*pjmai-help*" "aliases"))

(defun pjmai-list ()
  "List all projects."
  (interactive)
  (pjmai--display "*pjmai-list*" "list"))

(defun pjmai-list-long ()
  "List all projects with extended info."
  (interactive)
  (pjmai--display "*pjmai-list*" "list" "--long"))

(defun pjmai-show ()
  "Show current project."
  (interactive)
  (pjmai--display "*pjmai-show*" "show"))

(defun pjmai-history (&optional n)
  "Show navigation history, or jump to entry N."
  (interactive "P")
  (if n
      (pjmai--display "*pjmai-history*" "history" (format "%s" (prefix-numeric-value n)))
    (pjmai--display "*pjmai-history*" "history")))

(defun pjmai-context (&optional name)
  "Show project context, optionally for NAME."
  (interactive (list (let ((s (pjmai--read-project "Context project (RET for current): ")))
                       (unless (string-empty-p s) s))))
  (if name
      (pjmai--display "*pjmai-context*" "context" name)
    (pjmai--display "*pjmai-context*" "context")))

(defun pjmai-exports ()
  "Show project exports."
  (interactive)
  (pjmai--display "*pjmai-exports*" "exports"))

;;; --- State-changing commands ---

(defun pjmai-change (name)
  "Change to project NAME.
Opens or switches to a per-project shell buffer with correct
`default-directory'."
  (interactive (list (pjmai--read-project "Change to project: ")))
  (pjmai-shell name))

(defun pjmai--call-status (&rest args)
  "Run pjmai with ARGS and return the exit code as an integer."
  (with-temp-buffer
    (apply #'process-file pjmai-program nil (current-buffer) nil args)))

(defun pjmai-query (name)
  "Query whether project NAME exists."
  (interactive (list (pjmai--read-project "Query project: ")))
  (if (zerop (pjmai--call-status "query" "-p" name))
      (message "Project '%s' exists" name)
    (message "Project '%s' not found" name)))

(defun pjmai-add (name path)
  "Add project NAME at PATH."
  (interactive
   (list (read-string "Project name: ")
         (read-directory-name "Project path: ")))
  (pjmai--call "add" name "-f" (expand-file-name path))
  (message "Added project '%s' at %s" name path))

(defun pjmai-edit (name)
  "Edit project NAME metadata."
  (interactive (list (pjmai--read-project "Edit project: ")))
  (let* ((desc (read-string (format "Description for %s (RET to skip): " name)))
         (lang (read-string (format "Language for %s (RET to skip): " name)))
         (args (list "edit" name)))
    (unless (string-empty-p desc)
      (setq args (append args (list "--description" desc))))
    (unless (string-empty-p lang)
      (setq args (append args (list "--language" lang))))
    (if (= (length args) 2)
        (message "No changes specified")
      (apply #'pjmai--call args)
      (message "Updated project '%s'" name))))

(defun pjmai-push (name)
  "Push to stack and switch to project NAME."
  (interactive (list (pjmai--read-project "Push to project: ")))
  (pjmai--display "*pjmai-push*" "push" name))

(defun pjmai-pop ()
  "Pop from project stack."
  (interactive)
  (pjmai--display "*pjmai-pop*" "pop"))

(defun pjmai-remove (name)
  "Remove project NAME after confirmation."
  (interactive (list (pjmai--read-project "Remove project: ")))
  (when (yes-or-no-p (format "Remove project '%s'? " name))
    (pjmai--call "remove" name)
    (message "Removed project '%s'" name)))

(defun pjmai-rename (old new)
  "Rename project from OLD to NEW."
  (interactive
   (list (pjmai--read-project "Rename project: ")
         (read-string "New name: ")))
  (pjmai--call "rename" old new)
  (message "Renamed '%s' to '%s'" old new))

;;; --- Group commands ---

(defun pjmai-group-list ()
  "List project groups."
  (interactive)
  (pjmai--display "*pjmai-groups*" "group" "list"))

(defun pjmai-group-show (&optional name)
  "Show group details, optionally for NAME."
  (interactive
   (list (let ((s (read-string "Group name (RET for current): ")))
           (unless (string-empty-p s) s))))
  (if name
      (pjmai--display "*pjmai-group-show*" "group" "show" name)
    (pjmai--display "*pjmai-group-show*" "group" "show")))

(defun pjmai-group-prompt ()
  "Show group prompt string."
  (interactive)
  (pjmai--display "*pjmai-group-prompt*" "group" "prompt"))

;;; --- Stack commands ---

(defun pjmai-stack-show ()
  "Show project stack."
  (interactive)
  (pjmai--display "*pjmai-stack*" "stack" "show"))

(defun pjmai-stack-clear ()
  "Clear the project stack."
  (interactive)
  (when (yes-or-no-p "Clear the project stack? ")
    (pjmai--call "stack" "clear" "-y")
    (message "Stack cleared")))

;;; --- Dired integration ---

(defun pjmai-dired (name)
  "Open dired at project NAME root."
  (interactive (list (pjmai--read-project "Dired project: ")))
  (dired (pjmai-resolve-path name)))

;;; --- Keymap ---

(defvar pjmai-command-map
  (let ((map (make-sparse-keymap))
        (gmap (make-sparse-keymap))
        (kmap (make-sparse-keymap)))
    ;; Core commands
    (define-key map (kbd "h") #'pjmai-help)
    (define-key map (kbd "c") #'pjmai-change)
    (define-key map (kbd "s") #'pjmai-show)
    (define-key map (kbd "l") #'pjmai-list)
    (define-key map (kbd "L") #'pjmai-list-long)
    (define-key map (kbd "q") #'pjmai-query)
    (define-key map (kbd "t") #'pjmai-context)
    (define-key map (kbd "x") #'pjmai-exports)
    (define-key map (kbd "H") #'pjmai-history)
    (define-key map (kbd "d") #'pjmai-dired)
    ;; State-changing
    (define-key map (kbd "a") #'pjmai-add)
    (define-key map (kbd "e") #'pjmai-edit)
    (define-key map (kbd "r") #'pjmai-remove)
    (define-key map (kbd "R") #'pjmai-rename)
    (define-key map (kbd "p") #'pjmai-push)
    (define-key map (kbd "o") #'pjmai-pop)
    ;; Groups
    (define-key map (kbd "g") gmap)
    (define-key gmap (kbd "l") #'pjmai-group-list)
    (define-key gmap (kbd "s") #'pjmai-group-show)
    (define-key gmap (kbd "p") #'pjmai-group-prompt)
    ;; Stack
    (define-key map (kbd "k") kmap)
    (define-key kmap (kbd "s") #'pjmai-stack-show)
    (define-key kmap (kbd "c") #'pjmai-stack-clear)
    map)
  "Keymap for pjmai commands, bound under `pjmai-key-prefix'.")

;;; --- Minor mode ---

(defvar pjmai-mode-map (make-sparse-keymap)
  "Keymap for `pjmai-global-mode'.
Populated when the mode is activated.")

;;;###autoload
(define-minor-mode pjmai-global-mode
  "Global minor mode for pjmai-rs project management keybindings."
  :global t
  :lighter " pjm"
  :keymap pjmai-mode-map
  (if pjmai-global-mode
      (define-key pjmai-mode-map (kbd pjmai-key-prefix) pjmai-command-map)
    (define-key pjmai-mode-map (kbd pjmai-key-prefix) nil)))

(provide 'pjmai)

;;; pjmai.el ends here
