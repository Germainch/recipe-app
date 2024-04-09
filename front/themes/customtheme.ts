
import type { CustomThemeConfig } from '@skeletonlabs/tw-plugin';

export const myCustomTheme: CustomThemeConfig = {
    name: 'my-custom-theme',
    properties: {
		// =~= Theme Properties =~=
		"--theme-font-family-base": `system-ui`,
		"--theme-font-family-heading": `system-ui`,
		"--theme-font-color-base": "0 0 0",
		"--theme-font-color-dark": "255 255 255",
		"--theme-rounded-base": "4px",
		"--theme-rounded-container": "4px",
		"--theme-border-base": "2px",
		// =~= Theme On-X Colors =~=
		"--on-primary": "255 255 255",
		"--on-secondary": "255 255 255",
		"--on-tertiary": "255 255 255",
		"--on-success": "0 0 0",
		"--on-warning": "0 0 0",
		"--on-error": "255 255 255",
		"--on-surface": "255 255 255",
		// =~= Theme Colors  =~=
		// primary | #415f86
		"--color-primary-50": "227 231 237", // #e3e7ed
		"--color-primary-100": "217 223 231", // #d9dfe7
		"--color-primary-200": "208 215 225", // #d0d7e1
		"--color-primary-300": "179 191 207", // #b3bfcf
		"--color-primary-400": "122 143 170", // #7a8faa
		"--color-primary-500": "65 95 134", // #415f86
		"--color-primary-600": "59 86 121", // #3b5679
		"--color-primary-700": "49 71 101", // #314765
		"--color-primary-800": "39 57 80", // #273950
		"--color-primary-900": "32 47 66", // #202f42
		// secondary | #4c4983
		"--color-secondary-50": "228 228 236", // #e4e4ec
		"--color-secondary-100": "219 219 230", // #dbdbe6
		"--color-secondary-200": "210 210 224", // #d2d2e0
		"--color-secondary-300": "183 182 205", // #b7b6cd
		"--color-secondary-400": "130 128 168", // #8280a8
		"--color-secondary-500": "76 73 131", // #4c4983
		"--color-secondary-600": "68 66 118", // #444276
		"--color-secondary-700": "57 55 98", // #393762
		"--color-secondary-800": "46 44 79", // #2e2c4f
		"--color-secondary-900": "37 36 64", // #252440
		// tertiary | #704700
		"--color-tertiary-50": "234 227 217", // #eae3d9
		"--color-tertiary-100": "226 218 204", // #e2dacc
		"--color-tertiary-200": "219 209 191", // #dbd1bf
		"--color-tertiary-300": "198 181 153", // #c6b599
		"--color-tertiary-400": "155 126 77", // #9b7e4d
		"--color-tertiary-500": "112 71 0", // #704700
		"--color-tertiary-600": "101 64 0", // #654000
		"--color-tertiary-700": "84 53 0", // #543500
		"--color-tertiary-800": "67 43 0", // #432b00
		"--color-tertiary-900": "55 35 0", // #372300
		// success | #84cc16
		"--color-success-50": "237 247 220", // #edf7dc
		"--color-success-100": "230 245 208", // #e6f5d0
		"--color-success-200": "224 242 197", // #e0f2c5
		"--color-success-300": "206 235 162", // #ceeba2
		"--color-success-400": "169 219 92", // #a9db5c
		"--color-success-500": "132 204 22", // #84cc16
		"--color-success-600": "119 184 20", // #77b814
		"--color-success-700": "99 153 17", // #639911
		"--color-success-800": "79 122 13", // #4f7a0d
		"--color-success-900": "65 100 11", // #41640b
		// warning | #EAB308
		"--color-warning-50": "252 244 218", // #fcf4da
		"--color-warning-100": "251 240 206", // #fbf0ce
		"--color-warning-200": "250 236 193", // #faecc1
		"--color-warning-300": "247 225 156", // #f7e19c
		"--color-warning-400": "240 202 82", // #f0ca52
		"--color-warning-500": "234 179 8", // #EAB308
		"--color-warning-600": "211 161 7", // #d3a107
		"--color-warning-700": "176 134 6", // #b08606
		"--color-warning-800": "140 107 5", // #8c6b05
		"--color-warning-900": "115 88 4", // #735804
		// error | #D41976
		"--color-error-50": "249 221 234", // #f9ddea
		"--color-error-100": "246 209 228", // #f6d1e4
		"--color-error-200": "244 198 221", // #f4c6dd
		"--color-error-300": "238 163 200", // #eea3c8
		"--color-error-400": "225 94 159", // #e15e9f
		"--color-error-500": "212 25 118", // #D41976
		"--color-error-600": "191 23 106", // #bf176a
		"--color-error-700": "159 19 89", // #9f1359
		"--color-error-800": "127 15 71", // #7f0f47
		"--color-error-900": "104 12 58", // #680c3a
		// surface | #474862
		"--color-surface-50": "227 228 231", // #e3e4e7
		"--color-surface-100": "218 218 224", // #dadae0
		"--color-surface-200": "209 209 216", // #d1d1d8
		"--color-surface-300": "181 182 192", // #b5b6c0
		"--color-surface-400": "126 127 145", // #7e7f91
		"--color-surface-500": "71 72 98", // #474862
		"--color-surface-600": "64 65 88", // #404158
		"--color-surface-700": "53 54 74", // #35364a
		"--color-surface-800": "43 43 59", // #2b2b3b
		"--color-surface-900": "35 35 48", // #232330

	}
}