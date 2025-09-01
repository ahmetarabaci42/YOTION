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
<img width="1596" height="703" alt="Languages" src="https://github.com/user-attachments/assets/5b6dcf2a-2f1e-4251-b4f9-003b6281b55c" />

<img width="1567" height="885" alt="Languages2" src="https://github.com/user-attachments/assets/5ff7989a-bbe1-48d0-a311-cd0055097dc5" />


## 📘 Vocabulary Section

This is where you can manage the words of the language you are studying.  
All added words appear here with their translations and difficulty levels.

### Screenshot
<img width="1595" height="965" alt="Vocabulary2" src="https://github.com/user-attachments/assets/75728f5f-fedc-4ec7-977d-3b95c2011265" />


---

### ➕ Adding a New Word

From this view you can add new words into your vocabulary list.  
Besides the **Word** and **Translation**, you also have optional fields for **Pronunciation** and **Example Sentence**.  
Each word can be assigned a **Difficulty Level** (e.g., Beginner, Intermediate, Advanced).

### Screenshot
<img width="1587" height="850" alt="Vacabulary1" src="https://github.com/user-attachments/assets/494f96d4-4705-4e68-a208-acbbeb91c8bf" />

