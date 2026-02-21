function App() {
	return (
		<main className="min-h-screen bg-background text-foreground p-8 flex flex-col gap-6">
			<div>
				<h1 className="text-2xl font-semibold text-foreground">pzql</h1>
				<p className="text-muted-foreground text-sm mt-1">Theme is working</p>
			</div>

			<div className="flex gap-3">
				<button
					type="button"
					className="bg-primary text-primary-foreground px-4 py-2 rounded-md text-sm font-medium"
				>
					Primary
				</button>
				<button
					type="button"
					className="bg-secondary text-secondary-foreground px-4 py-2 rounded-md text-sm font-medium"
				>
					Secondary
				</button>
				<button
					type="button"
					className="bg-destructive text-white px-4 py-2 rounded-md text-sm font-medium"
				>
					Destructive
				</button>
			</div>

			<div className="bg-card text-card-foreground border border-border rounded-lg p-4 max-w-sm">
				<h2 className="font-medium text-foreground">Card</h2>
				<p className="text-muted-foreground text-sm mt-1">
					This uses card, border, and muted-foreground tokens.
				</p>
			</div>

			<div className="flex gap-2">
				<input
					className="bg-input border border-border text-foreground placeholder:text-muted-foreground px-3 py-2 rounded-md text-sm outline-none focus:ring-2 focus:ring-ring"
					placeholder="Input field..."
				/>
				<input
					className="bg-input border border-border text-foreground placeholder:text-muted-foreground px-3 py-2 rounded-md text-sm outline-none focus:ring-2 focus:ring-ring"
					placeholder="Another input..."
				/>
			</div>

			<div className="flex gap-2 flex-wrap">
				{[
					"background",
					"foreground",
					"card",
					"primary",
					"secondary",
					"muted",
					"accent",
					"destructive",
					"border",
				].map((token) => (
					<span
						key={token}
						className="bg-muted text-muted-foreground text-xs px-2 py-1 rounded"
					>
						{token}
					</span>
				))}
			</div>
		</main>
	);
}

export default App;
