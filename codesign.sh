#!/bin/sh

# This is bad, but post build scripts still not implemented yet for cargo
# https://github.com/rust-lang/cargo/issues/545

codesign -f -s - --timestamp=none --entitlements assets/app.entitlements ./target/*/yas_scanner_{genshin,starrail}
