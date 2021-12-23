import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [
		{
			name: 'modify-response-headers',
			configureServer: (server) => {
				server.middlewares.use((_, res, next) => {
					res.setHeader('Cross-Origin-Embedder-Policy', 'require-corp');
					res.setHeader('Cross-Origin-Opener-Policy', 'same-origin');
					next();
				});
			},
		},
	],
});
