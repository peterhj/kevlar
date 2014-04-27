ROOT    := $(abspath ..)
include $(ROOT)/config.mk

LIBKEVLAR := $(shell rustc --crate-file-name libkevlar/lib.rs)
KEVLARD := kevlard

.PHONY: all clean

all: $(LIBKEVLAR) $(KEVLARD)

clean:
	rm -f *.d
	rm -f $(LIBKEVLAR)
	rm -f $(KEVLARD)

-include $(LIBKEVLAR).d
$(LIBKEVLAR): libkevlar/lib.rs
	$(RUSTC) $(RUST_FLAGS) --dep-info $@.d --out-dir . $<

$(KEVLARD): kevlard.rs $(LIBKEVLAR)
	$(RUSTC) $(RUST_FLAGS) $<
