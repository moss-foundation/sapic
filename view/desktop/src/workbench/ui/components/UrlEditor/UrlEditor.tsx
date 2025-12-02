import { useContext, useEffect, useRef } from "react";

import { EndpointViewContext } from "@/workbench/views/EndpointView/EndpointViewContext";
import { HighlightStyle, LanguageSupport, LRLanguage, syntaxHighlighting, syntaxTree } from "@codemirror/language";
import { EditorState } from "@codemirror/state";
import { EditorView } from "@codemirror/view";
import { tags } from "@lezer/highlight";
import { parser } from "@repo/lezer-grammar";

import { tooltipOnVariableHover } from "./extensions/tooltipOnVariableHover";

interface UrlEditorProps {
  value?: string;
  onChange?: (value: string) => void;
}

export const UrlEditor = ({ value = "/:test/{{var}}", onChange }: UrlEditorProps) => {
  const ctx = useContext(EndpointViewContext);

  const editorRef = useRef<HTMLDivElement>(null);
  const viewRef = useRef<EditorView | null>(null);

  useEffect(() => {
    if (!editorRef.current) return;

    const language = LRLanguage.define({ parser });
    const languageSupport = new LanguageSupport(language);

    const highlightStyle = HighlightStyle.define([
      // {{ and }} and / and :
      { tag: tags.punctuation, color: "black" }, //#9e9e9e
      // The variable names (env, test1)
      { tag: tags.variableName, color: "blue", fontWeight: "bold" }, //#9c27b0
      // The normal URL text
      { tag: tags.content, color: "black" }, //#424242
      // (Optional) If you kept the parent tag mapping
      { tag: tags.keyword, color: "black" }, //#9c27b0
    ]);

    const state = EditorState.create({
      doc: value,
      extensions: [
        languageSupport,
        syntaxHighlighting(highlightStyle),
        tooltipOnVariableHover(ctx),
        EditorView.updateListener.of((update) => {
          if (update.docChanged && onChange) {
            onChange(update.state.doc.toString());
          }
          // Track parser output changes
          if (update.docChanged || update.viewportChanged || update.selectionSet) {
            const currentTree = syntaxTree(update.state);
            console.log("currentTree", currentTree);
          }
        }),
      ],
    });

    const view = new EditorView({
      state,
      parent: editorRef.current,
    });

    viewRef.current = view;

    return () => view.destroy();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  // Update editor content when value prop changes
  useEffect(() => {
    if (viewRef.current && value !== viewRef.current.state.doc.toString()) {
      viewRef.current.dispatch({
        changes: {
          from: 0,
          to: viewRef.current.state.doc.length,
          insert: value,
        },
      });
    }
  }, [value]);

  return <div ref={editorRef} className="border border-gray-200 p-2" />;
};
