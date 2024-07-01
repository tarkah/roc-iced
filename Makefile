.PHONY: build
build:
	roc build platform/app.roc

.PHONY: bundle
bundle:
	roc build --bundle .tar.br platform/main.roc

.PHONY: clean
clean:
	find examples/ -type f ! -name '*.roc' -delete
	rm -f platform/app*
	rm -f platform/libapp.*
	rm -f platform/dynhost
	rm -f platform/linux-x64.rh
	rm -f platform/metadata_linux-x64.rm
	rm -f platform/*.tar*
