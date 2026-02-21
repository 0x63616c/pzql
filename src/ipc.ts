import { commands as tauriCommands } from "./bindings";
import { wsCommands } from "./ws-bindings";

const isTauri = "__TAURI_INTERNALS__" in window;

// All app code imports from here. Never import from bindings.ts directly.
export const commands = isTauri ? tauriCommands : wsCommands;
