# e2: Smart Contract Insights & Auditor's Tool
e2 is a Rust-based tool for visualizing and auditing the call graph of a Rust-based smart contract project. By generating a graphical representation of the project, e2 helps auditors and developers build a mental model of how the contract works, identify potential vulnerabilities, and improve their overall productivity by getting to the core of the code faster.


## Key Features
**Enhanced Call Graph**: Visualize internal and external calls, as well as defined and undefined contracts in the project.
**Comments & Summaries**: Extract comments from the code and summarize them to provide context for each function.
**Coverage Checker**: Identify blind spots in test coverage and analyze the quality of test cases.
**Code Quality Rating**: Generate a code quality score based on identified vulnerabilities, adherence to best practices, and other quality metrics.
**Test Suite Status**: Automatically check if all test cases are passing and highlight any failing tests.
**Security Analysis**: Identify attack vectors, list invariants, and assess how invariants could potentially be broken by attackers.
**AI-Powered Vulnerability Checks**: Integrate AI for advanced vulnerability analysis using a library of known issues for comprehensive auditing.
**Fix Recommendations**: Provide recommended fixes for identified vulnerabilities and potential issues.


## Project Structure & Implementation
To represent the structure of the code project itself (such as functions, modules, and control flow), e2 programmatically analyzes the codebase and visualizes relationships including:
- **Function Calls**: Generating a call graph to display function calls within the code.
- **Module Structure**: Representing how modules and submodules are interconnected.
- **Control Flow Graph**: Visualizing potential execution paths within a function or module.

There are two main steps to achieve this:
- **Extracting Code Structure (AST Analysis or Code Parsing)**:
e2 uses tools like syn and rust-analyzer to analyze the Abstract Syntax Tree (AST) of the source code, allowing it to extract relevant information about functions, structs, or control flow. This step forms the foundation for understanding the code structure.
- **Graph Representation (Using petgraph)**:
Once the structure is extracted, e2 represents it as a graph using the petgraph library. This makes it possible to visualize relationships, such as function calls, module connections, and control flow paths, and to output a .dot or .svg file as a visual representation.

This approach enables e2 to generate a comprehensive and customizable call graph, enhancing the visualization of the project’s structure for efficient auditing and analysis.


## Usage
To use e2 and generate a call graph, navigate to your Rust smart contract project directory and execute:
cargo e2 call-graph


## Output
The e2 tool generates an SVG file, call_graph.svg, in the current directory. This file contains a graphical representation of the function call relationships within the project, distinguishing between:

- **Internal Calls**: Functions defined within the current codebase.
- **External Calls**: Calls to external functions.
- **Defined Contracts**: Contracts and functions implemented within the project.
- **Undefined Contracts**: Functions or calls made to external contracts not defined within the project.


## Future Improvements
### Features Under Development
- **Legend Customization**: Adding a legend to the call graph to clarify types of calls and contract types.
- **Enhanced Comments & Summaries**: Extract comments from code, summarize them, and associate them with the relevant functions.
- **Test Coverage Analysis**: Provide detailed analysis of test coverage, identifying untested areas in the code.
- **Code Quality Metrics**: Offer an automated quality rating based on code structure, error handling, and best practice compliance.
- **Test Suite Status**: Track the current status of all test cases, highlighting any failing tests.
- **Security Analysis**: Identify potential attack vectors, list invariants, and analyze vulnerabilities within the project.
- **Fix Recommendations**: Suggest fixes for identified issues to enhance code security.
- **AI Vulnerability Analysis**: Integrate an AI module with a library of known vulnerabilities for more comprehensive security checks.


This tool aims to enhance productivity and deepen understanding of complex smart contract projects, providing both a visual aid and a foundation for secure and efficient development. As we continue adding features, e2 will become a valuable tool for smart contract development, security auditing, and project management.


## Contributions
Contributions are welcome! Please feel free to fork the repository and create a pull request for any bug fixes, feature enhancements, or new ideas.