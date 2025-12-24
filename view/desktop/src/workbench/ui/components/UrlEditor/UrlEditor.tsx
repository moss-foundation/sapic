import { EditorView, minimalSetup } from "codemirror";
import { useContext, useEffect, useRef } from "react";

import { EndpointViewContext } from "@/workbench/views/EndpointView/EndpointViewContext";
import { history } from "@codemirror/commands";
import { HighlightStyle, LanguageSupport, LRLanguage, syntaxHighlighting } from "@codemirror/language";
import { EditorState } from "@codemirror/state";
import { tags } from "@lezer/highlight";
import { parser } from "@repo/lezer-grammar";

import { tooltipOnVariableHover } from "./extensions/tooltipOnVariableHover";
import { hideVariableBracesPlugin } from "@/workbench/ui/components/UrlEditor/components/HideVariableBraces.tsx";

interface UrlEditorProps {
  value?: string;
  onChange?: (value: string) => void;
}

export const UrlEditor = ({ value = "", onChange }: UrlEditorProps) => {
  //TODO: the context is used to get the path and the query params for the tooltip, we will be passing them as props to the component later
  const ctx = useContext(EndpointViewContext);

  const editorRef = useRef<HTMLDivElement>(null);
  const viewRef = useRef<EditorView | null>(null);

  useEffect(() => {
    const view = viewRef.current;
    if (!view) return;

    const currentValue = view.state.doc.toString();

    if (value === currentValue) return;

    view.dispatch({
      changes: { from: 0, to: view.state.doc.length, insert: value },
      userEvent: "input.prop",
    });
  }, [value]);

  useEffect(() => {
    if (!editorRef.current) return;

    const language = LRLanguage.define({ parser });
    const languageSupport = new LanguageSupport(language);

    const highlightStyle = HighlightStyle.define([
      { tag: tags.punctuation, color: "black" },
      {
        tag: tags.variableName,
        color: "var(--moss-accent)",
        backgroundColor: "var(--moss-accent-secondary)",
        padding: "1px 2px",
        borderRadius: "4px",
        fontWeight: "bold",
      },
      { tag: tags.content, color: "black" },
      { tag: tags.keyword, color: "black" },
    ]);

    const startState = EditorState.create({
      doc: value,
      extensions: [
        minimalSetup,
        history(),
        languageSupport,
        syntaxHighlighting(highlightStyle),
        tooltipOnVariableHover(ctx),
        hideVariableBracesPlugin,

        EditorView.updateListener.of((update) => {
          const isExternal = update.transactions.some((tr) => tr.isUserEvent("input.prop"));
          if (update.docChanged && !isExternal) {
            onChange?.(update.state.doc.toString());
          }
        }),

        //this is to prevent the editor from multiple lines
        EditorState.transactionFilter.of((tr) => {
          return tr.newDoc.lines > 1 ? [] : [tr];
        }),
        EditorView.theme({
          "&.cm-focused": {
            outline: "none",
          },
          ".cm-line": {
            padding: "0",
          },
          ".cm-scroller": {
            overflow: "hidden",
          },
        }),
      ],
    });

    const view = new EditorView({
      state: startState,
      parent: editorRef.current,
    });

    viewRef.current = view;

    return () => view.destroy();

    // we create the editor only once
  }, []);

  return <div ref={editorRef} className="min-w-0" />;
};
