import { useEffect, useState } from "react";
import { ChatOllama } from "@langchain/ollama";
import { createReactAgent } from "@langchain/langgraph/prebuilt";
import { tool } from "@langchain/core/tools";
import { z } from "zod";
import { CreateWorkspaceInput, createWorkspaceInputSchema, CreateWorkspaceOutput } from "@repo/moss-workspace";
import { invokeTauriIpc } from "@/lib/backend/tauri";

async function setupAgent() {
  // Running a test model locally using Ollama
  const llm = new ChatOllama({ model: "qwen3:4b" });

  const alertSchema = z.object({
    message: z.string().describe("Message to be shown in the alert popup"),
  });

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
      name: "create workspace",
      description: "Create a workspace.",
      schema: createWorkspaceInputSchema,
    }
  );

  const agent = createReactAgent({ llm, tools: [alertTool, createWorkspaceTool] });
  return agent;
}

function useAgent() {
  const [agent, setAgent] = useState<any>(null);
  useEffect(() => {
    setupAgent().then((agent) => {
      setAgent(agent);
    });
  }, []);
  return agent;
}

const LangchainAgent = () => {
  const agent = useAgent();

  function handleClick() {
    const prompt = `Please create a workspace with the following name: TestWorkspace`;
    agent
      .invoke({
        messages: [{ role: "user", content: prompt }],
      })
      .then((response) => console.log(response));
  }

  return (
    <div>
      <button className="cursor-pointer rounded bg-green-500 p-2 text-white hover:bg-green-600" onClick={handleClick}>
        Test prompt
      </button>
    </div>
  );
};

export default LangchainAgent;
