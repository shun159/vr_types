## Copyright 2020 Eishun Kondoh
## SPDX-License-Identifier: Apache-2.0

GENC_SRCDIR	= gen-c/
BINDGEN_OPTS = -o src/vr_messages/vr_types_binding.rs \
		 					 --no-layout-tests \
							 --use-array-pointers-in-arguments \
							 --generate-block -- \
						   -I.

all: compile-idl bind

compile-idl:
	./sandesh_idl --gen c priv/vr.sandesh

bind:
	bindgen $(wildcard $(GENC_SRCDIR)/*.h) $(BINDGEN_OPTS)
