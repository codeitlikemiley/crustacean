# Tuts - Rust + Leptos Tutorial Platform

A specialized, frontend-only interactive learning platform for Rust. Build modern Rust applications with Leptos 0.8 while mastering Rust concepts through guided exercises and local validation.

## 🚀 Getting Started

### Prerequisites

- **Rust**: [Install Rust](https://rustup.rs/)
- **wasm32 target**: `rustup target add wasm32-unknown-unknown`
- **Trunk**: `cargo install --locked trunk`
- **Tailwind CSS**: (Handled automatically by Trunk in this project)

### Local Development

1. Clone the repository.
2. Run the development server:
   ```bash
   trunk serve
   ```
3. Open `http://localhost:3001` in your browser.

## ⌨️ Keyboard Navigation

Navigate the platform efficiently using these shortcuts:

### Global
- **Cmd/Ctrl + K**: Open/Close Command Palette
- **Escape**: Close Command Palette

### Course Catalog / Detail
- **ArrowDown / j**: Select Next Lesson
- **ArrowUp / k**: Select Previous Lesson
- **Enter / l**: Open Selected Lesson
- **Cmd/Ctrl + Enter**: Resume prioritized course

### Inside a Lesson
- **Cmd/Ctrl + Enter**: Run code / Advance to next step
- **Cmd/Ctrl + ]**: Next step
- **Cmd/Ctrl + [**: Previous step
- **Cmd/Ctrl + Shift + {**: Return to Catalog
- **Cmd/Ctrl + Shift + X**: Clear diagnostics
- **Ctrl + n / p**: Next / Previous step (macOS/Legacy)
- **Ctrl + j / k**: Next / Previous step (Vim-like)

## ✍️ Creating Content (Skills)

This project uses a dedicated skill for adding content located at `skills/software-development/rust-tuts.md`.

### Adding a New Course

1. **Create Data**: Add a new file in `src/data/` (e.g., `src/data/my_course.rs`).
2. **Define Modules**: Use the `TutorialModule` struct.
   - `ModuleType::Concept`: For reading/explanation.
   - `ModuleType::Practice`: For coding exercises with validation.
3. **Validation**: Define rules in `ValidationSpec`. We use a frontend-only validation engine (no backend compiler).
4. **Register**: Add your course to `src/data/courses.rs`.

### Using the Gemini Skill
If you are using Gemini CLI, activate the skill to get expert guidance:
`activate_skill("rust-tuts")`

## 🤝 Contributing a Lesson

We welcome community contributions! If you've created a great lesson or course that you'd like to see added to the platform, please submit a Pull Request (PR) by following these simple steps:

1. **Fork & Clone**: Fork this repository to your GitHub account and clone it locally.
2. **Create a Branch**: Create a new branch for your lesson (e.g., `git checkout -b feature/add-my-lesson`).
3. **Build Your Lesson**: Follow the "Creating Content" steps above to add your data file in `src/data/` and register it in `src/data/courses.rs`.
4. **Test Locally**: Run `trunk serve` and ensure your lesson renders correctly, all exercises validate properly, and there are no compiler warnings.
5. **Commit & Push**: Commit your changes with a clear message and push the branch to your fork.
6. **Open a PR**: Open a Pull Request from your fork to our `master` branch. Please include a brief description of what your lesson covers and the Rust concepts it teaches!

## 🚢 Deployment

The app is deployed to **GCP Cloud Run** and accessible at [masterustacean.goldcoders.dev](https://masterustacean.goldcoders.dev).

### Makefile Usage

We use a `Makefile` for streamlined operations. You can create a `.env` file to store your local GCP preferences without committing them to git.

**.env example:**
```env
PROJECT_ID=your-project-id
GCP_ACCOUNT=your-email@example.com
REGION=us-central1
```

The Makefile automatically runs `gcloud config set` before every GCP command to ensure you are always deploying to the correct account and project.

- `make build`: Build the Docker image locally.
- `make run`: Run the container locally at `localhost:8080`.
- `make ship`: Push and Deploy in one command (automatically sets GCP context).
- `make url`: Show the URL of the deployed service.
- `make open`: Open the deployed service in your default browser.

### CI/CD with GitHub Actions

Every push to the `master` branch triggers a rebuild and redeployment via GitHub Actions.

**Required GitHub Secrets:**
- `GCP_PROJECT_ID`: Your GCP Project ID (e.g., `my-first-project`).
- `GCP_SA_KEY`: A JSON Service Account Key with permissions for Cloud Build and Cloud Run.

### Domain Mapping
To map `masterustacean.goldcoders.dev`:
1. Go to **Cloud Run** > **Manage Custom Domains**.
2. Add a mapping for the `tuts` service.
3. Update your DNS records as provided by GCP.

## 🛠 Tech Stack

- **Frontend**: Leptos 0.8 (CSR)
- **Styling**: Tailwind CSS
- **Bundler**: Trunk
- **Deployment**: Google Cloud Run + GitHub Actions
- **Optimization**: Multi-stage Docker builds with `cargo-chef`
