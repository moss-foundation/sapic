export interface TemplateVariable {
  name: string;
  startIndex: number;
  endIndex: number;
}

// Extract variables from template string (e.g., {{variable}} and :variable)
export const extractVariables = (text: string, includeColonVariables = false): string[] => {
  const variables: string[] = [];

  const bracesRegex = /\{\{([^}]+)\}\}/g;
  let match;

  while ((match = bracesRegex.exec(text)) !== null) {
    const variable = match[1].trim();
    if (variable && !variables.includes(variable)) {
      variables.push(variable);
    }
  }

  // Extract :variable patterns if enabled
  if (includeColonVariables) {
    const colonRegex = /:([a-zA-Z_][a-zA-Z0-9_]*)/g;
    while ((match = colonRegex.exec(text)) !== null) {
      const variable = match[1];
      if (variable && !variables.includes(variable)) {
        variables.push(variable);
      }
    }
  }

  return variables;
};

export const createHighlightedHTML = (
  text: string,
  includeColonVariables = false,
  highlightedVariableStyles?: string
): string => {
  const defaultStyles =
    "background-(--moss-controls-background) text-(--moss-controls-foreground) border border-(--moss-controls-border) rounded-sm px-0.5 whitespace-nowrap inline-block tracking-tighter [height:20px] [line-height:18px] [vertical-align:middle]";
  const styles = highlightedVariableStyles || defaultStyles;

  const textWithNbsp = text.replace(/ /g, "&nbsp;");

  if (includeColonVariables) {
    const combinedRegex = /(\{\{[^}]*\}\}|:[a-zA-Z_][a-zA-Z0-9_]*)/g;
    return textWithNbsp.replace(combinedRegex, (match) => {
      return `<span class="${styles}">${match}</span>`;
    });
  } else {
    const regex = /(\{\{[^}]*\}\})/g;
    return textWithNbsp.replace(regex, (match) => {
      return `<span class="${styles}">${match}</span>`;
    });
  }
};

export const extractPlainText = (html: string): string => {
  const normalizedHtml = html.replace(/&nbsp;/g, " ");
  const div = document.createElement("div");
  div.innerHTML = normalizedHtml;
  return div.textContent || div.innerText || "";
};

export const getTextOffset = (container: Node, targetNode: Node, targetOffset: number): number => {
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

export const findTextNodeAtOffset = (container: Node, targetOffset: number): { node: Node; offset: number } | null => {
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

export const hasTemplateVariables = (text: string, includeColonVariables = false): boolean => {
  const bracePattern = /\{\{[^}]+\}\}/;
  const hasColorVariables = includeColonVariables && /:([a-zA-Z_][a-zA-Z0-9_]*)/.test(text);
  return bracePattern.test(text) || hasColorVariables;
};

export const setCursorPosition = (element: HTMLElement, offset: number): void => {
  try {
    const textPosition = findTextNodeAtOffset(element, offset);
    if (textPosition) {
      const range = document.createRange();
      const selection = window.getSelection();

      range.setStart(textPosition.node, Math.min(textPosition.offset, (textPosition.node.textContent || "").length));
      range.collapse(true);
      selection?.removeAllRanges();
      selection?.addRange(range);
    }
  } catch (e) {}
};

export const getCurrentCursorOffset = (element: HTMLElement): number => {
  try {
    const selection = window.getSelection();
    if (selection && selection.rangeCount > 0 && element.contains(selection.focusNode)) {
      const range = selection.getRangeAt(0);
      return getTextOffset(element, range.startContainer, range.startOffset);
    }
  } catch (e) {}
  return 0;
};
