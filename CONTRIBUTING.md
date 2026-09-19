# 🤝 Contributing to WORPEN

Thank you for your interest in contributing to **WORPEN**! This project is built with passion and we welcome contributions from the community.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Pull Request Process](#pull-request-process)
- [Attribution](#attribution)

---

## 🌟 Code of Conduct

- **Be Respectful**: Treat everyone with respect and kindness
- **Be Constructive**: Provide helpful feedback and suggestions
- **Be Professional**: Keep discussions focused on technical matters
- **Give Credit**: Always acknowledge contributions and sources

---

## 🚀 Getting Started

### Prerequisites

- **Rust 1.75+**: [Install Rust](https://rustup.rs/)
- **Git**: [Install Git](https://git-scm.com/)
- **Python 3.8+**: For running test scripts (optional)

### Fork and Clone

```bash
# Fork the repository on GitHub
# Then clone your fork
git clone https://github.com/YOUR_USERNAME/worpen.git
cd worpen

# Add upstream remote
git remote add upstream https://github.com/matinsanei/worpen.git
```

---

## 💻 Development Setup

### 1. Build the Project

```bash
cd backend
cargo build
```

### 2. Run Tests

```bash
# Run Rust tests
cargo test --all-features

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Run Python integration tests (optional)
cd ..
python tests/test_all_yaml.py
```

### 3. Start Development Server

```bash
cd backend
cargo run
```

Server starts at: `http://127.0.0.1:3000`

---

## 🔧 Making Changes

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/bug-description
```

### 2. Code Style Guidelines

#### Rust Code
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes with no warnings
- Add tests for new functionality

#### Documentation
- Update README.md if adding new features
- Add examples in `documentation/` folder
- Include inline comments for complex logic

### 3. Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```bash
feat: Add new YAML operation for loops
fix: Resolve variable scoping issue
docs: Update installation guide
test: Add integration tests for helpers
refactor: Simplify expression parser
```

---

## 📤 Pull Request Process

### 1. Before Submitting

- ✅ All tests pass (`cargo test --all-features`)
- ✅ Clippy passes (`cargo clippy -- -D warnings`)
- ✅ Code is formatted (`cargo fmt`)
- ✅ Documentation is updated
- ✅ Commit messages follow conventions

### 2. Create Pull Request

1. Push your branch to your fork
2. Go to [WORPEN Repository](https://github.com/matinsanei/worpen)
3. Click "New Pull Request"
4. Select your branch
5. Fill in the PR template:

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Documentation update
- [ ] Performance improvement

## Testing
- [ ] All tests pass
- [ ] Added new tests
- [ ] Manual testing completed

## Checklist
- [ ] Code follows project style
- [ ] Documentation updated
- [ ] No breaking changes
```

### 3. Review Process

- Maintainers will review your PR within 2-3 days
- Address any feedback or requested changes
- Once approved, your PR will be merged

---

## 🎯 Attribution

**IMPORTANT**: This project requires attribution in all uses.

When contributing:
- ✅ Your code becomes part of WORPEN
- ✅ You retain copyright to your contributions
- ✅ Your name will be added to contributors list
- ✅ Attribution to **Matin Sanei** must be maintained in derivative works

See [LICENSE](LICENSE) for full attribution requirements.

---

## 📞 Need Help?

- 💬 **Discussions**: [GitHub Discussions](https://github.com/matinsanei/worpen/discussions)
- 🐛 **Bug Reports**: [GitHub Issues](https://github.com/matinsanei/worpen/issues)
- 📧 **Email**: saneimatin33@gmail.com

---

## 🙏 Thank You!

Every contribution helps make WORPEN better. Whether it's:
- 🐛 Reporting bugs
- 💡 Suggesting features
- 📝 Improving documentation
- 💻 Writing code
- ⭐ Starring the repo

**Your support matters!** 🚀

---

<div align="center">

**Built with ❤️ by [Matin Sanei](https://github.com/matinsanei)**

[⬆️ Back to Top](#-contributing-to-worpen)

</div>
