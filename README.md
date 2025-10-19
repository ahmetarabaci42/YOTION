<a id="en"></a>

<p align="center">
  <img width="430" height="297" alt="Logo1" src="https://github.com/user-attachments/assets/3e436d21-771c-41da-8279-9974fe2a618f" />
</p>
<h1 align="center">YOTION</h1>

<p align="center">
  A desktop application inspired by <b>Notion</b>, built with <b>React + Tauri</b>.<br/>
  Runs on the web during development and ships as a lightweight desktop app for Windows, macOS, and Linux.
</p>

<p align="center">
  <a href="#tr">🇹🇷 Türkçe</a> • <a href="#en">🇬🇧 English</a>
</p>


## 🚀 Features

* ⚡ **Vite + React + TypeScript** for a fast DX
* 🎨 **Tailwind CSS** with a Notion‑like theme (custom colors & system fonts)
* 🖥️ **Tauri integration** for multi‑platform desktop builds
* 📝 **Monaco Editor** support for rich code/text editing
* 🔐 **Personal Vault** for secure storage of passwords and sensitive data
* 🌐 **Language Learning** with vocabulary management and flashcards
* 📝 **Tech Notes** for organizing code snippets and technical knowledge
* 📂 **Project Management** with tasks and progress tracking
* 🗓️ **Planner** with events and daily notes

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


<img width="1601" height="671" alt="TechNotes1" src="https://github.com/user-attachments/assets/daf9ce95-636f-4d5a-93a3-020af90bca1b" />


---

### Inside a Space
When you open a space, you can view all snippets that belong to it.  
Each snippet contains a title, description, and optionally code examples.  

<img width="1571" height="982" alt="TechNotes2" src="https://github.com/user-attachments/assets/2a80c7c8-ef1d-4b15-a9c8-09a822be8ea9" />


---

### Creating a Snippet
This view allows you to add new snippets into a space.  
You can provide a **title**, **description**, select a **language**, and write your code or notes.  
This makes it easy to organize reusable code snippets or quick references.  

 
<img width="847" height="977" alt="TechNotes3" src="https://github.com/user-attachments/assets/e9b3a030-4d1c-4add-b92a-7804a4c9a74f" />


---

### Multiple Spaces Example
You can switch between different spaces to manage notes in various categories.  
For example, one space can hold C++ code snippets, another for hardware schematics, and another for Rust.  


<img width="1600" height="557" alt="TechNotes4" src="https://github.com/user-attachments/assets/75a1e46b-1b09-4ac5-83ef-78a5ca67658a" />
  
<img width="1596" height="973" alt="TechNotes5" src="https://github.com/user-attachments/assets/c0dbfba3-e95f-4634-aff9-00ca846a6fc7" />


## 📂 Projects & Tasks Section

This section is mainly for managing projects and assigning tasks.  
It is kept simple to quickly create projects and add related tasks.

---

### Projects Overview
This area shows all existing projects.  
Each project card contains its **status**, **priority**, and start/end dates.

  
<img width="1595" height="720" alt="Projects1" src="https://github.com/user-attachments/assets/e3f4e214-ca50-4d92-834e-ed362b41b534" />


---

### Creating a Project
From this view you can create a new project by entering a **name**, **description**, choosing **status**, **priority**, and setting start/end dates.

 
<img width="561" height="640" alt="Projects2" src="https://github.com/user-attachments/assets/e32fc42d-d2ab-4e6b-93fd-883d2a79af5d" />


---

### Project Tasks
Inside a selected project you can view all tasks related to it.  
Each task shows its title, description, due date, status, and priority.

<img width="1597" height="682" alt="Projects3" src="https://github.com/user-attachments/assets/685aabfb-7227-43f1-bf16-e28536619c6d" />


---

### Creating a Task
This form allows you to create a new task under a selected project.  
You can provide a **title**, **description**, choose **status**, **priority**, and a **due date**.


<img width="557" height="641" alt="Projects4" src="https://github.com/user-attachments/assets/f2a23abd-4e31-4314-9fb2-864baf568808" />



## 🔐 Personal Vault Section

The **Personal Vault** is a secure storage system for your sensitive information.  
It allows you to safely store passwords, personal accounts, and other confidential data with encryption.

---

### Personal Accounts
This section lets you store login credentials for various services.  
Each account entry includes **title**, **email**, **password**, **website**, **notes**, and **category** for better organization.

### Personal Information
Store any sensitive personal information that you want to keep secure.  
You can mark information as sensitive to enable encryption, or keep it as plain text for quick access.

### Security Features
- **Encryption**: Sensitive data is encrypted using AES encryption
- **Categories**: Organize your data with custom categories
- **Search**: Quickly find specific accounts or information
- **Local Storage**: All data is stored locally on your device

