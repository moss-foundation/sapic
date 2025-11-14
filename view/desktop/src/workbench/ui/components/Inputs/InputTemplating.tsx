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
    relative overflow-hidden
    flex w-full
    rounded-sm border transition-[outline]
    placeholder-(--moss-controls-placeholder)
    background-(--moss-controls-background) 
    border-(--moss-controls-border)
    text-(--moss-controls-foreground)
    has-data-invalid:border-(--moss-error)
    font-normal
    z-10
  `, 
{
  variants: {
    size: {
      sm: "h-6 px-2",
      md: "h-7 px-2",
    },
  },
});

const editorStyles = cva(
  `flex w-full resize-none items-center border-none bg-transparent font-[Inter] text-[13px] outline-none`,
  {
    variants: {
      size: {
        sm: "h-6",
        md: "h-7",
      },
    },
  }
);

const highlightedVariableStyles =
  "background-(--moss-accent-secondary) text-(--moss-accent) rounded-sm px-0.5 whitespace-nowrap inline-block tracking-tighter" +
  "[height:18px] [word-break:keep-all]";

export const InputTemplating = React.forwardRef<HTMLInputElement, InputTemplatingProps>(
  (
    { className, size = "sm", onTemplateChange, onChange, placeholder, highlightColonVariables = false, ...props },
    forwardedRef
  ) => {
    const [value, setValue] = useState(props.value || "");
    const [isFocused, setIsFocused] = useState(false);
    const editorRef = useRef<HTMLDivElement>(null);
    const hiddenInputRef = useRef<HTMLInputElement>(null);
    const isUpdatingContent = useRef(false);

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

    useEffect(() => {
      if (props.value !== undefined) {
        setValue(props.value);
        updateEditorContent(String(props.value));
      }
    }, [props.value]);

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
      <div
        className={cn(containerStyles({ size }), className)}
        style={{
          overflow: "hidden",
          zIndex: isFocused ? 50 : "auto",
          width: isFocused ? "100%" : "auto",
          height: isFocused ? "max-content" : "auto",
          backgroundColor: isFocused ? "var(--moss-controls-background)" : "transparent",
        }}
      >
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
            "word-break-keep-all overflow-wrap-normal",
            "empty:before:text-(--moss-controls-placeholder) empty:before:pointer-events-none empty:before:flex empty:before:h-full empty:before:items-center empty:before:whitespace-nowrap empty:before:leading-[inherit] empty:before:content-[attr(data-placeholder)]",
            "[&_*]:inline [&_*]:whitespace-normal [&_*]:break-words"
          )}
          contentEditable
          onInput={handleInput}
          onKeyDown={handleKeyDown}
          onPaste={handlePaste}
          onCopy={handleCopy}
          onFocus={() => setIsFocused(true)}
          onBlur={() => setIsFocused(false)}
          data-placeholder={placeholder}
          suppressContentEditableWarning={true}
          style={{
            whiteSpace: "nowrap",
            wordWrap: isFocused ? "break-word" : "normal",
            overflowWrap: isFocused ? "break-word" : "normal",
            width: "100%",
          }}
        />
      </div>
    );
  }
);

InputTemplating.displayName = "InputTemplating";

export default InputTemplating;
