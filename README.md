# Mesh

`pep-mesh` crate implements triangle mesh and half-edge mesh structure, which can be used in kinds of CG project.

This crate is now in progress and mainly aims to my personal usage.

## Features

* Triangle mesh
  * Vertex attributes with different type
  * Indices can be `u16` or `u32`
  * Cast vertex attribute to byte slice using [bytemuck](https://github.com/Lokathor/bytemuck) crate (with `bytemuck` feature)
* Half-edge mesh
  * Basic half-edge mesh
  * load from `.ply` and save to `.ply` using [ply-rs](https://github.com/Fluci/ply-rs/tree/master) crate
