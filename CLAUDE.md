# Project Overview

Storybook is a Rust application with an opinionated design on story writing. 
The objective is to provide a snappy, reliable desktop application for writing fiction of any scale.
With a layout commonly seen with programming IDE, especially the jetbrains suite. 

The special sauce is going to be LOCAL offline LLM capabilities to dynamically assist
in text generation, native in-app interactions (create a character, create a town, generate historical event, etc).

# Tech-Stack

- Graphical User Inteface: Iced
- LLM Framework: Kalosm (Cude and rocm accelerated)
- LLM Model: ministral-3:3b parameters
- spec driven design document folder: /docs
- Rust Version: rustc 1.94.0
- Design Language: Material Design 3
- UI Theme: Default dark-mode with light mode option. Not defined by system theme.
- Resolution: Application resolution should be scalable from small 720p displays to ultrawide 5220x1440p displays. 
  - Mostly through dynamic scalling and the ability to move tabs around. Again inspired from Jetbrains IDE suites.

# Core Features

- Tab container workspace: Allowing the user to open multiple tool views side by side
- Core focus on the writing component. Leveraging Markdown of style and formatting. The writing pane should be the default
- A character database and graphical user interface to interact with it. Containing critical information such as name, age, motive, alignment, description, link to related "world events" and lore (balads, memos, notes, etc), family lineage (can show a the family tree)
- A world event timeseries database. Displays chronological order of world events, displayed on a timeline that the user can scroll, add or edit. World events contain dates, relevant parties / characters, dates, etc
- An additional database as a lore repository. Can contain letters, balads, story/legend paraphrases
- An ai assistant panel built right into the app. Contains 3 modes: chat, instruct, edit
  - Edit mode performs edit on the current story chapter. Called story blocks
  - Chat: Simply chat with the model discuss ideas etc
  - Instruct: Perform actions / task -> example create a new chapter. Generate a new paragraph. Create a new character etc
  - Note: Instruct actions should be able to be performed from within Edit mode as well.
- Workspace driven. The idea is to mimick Jetbrains IDE project management. Each project is a story world. A project represents a specific folder. With it's metadata/settings stored in a .storybook folder with .json config files
  - App launches to the project management pane if no previously loaded projects exist or can be found. The user can then create, load or delete a project. A project has a name and file url and last opened timestamp. No description

# Workflow

For claude code the following MCP servers are available
- Context7: For fetching api documentation

## Spec driven development
- Before any action clearly outline the plan of action into clear iterative tasks. With a task-tag-id: ex. FEAT1-add-characters, HOTFIX1-fix-bug
  - Each step is clearly scoped. Ex. Add the ability to create a character
- Every code change should be reviewed and accompanied by an addition to the unit tests or an edit to an existing test.
- Test should remain lean and straight to the point. 
- After the completion of each task, the app should compile and launch. 
- At the end of each task session create a versioned and timestamped checkpoint with the git logs and summarized change logs.
- Multiple agents should be dispatched simultaneously if tasks can be completed in parallel
- Each session should be scope to it's own branch, when resuming unfinished work continue on the same branch. New task require a branch that matches task-tag-id
- Always commit successful results to local and remote git branch.
