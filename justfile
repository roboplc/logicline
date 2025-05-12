all:
  cargo test
  cargo test --no-default-features -F locking-rt-safe
  clippy

prepare: build-logicline-view build-ll-default-view

build-logicline-view:
  cd logicline-view && npm install && npm run build

build-ll-default-view:
  cd ll-default-view && npm install && npm run build

pub-logicline-view:
  cd logicline-view && npm run build && npm publish --access public
