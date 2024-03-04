root=$(dirname "$(dirname "$0")");
project_name="data_process"
target_dir="$root/target"
out_dir="$target_dir/$project_name"
ui_dir="$root/crates/process_web/ui"

echo "root: $root";
rm -rf "$out_dir"
mkdir "$out_dir"

echo "开始构建..."
RUSTFLAGS="-C linker=x86_64-linux-musl-gcc" cargo build --release --target=x86_64-unknown-linux-musl
cp "$target_dir"/x86_64-unknown-linux-musl/release/data_process $out_dir
cp "$root"/.env "$out_dir"
cp "$root"/.env.prod "$out_dir"
cp -r "$root"/static "$out_dir"
cp -r "$root"/libs "$out_dir"

echo "开始构建ui..."
cd "$ui_dir" && npm run build
cp -r "$ui_dir"/.next/static/ "$ui_dir"/.next/standalone/.next/static
cp -r "$ui_dir"/public/ "$ui_dir"/.next/standalone/public
mkdir "$out_dir"/web-ui
cp -r "$ui_dir"/.next/standalone/ "$out_dir"/web-ui

cd "$out_dir" && tar -cvzf $project_name.tar.gz .
echo "已输出到 $out_dir"
echo "已压缩到 $project_name.tar.gz"
