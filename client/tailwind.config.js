/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  darkMode: 'media', // or 'class'
  theme: {
    extend: {
      backgroundColor: {
        'primary': '#1F2937',
        'secondary': '#4B5563',
        'tertiary': '#6B7280',
        'accent': '#10B981',
      },
      textColor: {
        'light': '#E5E7EB',
        'dark': '#1F2937',
        'tertiary': '#6B7280',
        'accent': '#10B981',
      },
      borderColor: {
        'light': '#E5E7EB',
        'dark': '#1F2937',
        'tertiary': '#6B7280',
        'accent': '#10B981',
      },
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
}

