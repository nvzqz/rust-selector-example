# Rust `@selector` Example

This repository is my attempt at creating Objective-C selectors in Rust at
compile time using `#[link_section]`.

## Current Status

When trying to compile this current project, I get this error at link time:

```txt
0   0x1042679a4  __assert_rtn + 127
1   0x104268c35  mach_o::relocatable::PointerToCStringSection<x86_64>::targetCString(mach_o::relocatable::Atom<x86_64> const*, ld::IndirectBindingTable const&) const (.cold.2) + 0
2   0x10416595c  mach_o::relocatable::PointerToCStringSection<x86_64>::targetCString(mach_o::relocatable::Atom<x86_64> const*, ld::IndirectBindingTable const&) const + 154
3   0x1041657c2  mach_o::relocatable::PointerToCStringSection<x86>::contentHash(mach_o::relocatable::Atom<x86> const*, ld::IndirectBindingTable const&) const + 70
4   0x104164807  mach_o::relocatable::Atom<arm64>::contentHash(ld::IndirectBindingTable const&) const + 39
5   0x1041b403b  std::__1::__hash_iterator<std::__1::__hash_node<std::__1::__hash_value_type<ld::Atom const*, unsigned int>, void*>*> std::__1::__hash_table<std::__1::__hash_value_type<ld::Atom const*, unsigned int>, std::__1::__unordered_map_hasher<ld::Atom const*, std::__1::__hash_value_type<ld::Atom const*, unsigned int>, ld::tool::SymbolTable::ReferencesHashFuncs, true>, std::__1::__unordered_map_equal<ld::Atom const*, std::__1::__hash_value_type<ld::Atom const*, unsigned int>, ld::tool::SymbolTable::ReferencesHashFuncs, true>, std::__1::allocator<std::__1::__hash_value_type<ld::Atom const*, unsigned int> > >::find<ld::Atom const*>(ld::Atom const* const&) + 37
6   0x1041b01d4  ld::tool::SymbolTable::findSlotForReferences(ld::Atom const*, ld::Atom const**) + 130
7   0x1041b782c  ld::tool::Resolver::convertReferencesToIndirect(ld::Atom const&) + 190
8   0x1041b7509  ld::tool::Resolver::doAtom(ld::Atom const&) + 1123
9   0x10416242e  mach_o::relocatable::File<arm64>::forEachAtom(ld::File::AtomHandler&) const + 54
10  0x1041aba63  ld::tool::InputFiles::forEachInitialAtom(ld::File::AtomHandler&, ld::Internal&) + 521
11  0x1041ba640  ld::tool::Resolver::resolve() + 44
12  0x1041485fb  main + 799
13  0x7fff69d69cc9  start + 1

A linker snapshot was created at:
    /tmp/rust_selector_example-2020-08-07-034638.ld-snapshot

ld: Assertion failed: (0 && "unsupported reference to selector"), function targetCString, file /Library/Caches/com.apple.xbs/Sources/ld64/ld64-556.6/src/ld/parsers/macho_relocatable_file.cpp, line 6563.
```

You can see the snapshot at [`out/rust_selector_example.ld-snapshot`](https://github.com/nvzqz/rust-selector-example/blob/main/out/rust_selector_example.ld-snapshot).