---

## 🗓️ Planner Section

This module works like a simple calendar where you can create events and daily notes for specific dates.  
It allows you to plan your schedule and keep track of daily activities.

---

### Calendar Overview
This is the main view of the planner.  
Here you can see the calendar and any events assigned to the selected date.


<img width="1538" height="817" alt="Planner1" src="https://github.com/user-attachments/assets/59da8b88-b2ca-41bb-83b0-a25372652ef1" />


---

### Creating an Event
From this form you can create a new event for the selected date.  
Each event can have a **title**, **description**, **start/end time**, **type**, and **priority**.


<img width="556" height="642" alt="Planner2" src="https://github.com/user-attachments/assets/5b2fd185-75d6-4266-94b8-64f2ac451948" />


---

### Daily Notes
This section shows all notes assigned to the selected day.  
It’s useful for writing reminders or short logs specific to that date.


<img width="1596" height="710" alt="Planner3" src="https://github.com/user-attachments/assets/d5d74639-4ca8-4dc4-a01f-f3cb2cdb15e5" />


---

### Creating a Daily Note
From this form you can create a new daily note for the selected date.  
You can add a **title**, **content**, and optional **tags** to organize your notes.

  
<img width="837" height="680" alt="Planner4" src="https://github.com/user-attachments/assets/ee530643-edf8-4187-a500-95a58c8c7b60" />


---

---

<a id="tr"></a>

<p align="center">
  <img width="430" height="297" alt="Logo1" src="https://github.com/user-attachments/assets/3e436d21-771c-41da-8279-9974fe2a618f" />
</p>
<h1 align="center">YOTION</h1>

<p align="center">
  Notion'dan ilham alan, <b>React + Tauri</b> ile geliştirilmiş açık kaynak masaüstü uygulaması.<br/>
  Web üzerinde çalışır ve Windows, macOS, Linux için hafif bir masaüstü uygulaması olarak paketlenebilir.
</p>

<p align="center">
  <a href="#tr">🇹🇷 Türkçe</a> • <a href="#en">🇬🇧 English</a>
</p>


## 🚀 Özellikler

* ⚡  Hızlı DX için **Vite + React + TypeScript**
* 🎨  Notion benzeri tema (özel renkler ve sistem yazı tipleri) ile **Tailwind CSS**
* 🖥️  Çoklu platform masaüstü yapıları için **Tauri entegrasyonu**
* 📝 Zengin kod/metin düzenleme için **Monaco Editor** desteği
* 🔐 Şifreler ve hassas veriler için güvenli depolama **Kişisel Kasa**
* 🌐 Kelime yönetimi ve flashcard ile **Dil Öğrenme**
* 📝 Kod parçacıkları ve teknik bilgi düzenleme için **Teknik Notlar**
* 📂 Görevler ve ilerleme takibi ile **Proje Yönetimi**
* 🗓️ Etkinlikler ve günlük notlar ile **Planlayıcı**

---

## 🔧 Kurulum

```bash
git clone <repo-url>
cd yotion
npm install
```

---

## 💻 Geliştirme

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


## 📚 Notlar

