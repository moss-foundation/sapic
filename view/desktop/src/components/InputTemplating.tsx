import React, { useState, useRef, useEffect, useCallback } from "react";
import { cva } from "class-variance-authority";
import { cn } from "@/utils";

interface InputTemplatingProps extends Omit<React.InputHTMLAttributes<HTMLInputElement>, "size"> {
  size?: "sm" | "md";
  onTemplateChange?: (value: string, variables: string[]) => void;
}

//prettier-ignore
const containerStyles = cva(`
    relative
    flex items-center w-full
    rounded-sm border transition-[outline]
    placeholder-(--moss-controls-placeholder)
    background-(--moss-controls-outlined-bg) 
    border-(--moss-controls-outlined-border)
    text-(--moss-controls-outlined-text)
    has-data-invalid:border-(--moss-error)
    focus-within:outline-(--moss-primary)
    focus-within:has-data-invalid:outline-(--moss-error)
    focus-within:outline-2
    focus-within:-outline-offset-1
    font-normal
    overflow-hidden
  `, 
{
  variants: {
    size: {
      sm: "h-7 px-2",
      md: "h-9 px-2",
    },
  },
});

const editorStyles = cva(
  `white-space-nowrap word-break-keep-all flex h-full w-full resize-none flex-nowrap items-center overflow-x-auto overflow-y-hidden border-none bg-transparent font-mono text-sm outline-none`,
  {
    variants: {
      size: {
        sm: "max-h-7 min-h-7 text-sm leading-7",
        md: "text-md max-h-9 min-h-9 leading-8",
      },
    },
  }
);

const highlightedVariableStyles =
  "background-(--moss-templating-input-bg) text-(--moss-templating-input-text) border border-(--moss-templating-input-border) rounded-sm px-0.5 whitespace-nowrap inline-block tracking-tighter" +
  " [height:20px] [line-height:18px] [vertical-align:middle]";

// Function to extract variables from template string
const extractVariables = (text: string): string[] => {
  const regex = /\{\{([^}]+)\}\}/g;
  const variables: string[] = [];
  let match;

  while ((match = regex.exec(text)) !== null) {
    const variable = match[1].trim();
    if (variable && !variables.includes(variable)) {
      variables.push(variable);
    }
  }

  return variables;
};

// Function to create highlighted HTML while preserving spaces
const createHighlightedHTML = (text: string): string => {
  const regex = /(\{\{[^}]*\}\})/g;
  // Replace spaces with non-breaking spaces to preserve them
  const textWithNbsp = text.replace(/ /g, "&nbsp;");
  return textWithNbsp.replace(regex, (match) => {
    return `<span class="${highlightedVariableStyles}">${match}</span>`;
  });
};

// Function to extract plain text from HTML while preserving spaces
const extractPlainText = (html: string): string => {
  // Replace non-breaking spaces back to regular spaces
  const normalizedHtml = html.replace(/&nbsp;/g, " ");
  const div = document.createElement("div");
  div.innerHTML = normalizedHtml;
  return div.textContent || div.innerText || "";
};

// Function to calculate cursor position in terms of plain text
const getTextOffset = (container: Node, targetNode: Node, targetOffset: number): number => {
  const walker = document.createTreeWalker(container, NodeFilter.SHOW_TEXT, null);

  let offset = 0;
  let node: Node | null;

  while ((node = walker.nextNode())) {
    if (node === targetNode) {
      return offset + targetOffset;
    }
    offset += (node.textContent || "").length;
  }

  return offset;
};

// Function to find text node and offset from plain text position
const findTextNodeAtOffset = (container: Node, targetOffset: number): { node: Node; offset: number } | null => {
  const walker = document.createTreeWalker(container, NodeFilter.SHOW_TEXT, null);

  let currentOffset = 0;
  let node: Node | null;

  while ((node = walker.nextNode())) {
    const nodeText = node.textContent || "";
    if (currentOffset + nodeText.length >= targetOffset) {
      return {
        node,
        offset: targetOffset - currentOffset,
      };
    }
    currentOffset += nodeText.length;
  }

  return null;
};

