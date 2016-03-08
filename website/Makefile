.PHONY: site docs publish

site: ./site/*
	cp -r ./site/* out/

docs:
	cargo doc; true
	rm -rf ./out/doc
	cp -r ../target/doc ./out

content: site docs

publish:
	cd out; \
	git add -A && \
	git commit -m "update" && \
	git push origin gh-pages
