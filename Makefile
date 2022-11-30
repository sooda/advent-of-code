# Because cargo would be so boring for the puzzles, let's learn to use plain rustc.

YEARS := $(shell seq 16 21)
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

TARGETS := $(foreach y,$(YEARS),$(patsubst %,$(y)/%,$(DAYS)))
OUTPUTS := $(patsubst %,%.out,$(TARGETS))

all: $(TARGETS)

run-all: $(OUTPUTS)

%: %.rs
	rustc $(OPTS) $(LIBS) -o $@ $<

# FIXME: use stdin instead
$(foreach y,16 17 18,$(patsubst %,$(y)/%.out,$(DAYS))): %.out: %
	$< $<.input | tee $@

$(foreach y,19 20 21,$(patsubst %,$(y)/%.out,$(DAYS))): %.out: %
	$< < $<.input | tee $@

# md5 for various 2016 days
16/5 16/14 16/17: %: %.rs
	rustc $(OPTS) $(LIBS) -L $(call libpath,rust-crypto) -o $@ $<

16/24: 16/24.rs
	rustc $(OPTS) $(LIBS) -L $(call libpath,permutohedron) -o $@ $<

17/21: 17/21.rs
	rustc $(OPTS) $(LIBS) --cfg 'csimode="fancy"' -o $@ $<

clean:
	rm -f $(TARGETS) $(OUTPUTS)
