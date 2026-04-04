/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        primary: {
          DEFAULT: '#D97757',
          hover: '#C56A4A',
          light: '#E8976F',
        },
        secondary: {
          DEFAULT: '#10B981',
          hover: '#059669',
        },
        accent: {
          DEFAULT: '#F59E0B',
          hover: '#D97706',
        },
        surface: {
          DEFAULT: '#FFFFFF',
          dark: '#2D2D2D',
        },
        background: {
          DEFAULT: '#FAFAFA',
          dark: '#1A1A1A',
        },
        text: {
          DEFAULT: '#1F2937',
          dark: '#F9FAFB',
          secondary: '#6B7280',
        },
        border: {
          DEFAULT: '#E5E7EB',
          dark: '#4B5563',
        },
      },
      fontFamily: {
        sans: ['Inter', '-apple-system', 'BlinkMacSystemFont', 'Segoe UI', 'sans-serif'],
      },
      spacing: {
        '18': '4.5rem',
        '88': '22rem',
      },
      borderRadius: {
        '4xl': '2rem',
      },
      animation: {
        'float': 'float 3s ease-in-out infinite',
        'pulse-slow': 'pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite',
      },
      keyframes: {
        float: {
          '0%, 100%': { transform: 'translateY(0)' },
          '50%': { transform: 'translateY(-20px)' },
        },
      },
    },
  },
  plugins: [],
}
