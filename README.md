# SoftwareEngineeringTeam3-Assignment1
This is a repository for Assignment 1, allowing for version control and easy submission of work done for the CS416 - Software Engineering course.

---

## Running the sorting algorithms (using each language’s main file)

### Quick start (clone → pull → run)
```bash
git clone PASTE_THE_HTTPS_URL_HERE
cd repo-folder-name
git pull
```

### Java (runs from `Java/Main.java`)
```bash
cd Java
javac *.java
java Main MERGESORT
```

Other Java algorithm options:
`MERGESORT | QUICKSORT | INSERTIONSORT | BUBBLESORT | SELECTIONSORT`

---

### Python (runs from `main.py`)
```bash
cd Python
python main.py MERGESORT
```

Optional: pass your own numbers after the algorithm (space-separated):
```bash
python main.py MERGESORT 100 5 10 1 11
```

---

### Go (runs from `Go/main.go`)
```bash
cd Go
go run . MERGE
```

Go algorithm options:
`BUBBLE | INSERTION | SELECTION | MERGE | QUICK`

---

### Julia (assumes `Julia/main.jl` exists)
```bash
cd Julia
julia main.jl MERGESORT
```

---

### Rust (assumes `Rust/main.rs` exists)

If you’re compiling a single file:
```bash
cd Rust
rustc main.rs
./main MERGESORT
```

(Windows: run `main.exe` instead of `./main`)

If you’re using Cargo (only if a `Cargo.toml` exists):
```bash
cargo run -- MERGESORT
```

---

**NOTE:** All of these commands will be for **CMD** or **Git Bash**. If you are using another terminal, I'm sorry for your loss.

## 1) How to clone the repo (first time only)

### A) Go to the folder you want the project to live in
KNOW YOUR FILEPATH, IT WILL NOT BE THE SAME 0_0

**Windows**
```bash
cd C:\Users\Admin\exampleFolderName
```

**Mac/Linux**
```bash
cd ~/exampleFolderName
```

### B) Clone it
Go to GitHub, click the green **Code** button, then copy the **HTTPS** URL.

run:
```bash
git clone PASTE_THE_HTTPS_URL_HERE
```

### C) Enter the repo folder
```bash
cd repo-folder-name
```

Finally, open the repo folder in VSCode (or another IDE).
If you have VSCode installed, you can use the terminal command
```
code
```

---

## 2) How to download the latest changes (pull)

**ALWAYS do this before you start working.**
```bash
git pull
```

If you’re on a different branch than `main`, do:
```bash
git pull origin your-branch-name
```

---

## 3) How to check changed files or for which files you have staged (status)

Use this constantly:
```bash
git status
```

To see what changed:
```bash
git diff
```

---

## 4) How to make a new branch (recommended way to work)

**Do NOT work directly on `main` unless you are told so or the team says it’s okay.**

Create and switch to a new branch:
```bash
git checkout -b yourname-AssignmentRequiredNamingConvention
```

Examples:
```bash
git checkout -b Nick-Hw1PythonMergeSort
git checkout -b Krish-Hw1JuliaQuickSort
```

By keeping your name on the file and the branch, team members can see which parts of the assignment are in progress.
Please create a new branch and then create new files for the assignment 🙇‍♂️🙏

---

## 5) How to save your changes (add + commit)

### A) Add your changes

Add everything:
```bash
git add .
```

OR add one file:
```bash
git add path/to/file
```

### B) Commit with a message
```bash
git commit -m "Short description of what you did"
```

Good commit messages:
- `Fixed an issue with ExampleAssignment`
- `Finished Python Mergesort`
- `Update README cloning instructions.`

---

## 6) How to upload your changes (push)

### First push of a new branch
```bash
git push -u origin yourname-AssignmentRequiredNamingConvention
```

After that, normal pushes are just:
```bash
git push
```

---

## 7) How to open a Pull Request (PR) on GitHub

1. Go to the repo on GitHub.
2. You’ll usually see a banner: **“Compare & pull request”** → click it.
3. Add a title + description.
4. Submit PR.
5. Wait for review / approval of at least 2 members of the team.

---

## 8) How to update your branch with the newest main so you don’t explode things

Before you push big changes, update:

```bash
git checkout main
git pull
git checkout yourname-HwRequiredNamingConvention
git merge main
```

If there are merge conflicts, Git will tell you which files need fixing.

---

# Common “I messed up” fixes

## “I edited stuff but I want to throw it away”
This deletes uncommitted changes.
```bash
git reset --hard
```

## “I want to unstage files (undo git add)”
```bash
git reset
```

## “My commit message sucks”
If you haven’t pushed yet:
```bash
git commit --amend -m "Better message"
```

## “I’m on the wrong branch”

See branches:
```bash
git branch
```

Switch branches:
```bash
git checkout branch-name
```

---

# The daily routine (do this every time)

1) Pull latest:
```bash
git pull
```

2) Make changes.

3) Check status:
```bash
git status
```

4) Add + commit:
```bash
git add .
git commit -m "What you changed"
```

5) Push:
```bash
git push
```
