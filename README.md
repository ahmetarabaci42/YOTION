# YOTION

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


## 🃏 Flashcards Section

The Flashcards feature helps you review your vocabulary in an interactive way.  
Each session presents a word or phrase, and you try to recall the answer before revealing it.  
This method supports spaced repetition and makes memorization more effective.

### Screenshots
<img width="1602" height="631" alt="FlashCards1" src="https://github.com/user-attachments/assets/1c3dcd1a-2c2e-4c6d-93c2-9b17d8857676" />

<img width="1601" height="727" alt="FlashCards2" src="https://github.com/user-attachments/assets/e42e8b01-cb48-4469-a93a-21b92aecd959" />


---

### 🔄 Review Flow

- When a card appears, try to recall the translation or meaning.  
- Click **Show Answer** to reveal the solution.  
- After seeing the answer, you can grade your response as **Hard**, **Good**, or **Easy**.  
- The system adjusts the review schedule based on your choice, making difficult words appear more often while mastered words appear less frequently.


## 📝 Tech Notes Section

The **Tech Notes** feature allows you to organize your technical knowledge under different spaces and snippets.

---

### Spaces Overview
This area lets you create separate spaces for different topics.  
You can think of each space as a folder where you will group your related notes.  

📷 Example:  
<img width="1601" height="671" alt="TechNotes1" src="https://github.com/user-attachments/assets/daf9ce95-636f-4d5a-93a3-020af90bca1b" />


---

### Inside a Space
When you open a space, you can view all snippets that belong to it.  
Each snippet contains a title, description, and optionally code examples.  

📷 Example:  
<img width="1571" height="982" alt="TechNotes2" src="https://github.com/user-attachments/assets/2a80c7c8-ef1d-4b15-a9c8-09a822be8ea9" />


---

### Creating a Snippet
This view allows you to add new snippets into a space.  
You can provide a **title**, **description**, select a **language**, and write your code or notes.  
This makes it easy to organize reusable code snippets or quick references.  

📷 Example:  
<img width="847" height="977" alt="TechNotes3" src="https://github.com/user-attachments/assets/e9b3a030-4d1c-4add-b92a-7804a4c9a74f" />


---

### Multiple Spaces Example
You can switch between different spaces to manage notes in various categories.  
For example, one space can hold C++ code snippets, another for hardware schematics, and another for Rust.  

📷 Examples:  
<img width="1600" height="557" alt="TechNotes4" src="https://github.com/user-attachments/assets/75a1e46b-1b09-4ac5-83ef-78a5ca67658a" />
  
<img width="1596" height="973" alt="TechNotes5" src="https://github.com/user-attachments/assets/c0dbfba3-e95f-4634-aff9-00ca846a6fc7" />


