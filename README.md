Axon is a demonstration machine learning feature store.

[_Axon_][wikipedia]: projection from a nerve cell that transmits signals.

[wikipedia]: https://wikipedia.org/wiki/Axon

### Implementation Notes

* The feature config JSON schema could be generted through the use of the
  [`schemars`][schemars] crate, but there appear to be some compromises involved
  in schema generation, especially for a type as flexible as the `Metadata`
  type, so currently the schema is being written by hand.

[schemars]: https://docs.rs/schemars/latest/schemars/
