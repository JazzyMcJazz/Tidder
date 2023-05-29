import adapter from '@sveltejs/adapter-vercel';
import { vitePreprocess } from '@sveltejs/kit/vite';
import "dotenv/config";

const PUBLIC_API_URL = process.env.PUBLIC_API_URL || 'https://tidder-api.jazzymcjazz.dk';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),

	kit: {
		adapter: adapter(),
		csrf: {
			checkOrigin: true,
		},
		
		// Content Security Policy
		csp: {
			mode: 'auto',
			directives: {
				'default-src': ['none'],
				'script-src': ['self'],
				'style-src': ['self', 'sha256-xyz4zkCjuC3lZcD2UmnqDG0vurmq12W/XKM5Vd0+MlQ='],
				'connect-src': ['self', PUBLIC_API_URL],
				'img-src': ['self', PUBLIC_API_URL],
				'frame-ancestors': ['none'],
				'base-uri': ['none'],
				'form-action': ['self', PUBLIC_API_URL],
			},
		}
	}
};

export default config;
