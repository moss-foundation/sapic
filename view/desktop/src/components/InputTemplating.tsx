import { cva } from "class-variance-authority";
import React, { useCallback, useEffect, useRef, useState } from "react";

import { cn } from "@/utils";
import {
  createHighlightedHTML,
  extractPlainText,
  extractVariables,
  getCurrentCursorOffset,
  getTextOffset,
  setCursorPosition,
} from "@/utils/templating";

interface InputTemplatingProps extends Omit<React.InputHTMLAttributes<HTMLInputElement>, "size"> {
  size?: "sm" | "md";
  onTemplateChange?: (value: string, variables: string[]) => void;
  highlightColonVariables?: boolean;
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
  `white-space-nowrap word-break-keep-all flex h-full w-full resize-none flex-nowrap items-center overflow-x-auto overflow-y-hidden border-none bg-transparent outline-none`,
  {
    variants: {
      size: {
        sm: "max-h-7 min-h-7 leading-7",
        md: "max-h-9 min-h-9 leading-8",
      },
    },
  }
);

const highlightedVariableStyles =
  "background-(--moss-templating-input-bg) text-(--moss-templating-input-text) border border-(--moss-templating-input-border) rounded-sm px-0.5 whitespace-nowrap inline-block tracking-tighter" +
  " [height:20px] [line-height:18px] [vertical-align:middle]";

export const InputTemplating = React.forwardRef<HTMLInputElement, InputTemplatingProps>(
  (
    { className, size = "sm", onTemplateChange, onChange, placeholder, highlightColonVariables = false, ...props },
    forwardedRef
  ) => {
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

    const updateEditorContent = useCallback(
      (text: string) => {
        if (editorRef.current && !isUpdatingContent.current) {
          isUpdatingContent.current = true;

          const selection = window.getSelection();
          let cursorOffset = 0;
          let shouldRestoreCursor = false;

          if (selection && selection.rangeCount > 0 && editorRef.current.contains(selection.focusNode)) {
            try {
              cursorOffset = getCurrentCursorOffset(editorRef.current);
              shouldRestoreCursor = true;
            } catch (e) {
              shouldRestoreCursor = false;
            }
          }

          const highlightedHTML = createHighlightedHTML(text, highlightColonVariables, highlightedVariableStyles);
          editorRef.current.innerHTML = highlightedHTML;

          if (shouldRestoreCursor) {
            setCursorPosition(editorRef.current, cursorOffset);
          }

          isUpdatingContent.current = false;
        }
      },
      [highlightColonVariables]
    );

    const handleInput = (e: React.FormEvent<HTMLDivElement>) => {
      if (isUpdatingContent.current) return;

      const plainText = extractPlainText(e.currentTarget.innerHTML);
      setValue(plainText);

      if (hiddenInputRef.current) {
        hiddenInputRef.current.value = plainText;
      }

      const variables = extractVariables(plainText, highlightColonVariables);
      onTemplateChange?.(plainText, variables);

      if (onChange && hiddenInputRef.current) {
        const syntheticEvent = {
          target: hiddenInputRef.current,
          currentTarget: hiddenInputRef.current,
        } as React.ChangeEvent<HTMLInputElement>;
        onChange(syntheticEvent);
      }

      setTimeout(() => {
        updateEditorContent(plainText);
      }, 0);
    };

    const handleKeyDown = (e: React.KeyboardEvent<HTMLDivElement>) => {
      if (e.key === "Enter") {
        e.preventDefault();
        return;
      }

      if (e.key === "Tab") {
        e.preventDefault();
        return;
      }

      if (e.key === "Delete" || e.key === "Backspace") {
        e.preventDefault();

        if (!editorRef.current) return;

        const selection = window.getSelection();
        if (!selection || selection.rangeCount === 0) return;

        const range = selection.getRangeAt(0);
        const plainText = extractPlainText(editorRef.current.innerHTML);

        let newText = plainText;
        let newCursorPosition = 0;

        if (!range.collapsed) {
          const startOffset = getTextOffset(editorRef.current, range.startContainer, range.startOffset);
          const endOffset = getTextOffset(editorRef.current, range.endContainer, range.endOffset);

          newText = plainText.slice(0, startOffset) + plainText.slice(endOffset);
          newCursorPosition = startOffset;
        } else {
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

        const variables = extractVariables(newText, highlightColonVariables);
        onTemplateChange?.(newText, variables);

        if (onChange && hiddenInputRef.current) {
          const syntheticEvent = {
            target: hiddenInputRef.current,
            currentTarget: hiddenInputRef.current,
          } as React.ChangeEvent<HTMLInputElement>;
          onChange(syntheticEvent);
        }

        setTimeout(() => {
          updateEditorContent(newText);
          setTimeout(() => {
            setCursorPosition(editorRef.current!, newCursorPosition);
          }, 0);
        }, 0);

        return;
      }

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

        setCursorPosition(editorRef.current, newOffset);
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
          onChange={() => {}}
          className="hidden"
          {...props}
        />

        <div
          ref={editorRef}
          className={cn(
            editorStyles({ size }),
            "word-break-keep-all overflow-wrap-normal font-[Inter] text-[13px]",
            "empty:before:pointer-events-none empty:before:flex empty:before:h-full empty:before:items-center empty:before:leading-[inherit] empty:before:whitespace-nowrap empty:before:text-(--moss-requestpage-placeholder-color) empty:before:content-[attr(data-placeholder)]",
            "[&_*]:inline [&_*]:whitespace-nowrap"
          )}
          contentEditable
          onInput={handleInput}
          onKeyDown={handleKeyDown}
          onPaste={handlePaste}
          onCopy={handleCopy}
          data-placeholder={placeholder}
          suppressContentEditableWarning={true}
        />
      </div>
    );
  }
);

InputTemplating.displayName = "InputTemplating";

export default InputTemplating;
