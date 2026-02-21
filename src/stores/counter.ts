import { create } from "zustand";

type CounterStore = {
	count: number;
	increment: () => void;
	decrement: () => void;
	reset: () => void;
};

export const useCounterStore = create<CounterStore>((set) => ({
	count: 0,
	increment: () => set((s) => ({ count: s.count + 1 })),
	decrement: () => set((s) => ({ count: s.count - 1 })),
	reset: () => set({ count: 0 }),
}));
