# Prakrit Noun Generator  

This is a Rust-based program that generates noun forms for **Prakrit words** based on their endings and grammatical cases. It applies specific linguistic rules to transform root words into various forms used in Prakrit grammar. The program accurately handles vowel mutations and suffix transformations to produce grammatically consistent noun forms.

This currently supports a-ending, i-ending and u-ending masculine words only. Support for feminine and neuter words will be added soon.

---

## Features  
- Accepts user input for a Prakrit word  
- Identifies the word ending (`a`, `i`, `u`, etc.)  
- Applies suffix transformations based on specific linguistic rules  
- Outputs both **singular** and **plural** forms for seven grammatical cases

---

## Linguistic Rules Applied  
### 1. **For `a`-ending words**  
- `a → A` in some cases  
- `a → e` in certain plural forms  

### 2. **For `i/u`-ending words**  
- `i → I`, `u → U` in specific cases  
- Removes last vowel when specific suffix is applied  

---

## Code Structure  
### 📂 `main.rs`  
- **`main()`** – Handles user input and calls transformation functions  
- **`remove_last_vowel()`** – Removes the last vowel from the word  
- **`replace_vowel()`** – Replaces or modifies the last vowel based on transformation rules  
- **`generate_and_print_forms()`** – Generates and prints noun forms for different cases  

---
### Usage

There is no front-end interface for using this tool. To use it:

-   Clone this repository.
-   Compile `main.rs` using Rust's `rustc` compiler.
-   Run the compiled executable file.

  Alternatively, you can copy the code from `dev_generator.rs` or `slp_generator.rs` and run it on the [Rust Playground](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2024).

### Input Example  
Example Input:  
```
Enter the Prakrit word:
samaNa
```
Example Output:  
```
First Case:
  Singular:
    samaNo
  Plural:
    samaNA
    samaNe
```

---

## Example Outputs  
| Input | First Case (Singular) | First Case (Plural) |
|-------|-----------------------|---------------------|
| deva | devo | devA, deve |
| muNi | muNI | muNau, muNao, muNiNo, muNI |
| sAhu | sAhU | sAhau, sAhao, sAhuNo, sAhU, sAhavo |

---

## Contributing  
Contributions are welcome! To contribute:  
1. Fork the repository  
2. Create a new branch (`git checkout -b feature/your-feature`)  
3. Commit your changes (`git commit -m 'Add some feature'`)  
4. Push to the branch (`git push origin feature/your-feature`)  
5. Open a pull request  

---

## License  
This project is licensed under the **MIT License** – feel free to modify and share!  

---

## Last Commit  
[![GitHub last commit](https://img.shields.io/github/last-commit/your-username/prakrit-noun-generator)](https://github.com/your-username/prakrit-noun-generator/commits/main)  

---

## Issues and Feedback  
If you encounter any issues or have suggestions for improvements, please create an issue in the repository.  
