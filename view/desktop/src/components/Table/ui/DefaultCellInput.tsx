import { useState, useRef, useEffect, useCallback } from "react";

import { cn } from "@/utils";
import { CellContext } from "@tanstack/react-table";

import { ParameterData } from "../types";
import {
  createHighlightedHTML,
  extractPlainText,
  setCursorPosition,
  getCurrentCursorOffset,
  hasTemplateVariables,
} from "@/utils/templating";

interface DefaultCellInputProps {
  info: CellContext<ParameterData, number | string> & { focusOnMount?: boolean };
  enableTemplating?: boolean;
  highlightColonVariables?: boolean;
}

export const DefaultInputCell = ({
  info,
  enableTemplating = false,
  highlightColonVariables = false,
}: DefaultCellInputProps) => {
  const [value, setValue] = useState(String(info.getValue() || ""));
  const isDisabled = info.row.original.properties?.disabled || false;

  const editorRef = useRef<HTMLDivElement>(null);
  const hiddenInputRef = useRef<HTMLInputElement>(null);
  const isUpdatingContent = useRef(false);
  const isActivelyEditing = useRef(false);
  const automationTimeoutRef = useRef<NodeJS.Timeout>();
  const lastPropValue = useRef(String(info.getValue() || ""));
  const hasFocus = useRef(false);

  useEffect(() => {
    const currentPropValue = String(info.getValue() || "");
    if (!isActivelyEditing.current && !hasFocus.current && currentPropValue !== lastPropValue.current) {
      setValue(currentPropValue);
    }

    lastPropValue.current = currentPropValue;
  }, [info.getValue()]);

  const highlightedVariableStyles =
    "background-(--moss-templating-input-bg) text-(--moss-templating-input-text) border border-(--moss-templating-input-border) rounded-sm px-0.5 whitespace-nowrap inline-block tracking-tighter [height:20px] [line-height:18px] [vertical-align:middle]";

  const triggerAutomation = useCallback(
    (newValue: string) => {
      if (info.column.id === "key" || info.column.id === "value") {
        if (automationTimeoutRef.current) {
          clearTimeout(automationTimeoutRef.current);
        }

        automationTimeoutRef.current = setTimeout(() => {
          if (!hasFocus.current) {
            info.table.options.meta?.updateData(info.row.index, info.column.id, newValue);
          }
        }, 300);
      }
    },
    [info.row.index, info.column.id, info.table.options.meta]
  );

  const handleFocus = useCallback(() => {
    isActivelyEditing.current = true;
    hasFocus.current = true;
    if (automationTimeoutRef.current) {
      clearTimeout(automationTimeoutRef.current);
    }
  }, []);

  const handleBlur = useCallback(() => {
    isActivelyEditing.current = false;
    hasFocus.current = false;

    if (automationTimeoutRef.current) {
      clearTimeout(automationTimeoutRef.current);
    }

    info.table.options.meta?.updateData(info.row.index, info.column.id, value);
  }, [info.row.index, info.column.id, info.table.options.meta, value]);

  const updateEditorContent = useCallback(
    (text: string) => {
      if (editorRef.current && !isUpdatingContent.current && enableTemplating) {
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
    [enableTemplating, highlightColonVariables]
  );

  const handleTemplatingInput = (e: React.FormEvent<HTMLDivElement>) => {
    if (isUpdatingContent.current) return;

    const plainText = extractPlainText(e.currentTarget.innerHTML);
    setValue(plainText);

    if (hiddenInputRef.current) {
      hiddenInputRef.current.value = plainText;
    }

    triggerAutomation(plainText);

    requestAnimationFrame(() => {
      updateEditorContent(plainText);
    });
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLDivElement>) => {
    if (e.key === "Enter" || e.key === "Tab") {
      e.preventDefault();
      e.currentTarget.blur();
      return;
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
    if (enableTemplating && info.focusOnMount && editorRef.current) {
      const editableDiv = editorRef.current;
      if (editableDiv) {
        editableDiv.focus();

        const range = document.createRange();
        const selection = window.getSelection();
        if (editableDiv.textContent) {
          range.setStart(editableDiv.childNodes[0] || editableDiv, editableDiv.textContent.length);
          range.collapse(true);
          selection?.removeAllRanges();
          selection?.addRange(range);
        }
      }
    }
  }, [enableTemplating, info.focusOnMount]);

  useEffect(() => {
    if (enableTemplating && !isUpdatingContent.current) {
      updateEditorContent(value);
    }
  }, [value, updateEditorContent, enableTemplating]);

  useEffect(() => {
    return () => {
      if (automationTimeoutRef.current) {
        clearTimeout(automationTimeoutRef.current);
      }
    };
  }, []);

  const hasVariables = enableTemplating && hasTemplateVariables(value, highlightColonVariables);

  if (!enableTemplating) {
    return (
      <input
        className={cn(
          "w-full truncate px-2 py-1.5 focus:outline-1 focus:outline-(--moss-primary) disabled:text-(--moss-gray-1)/50",
          {
            "opacity-60": isDisabled,
          }
        )}
        value={value}
        onChange={(e) => {
          setValue(e.target.value);
          triggerAutomation(e.target.value);
        }}
        onFocus={handleFocus}
        autoFocus={info.focusOnMount}
        onBlur={handleBlur}
        placeholder={info.column.id}
      />
    );
  }

  return (
    <div className="relative w-full">
      <input ref={hiddenInputRef} type="text" value={value} onChange={() => {}} className="hidden" />

      <div
        ref={editorRef}
        className={cn(
          "w-full resize-none overflow-hidden border-none bg-transparent px-2 py-1.5 whitespace-nowrap focus:outline-1 focus:outline-(--moss-primary)",
          "word-break-keep-all overflow-wrap-normal font-[Inter] text-[13px]",
          "empty:before:pointer-events-none empty:before:flex empty:before:h-full empty:before:items-center empty:before:leading-[inherit] empty:before:whitespace-nowrap empty:before:text-(--moss-requestpage-placeholder-color) empty:before:content-[attr(data-placeholder)]",
          "[&_*]:inline [&_*]:whitespace-nowrap",
          {
            "opacity-60": isDisabled,
            "text-(--moss-requestpage-text-disabled)": isDisabled,
            "text-(--moss-primary-text)": !isDisabled && !hasVariables,
          }
        )}
        contentEditable
        onInput={handleTemplatingInput}
        onKeyDown={handleKeyDown}
        onPaste={handlePaste}
        onCopy={handleCopy}
        onFocus={handleFocus}
        onBlur={handleBlur}
        data-placeholder={info.column.id}
        suppressContentEditableWarning={true}
      />
    </div>
  );
};
