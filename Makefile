BLUEPRINT_DIR := resources/blueprints
UI_DIR := resources/ui
BLUEPRINTS := $(wildcard $(BLUEPRINT_DIR)/*.blp)
UI_FILES := $(patsubst $(BLUEPRINT_DIR)/%.blp,$(UI_DIR)/%.ui,$(BLUEPRINTS))

.PHONY: compile-blueprints check-blueprints remove-gresource-file compile-resources export-models run

# Compile a changed Blueprint source into its GtkBuilder XML counterpart.
$(UI_DIR)/%.ui: $(BLUEPRINT_DIR)/%.blp
	blueprint-compiler compile --output $@ $<

# Compile all Blueprint sources that are newer than their generated UI files.
compile-blueprints: $(UI_FILES)

# Verify that every Blueprint source compiles without writing generated files.
check-blueprints:
	@for blueprint in $(BLUEPRINTS); do blueprint-compiler compile --output /dev/null $$blueprint || exit 1; done

# Remove the previously compiled GResource bundle.
remove-gresource-file:
	rm -f resources/mcskineditor.gresource

# Compile Blueprint sources and package UI, CSS, shaders, and media into GResource.
compile-resources: compile-blueprints
	glib-compile-resources resources/mcskineditor.gresource.xml --target=resources/mcskineditor.gresource --sourcedir=resources

# Regenerate embedded OBJ model assets through the dedicated Rust test.
export-models:
	cargo test write_obj_assets -- --nocapture

# Rebuild all UI resources and launch the application.
run: remove-gresource-file compile-resources
	cargo run
