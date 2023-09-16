CLIPPY_PARAMS = -- \
	-W clippy::cargo \
	-W clippy::cast_lossless \
	-W clippy::dbg_macro \
	-W clippy::expect_used \
	-W clippy::if_not_else \
	-W clippy::items_after_statements \
	-W clippy::large_stack_arrays \
	-W clippy::linkedlist \
	-W clippy::manual_filter_map \
	-W clippy::match_same_arms \
	-W clippy::needless_continue \
	-W clippy::needless_pass_by_value \
	-W clippy::option_if_let_else \
	-W clippy::redundant_closure_for_method_calls \
	-W clippy::semicolon_if_nothing_returned \
	-W clippy::similar_names \
	-W clippy::single_match_else \
	-W clippy::trivially_copy_pass_by_ref \
	-W clippy::unnested_or_patterns \
	-W clippy::unseparated-literal-suffix \
	-A clippy::multiple_crate_versions \
	-A clippy::wildcard_dependencies \
	-D warnings

check:
	cargo fmt --check
	cargo clippy $(CLIPPY_PARAMS) --no-deps
