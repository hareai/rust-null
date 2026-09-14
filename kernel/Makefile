# SPDX-License-Identifier: GPL-2.0

KDIR ?= /lib/modules/$(shell uname -r)/build

default:
	$(MAKE) -C $(KDIR) M=$(CURDIR) modules

clean:
	$(MAKE) -C $(KDIR) M=$(CURDIR) clean

# Opt-in. Do not run on a machine you are not prepared to test.
load: default
	@if lsmod | grep -q '^rust_null '; then rmmod rust_null; fi
	insmod $(CURDIR)/rust_null.ko
	@test -e /dev/rust-null
	@ls -l /dev/rust-null

unload:
	@if lsmod | grep -q '^rust_null '; then rmmod rust_null; fi
	@test ! -e /dev/rust-null
