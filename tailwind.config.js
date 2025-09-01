/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        'notion-bg': '#f7f6f3',
        'notion-text': '#37352f',
        'notion-muted': '#9b9a97',
        'notion-border': '#e9e9e7',
        'notion-hover': '#f1f1ef',
        'notion-accent': '#2383e2',
      },
      fontFamily: {
        'sans': ['-apple-system', 'BlinkMacSystemFont', 'Segoe UI', 'Roboto', 'sans-serif'],
      },
    },
  },
  plugins: [],
}
