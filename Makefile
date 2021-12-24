
# 
.SILENT:
.PHONY: format test watch clean

## help: this help file
help:
	@( echo "" && echo "Makefile targets..." && echo "" )
	@( cat Makefile | grep '^##' | sed -e 's/##/ -/' | sort && echo "" )

## format: run the cargo formatter
format:
	@echo "not implemented yet..."

## clean: clean the release
clean:
	cargo clean

## test: test the application
test:
	clear && cargo test

## watch: watch test and lib files and compile and test on change
watch:
	watchexec -c -d 500 cargo test

