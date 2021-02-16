# Atelier: crate atelier_describe

Provides the ability to write documentation for [Smithy](https://github.com/awslabs/smithy) models.

[![crates.io](https://img.shields.io/crates/v/atelier_openapi.svg)](https://crates.io/crates/atelier_describe)
[![docs.rs](https://docs.rs/atelier_openapi/badge.svg)](https://docs.rs/atelier_describe)

This crate provides two mechanisms for generating human-readable documentation for a Smithy model
using the crate [somedoc](https://crates.io/crates/somedoc).

Firstly, the `DocumentationWriter` structure implements the `atelier_core::io::ModelWriter` trait and so may be used 
in the same manner as other model writers. The `ModelWriter::new` function takes an argument that will denote the 
format to produce, but provides little other control over the generation. Internally this writer implementation calls 
the following function.

The function `describe_model` will produce a `somedoc::model::Document` instance from a `atelier_core::modrel::Model`. 
This instance may then be rendered according to the writers provided by somedoc. This provides complete control over 
the actual formatting step, and the same generated Document may be written multiple times if required.

# Examples

The following demonstrates how to use the `describe_model` function.

```rust
use atelier_core::model::Model;
# use atelier_core::Version;
use atelier_describe::describe_model;
use somedoc::write::{write_document_to_string, OutputFormat};
# fn make_model() -> Model { Model::new(Version::default()) }

let model = make_model();
let documentation = describe_model(&model).unwrap();

let doc_string = write_document_to_string(&documentation, OutputFormat::Html).unwrap();
```

The following example demonstrates the `ModelWriter` trait and outputs the documentation to
stdout.

```rust
use atelier_core::model::Model;
use atelier_core::io::ModelWriter;
# use atelier_core::Version;
use atelier_describe::{describe_model, DocumentationWriter};
use somedoc::write::{write_document_to_string, OutputFormat};
use std::io::stdout;
# fn make_model() -> Model { Model::new(Version::default()) }

let model = make_model();
let mut writer = DocumentationWriter::new(OutputFormat::Html);
let documentation = writer.write(&mut stdout(), &model).unwrap();
```

## Changes

**Version 0.1.0**

This initial version is reasonably usable, although incomplete.


## TODO

1. Complete both prelude, and custom, trait output.
1. Test cases.