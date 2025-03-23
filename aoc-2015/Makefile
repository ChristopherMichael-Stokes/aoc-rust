day := 0
day_str = $(shell printf "%02d" $(day))
solution_file = src/solutions/day$(day_str).rs
template_file := solution_template.rs.in
module_config := src/solutions/mod.rs

new_inputs:
	touch inputs/day$(day_str).txt
	touch inputs/day$(day_str)_dummy.txt
	@DAY=$(day) envsubst < $(template_file) > $(solution_file)

	@echo "pub mod day$(day_str);" >> $(module_config)

.PHONY:
	new_inputs
