# Prakrit Noun Generator  
![GitHub](https://img.shields.io/github/license/svyoma/prakrit-verb)					![GitHub last commit](https://img.shields.io/github/last-commit/svyoma/prakrit-verb)


# Prakrit Noun Declension Generator

A Rust-based CLI tool for generating grammatical noun forms in **Prakrit**, covering all seven cases in both singular and plural forms. This tool currently supports **feminine** and **neuter** nouns, with accurate morphological transformations based on classical grammar rules.

---

## 📚 Features

- ✅ Full **declension tables** for each noun
- ✅ Supports **feminine and neuter** gender
- ✅ Handles stems ending in **-a, -A, -i, -I, -u, -U**
- ✅ Applies precise **vowel transformation rules**
- ✅ Written in **Rust** for performance and speed
- ✅ Easy to extend with more grammatical logic

---

## 🧠 How It Works

The tool reads a Prakrit noun from user input and identifies its stem ending. Based on the gender and vowel ending, it generates all valid inflectional forms using suffix patterns and vowel mutation rules.

## 🚀 Getting Started

### Prerequisites

- Rust (latest stable recommended). Install from [rustup.rs](https://rustup.rs/)

### Run the program

```bash
git clone https://github.com/svyoma/prakrit-noun.git
cd prakrit-noun
cargo run
```

You'll be prompted to enter a Prakrit noun. The tool will print out a full declension table.
📁 Folder Structure
```
prakrit-noun/
│
├── src/
│   └── main.rs       # Core declension logic
│
├── Cargo.toml        # Rust project configuration
└── README.md
```
📝 License

This project is licensed under the MIT License.
👤 Author: (Vyom A. Shah)[https://github.com/svyoma] – working on tools for classical languages and scripts.
