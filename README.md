# aplis

A general purpose unikernel OS written in Idris2 (with minimal C). Only 64 bit UEFI systems are supported.


The main idea behind this project is to model system resources in a safe manner using the dependent and linear types of Idris2. All programs must be builtin to the OS at compile time. This simplifies the implementation of the OS in many ways - context switches, memory protection, etc. are all not needed, reuslting in a performant and safe system. There is no risk of an application misusing a device/resource or making a mistake in a protocol, as any invariants are encoded within the type system. There are still some open questions (mainly relating to handling mutliple cores), but the OS is mainly experimental, and so these will be tackled further down the line.


## Why Dependent and Linear Types?

Many of the APIs exposed by the raw drivers of an operating system must be used carefully in order to ensure that the resource is used correctly. For example, consider graphics drivers. It is likely before we can immediately start blitting pixels to the screen, we may need to search for a graphics card, enable the device, get/set its resolution etc. These operations need to be done in a specific order, and a program will crash if not done correctly. These stateful properties can easily be expressed in a dependently typed system. Linear typing is also exceptionally useful. It is notoriously awkward to safely express side-effectful functions in a pure language - unfortunately, an operating system which directly interacts with the hardware is inevitably going to involve side effects. With linear types, we can represent resources (e.g. the graphics framebuffer) as a linear resource, ensuring the resource is not copied or destroyed - operations on the resource invalidate (or consume) that "version" of the resource, and produce a new one. This model allows impure functions to be modeled safely.

## Potential Issues

- Idris2 performance - I don't have a clue how fast/memory efficient Idris2 actually is, hopefully its not unusably slow.
- GC - Writing an operating system in a language which uses garbage collection may result in noticable micropauses during usage
- Parallelism/Concurrency - Need a way to handle concurrency safely within the Idris code. Linear resources are still safely handled even with concurrency, so probably just need to implement some multiprocessing primitives in C and expose them safely in Idris code.
- Practicality - having to compile all programs in _might_ be a nightmare, perhaps the OS should provide a program which ovewrites itself with a new version. 


## Current Task

- Write memory allocator (Idris2 requires dynamic memory - malloc)
- Fork Idris2 to work on bare metal. Requires updated prelude without the majority of IO functions. Implement the stl functions which the Idris2 runtime uses.
