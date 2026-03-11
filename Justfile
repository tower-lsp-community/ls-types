_default:
	@just --list --unsorted --list-heading '' --list-prefix '—— '

meta_model_path := "spec/_specifications/lsp/3.18/metaModel/metaModel.json"
config_path := "config.toml"
output_path := "src/generated.rs"

generate *args:
	# do not forget to git submodule update --init --recursive
	cargo xtask generate {{meta_model_path}} --config {{config_path}} --output {{output_path}} {{args}}
	cargo fmt -- {{output_path}}
