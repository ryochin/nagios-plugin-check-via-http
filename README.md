nagios plugin: check_via_http
=============================

Simple Nagios Plugin of remote HTTP Executor

Usage
-----

```sh
check_via_http -H localhost -p 5000 -u /check/something -q 'id=42' -q 'criteria=high'
```

```
Simple Nagios Plugin of remote HTTP Executor

USAGE:
    check_via_http [OPTIONS]

OPTIONS:
    -h, --help                   Print help information
    -H, --hostname <HOSTNAME>    HTTP hostname [default: localhost]
    -p, --port <PORT>            HTTP port [default: 80 on http, 443 on https]
    -q, --query <QUERY>          Queries (multiple times)
    -s, --ssl                    use HTTPS (https://)
    -t, --timeout <TIMEOUT>      Timeout [default: 15]
    -u, --uri <URI>              HTTP uri [default: /]
    -V, --version                Print version information
```

Server Side Spec
----------------

Your web server should return JSON as below:

```json
{
  "code": 0,
  "description": "everyhing is ok"
}
```

JSON schema

```json
{
  "$schema": "https://json-schema.org/draft/2019-09/schema",
  "type": "object",
  "required": ["code", "description"],
  "additionalProperties": false,
  "properties": {
    "code": {
      "type": "integer",
      "minimum": 0,
      "maximum": 3
    },
    "description": {
      "type": "string"
    }
  }
}
```

License
-------

The MIT License
