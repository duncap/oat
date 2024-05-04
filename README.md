# OAT (Oval Abstract Trees)
OAT is an innovative, open-source protocol for cross-language compilation. It leverages a sophisticated file indexing system to compile OATLANG code into multiple popular programming languages, enhancing interoperability and code reuse across software development projects.

## Key Features
Cross-Language Compilation: Seamlessly compile OATLANG into various programming languages, making your code broadly accessible and reusable.
Open-Source Protocol: Fully transparent and community-driven, OAT encourages contributions and enhancements from developers worldwide.
Robust File Indexing: A comprehensive index system supports efficient file management and referencing, which is crucial for large-scale and complex software development.
Installation
To start using OAT, follow these installation steps:

## Prerequisites:
Ensure you have Rust installed on your system. You can install Rust via [rustup](https://rustup.rs/).

### Clone the Repository:
```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
git clone https://github.com/yourusername/oat.git
cd oat
```

### Compile the Source:
cargo build --release

### Install:
The build process will produce an executable in the target/release directory. You can move this executable to a suitable directory in your PATH for easier access:

sudo cp target/release/oat /usr/local/bin/

## Usage
To compile OATLANG code using OAT, run the following command:

oat 'COMMAND{"YourArgument"}$'

For example, to compile a print statement into a language, you would use:

oat 'PRINT{"Hello World"}$'

Follow the on-screen prompts to select the target language.

Community and Contributions
We actively encourage contributions, whether you're fixing bugs, adding new features, or improving documentation. Check this repository for issues that need attention.

How to Contribute
Fork the Repository: Make a copy of the project to your GitHub account.
Create a Branch: Make changes in a new branch in your fork.
Submit a Pull Request: Submit a pull request to the main OAT repository after making changes. Include a clear description of your improvements.

## Licensing
OAT is licensed under the Lesser GNU General Public License (LGPL). This allows the software to be freely used and modified. The LGPL ensures that OAT and any derivatives remain open-source, thus supporting community-driven development and ensuring that enhancements to the core components are accessible to all users.

Adhering to this license, OAT supports open and proprietary software developments, making it ideal for various applications.
