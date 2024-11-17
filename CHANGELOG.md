# Changelog

All notable changes to this project will be documented in this file.
This file will include each commit message and the commit message will be grouped by
the changelog generator (git-cliff).

---

## [0.1.0] - 2024-11-17

### Features
- *(docs)* Update readme
- *(docs)* Add contributing document
- *(base)* Use genpdfi instead of genpdf
- *(cargo)* Add version to genpdf package
- *(base)* Rename project to markdown2pdf
- *(bin)* Set lto to 'thin' and enable strip
- *(bin)* Handle the response result of parse
- *(pdf)* Improve error returning from Pdf
- *(pdf)* Handle code blocks in pdf converter
- *(markdown)* Parse multiline code blocks and code snippet language
- *(lib)* Improve documentation comments
- *(docs)* Add configuration header to readme
- *(config)* Read mdprc from the root directory
- *(lib)* Implement config parsing into library
- *(config)* Add module for parsing toml into StyleMatch
- *(config)* Add configuration toml example
- *(lib)* Add documentation comments & improve lib public methods
- *(pdf)* Call add_link for Link elements
- *(cargo)* Use fork of genpdf-rs-improved
- *(styling)* Add new roboto font & change the fonts structure
- *(styling)* Implement styling on pdf, to create pdfs based on style match
- *(styling)* Improve styling & add new paramethers and styles
- *(bin)* Add makefile for easy build
- *(styling)* Add basic styling structure
- *(bin)* Remove help.txt & add to main.rs
- *(bin)* Update both package names to mdp
- *(bin)* Update binary name to mpd
- *(bin)* Improve cli & add docummentation
- *(pdf)* Improve transforming lexer output to pdf
- *(markdown)* Make Token cloneable
- *(pdf)* Add basic logic for token to PDF element conversion
- *(pdf)* Add pdf class to convert markdown to pdf
- *(markdown)* Refactor text parsing to correctly handle special characters
- *(markdown)* Update emphasis structure to level based
- *(markdown)* Parse emphasis level correctly
- *(markdown)* Implement parsing nested tokens functionality
- *(markdown)* Bring back markdown lexer
- *(assets)* Remove test_data and move testing markdowns on local only
- *(lib)* Remove markdown lexer
- *(lexer)* Add simple lexer to parse markdown
- *(cargo)* Update the structure of cargo
- *(docs)* Add README.md
- Init cargo project

### Bug Fixes
- *(config)* Remove config path printing
- *(markdown)* Single line code block handling
- *(bin)* Update the mdp caller in main
- *(styling)* Add cross platform font path generation
- *(pdf)* Missing space after hyper links
- *(markdown)* Link item parsing

### Miscellaneous Tasks
- *(base)* Rename project to mdp
