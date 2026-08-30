RUSTC ?= rustc
LABS := moved-value multiple-mutable-borrows lifetime-too-short trait-bound-not-satisfied send-sync

.PHONY: check

check:
	@for lab in $(LABS); do \
		expected=$$(cat $$lab/expected-error.txt); \
		output=$$(mktemp); \
		if $(RUSTC) --edition=2021 --crate-type lib $$lab/broken.rs --emit=metadata -o $$output.rmeta >$$output 2>&1; then \
			echo "$$lab: broken.rs unexpectedly compiled"; \
			rm -f $$output $$output.rmeta; \
			exit 1; \
		fi; \
		if ! grep -q "$$expected" $$output; then \
			echo "$$lab: expected compiler error $$expected"; \
			cat $$output; \
			rm -f $$output $$output.rmeta; \
			exit 1; \
		fi; \
		if ! $(RUSTC) --edition=2021 --crate-type lib $$lab/fixed.rs --emit=metadata -o $$output.rmeta; then \
			echo "$$lab: fixed.rs did not compile"; \
			rm -f $$output $$output.rmeta; \
			exit 1; \
		fi; \
		rm -f $$output $$output.rmeta; \
		echo "$$lab: ok"; \
	done
