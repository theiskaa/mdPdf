.PHONY: build build-in-local

BUILD := cargo build --release
MOVETO_MAIN := sudo mv target/release/markdown2pdf /usr/bin/markdown2pdf
MOVETO_LOCAL := sudo mv target/release/markdown2pdf /usr/local/bin/markdown2pdf

build:
	@echo " ------------------------------- "
	@echo "| Running cargo build --release |"
	@echo " ------------------------------- "
	@$(BUILD)
	@echo " ------------------------------------------------ "
	@echo "| Build Completed! Moving executable to base bin |"
	@echo " ------------------------------------------------ "
	@$(MOVETO_MAIN)
	@echo " ------------------------------------------------------------ "
	@echo "| Moved target/release/markdown2pdf to /usr/bin/markdown2pdf |"
	@echo " ------------------------------------------------------------ "

build-in-local:
	@echo " ------------------------------- "
	@echo "| Running cargo build --release |"
	@echo " ------------------------------- "
	@$(BUILD)
	@echo " ------------------------------------------------ "
	@echo "| Build Completed! Moving executable to base bin |"
	@echo " ------------------------------------------------ "
	@$(MOVETO_LOCAL)
	@echo " ------------------------------------------------------------------ "
	@echo "| Moved target/release/markdown2pdf to /usr/local/bin/markdown2pdf |"
	@echo " ------------------------------------------------------------------ "
