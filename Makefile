APP_FILE := DB\ Login\ Hook.app
PKG_FILE := DB\ Login\ Hook.pkg

default: notarize

target/aarch64-apple-darwin/release/db-hook: src/* Cargo.toml Cargo.lock build.rs
	cargo build --release

out/${APP_FILE}: res/Resources/Icon.png res/Info.plist
	mkdir -p out
	rm -rf out/${APP_FILE}
	mkdir -p out/${APP_FILE}
	cp -r res out/${APP_FILE}/Contents
	mkdir out/${APP_FILE}/Contents/MacOS

package: target/aarch64-apple-darwin/release/db-hook out/${APP_FILE}
	cp target/aarch64-apple-darwin/release/db-hook out/${APP_FILE}/Contents/MacOS/
	xattr -cr out/${APP_FILE}
	codesign -s "Developer ID Application: AS207960 Cyfyngedig" --options=runtime out/${APP_FILE}
	productbuild --sign "Developer ID Installer: AS207960 Cyfyngedig" --component out/${APP_FILE} /Applications out/${PKG_FILE}

notarize: package
	xcrun notarytool submit out/${PKG_FILE} --keychain-profile "notarytool-password" --wait
	xcrun stapler staple out/${PKG_FILE}

clean:
	cargo clean
	rm -rf out