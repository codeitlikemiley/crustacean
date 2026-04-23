# Goldcoders - Rust Mastery Course

Welcome to the **Rust Mastery Course**, an interactive, hands-on learning platform designed to take you from a complete Rust newbie to an advanced systems programmer. Whether you want to conquer the borrow checker, understand lifetimes, or master unsafe Rust and procedural macros, this platform provides bite-sized, interactive lessons to get you there.

You can dive into the curriculum [online for free](https://masterustacean.goldcoders.dev), or you can run it locally on your own machine. Even better, because the entire platform is open source, you can use our built-in AI tools to generate and teach your very own custom Rust courses!

## 🚀 Getting Started Locally

Running the platform locally allows you to learn offline, experiment with the exercises, and author your own content.

### Prerequisites

- **Rust**: [Install Rust](https://rustup.rs/)
- **wasm32 target**: `rustup target add wasm32-unknown-unknown`
- **Trunk**: `cargo install --locked trunk`

### Running the App

1. Clone the repository: 
   ```bash
   git clone https://github.com/codeitlikemiley/crustacean.git
   cd crustacean
   ```
2. Run the development server:
   ```bash
   trunk serve
   ```
3. Open `http://localhost:3001` in your browser.

## ⌨️ Keyboard Navigation

Navigate the platform efficiently using these shortcuts without taking your hands off the keyboard:

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

## ✍️ Creating Content (AI Skills)

Want to teach a topic or create a custom curriculum? This project heavily utilizes AI-assisted content generation so you can build new courses in minutes. We provide a dedicated **"Skill"** (instruction set) located at `skills/software-development/rust-tuts.md`. 

### Using the Agent Skill

To quickly generate high-quality, validating Rust courses, you should rely on an AI agent (like Antigravity or a Gemini CLI) equipped with this skill:

1. **Activate the Skill**:
   - Instruct your agent to read the skill file: `Please review the skill at skills/software-development/rust-tuts.md`.
   - The agent will learn our exact data structure and our "frontend-only" validation philosophy.

2. **Generate a Course**:
   - Ask the agent to generate a course on a specific topic (e.g., "Create a Deep Dive Focus course on Rust Atomics").
   - The agent will automatically generate a new data file with balanced `Concept` and `Practice` lessons.
   - It will write custom regex/contains matching rules that act as our "compiler-less" validation engine.

3. **Register the Course**:
   - Have the agent (or manually) expose the new module in `src/data/mod.rs` and `src/data/lesson_pool.rs`.
   - Finally, add the `Course` struct definition to the `COURSES` array in `src/data/courses.rs`.

## 🤝 Contributing a Lesson

We welcome community contributions! If you've created a great lesson or course that you'd like to see added to the platform, please submit a Pull Request (PR) by following these simple steps:

1. **Fork & Clone**: Fork this repository to your GitHub account and clone it locally.
2. **Create a Branch**: Create a new branch for your lesson (e.g., `git checkout -b feature/add-my-lesson`).
3. **Build Your Lesson**: Follow the "Creating Content" steps above.
4. **Test Locally**: Run `trunk serve` and ensure your lesson renders correctly, all exercises validate properly, and there are no compiler warnings.
5. **Commit & Push**: Commit your changes and push the branch to your fork.
6. **Open a PR**: Open a Pull Request from your fork to our `master` branch.

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

## 🛠 Tech Stack

If you're curious about how this platform was built, here is the underlying architecture:

- **Frontend**: Leptos 0.8 (Client-Side Rendering)
- **Styling**: Tailwind CSS
- **Bundler**: Trunk
- **Deployment**: Google Cloud Run + GitHub Actions
- **Optimization**: Multi-stage Docker builds with `cargo-chef`
