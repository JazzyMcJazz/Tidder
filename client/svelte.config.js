import adapter from '@sveltejs/adapter-vercel';
import { vitePreprocess } from '@sveltejs/kit/vite';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),

	kit: {
		adapter: adapter(),
		csrf: {
			checkOrigin: true,
		},
		csp: {
			mode: 'hash',
			directives: {
				'default-src': ['none'],
				'script-src': ['self'],
				'style-src': ['self'],
				'connect-src': ['self', 'http://localhost:3000'],
				'img-src': ['self', 'http://localhost:3000'],
			},
			//frame-src 'none'; form-action 'self' ${PUBLIC_API_URL}; base-uri 'none';`,
		}
	}
};

export default config;
