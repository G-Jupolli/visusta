# Integrating GPU

The current state of the project is that it is 100% CPU bound.  \
I want to implement pipelining and allowing use of the GPU would help here.

-# This is mostly a learning project and I want to learn some GPU / wgpu stuff anyways.

## Trait

I've been advised to use a trait that implements the filters.  \
In doing this, I can have a CPU & GPU ZSTs.   \
On startup, I can store these in a global var to dictate if the user has access  \
to a GPU, also useful if I bother to do a server implementation of this.

## Restructure & Docs

The processors being functions in main needs to be split up.  \
If I have separate implementations for CPU & GPU I want to make a unified dir  \
That is just the theory of all the filters.

The only real way to do this is to keep the file structure the same.
