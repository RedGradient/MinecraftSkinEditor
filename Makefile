remove-gresource-file:
	rm -f resources/mcskineditor.gresource

compile-resources:
	glib-compile-resources resources/mcskineditor.gresource.xml --target=resources/mcskineditor.gresource --sourcedir=resources

run:
	make remove-gresource-file
	make compile-resources
	cargo run