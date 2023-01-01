# Because cargo would be so boring for the puzzles, let's learn to use plain rustc.

THIS_DAY := $(shell date +%d)
#THIS_YEAR := $(patsubst %,23/%,$(shell seq $(THIS_DAY)))
YEARS := $(shell seq 16 22)
DAYS := $(shell seq 1 25)

OPTS := -g -O --edition=2021

# not that helpful during quick development,
# but good to periodically check the entire repo
#OPTS += --deny warnings

# git submodules could be good though, this was a quick hack years ago
LIBPATH := ${HOME}/programs/rustlibs/%/target/release/deps
libpath = $(patsubst %,$(LIBPATH),$(1))

# helpful in many puzzles to parse the input
LIBS := -L $(call libpath,regex)

TARGETS := $(foreach y,$(YEARS),$(patsubst %,$(y)/%,$(DAYS))) $(THIS_YEAR)
OUTPUTS := $(patsubst %,%.out,$(TARGETS))

MAKEFLAGS += --no-builtin-rules

all: $(TARGETS)

run-all: $(OUTPUTS)

%: %.rs
	rustc $(OPTS) $(LIBS) -o $@ $<

# FIXME: use stdin instead
$(patsubst %,%.out,$(filter 16/% 17/% 18/%,$(TARGETS))): %.out: % %.input
	$< $<.input 2>&1 | tee $@

$(patsubst %,%.out.sample,$(filter 16/% 17/% 18/%,$(TARGETS))): %.out.sample: % %.sample
	$< $<.sample 2>&1 | tee $@

$(patsubst %,%.out,$(filter-out 16/% 17/% 18/%,$(TARGETS))): %.out: % %.input
	$< < $<.input 2>&1 | tee $@

$(patsubst %,%.out.sample,$(filter-out 16/% 17/% 18/%,$(TARGETS))): %.out.sample: % %.sample
	$< < $<.sample 2>&1 | tee $@

# md5 for various 2016 days
16/5 16/14 16/17: %: %.rs
	rustc $(OPTS) $(LIBS) -L $(call libpath,rust-crypto) -o $@ $<

16/24: 16/24.rs
	rustc $(OPTS) $(LIBS) -L $(call libpath,permutohedron) -o $@ $<

17/21: 17/21.rs
	rustc $(OPTS) $(LIBS) --cfg 'csimode="fancy"' -o $@ $<

clean:
	rm -f $(TARGETS) $(OUTPUTS)
