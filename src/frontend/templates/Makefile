.PHONY: all clean
PROJECT:=$(basename $(CURDIR)).sb3
$(PROJECT): goboscript.toml $(wildcard *.gs lib/**/*.gs assets/**/*.png assets/**/*.svg)
	goboscript build
all: $(PROJECT)
clean:
	rm -rf "$(PROJECT)"
