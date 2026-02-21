import { useCounterStore } from "../stores/counter";

export function CounterDemo() {
	const count = useCounterStore((s) => s.count);
	const increment = useCounterStore((s) => s.increment);
	const decrement = useCounterStore((s) => s.decrement);
	const reset = useCounterStore((s) => s.reset);

	return (
		<div>
			<h2>Counter Demo</h2>
			<p>Count: {count}</p>
			<div className="row">
				<button type="button" onClick={decrement}>
					-
				</button>
				<button type="button" onClick={reset}>
					reset
				</button>
				<button type="button" onClick={increment}>
					+
				</button>
			</div>
		</div>
	);
}
