# Security policy

## Demonstration scope

This repository contains an isolated demonstration of one ML-KEM-1024 engine.
It is not the Majax production implementation and it does not expose the Majax
service architecture. It contains no production key, token, credential, user
record, endpoint configuration or operational secret.

Successful execution proves only the properties checked by the included test
suite. It does not constitute an audit, certification or security assessment of
Majax or of any system using this demonstrator.

## Supported versions

Security corrections are applied to the latest stable release and the current
`development` revision. Older revisions are retained for traceability but are
not supported. The complete branch and release policy appears in
`MAINTENANCE.md`.

## Reporting a vulnerability

Please use the private security advisory feature of the GitHub repository. Do
not disclose a suspected vulnerability in a public issue before a correction is
available.

The project does not promise a fixed response time. Private security reports
receive priority over routine issues, and material corrections follow a
coordinated disclosure process appropriate to their impact.

Include the affected revision, the execution platform, the observed behaviour,
the expected behaviour and the smallest reproducible test case. Do not include
real credentials, production data or secrets in the report.

## Operational boundaries

The service binds to loopback by default. Public exposure requires a separate
TLS reverse proxy, explicit rate limiting and normal operational monitoring.

The private key and shared secrets exist only inside a short-lived execution.
The HTTP API does not return them and the demonstration does not persist them.

The supplied x86-64 binary is authenticated by the checksum recorded in
`CHECKSUMS.sha256`. Users who require an independently built artifact should use
the default source-based Docker build.
