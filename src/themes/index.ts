export interface Theme {
	background: string;
	foreground: string;
	card: string;
	cardForeground: string;
	popover: string;
	popoverForeground: string;
	primary: string;
	primaryForeground: string;
	secondary: string;
	secondaryForeground: string;
	muted: string;
	mutedForeground: string;
	accent: string;
	accentForeground: string;
	destructive: string;
	border: string;
	input: string;
	ring: string;
	radius: string;
	sidebar: string;
	sidebarForeground: string;
	sidebarPrimary: string;
	sidebarPrimaryForeground: string;
	sidebarAccent: string;
	sidebarAccentForeground: string;
	sidebarBorder: string;
	sidebarRing: string;
}

export function applyTheme(theme: Theme): void {
	const root = document.documentElement;
	root.style.setProperty("--background", theme.background);
	root.style.setProperty("--foreground", theme.foreground);
	root.style.setProperty("--card", theme.card);
	root.style.setProperty("--card-foreground", theme.cardForeground);
	root.style.setProperty("--popover", theme.popover);
	root.style.setProperty("--popover-foreground", theme.popoverForeground);
	root.style.setProperty("--primary", theme.primary);
	root.style.setProperty("--primary-foreground", theme.primaryForeground);
	root.style.setProperty("--secondary", theme.secondary);
	root.style.setProperty("--secondary-foreground", theme.secondaryForeground);
	root.style.setProperty("--muted", theme.muted);
	root.style.setProperty("--muted-foreground", theme.mutedForeground);
	root.style.setProperty("--accent", theme.accent);
	root.style.setProperty("--accent-foreground", theme.accentForeground);
	root.style.setProperty("--destructive", theme.destructive);
	root.style.setProperty("--border", theme.border);
	root.style.setProperty("--input", theme.input);
	root.style.setProperty("--ring", theme.ring);
	root.style.setProperty("--radius", theme.radius);
	root.style.setProperty("--sidebar", theme.sidebar);
	root.style.setProperty("--sidebar-foreground", theme.sidebarForeground);
	root.style.setProperty("--sidebar-primary", theme.sidebarPrimary);
	root.style.setProperty(
		"--sidebar-primary-foreground",
		theme.sidebarPrimaryForeground,
	);
	root.style.setProperty("--sidebar-accent", theme.sidebarAccent);
	root.style.setProperty(
		"--sidebar-accent-foreground",
		theme.sidebarAccentForeground,
	);
	root.style.setProperty("--sidebar-border", theme.sidebarBorder);
	root.style.setProperty("--sidebar-ring", theme.sidebarRing);
}
