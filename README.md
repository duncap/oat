# OAT (Oval Abstract Trees)
**OAT** is an innovative, open-source protocol for cross-language compilation. It leverages a sophisticated file indexing system to compile OATLANG code into multiple popular programming languages, enhancing interoperability and code reuse across software development projects.

## Key Features
### 1. Cross-Language Compilation
Seamlessly compile OATLANG into various programming languages, making your code broadly accessible and reusable.

### 2. Open-Source Protocol
Fully transparent and community-driven, OAT encourages contributions and enhancements from developers worldwide.

### 3. Robust File Indexing
A comprehensive index system supports efficient file management and referencing, which is crucial for large-scale and complex software development.

# Installation
To start using OAT, follow these installation steps:

### Prerequisites
Ensure you have Rust installed on your system. You can install Rust via [rustup](https://rustup.rs/).

### 1. Clone the Repository
```bash
git clone https://github.com/duncap/oat.git
cd oat
```

### 2. Compile the Source
```bash
cargo build --release
```
### 3. Install
The build process will produce an executable in the **target/release** directory. You can move this executable and the `languages` directory to a suitable directory in your PATH for easier access:
```bash
sudo cp target/release/oat /usr/local/bin/
sudo cp -r languages /usr/local/bin/
```

# Usage
To compile the OATLANG code using OAT, you must create a **.oat** file. Here's an example:
```javascript
PRINT_STR{"Hello World!"}$
```
After creating your file, run the following command in the same directory as your .oat file is available in:
```bash
oat <name>.oat bin
```
OAT will compile the OATLANG file to binary, which you can then run to execute the binary file.
```bash
./<name>
```
# Community and Contributions
We actively encourage contributions, whether you're fixing bugs, adding new features, or improving documentation. Check this repository for issues that need attention.

## How to Contribute
1. Fork the Repository
Make a copy of the project to your GitHub account.

2. Create a Branch
Make changes in a new branch in your fork.

3. Submit a Pull Request
Submit a pull request to the main OAT repository after making changes. Include a clear description of your improvements.

# Licensing
OAT is licensed under the Lesser GNU General Public License (LGPL). This allows the software to be freely used and modified. The LGPL ensures that OAT and any derivatives remain open-source, thus supporting community-driven development and ensuring that enhancements to the core components are accessible to all users.

This license allows OAT to support open and proprietary software developments, making it ideal for various applications.
