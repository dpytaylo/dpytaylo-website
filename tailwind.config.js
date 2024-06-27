/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "./app/src/**/*.rs"],
  theme: {
    extend: {
      backgroundImage: {
        "menu-icon": "url('/assets/menu.svg')",
        "close-icon": "url('/assets/close.svg')",
        "mail-icon": "url('/assets/mail.svg')",
        "github-icon": "url('/assets/github.svg')",
        "telegram-icon": "url('/assets/telegram.svg')",
        "discord-icon": "url('/assets/discord.svg')",
      },

      minWidth: {
        '56': '14rem',
      },

      screens: {
        "hmw": "900px",
      },

      fontFamily: {
        "inter": ["Inter"],
        "mono": ["ui-monospace", "SFMono-Regular", "Menlo", "Monaco", "Consolas", "'Liberation Mono'", "'Courier New'", "monospace"]
      },

      keyframes: {
        animatedgradient: {
          '0%': { backgroundPosition: '0% 50%' },
          '50%': { backgroundPosition: '100% 50%' },
          '100%': { backgroundPosition: '0% 50%' },
        },
      },

      backgroundSize: {
        '300%': '300%',
      },

      animation: {
        gradient: 'animatedgradient 9s ease infinite alternate',
      },
    },
  },
  plugins: [],
}
