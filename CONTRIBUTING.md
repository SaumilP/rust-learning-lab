# 🤝 Contributing to Rust Learning Lab

Thank you for your interest in contributing! <br />
This project aims to be a **high-quality, beginner-friendly Rust learning resource**. <br />
All contributions that improve clarity, correctness, or learning experience are welcome.

---

## 📌 Ways to contribute

You can help by:

- Fixing bugss or incorrect explanations
- Improve documentation or README files
- Adding exercises or solutions
- Adding new examples or mini-projects
- Refactoring code to be more idiomatic
- Improving test coverage
- Reporting issues or suggesting improvements

---

## ⏱️ Contribution Guidelines

### 1. Follow the Learning Philosophy

All contributions should:

- Prefer **clarify over cleverness**
- Use **idiomatic Rust**
- Avoid unnecessary complexity
- Be understandable by beginner/intermediate developers

If adding advanced concepts, **explain them clearly**.

---

### 2. Project structure rules

- Each topic folder **must include `README.md`**
- Code examples should be:
  - Small
  - Focused
  - Runnable with `cargo run`
- Larger examples should be modular (`src/` split logically)

---

### 3. Code style

Before submitting:

```bash
cargo fmt
cargo clippy --all-targets --all-features
cargo test
```

Guidelines:

- Prefer explicit code over magic
- Avoid `unwrap()` in non-trival examples
- Use meaningful variable and function names
- Add comments only when they add value

---

### 4. Adding new Content

#### New example or Module Checklist

- [ ] Clear folder name
- [ ] `README.md` explaining:
  - What is being taught
  - Why it matters
  - Common mistakes
- [ ] Runnable code
- [ ] At least one exercise (if applicable)
- [ ] Tests (when reasonable)

---

### 5. Exercises & Challenges

Exercises should:

- Be incremental
- Encourage compiler-driven learning
- Avoid requiring external crates unless necessary
- Include comments or hints (not full solutions)

---

### 6. Issues & Discussions

Before opening an issue:

- Check existing issues
- Be specific and concise
- Include code snippets or error messages when relevant

Use issues for:

- Bugs
- Incorrect explanations
- Learning flow improvements
- Content suggestions

---

## 🔀 Pull Request Process

1. Fork the repository
2. Create a feature branch:

```bash
git checkout -b feature/my-improvement
```

3. Commit with clear message:

```text
Add ownership exercise for move semantics
```

4. Push and open a Pull Request

Your PR should:

- Explain *what* you changed
- Explain *why* it improves the repo
- Reference related issues if applicable

---

## 🧪 CI & Reviews

- All PRs must pass CI checks
- Reviews focus on:
  - Clarity
  - Correctness
  - Idiomatic Rust
  - Educational value

---

## 📔 Code of Conduct

Be respectful and constructive. <br/>
This repository is meant to be a **safe learning space** for everyone.

---

## 🙏 Thank you

Every contribution - small or large - helps make Rust more accessible. <br />
Thank you for helping others learn Rust the right way 🦀.