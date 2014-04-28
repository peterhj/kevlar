ROOT    := $(abspath ..)
include $(ROOT)/config.mk

LIBKEVLAR := $(shell rustc --crate-file-name libkevlar/lib.rs)
TEST    := test

.PHONY: all clean

all: $(LIBKEVLAR) $(TEST)

clean:
	rm -f *.d
	rm -f $(LIBKEVLAR)
	rm -f $(TEST)

-include $(LIBKEVLAR).d
$(LIBKEVLAR): libkevlar/lib.rs
	$(RUSTC) $(RUST_FLAGS) --dep-info $@.d --out-dir . $<

$(TEST): test.rs $(LIBKEVLAR)
	$(RUSTC) $(RUST_FLAGS) $<
