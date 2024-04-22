CONFIGURATION=release
FACTOR_N=24
GNUSED=gsed

.PHONY: build patch clean
build: patch
	swift build -c $(CONFIGURATION)

patch:
	$(GNUSED) -i'' -e '1clet FACTOR_N = $(FACTOR_N)' Sources/main.swift

clean:
	rm -rf .build
