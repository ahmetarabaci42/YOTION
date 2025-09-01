# Yotion

A Notion‑inspired desktop app powered by **React + Tauri**. Runs on the web during development and ships as a lightweight desktop app for Windows, macOS, and Linux.

---

## 🚀 Features

* ⚡ **Vite + React + TypeScript** for a fast DX
* 🎨 **Tailwind CSS** with a Notion‑like theme (custom colors & system fonts)
* 🖥️ **Tauri integration** for multi‑platform desktop builds
* 📝 **Monaco Editor** support for rich code/text editing

---

## 🔧 Installation

```bash
git clone <repo-url>
cd yotion
npm install
```

---

## 💻 Development

```bash
# Web dev server
npm run dev

# Desktop (Tauri) dev
npm run tauri:dev
```

> The app bootstraps from `index.html` and `src/main.tsx`. Tailwind scans `index.html` and everything under `src/**`.

---

## 📦 Build

```bash
# Web production build
npm run build

# Desktop build (Tauri)
npm run tauri:build
```

> Web builds output to `dist/`. Tauri creates platform‑specific bundles under `src-tauri/target/**/bundle/**`.

---

## 🛠️ Tech Stack

* [React](https://react.dev/)
* [TypeScript](https://www.typescriptlang.org/)
* [Vite](https://vitejs.dev/)
* [Tailwind CSS](https://tailwindcss.com/)
* [Tauri](https://tauri.app/)
* [Monaco Editor](https://microsoft.github.io/monaco-editor/)

---

## 📚 Notes

* Node.js 18+ recommended. For desktop builds, ensure the Tauri prerequisites for your OS (Rust toolchain, platform SDKs) are installed. You can still use the web build without Tauri.
* The **Languages** section is where you add the languages you want to study. You can add as many as you like depending on what you are currently working on. Each language includes a name, code, and flag emoji so you can better organize your vocabulary notes.

---

## 🌐 Languages Section

This section is where you add the languages you want to study.  
You can add as many as you like depending on what you are currently working on.  
Each language includes a **name**, **code**, and **flag emoji** so you can better organize your vocabulary notes.

### Screenshots
![Languages Overview](./6475f1dd-3d63-4fc0-a004-8161a4e6f7d4.png)
![Add Language Form](./b22b5007-3cfb-40b6-9f0f-f0dcec51a998.png)
