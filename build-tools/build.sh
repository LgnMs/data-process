out_dir="../target/out"

rm -rf "$out_dir"
mkdir "$out_dir"
#cargo build --release
cp ../target/x86_64-unknown-linux-musl/release/data_process $out_dir
cp ../.env "$out_dir"
cp ../.env.prod "$out_dir"
cp -r ../static "$out_dir"
cp -r ../libs "$out_dir"
#cd ../crates/process_web/ui && pnpm run build
mkdir $out_dir/web-ui
cp ../crates/process_web/ui/.next/process_web_ui.zip $out_dir/web-ui

