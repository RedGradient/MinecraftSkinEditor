remove-resources-bundle:
	rm resources/mcskineditor.gresource

compile-resources:
	glib-compile-resources resources/mcskineditor.gresource.xml --target=resources/mcskineditor.gresource --sourcedir=resources

build:
	make remove-resources-bundle
	make compile-resources
	cargo run