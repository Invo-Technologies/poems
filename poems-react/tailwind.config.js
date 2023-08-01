/** @type {import('tailwindcss').Config} */
import daisyui from 'daisyui';

export default {
  content: [
    './src/**/*.{js,ts,jsx,tsx}',
    './src/components/**/*.{js,ts,jsx,tsx}',
  ],
  theme: {
    extend: {
      colors: {
        account: { dark: '#4682B4', light: '#B0E0E6' },
        generation: { dark: '#32CD32', light: '#90EE90' },
        aleo: { dark: '#0E5DFB', light: '#01E0F1' },
        return: { dark: '#9370DB', light: '#B19CD9' },
        decrypt: { dark: '#FF8C00', light: '#FFD700' },
      },
    },
  },
  daisyui: {
    themes: [
      {
        mytheme: {
          primary: '#641ae6',

          secondary: '#d926a9',

          accent: '#1fb2a6',

          neutral: '#2a323c',

          'base-100': '#1d232a',

          info: '#3abff8',

          success: '#36d399',

          warning: '#fbbd23',

          error: '#f87272',
        },
      },
    ],
  },
  plugins: [daisyui],
};
