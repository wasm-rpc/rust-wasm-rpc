set -e

TARGET_PATH="../../target/wasm32-unknown-unknown/release/"
DIST_DIR="`pwd`/dist"

scriptsDir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd $scriptsDir/..
moduleName=$(cat Cargo.toml | grep name | sed -n 's/name *= *"\(.*\)"/\1/p')
wasmFilename="$moduleName.wasm"
minifiedWasmFileName="$moduleName.min.wasm"
wastFilename="$moduleName.wast"
cargo +nightly build --target wasm32-unknown-unknown --release
if ! command -v  wasm-gc > /dev/null; then
  cargo install wasm-gc
fi
cd $TARGET_PATH
wasm-snip --snip-rust-fmt-code $wasmFilename -o $wasmFilename
wasm-snip --snip-rust-panicking-code $wasmFilename -o $wasmFilename

wasm2wast $wasmFilename -o $wastFilename
funcs=`cat $wastFilename | sed -n  's/^[[:space:]]*(func \([^(]*\) .*/\1/p'`

i=0
IFS=$'\n'
set -f
for func in ${funcs[@]}
do
  match=`echo $func | sed -e 's/[]\/$*.^[]/\\\&/g'`
  sed -i '' "s|$match|\$func_$i|g" $wastFilename
  ((i++))
done

sed -i '' '/(data (i32.const [0-9]*) "[\\00]*"/d' $wastFilename

wast2wasm $wastFilename -o $minifiedWasmFileName
wasm-gc $minifiedWasmFileName -o $minifiedWasmFileName

mkdir -p $DIST_DIR
cp -R $minifiedWasmFileName "$DIST_DIR/$wasmFilename"
