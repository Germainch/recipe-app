import { join } from 'path'
import type { Config } from 'tailwindcss'
// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-expect-error
import forms from '@tailwindcss/forms';
// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-expect-error
import typography from '@tailwindcss/typography';
import { skeleton } from '@skeletonlabs/tw-plugin'


export default {
	darkMode: 'class',
	content: ['./src/**/*.{html,js,svelte,ts}', join(require.resolve('@skeletonlabs/skeleton'), '../**/*.{html,js,svelte,ts}')],
	theme: {
		extend: {},
	},
	plugins: [
		forms,
		typography,
		skeleton({
			themes: {
				preset: [
					{
						name: 'skeleton',
						enhancements: true,
					},
					"modern",
					"crimson",
					"hamlindigo"
				],
			},
		}),
	],
} satisfies Config;
