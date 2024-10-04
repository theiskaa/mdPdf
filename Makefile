.PHONY: build build-in-local

BUILD := cargo build --release
MOVETO_MAIN := sudo mv target/release/mdp /usr/bin/mdp
MOVETO_LOCAL := sudo mv target/release/mdp /usr/local/bin/mdp

build:
	@echo " ------------------------------- "
	@echo "| Running cargo build --release |"
	@echo " ------------------------------- "
	@$(BUILD)
	@echo " ------------------------------------------------ "
	@echo "| Build Completed! Moving executable to base bin |"
	@echo " ------------------------------------------------ "
	@$(MOVETO_MAIN)
	@echo " ------------------------------------------------ "
	@echo "| Moved target/release/mdp to /usr/bin/mdp |"
	@echo " ------------------------------------------------ "

build-in-local:
	@echo " ------------------------------- "
	@echo "| Running cargo build --release |"
	@echo " ------------------------------- "
	@$(BUILD)
	@echo " ------------------------------------------------ "
	@echo "| Build Completed! Moving executable to base bin |"
	@echo " ------------------------------------------------ "
	@$(MOVETO_LOCAL)
	@echo " ------------------------------------------------ "
	@echo "| Moved target/release/mdp to /usr/local/bin/mdp |"
	@echo " ------------------------------------------------ "
