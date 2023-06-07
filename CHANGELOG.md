# Charcoal Changelog

### V0.1.1
Contains Breaking Changes
- Better support for future use in single threaded environments:
    - Now requires that a param is passed to join_channel to indicate whether to create a new job or use a pre-existing one
- Now Supports SASL Kafka Authentication