* Node.js 18+ önerilir. Masaüstü sürümleri için, işletim sisteminizin Tauri önkoşullarının (Rust araç zinciri, platform SDK'ları) yüklü olduğundan emin olun. Tauri olmadan da web sürümünü kullanabilirsiniz.
* **Diller** bölümünde, öğrenmek istediğiniz dilleri ekleyebilirsiniz. Şu anda üzerinde çalıştığınız şeye bağlı olarak istediğiniz kadar dil ekleyebilirsiniz. Her dilin bir adı, kodu ve bayrak emojisi vardır, böylece kelime notlarınızı daha iyi düzenleyebilirsiniz.

---

## 🌐 Diller Bölümü

Bu bölüm, öğrenmek istediğiniz dilleri eklediğiniz yerdir.  
Şu anda üzerinde çalıştığınız şeye bağlı olarak istediğiniz kadar dil ekleyebilirsiniz.  
Her dil, kelime notlarınızı daha iyi düzenleyebilmeniz için bir **isim**, **kod** ve **bayrak emojisi** içerir.

### Ekran Görüntüleri

<img width="1596" height="703" alt="Languages" src="https://github.com/user-attachments/assets/5b6dcf2a-2f1e-4251-b4f9-003b6281b55c" />

<img width="1567" height="885" alt="Languages2" src="https://github.com/user-attachments/assets/5ff7989a-bbe1-48d0-a311-cd0055097dc5" />

## 📘 Kelime Bölümü

Burada, öğrendiğiniz dilin kelimelerini yönetebilirsiniz.  
Eklenen tüm kelimeler, çevirileri ve zorluk seviyeleriyle birlikte burada görünür.

### Ekran görüntüsü
<img width="1595" height="965" alt="Vocabulary2" src="https://github.com/user-attachments/assets/75728f5f-fedc-4ec7-977d-3b95c2011265" />


---

### ➕ Yeni Kelime Ekleme

Bu görünümden kelime listesine yeni kelimeler ekleyebilirsiniz.  
**Kelime** ve **Çeviri** dışında, **Telaffuz** ve **Örnek Cümle** için isteğe bağlı alanlar da bulunmaktadır.  
Her kelimeye bir **Zorluk Seviyesi** atanabilir (ör. Başlangıç, Orta, İleri).

### Ekran görüntüsü
<img width="1587" height="850" alt="Vacabulary1" src="https://github.com/user-attachments/assets/494f96d4-4705-4e68-a208-acbbeb91c8bf" />

## 🃏 Flashcards Bölümü

Flashcards özelliği, kelime dağarcığınızı interaktif bir şekilde gözden geçirmenize yardımcı olur.  
Her oturumda bir kelime veya kelime öbeği sunulur ve siz, cevap açıklanmadan önce cevabı hatırlamaya çalışırsınız.  
Bu yöntem, aralıklı tekrarı destekler ve ezberlemeyi daha etkili hale getirir.

### Ekran Görüntüleri
<img width="1602" height="631" alt="FlashCards1" src="https://github.com/user-attachments/assets/1c3dcd1a-2c2e-4c6d-93c2-9b17d8857676" />

<img width="1601" height="727" alt="FlashCards2" src="https://github.com/user-attachments/assets/e42e8b01-cb48-4469-a93a-21b92aecd959" />


---

### 🔄 Gözden Geçirme Akışı

- Bir kart göründüğünde, çevirisini veya anlamını hatırlamaya çalışın.
- Çözümü görmek için **Cevabı Göster**'e tıklayın.
- Cevabı gördükten sonra, yanıtınızı **Zor**, **İyi** veya **Kolay** olarak derecelendirebilirsiniz.
- Sistem, seçiminize göre inceleme programını ayarlar, böylece zor kelimeler daha sık görünürken, öğrenilen kelimeler daha az sıklıkta görünür.


## 📝 Teknik Notlar Bölümü

**Teknik Notlar** özelliği, teknik bilgilerinizi farklı alanlar ve parçacıklar altında düzenlemenizi sağlar.

---

### Alanlara Genel Bakış
Bu alanda farklı konular için ayrı alanlar oluşturabilirsiniz.  
Her alanı, ilgili notlarınızı gruplandıracağınız bir klasör olarak düşünebilirsiniz.  


<img width="1601" height="671" alt="TechNotes1" src="https://github.com/user-attachments/assets/daf9ce95-636f-4d5a-93a3-020af90bca1b" />


---

### Bir Alanın İçi
Bir alanı açtığınızda, o alana ait tüm snippet'leri görüntüleyebilirsiniz.  
Her snippet bir başlık, açıklama ve isteğe bağlı olarak kod örnekleri içerir.  


<img width="1571" height="982" alt="TechNotes2" src="https://github.com/user-attachments/assets/2a80c7c8-ef1d-4b15-a9c8-09a822be8ea9" />


---

### Snippet Oluşturma
Bu görünüm, bir alana yeni snippet'ler eklemenizi sağlar.  
Bir **başlık** ve **açıklama** girebilir, bir **dil** seçebilir ve kodunuzu veya notlarınızı yazabilirsiniz.  
Bu, yeniden kullanılabilir kod snippet'lerini veya hızlı referansları düzenlemeyi kolaylaştırır.  


<img width="847" height="977" alt="TechNotes3" src="https://github.com/user-attachments/assets/e9b3a030-4d1c-4add-b92a-7804a4c9a74f" />


---

### Birden Fazla Alan Örneği
Farklı alanlar arasında geçiş yaparak çeşitli kategorilerdeki notları yönetebilirsiniz.  
Örneğin, bir alanda C++ kod parçacıkları, başka bir alanda donanım şemaları ve başka bir alanda Rust kodları saklayabilirsiniz.  


<img width="1600" height="557" alt="TechNotes4" src="https://github.com/user-attachments/assets/75a1e46b-1b09-4ac5-83ef-78a5ca67658a" />
  
<img width="1596" height="973" alt="TechNotes5" src="https://github.com/user-attachments/assets/c0dbfba3-e95f-4634-aff9-00ca846a6fc7" />

## 📂 Projeler ve Görevler Bölümü

Bu bölüm esas olarak projeleri yönetmek ve görevleri atamak için kullanılır.  
Projeleri hızlı bir şekilde oluşturmak ve ilgili görevleri eklemek için basit tutulmuştur.

---

### Projeler Genel Bakış
Bu alanda mevcut tüm projeler gösterilir.  
Her proje kartında **durumu**, **önceliği** ve başlangıç/bitiş tarihleri bulunur.


<img width="1595" height="720" alt="Projects1" src="https://github.com/user-attachments/assets/e3f4e214-ca50-4d92-834e-ed362b41b534" />


---

### Proje Oluşturma
Bu görünümden **ad**, **açıklama** girerek, **durum**, **öncelik** seçerek ve başlangıç/bitiş tarihlerini belirleyerek yeni bir proje oluşturabilirsiniz.


<img width="561" height="640" alt="Projects2" src="https://github.com/user-attachments/assets/e32fc42d-d2ab-4e6b-93fd-883d2a79af5d" />


---
### Proje Görevleri
Seçilen bir projenin içinde, o projeyle ilgili tüm görevleri görüntüleyebilirsiniz.  
Her görevde başlık, açıklama, son tarih, durum ve öncelik gösterilir.


<img width="1597" height="682" alt="Projects3" src="https://github.com/user-attachments/assets/685aabfb-7227-43f1-bf16-e28536619c6d" />


---

### Görev Oluşturma
Bu form, seçilen bir proje altında yeni bir görev oluşturmanıza olanak tanır.  
Bir **başlık**, **açıklama** girebilir, **durum**, **öncelik** ve **son tarih** seçebilirsiniz.


<img width="557" height="641" alt="Projects4" src="https://github.com/user-attachments/assets/f2a23abd-4e31-4314-9fb2-864baf568808" />



## 🔐 Kişisel Kasa Bölümü

**Kişisel Kasa**, hassas bilgileriniz için güvenli bir depolama sistemidir.  
Şifreler, kişisel hesaplar ve diğer gizli verilerinizi şifreleme ile güvenle saklamanıza olanak tanır.

---

### Kişisel Hesaplar
Bu bölüm, çeşitli hizmetler için giriş bilgilerinizi saklamanıza olanak tanır.  
Her hesap girişi, daha iyi organizasyon için **başlık**, **e-posta**, **şifre**, **web sitesi**, **notlar** ve **kategori** bilgilerini içerir.

### Kişisel Bilgiler
Güvende tutmak istediğiniz herhangi bir hassas kişisel bilgiyi saklayın.  
Bilgileri hassas olarak işaretleyerek şifrelemeyi etkinleştirebilir veya hızlı erişim için düz metin olarak tutabilirsiniz.

### Güvenlik Özellikleri
- **Şifreleme**: Hassas veriler AES şifreleme kullanılarak şifrelenir
- **Kategoriler**: Verilerinizi özel kategorilerle düzenleyin
- **Arama**: Belirli hesapları veya bilgileri hızlıca bulun
- **Yerel Depolama**: Tüm veriler cihazınızda yerel olarak saklanır

---

## 🗓️ Planlayıcı Bölümü

Bu modül, belirli tarihler için etkinlikler ve günlük notlar oluşturabileceğiniz basit bir takvim gibi çalışır.  
Programınızı planlamanıza ve günlük etkinliklerinizi takip etmenize olanak tanır.

---

### Takvim Genel Bakış
Bu, planlayıcının ana görünümüdür.  
Burada takvimi ve seçilen tarihe atanan tüm etkinlikleri görebilirsiniz.


<img width="1538" height="817" alt="Planner1" src="https://github.com/user-attachments/assets/59da8b88-b2ca-41bb-83b0-a25372652ef1" />


---

### Etkinlik Oluşturma
Bu formdan, seçilen tarih için yeni bir etkinlik oluşturabilirsiniz.  
Her etkinlikte **başlık**, **açıklama**, **başlangıç/bitiş saati**, **tür** ve **öncelik** bilgileri bulunabilir.


<img width="556" height="642" alt="Planner2" src="https://github.com/user-attachments/assets/5b2fd185-75d6-4266-94b8-64f2ac451948" />


---

### Günlük Notlar
Bu bölüm, seçilen güne atanan tüm notları gösterir.  
Bu, o tarihe özgü hatırlatıcılar veya kısa günlükler yazmak için kullanışlıdır.


<img width="1596" height="710" alt="Planner3" src="https://github.com/user-attachments/assets/d5d74639-4ca8-4dc4-a01f-f3cb2cdb15e5" />


---

### Günlük Not Oluşturma
Bu formdan, seçilen tarih için yeni bir günlük not oluşturabilirsiniz.  
Notlarınızı düzenlemek için **başlık**, **içerik** ve isteğe bağlı **etiketler** ekleyebilirsiniz.


<img width="837" height="680" alt="Planner4" src="https://github.com/user-attachments/assets/ee530643-edf8-4187-a500-95a58c8c7b60" />


