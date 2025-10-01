import { useKeys } from "rooks";
import { platform } from "@tauri-apps/plugin-os";
import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

type ShortcutDecl = {
  name: string;
  macos: string[];
  default: string[];
};

const shortcuts: ShortcutDecl[] = [
  {
    name: "println",
    macos: ["Meta", "KeyP"],
    default: ["Control", "KeyP"],
  },
  {
    name: "alert",
    macos: ["Meta", "KeyA"],
    default: ["Control", "KeyA"],
  },
];

const Shortcut = () => {
  useEffect(() => {
    const unlisten = listen("alert", (_event) => {
      alert("Triggering alert using shortcut");
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  const currentPlatform = platform();

  for (const decl of shortcuts) {
    const keys = currentPlatform == "macos" ? decl.macos : decl.default;
    // eslint-disable-next-line react-hooks/rules-of-hooks
    useKeys(keys, (_event) => {
      const commandName = `shortcut.${decl.name}`;
      console.log(`Executing command '${commandName}'`);
      invoke("execute_command", {
        cmd: commandName,
        args: {},
      }).then(() => console.log("Shortcut triggered"));
    });
  }

  return <></>;
};

export default Shortcut;