export const InputTemplating = React.forwardRef<HTMLInputElement, InputTemplatingProps>(
  ({ className, size = "sm", onTemplateChange, onChange, placeholder, ...props }, forwardedRef) => {
    const [value, setValue] = useState(props.value || "");
    const editorRef = useRef<HTMLDivElement>(null);
    const hiddenInputRef = useRef<HTMLInputElement>(null);
    const isUpdatingContent = useRef(false);

    useEffect(() => {
      if (props.value !== undefined) {
        setValue(props.value);
        updateEditorContent(String(props.value));
      }
    }, [props.value]);

    const updateEditorContent = useCallback((text: string) => {
      if (editorRef.current && !isUpdatingContent.current) {
        isUpdatingContent.current = true;

        // Store current cursor position safely
        const selection = window.getSelection();
        let cursorOffset = 0;
        let shouldRestoreCursor = false;

        // Only try to get cursor position if we have a valid selection
        if (selection && selection.rangeCount > 0 && editorRef.current.contains(selection.focusNode)) {
          try {
            const range = selection.getRangeAt(0);
            // Calculate cursor position relative to the plain text content
            cursorOffset = getTextOffset(editorRef.current, range.startContainer, range.startOffset);
            shouldRestoreCursor = true;
          } catch (e) {
            // Selection might not be valid, skip cursor restoration
            shouldRestoreCursor = false;
          }
        }

        const highlightedHTML = createHighlightedHTML(text);
        editorRef.current.innerHTML = highlightedHTML;

        // Restore cursor position only if we had a valid selection
        if (shouldRestoreCursor) {
          try {
            const newRange = document.createRange();
            const newSelection = window.getSelection();

            // Find the appropriate text node to place cursor using plain text offset
            const textPosition = findTextNodeAtOffset(editorRef.current, cursorOffset);

            if (textPosition && newSelection) {
              newRange.setStart(
                textPosition.node,
                Math.min(textPosition.offset, (textPosition.node.textContent || "").length)
              );
              newRange.collapse(true);
              newSelection.removeAllRanges();
              newSelection.addRange(newRange);
            }
          } catch (e) {
            // If cursor restoration fails, just ignore it
          }
        }

        isUpdatingContent.current = false;
      }
    }, []);

    const handleInput = (e: React.FormEvent<HTMLDivElement>) => {
      if (isUpdatingContent.current) return;

      const plainText = extractPlainText(e.currentTarget.innerHTML);
      setValue(plainText);

      if (hiddenInputRef.current) {
        hiddenInputRef.current.value = plainText;
      }

      const variables = extractVariables(plainText);
      onTemplateChange?.(plainText, variables);

      if (onChange && hiddenInputRef.current) {
        const syntheticEvent = {
          target: hiddenInputRef.current,
          currentTarget: hiddenInputRef.current,
        } as React.ChangeEvent<HTMLInputElement>;
        onChange(syntheticEvent);
      }

      // Re-apply highlighting after a short delay
      setTimeout(() => {
        updateEditorContent(plainText);
      }, 0);
    };

    const handleKeyDown = (e: React.KeyboardEvent<HTMLDivElement>) => {
      // Handle Enter key to prevent line breaks
      if (e.key === "Enter") {
        e.preventDefault();
        return;
      }

      if (e.key === "Tab") {
        e.preventDefault();
        return;
      }

      // Handle Delete and Backspace keys to work properly with variable spans
      if (e.key === "Delete" || e.key === "Backspace") {
        e.preventDefault();

        if (!editorRef.current) return;

        const selection = window.getSelection();
        if (!selection || selection.rangeCount === 0) return;

        const range = selection.getRangeAt(0);
        const plainText = extractPlainText(editorRef.current.innerHTML);

        let newText = plainText;
        let newCursorPosition = 0;

        // Check if there's a selection (not just cursor position)
        if (!range.collapsed) {
          const startOffset = getTextOffset(editorRef.current, range.startContainer, range.startOffset);
          const endOffset = getTextOffset(editorRef.current, range.endContainer, range.endOffset);

          newText = plainText.slice(0, startOffset) + plainText.slice(endOffset);
          newCursorPosition = startOffset;
        } else {
          // No selection - single character deletion
          const currentOffset = getTextOffset(editorRef.current, range.startContainer, range.startOffset);
          newCursorPosition = currentOffset;

          if (e.key === "Delete") {
            if (currentOffset < plainText.length) {
              newText = plainText.slice(0, currentOffset) + plainText.slice(currentOffset + 1);
            }
          } else if (e.key === "Backspace") {
            if (currentOffset > 0) {
              newText = plainText.slice(0, currentOffset - 1) + plainText.slice(currentOffset);
              newCursorPosition = currentOffset - 1;
            }
          }
        }

        setValue(newText);

        if (hiddenInputRef.current) {
          hiddenInputRef.current.value = newText;
        }

        const variables = extractVariables(newText);
        onTemplateChange?.(newText, variables);

        if (onChange && hiddenInputRef.current) {
          const syntheticEvent = {
            target: hiddenInputRef.current,
            currentTarget: hiddenInputRef.current,
          } as React.ChangeEvent<HTMLInputElement>;
          onChange(syntheticEvent);
        }

        // Update editor content and restore cursor position
        setTimeout(() => {
          updateEditorContent(newText);
          setTimeout(() => {
            const textPosition = findTextNodeAtOffset(editorRef.current!, newCursorPosition);
            if (textPosition) {
              const newRange = document.createRange();
              const newSelection = window.getSelection();

              try {
                newRange.setStart(
                  textPosition.node,
                  Math.min(textPosition.offset, (textPosition.node.textContent || "").length)
                );
                newRange.collapse(true);
                newSelection?.removeAllRanges();
                newSelection?.addRange(newRange);
              } catch (e) {
                // If cursor positioning fails, just ignore it
              }
            }
          }, 0);
        }, 0);

        return;
      }

      // Handle arrow key navigation to skip over span boundaries
      if (e.key === "ArrowLeft" || e.key === "ArrowRight") {
        e.preventDefault();

        if (!editorRef.current) return;

        const selection = window.getSelection();
        if (!selection || selection.rangeCount === 0) return;

        const range = selection.getRangeAt(0);
        const currentOffset = getTextOffset(editorRef.current, range.startContainer, range.startOffset);
        const plainText = extractPlainText(editorRef.current.innerHTML);

        let newOffset = currentOffset;

        if (e.key === "ArrowLeft") {
          newOffset = Math.max(0, currentOffset - 1);
        } else {
          newOffset = Math.min(plainText.length, currentOffset + 1);
        }

        const textPosition = findTextNodeAtOffset(editorRef.current, newOffset);
        if (textPosition) {
          const newRange = document.createRange();
          const newSelection = window.getSelection();

          try {
            newRange.setStart(
              textPosition.node,
              Math.min(textPosition.offset, (textPosition.node.textContent || "").length)
            );
            newRange.collapse(true);
            newSelection?.removeAllRanges();
            newSelection?.addRange(newRange);
          } catch (e) {
            // If cursor positioning fails, just ignore it
          }
        }
      }
    };

    const handlePaste = (e: React.ClipboardEvent<HTMLDivElement>) => {
      e.preventDefault();
      const text = e.clipboardData.getData("text/plain").replace(/\n/g, " ");
      document.execCommand("insertText", false, text);
    };

    const handleCopy = (e: React.ClipboardEvent<HTMLDivElement>) => {
      e.preventDefault();

      if (!editorRef.current) return;

      const selection = window.getSelection();
      if (!selection || selection.rangeCount === 0) return;

      const range = selection.getRangeAt(0);
      const selectedContent = range.cloneContents();

      // Create a temporary div to extract plain text from the selection
      const tempDiv = document.createElement("div");
      tempDiv.appendChild(selectedContent);

      const plainText = extractPlainText(tempDiv.innerHTML);

      e.clipboardData.setData("text/plain", plainText);
    };

    useEffect(() => {
      updateEditorContent(String(value));
    }, [value, updateEditorContent]);

    return (
      <div className={cn(containerStyles({ size }), className)}>
        {/* Hidden input for form compatibility */}
        <input
          ref={forwardedRef || hiddenInputRef}
          type="text"
          value={value}
          onChange={() => {}} // Controlled by the contentEditable div
          className="hidden"
          {...props}
        />

        <div
          ref={editorRef}
          className={editorStyles({ size })}
          contentEditable
          onInput={handleInput}
          onKeyDown={handleKeyDown}
          onPaste={handlePaste}
          onCopy={handleCopy}
          data-placeholder={placeholder}
          suppressContentEditableWarning={true}
        />

        <style>
          {`
            [contenteditable][data-placeholder]:empty:before {
              content: attr(data-placeholder);
              color: var(--moss-controls-placeholder);
              pointer-events: none;
              white-space: nowrap;
              display: flex;
              align-items: center;
              height: 100%;
              line-height: inherit;
            }
            
            [contenteditable] {
              word-break: keep-all;
              overflow-wrap: normal;
            }
            
            [contenteditable] * {
              display: inline !important;
              white-space: nowrap !important;
            }
          `}
        </style>
      </div>
    );
  }
);

InputTemplating.displayName = "InputTemplating";

export default InputTemplating;
