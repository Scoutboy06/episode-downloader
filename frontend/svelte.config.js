import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),
	kit: {
		adapter: adapter(),
		alias: {
			'@/*': './src/lib/*',
			'@/components/*': './src/lib/components/*',
			'@/ui/*': './src/lib/components/ui/*',
			'@/routes/*': './src/routes/*',
			'@/utils.js': './src/lib/utils.ts'
		}
	}
};

export default config;
