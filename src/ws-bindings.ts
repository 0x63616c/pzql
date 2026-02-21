import type { commands as BoundCommands } from "./bindings";
import { wsInvoke } from "./ws-client";

// TypeScript errors here if bindings.ts adds a command you haven't implemented.
// That is intentional - it forces you to add the WS equivalent.
export const wsCommands = {
	greet: (name: string) => wsInvoke<string>("greet", { name }),
} satisfies typeof BoundCommands;
