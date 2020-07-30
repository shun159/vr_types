## Copyright 2020 Eishun Kondoh
## SPDX-License-Identifier: Apache-2.0

CC := gcc

SANDESH_SRC := $(wildcard sandesh/library/c/*.c)
SANDESH_SRC += $(wildcard sandesh/library/c/*/*.c)
SANDESH_SRC += $(wildcard gen-c/*.c)
SANDESH_SRC += dummy_handler.c
SANDESH_OBJ := $(patsubst %.c,%.o,$(SANDESH_SRC))
SANDESH_DEP := $(patsubst %.c,%.d,$(SANDESH_SRC))
SANDESH_PROG := $(patsubst %.c,%,$(SANDESH_SRC))
CFLAGS += -Isandesh/library/c
CFLAGS += -Isandesh/library/c/protocol
CFLAGS += -Isandesh/library/c/transport
CFLAGS += -I.
CFLAGS += -g -Wno-unused-but-set-variable
CFLAGS += -Wno-unused-parameter
CFLAGS += -Wno-unused-parameter
CFLAGS += -Wno-unused-function
CFLAGS += -Wno-strict-aliasing
CFLAGS += -Wno-sign-compare
CFLAGS += -Wno-format

BINDGEN_OPTS := -o src/vr_messages/vr_types_binding.rs \
                --no-layout-tests \
                --use-array-pointers-in-arguments \
                --generate-block -- \
                -I.

BINDING := gen-c/vr_types.h
BINDING_EXISTS := $(shell find -wholename $(BINDING))

all: sandesh

sandesh: generate-c
	@$(MAKE) build-sandesh

build-sandesh: $(SANDESH_DEP)
	@$(MAKE) $(SANDESH_OBJ)
	ar rcs gen-c/libvr_types.a $(SANDESH_OBJ)

%.d: %.c
	$(info GEN $@)
	@$(CC) -MM $(CFLAGS) $< | sed 's%\($*\)\.o[ :]*%\1.o $@ : %g' > $@

%: %.d

generate-c:
	./sandesh_idl --gen c priv/vr.sandesh

generate-bindgen:
	bindgen gen-c/vr_types.h $(BINDGEN_OPTS)

clean:
	@$(RM) ${SANDESH_DEP} $(SANDESH_OBJ) $(SANDESH_PROG)
	@$(RM) gen-c/*.c gen-c/*.h
