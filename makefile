MDBOOK_VER := v0.4.25
MDBOOK_URL := https://github.com/rust-lang/mdBook/releases/download/${MDBOOK_VER}/mdbook-${MDBOOK_VER}-x86_64-unknown-linux-gnu.tar.gz

none:

doc-deps:
	@mkdir -p bin
	@curl -sSL ${MDBOOK_URL} | tar -xz --directory=./bin

doc-build:
	@./bin/mdbook build

doc-clean:
	@rm -rf site/
	@rm -rf bin/
