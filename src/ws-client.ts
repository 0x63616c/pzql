const WS_URL = "ws://127.0.0.1:1421/ws";

type PendingCall = {
	resolve: (value: unknown) => void;
	reject: (reason: unknown) => void;
};

const pending = new Map<number, PendingCall>();
const eventListeners = new Map<string, Set<(payload: unknown) => void>>();
let seq = 0;
let ws: WebSocket | null = null;

function getSocket(): WebSocket {
	if (ws && ws.readyState === WebSocket.OPEN) return ws;

	ws = new WebSocket(WS_URL);

	ws.onmessage = (e) => {
		const msg = JSON.parse(e.data as string) as {
			id?: number;
			result?: unknown;
			error?: unknown;
			event?: string;
			payload?: unknown;
		};

		if (typeof msg.event === "string") {
			eventListeners.get(msg.event)?.forEach((cb) => {
				cb(msg.payload);
			});
			return;
		}

		if (msg.id !== undefined) {
			const p = pending.get(msg.id);
			if (!p) return;
			pending.delete(msg.id);
			if (msg.error != null) {
				p.reject(msg.error);
			} else {
				p.resolve(msg.result);
			}
		}
	};

	ws.onerror = (e) => console.error("ws-client error:", e);

	return ws;
}

export function wsInvoke<T>(
	cmd: string,
	args: Record<string, unknown> = {},
): Promise<T> {
	return new Promise((resolve, reject) => {
		const id = ++seq;
		pending.set(id, { resolve: resolve as (v: unknown) => void, reject });
		getSocket().send(JSON.stringify({ id, cmd, args }));
	});
}

export function wsListen<T>(
	event: string,
	cb: (payload: T) => void,
): () => void {
	let set = eventListeners.get(event);
	if (!set) {
		set = new Set();
		eventListeners.set(event, set);
	}
	set.add(cb as (payload: unknown) => void);
	getSocket(); // ensure socket is open
	return () => set.delete(cb as (payload: unknown) => void);
}
