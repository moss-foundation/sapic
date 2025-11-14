import { useEffect, useState } from "react";
import { z } from "zod";

import { invokeTauriIpc } from "@/infra/ipc/tauri";
import { ChatPromptTemplate, HumanMessagePromptTemplate, SystemMessagePromptTemplate } from "@langchain/core/prompts";
import { tool } from "@langchain/core/tools";
import { createReactAgent } from "@langchain/langgraph/prebuilt";
import { ChatMistralAI } from "@langchain/mistralai";
import { CreateWorkspaceInput, createWorkspaceInputSchema, CreateWorkspaceOutput } from "@repo/window";

async function setupModel() {
  const result = await invokeTauriIpc<string>("get_mistral_api_key");
  if (result.status == "error") {
    throw new Error("Mistral API key not found");
  }

  return new ChatMistralAI({
    apiKey: result.data,
    model: "magistral-medium-latest",
    temperature: 0.5,
  });
}

async function setupToolAgent() {
  // Running a test MistralAI model
  const model = await setupModel();

  const alertSchema = z.object({
    message: z.string().describe("Message to be shown in the alert popup"),
  });

  // @ts-expect-error We will fix the demo when we revisit AI
  const alertTool = tool(
    async ({ message }) => {
      alert(message);
      return "Successfully sent an alert popup";
    },
    {
      name: "alert",
      description: "Send an alert popup with a given message.",
      schema: alertSchema,
    }
  );

  // @ts-expect-error We will fix the demo when we revisit AI
  const createWorkspaceTool = tool(
    async (input) => {
      const result = await invokeTauriIpc<CreateWorkspaceOutput, CreateWorkspaceInput>("create_workspace", {
        input,
      });

      if (result.status === "error") {
        throw new Error(String(result.error));
      }

      return result.data;
    },
    {
      name: "create_workspace",
      description: "Create a workspace.",
      schema: createWorkspaceInputSchema,
    }
  );

  // @ts-expect-error We will fix the demo when we revisit AI
  return createReactAgent({ llm: model, tools: [alertTool, createWorkspaceTool] });
}

function useToolAgent() {
  const [agent, setAgent] = useState<Awaited<ReturnType<typeof setupToolAgent>> | null>(null);
  useEffect(() => {
    setupToolAgent().then(setAgent);
  }, []);
  return agent;
}

async function setupJsonGenerationAgent() {
  // Running a test MistralAI model
  const model = await setupModel();

  const promptTemplate = ChatPromptTemplate.fromMessages([
    SystemMessagePromptTemplate.fromTemplate(
      "You are a JSON data generator. FOLLOW THESE RULES EXACTLY:\n" +
        "1. Output only valid JSON (a single JSON value: object or array). Do NOT output any surrounding text, explanation, code fences, or markdown. Nothing but the JSON raw text.\n" +
        "2. Produce data that conforms to the provided JSON Schema or the verbal description in the user prompt. Respect types, required fields, enums, formats (email, uuid, date-time, uri), numeric ranges, min/max length and minItems/maxItems.\n" +
        '3. Use ISO 8601 for dates (e.g. "2025-08-10T14:30:00Z"). Use RFC 4122 v4 format for UUIDs. Use realistic-looking values (but fake) for names, emails, addresses unless options request otherwise.\n' +
        '4. If constraints conflict, contains external references or otherwise cannot be satisfied, return a JSON object: {{"error":"<short human-friendly reason>"}} and nothing else.\n' +
        '5. Do not invent extra top-level keys when schema has "additionalProperties": false. Only include keys allowed by the schema unless the user explicitly requests extras.\n' +
        "6. Do not show your reasoning process."
    ),
    HumanMessagePromptTemplate.fromTemplate("{input}"),
  ]);

  return promptTemplate.pipe(model);
}

function useJsonGenerationAgent() {
  const [agent, setAgent] = useState<Awaited<ReturnType<typeof setupJsonGenerationAgent>> | null>(null);
  useEffect(() => {
    setupJsonGenerationAgent().then(setAgent);
  }, []);
  return agent;
}

const AIDemo = () => {
  const toolAgent = useToolAgent();
  const jsonGenerationAgent = useJsonGenerationAgent();
  const [jsonSchema, setJsonSchema] = useState("Input Json Schema");
  const [jsonOutput, setJsonOutput] = useState("Output Json Object");

  function handleToolTestButton() {
    const prompt = `Please create a workspace with the following name: TestWorkspace. The mode should be design_first, and should not be opened on creation.`;
    toolAgent
      ?.invoke({
        messages: [{ role: "user", content: prompt }],
      })
      .then((response) => console.log(response));
  }

  async function handleJsonGenerationButton() {
    const stream = await jsonGenerationAgent?.stream({ input: jsonSchema });
    let currentOutput = "";
    for await (const chunk of stream!) {
      currentOutput += chunk.content;
      setJsonOutput(currentOutput);
    }
  }

  return (
    <div>
      <p>
        In order to test AI functionalities, you have to get a Mistral AI API Key at
        https://console.mistral.ai/api-keys.
      </p>
      <p>Then, create a ".env" file at the root, and type "MISTRAL_AI_API_KEY = YOUR_API_KEY" in it.</p>
      <button
        className="cursor-pointer rounded bg-green-500 p-2 text-white hover:bg-green-600"
        onClick={handleToolTestButton}
      >
        Test Tools
      </button>
      <div>
        <textarea
          value={jsonSchema}
          onChange={(e) => setJsonSchema(e.target.value)}
          className="h-50 mt-4 w-1/2 bg-white"
        />
        <textarea value={jsonOutput} readOnly className="h-50 mt-4 w-1/2 bg-white" />
      </div>
      <button
        className="cursor-pointer rounded bg-green-500 p-2 text-white hover:bg-green-600"
        onClick={handleJsonGenerationButton}
      >
        Test Json Generation
      </button>
    </div>
  );
};

export default AIDemo;
