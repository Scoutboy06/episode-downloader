import adapter from '@sveltejs/adapter-node';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),
	kit: {
		adapter: adapter(),
		alias: {
			"@/*": "./src/lib/*",
			"@/server": "./src/lib/server/*",
			"@/components/*": "./src/lib/components/*",
			"@/ui/*": "./src/lib/components/ui/*",
			"@/routes/*": "./src/routes/*",
			"@/utils.js": "./src/lib/utils.ts"
		}
	}
};

export default config;
